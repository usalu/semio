# 🐛️ Root cause — puzzle 2D brush and fill never work in the running app

One naming split between the puzzle2d **document** contract and the board **engine** contract breaks
both of the features this ticket is about. Every unit test passes anyway, because the tests
hand-build engine-shaped catalogs or translate on the spot — the production path is the only place
the translation is missing.

## The two contracts

**Document** — `Puzzle2dKindCatalogs` (`🗿️artifacts/◻2d/🦀️component.rs:340-356`,
`#[serde(rename_all = "camelCase")]`) serializes exactly four slices:

```
nodes · handles · edges · wires
```

The JSON Schema agrees and is closed: `Puzzle2dKindCatalogs` in
`…/🧬️schema/🔣️component.json` lists `required: [nodes, handles, edges, wires]` with
`additionalProperties: false`, so `nodeKinds` is not merely absent — it is **forbidden** on a
document.

**Engine** — `BoardHost::set_board_kind_catalogs_from_json`
(`🧰️framework/…/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️component.rs:4223`) reads only:

```
handleKinds · wireKinds · nodeKinds · edgeKinds · edgeTips
```

and it rejects any row that still carries the document's `label` field
(`reject_kind_catalog_row_legacy_label`, line 4215).

## Bug A — brush has zero candidates

`sync_host_fixture_content` (`…/✏️editor/🦀️component.rs:551`) pushed the document catalogs into the
engine **verbatim**, with no key translation and no row projection:

```rust
if let Some(catalogs) = envelope.fixture.get("meta").and_then(|v| v.get("kindCatalogs")) {
    if let Ok(json) = serde_json::to_string(catalogs) {
        let _ = host.set_board_kind_catalogs_from_json(&json);   // ← error discarded
    }
}
```

The engine finds none of its expected keys, so `host.node_kinds` stays **empty**. And
`brush_compatible_candidates` (engine line 5257) is:

```rust
for (kind_id, kind) in &self.node_kinds { … }
```

An empty map means the loop body never executes ⇒ **no brush candidate is ever produced, so brush
can never place anything.** The `let _ =` discarded the very error that would have revealed this.

There is no other production path that installs catalogs: the only correctly-shaped builder,
`catalogs_json_from_manifest_id` (`…/✏️editor/⚙️engine/🎲️board-host/🦀️component.rs:48`), lives inside
`#[cfg(test)] pub(crate) mod testkit` and has exactly one caller — a brush test.

**Why the tests stayed green:** the brush suite builds engine-shaped literals directly
(`"nodeKinds": [{…}]`, many sites), and the one test that starts from a real document translates by
hand — `…/⚙️engine/🖌️brush/🦀️component.rs:1210-1213`:

```rust
serde_json::json!({
    "handleKinds": kc.get("handles"),
    "nodeKinds":   kc.get("nodes"),      // ← the translation production lacked
})
```

That line is the bug's own confession: the test author had to convert, production did not.

## Bug B — fill faults immediately

`…/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:232` read the wrong slice **off the document**:

```rust
document.get("meta").…get("kindCatalogs").…get("nodeKinds")   // never exists on a document
    .ok_or("puzzle2d-fill-capture-node-kinds")
```

Since documents carry `nodes`, this always returned `Err`, so fill's capture stage failed on every
run and the job went straight to `Puzzle2dFillLifecycle::Faulted` with fault code
`puzzle2d-fill-capture-node-kinds`.

## Fixes applied

1. **New owned translation** — `board_kind_catalogs_json(fixture) -> Option<String>` in
   `…/✏️editor/🦀️component.rs`, next to `kind_catalog_entries`. It maps
   `nodes→nodeKinds`, `handles→handleKinds`, `edges→edgeKinds`, `wires→wireKinds` and **projects each
   row down to the keys the engine reads**, which also drops the document's `label` (whose presence
   would otherwise make the engine reject the row outright). Node-kind handle templates are narrowed
   to `handleKind`/`angle`/`radius`, and templates without a `handleKind` are skipped because the
   engine requires it.
2. **Wired into production** — `sync_host_fixture_content` now pushes that translated JSON.
3. **Fill slice corrected** — `set-fill-count`'s `node_kinds()` now reads `kindCatalogs.nodes`.
   Verified shape-compatible with its consumer `capture_kind_one` (line 451), which reads
   `id`/`shape`/`scale`/`icon`/`handles`; the document node-kind row requires `id`, `icon` and
   `handles`, and the absent `shape`/`scale` correctly fall back to circle / 1.0.

## Status of verification

Applied and reasoned against the schema, the Rust struct and the engine parser — **not yet compiled
or exercised at runtime**, because the shared cargo target-dir lock is saturated by concurrent
sessions. Compilation and a runtime check are still required before this is called done; the
regression test named below is the durable guard.

**Regression guard to add:** a test that starts from a *document-shaped* catalog (including `label`
fields, as real documents have), runs `board_kind_catalogs_json`, pushes the result into a real
`BoardHost`, and asserts `set_board_kind_catalogs_from_json` returns `Ok` **and** that the host then
yields a non-empty brush candidate page. Asserting only on the JSON keys would not have caught the
`label` rejection.

## Addendum — the browser-side path had the same defect

The plugin-side `BoardHost` is only one of **two** engines that need catalogs. The board the user
actually sees is the WASM canvas session in the browser, which `Board2dHost` feeds via
`session.setKindCatalogsJson(scene.glyphCatalogsJson)` — and `setKindCatalogsJson`
(`…/✏️editor/🌉️wasm/🦀️component.rs:224`) calls the very same `set_board_kind_catalogs_from_json`.

`puzzle2d_board_scene` (`…/✏️editor/🎭️modes/✏️edit/🦀️component.rs:121`) built that payload the same
untranslated way:

```rust
let glyph_catalogs_json = fixture.get("meta").and_then(|v| v.get("kindCatalogs"))
    .map_or_else(|| "{}".into(), |value| value.to_string());
```

So even with the plugin-side host fixed, the on-screen canvas would still have had empty
`node_kinds` — no brush preview, no candidates. Now routed through `board_kind_catalogs_json` as
well. **Both engines had to be fixed; fixing either alone would have looked like "still broken".**

### Complete fix set

| # | File | Change |
| --- | --- | --- |
| 1 | `…/✏️editor/🦀️component.rs` | new `board_kind_catalogs_json` — document→engine key mapping + per-row field projection |
| 2 | `…/✏️editor/🦀️component.rs` (`sync_host_fixture_content`) | plugin-side host now receives the translated catalogs |
| 3 | `…/✏️editor/🎭️modes/✏️edit/🦀️component.rs:121` | browser-side `glyph_catalogs_json` now translated too |
| 4 | `…/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs:232` | fill capture reads `kindCatalogs.nodes`, not the non-existent `nodeKinds` |
| 5 | `…/✏️editor/⚙️engine/🖌️brush/🦀️component.rs` | regression test `document_kind_catalogs_translate_into_engine_brush_candidates` |

### About the regression test

It is a **before/after behavioural** guard, not a key-shape assertion. The first draft asserted that
pushing the raw document catalogs returns `Err` — that was wrong, and worth recording: the engine's
`label` rejection only runs on rows found under keys it recognises, so raw document catalogs return
`Ok(())` while installing **nothing**. Silent success is exactly what made this bug survive. The
test therefore drives a real pointer gesture over a free handle on two hosts and asserts
`brushPreview` is absent with the raw catalogs and present with the translated ones.

### Related, deliberately out of scope

`puzzle5d`'s 2d window (`🗿️artifacts/🖐️5d/…/🪟️windows/◻2d/🦀️component.rs:107`) has
`board_kind_catalogs_value`, which maps 5d's own `parts/grips/fasteners/ropes` naming onto the
**document** `nodes/handles/edges/wires` naming — it is not a document→engine translation, so 5d
feeds `glyph_catalogs_json` in the same untranslated document shape and is very likely affected by
the identical defect. Not touched here: this ticket is puzzle 2d, and 5d is being actively worked in
`26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`. Flagged for whoever owns that.

### Verification state

- `rustfmt --check` parses all four edited files with no syntax error (parse-level only).
- `cargo check -p semio-s-plugin-puzzle --lib` is queued but **starved**: 85 cargo / 33 rustc
  processes from concurrent sessions, my process holding 0.83 s CPU over many minutes on the shared
  target-dir lock. **Type-checking and the runtime check are still outstanding.**

## Correction — the document catalogs are usually *absent*, and the manifest is the real source

The translation above is necessary but was **not sufficient**, and the first version of this document
overstated it. Checking what the shipped documents actually contain:

```
📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio   kind-catalogs: 0   manifest-id=concrete-forest
📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio  kind-catalogs: 0   manifest-id=nakagin
```

**Neither real example carries `meta.kindCatalogs` at all.** Both carry `meta.manifestId`. So for the
documents the dev app actually loads, `board_kind_catalogs_json` would have returned `None` and the
engine's `node_kinds` would still have been empty — brush and fill would still have been dead.

Their catalogs live in the compile-time manifest registry, and the manifests are rich enough for
brush: `🦀️nakagin.rs`'s `nodeKinds` carry `presentation.handles` templates
(`{"handleKind":"door capsule right","angle":-1.5707963267948966,"radius":3}`) and its `portKinds`
carry `presentation.color` + `defaultWireKind` — exactly the `NodeKindHandleTemplate` and
`HandleKindDef` data the engine wants.

The only code that read them was, again, the **test-only** `catalogs_json_from_manifest_id`. So the
real production gap was: *nothing ever resolves a document's manifest into engine catalogs.*

Added `manifest_board_kind_catalogs_json(manifest_id)` as production code alongside the translation,
and `board_kind_catalogs_json` now resolves in this order:

1. `meta.kindCatalogs` when the document carries its own → translate it;
2. otherwise `meta.manifestId` → build from `graph::manifest::manifest_by_id`.

Each manifest row becomes `id` + `name` + its flattened `presentation`. Port kinds lacking a
`presentation.color` are filtered out, because the engine errors with `HandleKindColorMissing` on a
colourless handle kind — and since the whole call is all-or-nothing, one such row would discard the
entire catalog push (nakagin's `Connector` port kind is exactly such a row).

Checked before widening the payload beyond the proven test helper's `handleKinds`+`nodeKinds`: the
engine rejects any row carrying `label`, and the only `label` in the three shipped manifests is a
*property name* inside `properties`, which is never copied onto a row — only `presentation` is
merged. So emitting `wireKinds` and `edgeKinds` as well is safe for the shipped manifests.

`graph` is a normal `[dependencies]` entry of the puzzle crate (Cargo.toml:86), so this needed no new
dependency.

### Final resolution order

`board_kind_catalogs_json` resolves document-first, manifest-second, and the document half
(`document_board_kind_catalogs_json`) deliberately returns `None` unless it contributes at least one
node kind:

```rust
meta.and_then(|meta| meta.get("kindCatalogs"))
    .and_then(document_board_kind_catalogs_json)
    .or_else(|| meta.and_then(|meta| meta.get("manifestId")).and_then(Value::as_str).and_then(manifest_board_kind_catalogs_json))
```

Without that guard a document carrying an *empty* catalog bundle — which is what
`Puzzle2dKindCatalogs::default()` serializes to, and the schema requires all four slices to be
present once `kindCatalogs` exists at all — would short-circuit the manifest and then actively
**clear** the engine's catalogs, reintroducing the same dead brush by a different route.

`default_empty_fixture()` carries no `meta` at all, so it resolves to `None` and pushes nothing,
exactly as before the change — no behaviour change for the many unit tests built on it.

## Second production bug: 35 unclassified interactive commands

Once the crate compiled and the component materialized, the wasm **panicked during descriptor
extraction** — which is what actually stops the app loading:

```
panicked at 🧰️framework/…/🔌️plugin/🦀️component.rs:5716:64:
app-definition.interactive-job-classification: unclassified interactive command
  '2d-detail:applyBoardEvents'; '2d-detail:brushCancelSlot'; '2d-detail:brushCommitSlot';
  '2d-detail:brushCycleCandidate'; '2d-detail:brushFillSessionAdopt'; …
```

140 complaints = **35 distinct commands × 4 scopes** (the three window kinds `2d-overview`,
`2d-detail`, `2d-selection`, plus the app id itself).

`AppDefinition::build_definition` now requires every interactive command to carry an
`InteractiveJobClassification`. puzzle2d declared exactly three — `addNode`, `forceLayout`,
`setActiveExample` — while its `puzzle2d_command_variants!` block declares 43. The other 35 were
never classified, so assembling the app definition panics before any UI exists.

Every command this ticket cares about was in that list: `applyBoardEvents` (the pointer→mutation
path), the whole `brush*` family (`brushOpenSlot`, `brushCommitSlot`, `brushCancelSlot`,
`brushCycleCandidate`, `brushSetCandidateIndex`), the whole fill family
(`brushFillSessionBegin/Step/Adopt/Cancel/Retry/Discard/Clear`, `setFillCount`), and the brush
options (`setBrushKindWeights`, `setBrushNodeSize`, `setSuggestionOffset`).

**Fix:** all 35 classified `BatchOnlyPendingRewrite`, matching the three puzzle2d already declared.
That is the correct classification here rather than `Migrated`: `Migrated` obliges the artifact to
register a tool-job factory and list the id in `PUZZLE2D_RETAINED_TOOL_IDS`, and puzzle2d's is
deliberately empty (`✏️editor/🦀️component.rs:958`) with no `ArtifactOwnedToolJobFactory` — a fact the
repo's own `verify interactivity tool-jobs` gate asserts. Declaring `Migrated` without factories
would trade a runtime panic for a gate failure.

Crate re-checks clean for wasm32 after the change.
