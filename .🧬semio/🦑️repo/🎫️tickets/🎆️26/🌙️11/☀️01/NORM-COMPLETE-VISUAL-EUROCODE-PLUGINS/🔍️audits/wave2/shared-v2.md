# Shared agent v2, wave 2

Nothing compiled by me (no cargo/rustc, no builds). All changes by careful reading; rebuild requested in integrate/latest.md and every build-en*/latest.md.

## Urgent items from the coordinator
- Structural-scene file cap: another agent had already split colour-maps and tessellation into `🎬️structural-scene/🧊️meshing` (registered, facade re-exports). I had started the same split with two sibling crates, noticed the clash and reverted all of it (sources back to `crate::` paths, no leftover crates or members). Facade is 19 files; app-surface siblings were not touched. Other OVER crates in the scan are artifact crates (en1999 diff, vdi3805/iso16757 model), not shared.
- `evaluate` command removed cleanly in app-surface (retained tool ids, publication contracts, macro arms, evaluate work state/work item, test, disposition fixture and oracle counts) and in all fifteen norm editors (row, mount, action, interactive job, describe, tool list, command dir, tests). Wire ordinals shift by one; the `SetSelectedCheckIndex` hex pins were updated by hand. Backups of the pre-edit editors are in the session scratchpad only.
- `InteractiveJobClassification::Migrated` is a framework enum variant used in 341 files of the framework; it is classified as framework-owned and left in place (norm uses it as the required non-default value). Replacing it framework-wide is out of the shared agent's ownership.

## Task 2: view.activeCombination (done)
`ViewLane.item`, `LaneEvent::ViewItem`, `is_view_write`, `view_write_events`; `resolve_set_fields` ignores `view.` writes, `lane_events` emits the view events for `setFields`. Twins regenerated, `--check` exit 0, unit test added in live-lanes. EN 1990 windows can read `transient.view.item`.

## Task 3: viewport config crate (registered, awaiting infra)
Workspace member + `[workspace.dependencies.semio-s-plugin-norm-viewport-config]` added to the norm `Cargo.toml`, input glob in the contract `📋️project.json`; the crate is already listed in `📋️build-owners.json`. I did not run `repo verify/sync` (no builds). Infra asked in requests.md for sync, build, wire witness and goldens. NOT done until it is green: owner macro per artifact, unit picker and viewport state persisted through it (local-only), artifact dependency.

## Task 1: per-entity live projection (partial)
Done: `Out { only }`, `wants(id)` (base-id aware) and a filtering `push` in all ten scene kits (en1996: `restricted_to`); no-op when unrestricted, so goldens stay byte-identical.
Not done, with the reasons found by reading:
1. `SceneProjector::assemble` builds layers, legends and the description itself (`describe`), while every artifact's `Out::finish` computes used layers, role legend, bounds and a hand-written summary from the whole entity set. A fragment-assembled scene therefore differs from `project()`, so "incremental equals full" cannot hold byte-for-byte. Needed: a `finish` hook on `StructuralProjection` (artifact-supplied layers/legends/description over the assembled entities) used by `assemble`.
2. Annotations, tracks and legends carry no owning entity, so an `only` projection cannot attribute them to a fragment; they must either live in `project_global` or gain an owner id in the `Annotation`/`ParameterTrack` schema.
3. The artifact projection reads the whole `CheckReport` (`binding(report, dependents)`, `check_tone(report, id)`); the trait supplies only per-entity `CheckSample`s, so the `Document` must be a bundle (snapshot, report, placements, combination) and the fingerprint must include the report rows of the entity and its dependents.
4. Entity ids per document order are only known by running the modules; `entities()` needs an id enumerator per artifact (kit `entity_kind_of` is the inverse only).
Per-module `wants` guards (about 100 loops) are not added; they are an optimisation once 1-3 are decided.

## Task 4: shared rustc errors
Newest runs (en1995 run 13, en1996 run 9, en1990 run 11, others) contain no rustc diagnostics in shared crates. The older errors (FieldSpec unit/choices, CheckStatus exhaustiveness, chip_id moves, Vec2 clone, Out of the UnitFamily list, E0283 in editor-wiring) are already fixed in the sources by earlier edits: re-read each site, all consistent.

## Files
Shared: `📇️registry/🧬️contract/🖥️app-surface/{🧵️retained-commands,🏭️command-and-tool-job-macros,🧪️tests,🫧️live-lanes,✏️edit-rules}`, retained-command-dispositions fixture, norm `Cargo.toml`, contract `📋️project.json`, `README.md`. Artifacts (mechanical): fifteen `✏️editor` trees, ten `🎬️scene/🧰️kit/🦀️.rs`.
