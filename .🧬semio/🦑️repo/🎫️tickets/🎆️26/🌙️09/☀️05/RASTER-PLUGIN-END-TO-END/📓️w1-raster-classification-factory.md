# 📓️ W1 — raster's editor action surface is dispatchable

Slice: `RasterPlayApp` / `create_raster_app()` — the `Migrated` classifications, the exact-owner
bounded-first-step proof block, `RasterRetainedCommandJobFactory`, the two store preparations, the
law test, and `🔏️publication-authority`. Untouched by design: W2's `initial_snapshot` / examples /
the `🎬️set-active-example` handler body, W3's `🚪️io/**` and the per-format artifact leaves, W4's
framework-API drift block (`ui_label`, `raster_measure_action`, the panel and window leaves).

This is the second W1 run. The first was killed by a rate limit at ~01:30 and its work was
auto-committed in `5e03e56997`: the factory, the store preparations, the proof block with
`factory_type:`, the sixteen `.action_interactive_job(.., Migrated)` rows, the
`retained_route_dispositions_are_exact_and_exhaustive` law test, and
`🔏️publication-authority/{🔣️.json,🧬️.schema.json}`. This run finished the one route that was
still missing — W2's `setActiveExample` — plus the schema/taxonomy questions and a stronger test.

## 1 — `setActiveExample` was a handler with no dispatch path

W2 authored `🎮️commands/🎬️set-active-example/🦀️.rs` (a `SetActiveExample { example_id }` payload
whose `handle` returns `Emit::mutations(..)`), but nothing referenced it: the module was not mounted,
there was no `RasterCommand` row, no retained id, no publication contract, no proof, no manifest
action and no classification. A `setActiveExample` action dispatched from the navbar switcher would
have been rejected outright.

Wired end to end, in the order the framework joins them:

| file:line | change |
| --- | --- |
| `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/🦀️.rs:596-597` | `pub mod set_active_example;` mount, `#[path]` to the leaf, alphabetically before `set_active_utility` |
| `…/✏️editor/🦀️.rs:215` | `app_commands!` row `"setActiveExample" as "set-active-example" => set_active_example::SetActiveExample`, **appended last** so no earlier binary ordinal moves |
| `…/✏️editor/🦀️.rs:221` | flat `use` of the payload module |
| `…/✏️editor/🦀️.rs:251` | `RASTER_RETAINED_TOOL_IDS` 16 → 17 |
| `…/✏️editor/🦀️.rs:288` | `ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[Artifact] }` |
| `…/✏️editor/🦀️.rs:698` | `bounded_first_step_tool_proofs!` `tools:` list 16 → 17 |
| `…/✏️editor/🦀️.rs:973` | `.mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))` |
| `…/✏️editor/🦀️.rs:1044` | `.action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)` |

Doc comments at `:230-233`, `:255-266`, `:966-969` and `:1580-1585` were rewritten to match (nine
document verbs → ten, "sixteen ids" → "seventeen", and the stale "whole-document replace is gone"
note, which was true only between `26/08/12` and this ticket).

### Decisions

- **Artifact lane, not Config.** W2's handler emits `Emit::mutations(..)` over `RasterMutation` —
  raster has no whole-document replace verb, so loading an example is spelled as real, undoable
  operations (delete every root layer, re-point the asset pool, plant the example forest). Lane is
  read off the handler, exactly as the other sixteen rows are. Puzzle3d's `setActiveExample` declares
  `[Artifact, Config]` because its handler also writes config; raster's does not.
- **`.mutation(..)`, not `raster_internal_action(..)`.** Both block2d (`:625`) and puzzle3d (`:7191`)
  declare it as a palette-visible mutation with the identical `LocalizedLabel::native("Set Active
  Example", "Aktives Beispiel festlegen")`. Copied verbatim rather than inventing a raster-only
  spelling.
- **Appended, never inserted.** `app_commands!` row order is the binary variant ordinal. Appending
  keeps `set-layer-visible` at `0102`, which `optional_field_rows_keep_their_declared_wire_bytes`
  pins to exact bytes.
- **Wire keyword `set-active-example`**, because that is the `#[dsl(keyword = ..)]` on W2's payload
  struct. Raster's session rows use shortened keywords (`camera`, `locale`); the row must state
  whatever the struct declares, not a house style.

## 2 — View actions stay `ActionKind::View`, and are still `Migrated`

`setCamera`, `setCameraZoom`, `setCompositeViewport`, `setBrushSize`, `setBrushOpacity` and
`setLocale` keep `ActionKind::View` in `🔖️Manifest` and publish into the **Config** lane — the
ticket-`26/07/31` rule that camera is runtime state and never a document field is untouched. `Kind`
and `InteractiveJobClassification` are orthogonal axes: `validate_interactive_job_classification`
blocks release on `Unclassified` regardless of kind, and UI dispatch rejects anything that is not
`Migrated`. Puzzle3d classifies its whole `Config`-lane block `Migrated` the same way. So all six are
`Migrated` **and** View/session-only.

`setActiveUtility` is deliberately absent from the `.action_interactive_job(..)` chain (see the
comment at `:1024-1028`): it is framework-injected by `.utility(..)` inside `build_definition`, after
the builder chain has run, and `ActionDefinition::resumable_framework_catalog` already classifies it
`Migrated`. Calling `.action_interactive_job("setActiveUtility", ..)` in the chain matches nothing.
It *is* a `RasterCommand` row, so it stays in the retained table, the contracts and the proofs.

## 3 — The law test

`retained_route_dispositions_are_exact_and_exhaustive` (`…/✏️editor/🦀️.rs:1459-1520`), the raster
analogue of block2d's `:698`. Counts bumped 16 → 17 at `:1466-1468`, `setActiveExample` added to the
`artifact_lane` set at `:1478` and to `every_command()` at `:1453`, and the wire-keyword match arm at
`:1554`; `command_wire_keywords_are_unique_across_every_row`'s row count at `:1531`.

New this run (`:1503-1520`) — the gate's own arithmetic, spelled out:

```rust
let migrated_ids: BTreeSet<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter())
    .filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated)
    .map(|action| action.id.as_str()).collect();
assert_eq!(
    RasterCommand::TOOL_JOB_IDS.iter().copied().filter(|tool_id| migrated_ids.contains(tool_id)).collect::<BTreeSet<_>>(),
    retained,
    "every bounded-first-step proof entry must be Migrated, and every Migrated tool-job row must be proven"
);
```

`expected = TOOL_JOB_IDS ∩ migrated` must equal the proof set — literally what
`validate_tool_job_rows` computes. A route that is proven but left unclassified, or classified but
unproven, now fails in the unit test instead of at runtime with `interactive-job.catalog-authority`.
`RASTER_RETAINED_TOOL_IDS` is a sound stand-in for the proof set because the macro derives the proofs
from that same literal list and the test asserts the two lengths are equal at `:1466-1467`.

**Why not iterate the proofs directly.** `ArtifactBoundedFirstStepProof`'s fields — including
`tool_id` — are private with no public accessor (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12609-12619`),
so `bounded_first_step_tool_proofs()` is only countable from a plugin crate, not enumerable. Block2d
has the same constraint and drives off its own constant the same way.

## 4 — `🔏️publication-authority`

`✏️s/🔌️plugins/🖨️raster/🔏️publication-authority/🔣️.json:11` — `setActiveExample` added to the
`Migrated`/`["Artifact"]` group (routes stay alphabetical). The fixture now carries all seventeen
routes, matching `RASTER_PUBLICATION_CONTRACTS` route-for-route and lane-for-lane.

**The sibling `🧬️.schema.json` is correct and is not a duplicate.** Puzzle carries its own
`🔏️publication-authority/🧬️.schema.json`; these two are the only such directories in the repo and
there is no central schema in `🧰️framework`. `diff` against puzzle's shows only the four intended
per-plugin differences: `$id` and the `schema` const (`semio.raster.…` vs `semio.puzzle.…`), and
`owners` `minItems`/`maxItems` 1 vs 3 with `owner` enum `["RasterPlayApp"]` vs puzzle's three apps.
Everything else — the group shape, the lane enum, the `blocker`-iff-`BatchOnlyPendingRewrite`
`allOf`, the eleven `laws` consts — is byte-identical.

Validated with the same third-party oracle puzzle's audit uses (Ajv, `strict: true`,
`allErrors: true`), plus puzzle's semantic checks and a cross-check of the fixture against the Rust
`ArtifactToolPublicationContract` rows scraped out of the editor source. Script:
`🟦️w1-publication-authority-ajv.ts` in this ticket folder — deliberately kept here rather than added
to the raster TS package, which is W3's region.

```
AJV OK
routes 17 unique 17
laws all true: true
hostOnlyExclusive: true
blocker rule: true
contract keys match: true
lanes match: true
```

### Taxonomy

`🔏️publication-authority` was an unclassified directory under raster.
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:5739-5743` adds:

```json
"plugin-publication-authority": {
  "ownerKindIds": ["members-of-plugins"],
  "memberNames": ["🔏️publication-authority"],
  "source": "registry"
}
```

**Why not mirror `puzzle-publication-authority` literally.** Puzzle's entry is owned by
`distribution-puzzle`, a `semanticDirectoryKind` (`:2862`, emoji `🧩️`, slug `^puzzle$`) that types the
`🧩️puzzle` directory itself. There is no `distribution-raster` — nothing in `semanticDirectoryKinds`
matches `🖨️raster`, so `🖨️raster` classifies through the member path as `members-of-plugins`, and its
children resolve with that as `parentKindId` (`🔍️discovery/🟦️.ts:1700-1703`). Adding a
`distribution-raster` kind would change `🖨️raster`'s own kind id, and `members-of-plugins` is a
declared `parentKindIds` entry for `test-case` among others — a large, unnecessary blast radius on a
contended file. Owning the member off `members-of-plugins` is the minimal correct edit and generalises
to any plugin that grows one of these fixtures. The two entries cannot collide: they have disjoint
`ownerKindIds`, so the `memberMatches.length === 1` requirement holds for both.

## 5 — Verification

`cd /Users/ueli/Documents/semio && CARGO_TARGET_DIR=target-s-e2e RUSTC_WRAPPER="" cargo check -p semio-s-plugin-raster --lib --message-format short`, foreground, at commit `5e03e56997` with this
run's working-tree edits. Verbatim, the complete error output:

```
✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/./././././././../../🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🖌️brush/🦀️.rs:36:28: error[E0308]: mismatched types: expected `ActionDescriptor`, found `Option<ActionDescriptor>`
✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/./././././././../../🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🖌️brush/🦀️.rs:50:28: error[E0308]: mismatched types: expected `ActionDescriptor`, found `Option<ActionDescriptor>`
✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/./././././././../../🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧽️eraser/🦀️.rs:37:28: error[E0308]: mismatched types: expected `ActionDescriptor`, found `Option<ActionDescriptor>`
✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/./././././././../../🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧽️eraser/🦀️.rs:51:28: error[E0308]: mismatched types: expected `ActionDescriptor`, found `Option<ActionDescriptor>`
error: could not compile `semio-s-plugin-raster` (lib) due to 4 previous errors; 79 warnings emitted
```

**All four are W4's, verbatim from their own plan.** `📓️w4-raster-framework-drift.md:82-93` claims
error class 4 and names these exact four sites (`🖌️brush/🦀️.rs:36,50` and `🧽️eraser/🦀️.rs:37,51`)
with the fix `raster_action(.., None)` → `raster_measure_action(..)`. Their
`raster_measure_action` returns `Option<ActionDescriptor>`; the framework's
`WindowMeasure::Slider::on_change` now wants a bare `ActionDescriptor`. Both files were last written
at 05:15, mid-run. Not touched.

**My region is clean.** Zero diagnostics in the whole 2 305-line run mention `set_active_example`,
`setActiveExample`, `app_commands`, `bounded_first_step`, `RASTER_RETAINED_*`,
`ArtifactToolPublicationContract`, `action_interactive_job`, `RasterRetainedCommandJobFactory` or
`RasterStorePreparation`. The run emitted 79 warnings including 14 from `✏️editor/🦀️.rs` itself, so
the crate reached full analysis rather than aborting at macro expansion — the generated
`RasterCommand::dispatch` arm for `setActiveExample` and the seventeen-entry proof block both
type-checked.

An earlier run of the same command (28 errors, the repo-wide `UiNode` → `Result<BuiltNode, …>` /
`Label` prelude migration) compiled a pre-edit snapshot: it reported `✏️editor/🦀️.rs:821` for a
`render` that sits at `:830` in the current tree. Discarded and re-run rather than reported.

**`cargo test -p semio-s-plugin-raster --lib retained_route_dispositions_are_exact_and_exhaustive`
was NOT run: the crate does not compile, blocked on W4's four errors above.** The test is written and
its region type-checks, but it has not been executed — do not read this report as a passing test.
Re-run it once W4's drift block lands.

## Files touched

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/🦀️.rs` (one `pub mod set_active_example;` mount at
  `:596-597` — W2 owns this file's test mounts, this is a command mount and was the only way to make
  their handler reachable)
- `✏️s/🔌️plugins/🖨️raster/🔏️publication-authority/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️w1-raster-classification-factory.md` (this file)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/🟦️w1-publication-authority-ajv.ts` (input script, keep)

Not touched, though the previous W1 run created them and this run validated them:
`🔏️publication-authority/🧬️.schema.json`.
