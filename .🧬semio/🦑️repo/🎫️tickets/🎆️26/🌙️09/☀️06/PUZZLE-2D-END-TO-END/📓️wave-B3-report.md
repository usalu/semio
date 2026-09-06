# Wave B3 — terminology (fail-closed) + reserved `import-media` route

All paths relative to `/Users/ueli/Documents/semio`. Nothing in this wave was compiled: the main session owns
every cargo run (see §5 for exactly what must be verified).

---

## 1. Fail-closed terminology

### 1.1 Label API — before / after

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`

| | before | after |
|---|---|---|
| locale parse | `is_de_locale(config) -> bool` = `config.locale.starts_with("de")`; everything else silently English | private `puzzle2d_locale(&str) -> Option<Locale>` matching **only** `"en" \| "en-US"` → `En`, `"de" \| "de-DE"` → `De` |
| locale accessor | — | `pub fn puzzle2d_config_locale(&Puzzle2dConfig) -> Option<Locale>` |
| labels | `pub fn puzzle2d_labels(&Puzzle2dConfig) -> &'static Puzzle2dLabels` (infallible; terminology `!= "reuse"` silently `Native`) | `pub fn puzzle2d_labels(&Puzzle2dConfig) -> Option<&'static Puzzle2dLabels>`, `Terminology::parse(...)?` |
| manifest matrix | `puzzle2d_localized` lived in the **editor** (`🦀️.rs`, old :2180) | moved into the terminology module; the editor re-exports it (`pub use …::{puzzle2d_localized, puzzle2d_localized_phrase};`) so `crate::editor::puzzle2d::puzzle2d_localized` keeps resolving for the window modules |
| phrase helper | absent | `pub fn puzzle2d_localized_phrase(field, en, de) -> LocalizedLabel` — mirrors `puzzle3d_localized_phrase` (3d `🗣️terminology/🦀️.rs:129-137`) |
| `is_de_locale` | public | **removed** (only caller was `context_menu`) |
| test | none | `label_resolution_has_no_locale_or_terminology_default` — 4 supported cells are `Some`; `""`, `"fr"`, `"de-AT"`, `"en-GB"` are `None`; terminology `"legacy"` is `None` |

### 1.2 New label fields (for B1)

Two fields added at the end of the `app_labels!` block, all four locale×terminology cells filled with the
repo-canonical wording (`🧰️framework/…/🌉️mcp/🧫️fixtures/🦀️.rs:135-136`, `🧊️3d/…/✏️editor/🦀️.rs:7217-7218`):

```
set_locale:      native_en "Set Locale",      native_de "Sprache festlegen",      reuse_en "Set Locale",      reuse_de "Sprache festlegen";
set_terminology: native_en "Set Terminology", native_de "Terminologie festlegen", reuse_en "Set Terminology", reuse_de "Terminologie festlegen";
```

**B1**: `setLocale` / `setTerminology` `ActionDefinition`s can now use
`puzzle2d_localized(|l| l.set_locale)` / `puzzle2d_localized(|l| l.set_terminology)` instead of an inline
`LocalizedLabel::native(...)`. Both helpers are in scope in the editor file already (re-exported at the
`use` block, line ~28-29).

### 1.3 Every caller touched

| file | site | change |
|---|---|---|
| `…/◻️2d/…/✏️editor/🦀️.rs` | `use` block (~:28-29) | `use …terminology::{puzzle2d_config_locale, puzzle2d_labels};` + `pub use …terminology::{puzzle2d_localized, puzzle2d_localized_phrase};`; dropped `is_de_locale`, `Puzzle2dLabels` (now unused in this file), `MediaError` |
| same | `render` | `puzzle2d_labels(config).ok_or_else(\|\| PluginAssemblyError::new("ui.localization.unsupported", "puzzle2d locale or terminology is not recognized"))?` — identical to 3d `🦀️.rs:7013` |
| same | `window_engagements` | `let Some(labels) = … else { return HashMap::new(); }` (3d `:7032`) |
| same | `window_measures` | same (3d `:7050`) |
| same | `tool_measures` | same (3d `:7068`) |
| same | `context_menu` | `let Some(locale) = puzzle2d_config_locale(config) else { return Vec::new(); }; let is_de = locale == semio_framework_plugin::Locale::De;` (3d `:7084` returns `Vec::new()` too) |
| same | `create_puzzle2d_app` | `.expect("default puzzle2d locale and terminology axes are explicit")` — `Puzzle2dConfig::default()` is `locale: "en-US"`, `terminology: "native"` (`🎚️config/🦀️.rs:37-43`), same as 3d `:9776` |
| `…/✏️editor/🎭️modes/✏️edit/☑️options/🖌️brush/🦀️.rs:151` | test | `.expect(...)` |
| `…/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:104,123,134` | tests | `.expect(...)` (the two `de-DE` cases keep proving German output) |
| `…/✏️editor/🎚️config/🦀️.rs:242-243` | doc | stale reference to the deleted `is_de_locale` replaced with the fail-closed contract |

No English fallback path remains: `grep -rn "is_de_locale" ✏️s/🔌️plugins/🧩️puzzle/` returns nothing.

---

## 2. Reserved `import-media` route

### 2.1 Evidence gathered before deciding

1. **The old docstring was factually wrong.** It claimed `kit.catalog` is block3d's object/vortex vocabulary
   and therefore unmappable. The 2d-dimension producer of `kit.catalog` is
   `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/…/✏️editor/🦀️.rs:555-570` (`Block2dPlayApp::export_media("catalog:out")`),
   whose payload is `crate::artifacts::block2d::schema::inferences::puzzle2d_manifest_fragment`
   (`…/◻️2d/…/🧬️schema/💡️inferences/🦀️.rs:68-91`) — literally a **puzzle2d manifest fragment**
   (`{schema:"manifest", id, name, axes, portKinds, wireKinds, edgeKinds, nodeKinds, kindCompatibility}`),
   documented there as *"the seam puzzle imports through its `Kit×Type` media port"*. So `kit:in` is honest
   and stays; it is a 2d↔2d vocabulary.
2. **`document:in` is not honest for puzzle2d.** The framework default needs
   `whole_document_operation` (`🧰️framework/…/🔌️plugin/🦀️.rs:11577-11599`); puzzle2d's 26-kind
   `Puzzle2dMutation` enum has no whole-snapshot variant, so no override was added. Any port other than
   `kit:in` now faults with an explicit message instead of a bare `NotImplemented`.
3. **`ArtifactApp::import_media` is dead code in the runtime.** `dispatch_import_media`
   (`🧰️framework/…/🔌️plugin/🦀️.rs:23336-23359`) routes **only** through
   `build_artifact_reserved_media_job` → `A::build_reserved_tool_job`. Nothing in the framework or `✏️s/`
   calls `ArtifactApp::import_media` outside its own default body and the `EditorApp<E>` forward. The
   puzzle2d override (`Err(MediaError::NotImplemented)`) was therefore **deleted**, not rewritten — keeping
   it would have been a second, unbounded, unreachable copy of the same mapping.
4. **`copy`/`cut`/`paste` need no app-owned factory.** `register_framework_reserved_factories`
   (`🧰️framework/…/🔌️plugin/🦀️.rs:14664-14686`) registers `FrameworkCopyJobFactory`/`Cut`/`Paste` for
   **every** `ArtifactApp`; puzzle3d owns none and is not flagged. Notably
   `FrameworkImportMediaJobFactory` is defined (`:14487`) but **never registered** — which is exactly why
   `import-media` must be app-owned, and why puzzle2d's media route was hard-dead
   (`qualified_tool_proof("import-media")` had nothing to resolve). Minimum honest scope = `import-media` only.
5. `toolJobPuzzleReservedRoutesExact` (`📜️script.ts:4849`) is puzzle5d-only and hardcodes the 5d path — it
   does not see 2d, so no new assertion is owed there.

### 2.2 What was implemented

New region `//#region 🧵️ReservedJobs` … `//#endregion 🧵️ReservedJobs` in
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`,
inserted **between `//#endregion 🧵️RetainedCommands` and `impl ArtifactEditor for Puzzle2dPlayApp`**
(lines **2878-3531** at the time of writing; the file is being edited concurrently, so treat the region
markers, not the numbers, as the boundary).

Contents:

* route constants — `PUZZLE2D_IMPORT_RAW_BYTES = 65_536`, `PUZZLE2D_IMPORT_SEMANTIC_ITEMS = 64` (per-collection
  cap), `PUZZLE2D_IMPORT_DECODED_ITEMS = 4_096`, `PUZZLE2D_IMPORT_WORK_UNITS = 4_096`,
  `PUZZLE2D_IMPORT_OUTPUT_BYTES = 1_048_576`, `PUZZLE2D_IMPORT_MUTATION_ITEMS = 65`,
  `PUZZLE2D_IMPORT_TOOL_ID`, `PUZZLE2D_IMPORT_PORT` (now also the id used by `io()`'s `kit:in` port spec),
  `PUZZLE2D_IMPORT_PAYLOAD_SCHEMA = "puzzle.2d.reserved.import-media.v1"`, plus the exact root-key /
  collection allow-lists.
* `struct Puzzle2dImportJobFactory` + `impl ToolJobFactory` (contract
  `resumable(65_536, 4_096, 4_096, 1_048_576, 7_500, 1, 1)`, `Migrated`) + `impl ArtifactOwnedToolJobFactory`
  (`TOOL_IDS = ["import-media"]`, `DOCUMENT_SCHEMA = PUZZLE2D_FIXTURE_SCHEMA`, publication lane `Artifact`).
  Mirrors `puzzle5d_reserved_factory!` (5d `🦀️.rs:122-166`) without the macro — one route, so a macro would
  only obscure it.
* job plumbing: `puzzle2d_job_payload`, `puzzle2d_job_fault`, `puzzle2d_import_checkpoint` (25-byte
  stage/cursor/decoded-items/progress state), `puzzle2d_retire_string_step`, `puzzle2d_retire_vec_backing`.
* row normalizers `puzzle2d_import_{handle,node,edge,wire}_kind`, `puzzle2d_import_handle_template`,
  `puzzle2d_upsert_catalog_row` (id-keyed, order-independent, reports whether the bundle actually changed —
  so an idempotent re-delivery on a `multiplicity: Many` port emits nothing).
* `enum Puzzle2dImportStage { Decode, HandleKinds, NodeKinds, EdgeKinds, WireKinds, Compatibility, CatalogMutation, Complete }`
  and `struct Puzzle2dImportJob` + `impl InteractiveJob` + `impl ArtifactReservedJob`.

**Mapping** (`kit.catalog` manifest fragment → `Puzzle2dKindCatalogs` / `meta.kindCompatibility`):

| fragment | document | notes |
|---|---|---|
| `portKinds[]` `{id, name, presentation:{color, defaultWireKind}}` | `catalogs.handles` (`Puzzle2dCatalogHandleKind`) | `name` → `label` (the row type has no `name`) |
| `nodeKinds[]` `{id, name, presentation:{meshUrl, handles:[{handleKind, angle, radius}]}}` | `catalogs.nodes` (`Puzzle2dCatalogNodeKind`) | each template → `Puzzle2dHandleTemplate`, synthesising `id = "{nodeKind}-h{index}"` when the producer omits one |
| `edgeKinds[]` | `catalogs.edges` | |
| `wireKinds[]` | `catalogs.wires` | `presentation.defaultEdgeKind` → `default_edge_kind` |
| `kindCompatibility[]` | `meta.kindCompatibility` | parsed with `dsl::FromValue` (so `specificity` goes through the real `Puzzle2dCompatSpecificity` scalar), emitted as `connect-kind-compatibility` |

**Emitted mutations** (lane `Artifact`): one `connect-kind-compatibility` per new-or-changed relation row,
plus at most one `replace-kind-catalogs` carrying the merged bundle. Published through
`ArtifactToolCompletion::complete(Ok(Emit { artifact_mutations, ui_scope: UiDirtyScope::Full, .. }))` — never a
direct state write, so a headless import is exactly as undoable/syncable as a UI edit.

**Bounding / resumability / cancellation**: `cx.is_cancelled()` first in every `step`; one row per step;
`cx.consume_fuel(1)` + `StepOutcome::CheckpointReady(...)` on every non-terminal step; `Decode` refuses an
unknown root key, a non-`manifest` `schema`, a non-array collection, a collection over 64 rows, and a
fragment+document item total over 4 096 before any mapping happens (admission strictly before decode).
`ArtifactReservedJob::close_step` retires the pending rejection, mutations, relations, all four catalog
slices, the fragment, `media_json`, `port`, the snapshot `Arc` and the completion authority one bounded item
at a time, with an exact `terminal_is_empty` witness (empty **and** zero-capacity on every owned `Vec`/`String`).

### 2.3 The one-line registration + the override — exact anchors for B1/B2

Both are inside `impl ArtifactEditor for Puzzle2dPlayApp`:

```rust
    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Puzzle2dImportJobFactory::new(&controller))?;   // ← B3's ONLY line here
        registry.register(Puzzle2dRetainedCommandJobFactory::new(&controller))
    }
```

and, immediately **before `fn render(`** (where `import_media` used to sit):

```rust
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> { … }
```

Everything else B3 touched in this file is (a) the `use` block lines ~28-29 and the
`semio_framework_plugin::{…}` list (dropped `MediaError`), (b) the `🧵️ReservedJobs` region, (c) the five
`puzzle2d_labels`/`context_menu` call sites, (d) the `io()` docstring + its `kit:in` port id, (e) deletion of
`fn import_media` and of the stray doc-comment that sat on `bounded_first_step_tool_proofs!`.
**B1/B2: do not re-add `fn import_media`, and leave the `🧵️ReservedJobs` region alone.**

### 2.4 Inventory JSON

`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8yj-importer-cohorts.json`,
`Puzzle2dPlayApp` row only (35 other rows byte-identical):

* `file`: `…/◻2d/…/🦀️component.rs` (non-existent — note the missing variation selector on `◻`) →
  `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
* `currentMonolith`: `true` → `false`
* `ports`: `[]` → `["kit:in"]`
* `input`: `port + Media(media-envelope) + …` → `port + Media(structured-json) + immutable snapshot` (matches 5d)
* `workBound`: `8388608/8192/4096/1` → `65536/4096/4096/1` (the factory's real contract, as 5d's row does)

### 2.5 Gate result

`bun ./📜️script.ts verify interactivity tool-jobs` **cannot run right now** — it aborts in
`toolJobScalarConfigCohortSelfTests` on a missing, unrelated file
(`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, another session's
in-flight tree) long before the importer census. So the gate's own predicate
(`📜️script.ts:5213-5236`) was replayed faithfully by
`🗑️generated/b3/check_importer_gate.py` (brace-matched `impl ArtifactEditor` block, `#[cfg(test)]` stripping,
the same eight substring checks) — **all eight pass, `failures: none`**, and `currentMonolith: false` is now
consistent with `migrated == true`.

---

## 3. Hand-offs

* **B1 — `🔏️publication-authority/🔣️.json`**: puzzle2d now owns a second registered factory publishing on the
  `Artifact` lane. 5d's row lists `"import-media"` in its `Migrated`/`["Artifact"]` group and the TS oracle
  filters it out of the source comparison via the `reserved5d` set
  (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts:77`). Do the same for 2d: add `"import-media"`
  to the `Migrated`/`["Artifact"]` group **and** an analogous `reserved2d` filter in `ownerOracle`, otherwise
  `exactArray([...pairs.keys()], appRoutes)` will fail (there is no `.action_interactive_job("import-media")`).
* **B1 — TS oracle**: the `owner.owner === "Puzzle2dPlayApp"` early-return branch
  (`📜️script.ts:85-89` of the puzzle TS script) still asserts puzzle2d has **no** proofs macro — already false
  before B3 and still false; that rewrite is yours.
* **B1 — actions**: `l.set_locale` / `l.set_terminology` are available (§1.2).
* **B2**: your `build_tool_job` arms and Work impls are untouched; the new region sits *above*
  `impl ArtifactEditor`, not inside it.

## 4. Files changed by B3

1. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs`
2. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
3. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs` (docstring only)
4. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🖌️brush/🦀️.rs`
5. `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`
6. `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/📊️p8yj-importer-cohorts.json`
7. `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🗑️generated/b3/check_importer_gate.py` (gate replay)

`rustfmt --edition 2021 --check` is clean for every line B3 wrote (the remaining diffs in the editor file are
pre-existing: the `semio_framework_plugin::{…}` import wrap, `board_kind_catalogs_json`,
`document_board_kind_catalogs_json`, `io()`'s over-indented `with_ports`, and peer-owned regions).

## 5. What the main session must compile-verify

1. `cargo check -p semio-s-plugin-puzzle` — the new region uses framework APIs that were read but never
   compiled here. Highest-risk spots, in order:
   * borrow ordering in `Puzzle2dImportJob::step` (`Complete` takes `self.mutations` **before** borrowing
     `self.completion`, and each catalog stage binds `let admitted = …;` before the `match` so the `self.rows`
     borrow ends at the `;`) — mirrors 5d, but never compiled;
   * the exact field sets of `Puzzle2dCatalogNodeKind` / `…HandleKind` / `…EdgeKind` / `…WireKind` /
     `Puzzle2dHandleTemplate` in the five struct literals;
   * `dsl::DslValue::from(&serde_json::Value)` + `dsl::FromValue` for `Puzzle2dKindCatalogs` /
     `Puzzle2dKindCompatibility`;
   * `Emit { .., ..Default::default() }` inference at the `completion.complete(...)` call.
2. `cargo test -p semio-s-plugin-puzzle` — the new
   `terminology::tests::label_resolution_has_no_locale_or_terminology_default`, plus the three fill/brush tests
   that now `.expect(...)`.
3. `--target wasm32-wasip2` build — the whole region is `#[cfg]`-free, but the app registration path changed.
4. Runtime: with a `dev 2d` boot, an unsupported locale must render nothing rather than English, and a
   `kit:in` delivery from block2d's `catalog:out` must produce a `replace-kind-catalogs` (+ any
   `connect-kind-compatibility`) edit that appears in history and undoes cleanly.
