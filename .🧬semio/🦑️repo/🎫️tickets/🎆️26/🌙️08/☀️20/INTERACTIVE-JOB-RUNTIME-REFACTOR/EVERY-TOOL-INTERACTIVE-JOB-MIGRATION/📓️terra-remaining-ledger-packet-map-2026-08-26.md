# Remaining Interactive-Job Ledger Packet Map

## Scope and evidence

- Official baseline: `📊️coordinator-official-tool-jobs-playbook-owner-2026-08-26.json` in this ticket (468 self-tests; 774 registrations; 258 accepted; 682 fail-closed remaining).
- Current-source check was static only: no Cargo, Nx, or verifier run. Counts below are therefore the official baseline unless explicitly marked **source-delta**.
- A packet selector is `owner | plugin/artifact | editor component`; its exact command IDs are the `remainingCommands` entries in the baseline JSON, grouped by the exact component path. The common path suffix is `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`, except `space-engine`, whose exact path is shown.

## Current-source delta: Flow

The baseline's 37-row `FlowPlayApp | 🌊️flow/🌊️flow` packet is stale in part. Current source declares only `duplicateWidget` and `duplicateWidgetStep` as `Migrated` at lines 528–529, and the `ArtifactEditor` implementation supplies a matching `BoundedFirstStepCommandJobFactory` contract at lines 324–333. Its continuation is persisted in config and redispatched as the hidden `duplicateWidgetStep` action; the search is chunk-bounded. This is a real bounded-first-step/resumable chain, not a cosmetic `Migrated` annotation.

The other 35 Flow IDs in the baseline have no current `action_interactive_job(...Migrated)` registration. Do not subtract the whole 37-row Flow packet: **inferred current remainder is 680, not 645**, pending a fresh official verifier run. The three scan-then-monolith packets are unaffected (53 rows remain: Process 26, Procedural2d 18, Sourcing 9).

## Ranked next eight file-disjoint packets

| Rank | Exact owner / selector | Baseline rows | Required architecture | Why now |
|---|---|---:|---|---|
| 1 | `RemodelPlayApp | 📸️remodel/📸️remodel` | 41 | Split bounded state/config edits from resumable media ingest and reconstruction stage execution. Persist an operation token, cursor/stage, cancellation, and payload handle; never route video/frame payloads through process staging. | Largest live editor packet; already has `RECONSTRUCTION_SESSIONS` mutable payload global and a reserved importer. |
| 2 | `CadPlayApp | 📐️cad/📐️cad` | Bounded for direct document/view edits; resumable for `importCadFile` and any serialization/export/save that can traverse unbounded data. | 40 rows plus a reserved importer; an existing host-configuration factory is not proof for the importer. |
| 3 | `SpaceApp | 🪐️space engine/🪐️space` | Bounded graph/config edits; resumable, operation-owned jobs for media and space-pack import/export. | 40 engine rows span multiple payload and pack routes; shared engine deserves one coherent factory boundary. |
| 4 | `ShootingPlayApp | 🎥️shooting/🎥️shooting` | Bounded transforms and settings; resumable asset/snapshot import and all-shot export with operation-owned image data. | 39 rows; `SHOOTING_EMBLEM_SCRATCH` is a child-content global that must not become a job payload shortcut. |
| 5 | `Puzzle2dPlayApp | 🧩️puzzle/◻2d` | Preserve the existing fill-session routes as resumable with serialized checkpoints; make simple selection/camera/config operations bounded. | 37 rows, with an established multi-step fill vocabulary; this avoids recreating a monolithic fill reducer. |
| 6 | `NotePlayApp | 🗒️note/🗒️note` | Bounded block/nudge/settings operations; resumable ink-event application and load/save payload work when input size is not fixed. | 36 rows and a clean, file-local split; no global-payload finding in the official ledger. |
| 7 | `FormsPlayApp | 📋️forms/📋️forms` | Bounded structural/form-value changes; resumable `setTryValue`/`setTryValues` only with operation-keyed session/checkpoint state. | 29 rows; it is the highest leverage packet coupled to seven global-store findings. |
| 8 | `Process3dPlayApp | 🏭️process/🧊️process3d` | Replace `process3d_retained_reduce` with route-specific bounded commands and resumable operation state for model import/export or iterative work. | Clears 26 of the 53 scan-then-monolith denials in one file; do not mark any route migrated while it invokes that reducer. |

## Complete remaining command packet map

All counts are baseline `remainingCommands` counts. `B` means bounded by default after route review; `R` means a resumable operation is required for the named unbounded/IO route family; `S` means the baseline has a scan-then-monolith denial and requires a real resumable decomposition.

| Owner | Plugin / artifact selector | Rows | Architecture |
|---|---|---:|---|
| RemodelPlayApp | 📸️remodel / 📸️remodel | 41 | B + R ingest/reconstruction |
| CadPlayApp | 📐️cad / 📐️cad | 40 | B + R CAD import/save/export |
| SpaceApp | 🪐️space / engine/🪐️space (`⚙️engine/🪐️space/🦀️component.rs`) | 40 | B + R media/pack import-export |
| ShootingPlayApp | 🎥️shooting / 🎥️shooting | 39 | B + R asset/snapshot import-export |
| FlowPlayApp | 🌊️flow / 🌊️flow | 37 | Source-delta: 2 migrated, 35 still B/R review |
| Puzzle2dPlayApp | 🧩️puzzle / ◻2d | 37 | B + R fill-session |
| NotePlayApp | 🗒️note / 🗒️note | 36 | B + R ink/load/save |
| Procedural3dPlayApp | 🌀️procedural / 🧊️procedural3d | 29 | B + R generation/evaluation |
| FormsPlayApp | 📋️forms / 📋️forms | 29 | B + R try-value batches |
| Process3dPlayApp | 🏭️process / 🧊️process3d | 26 | S: eliminate `process3d_retained_reduce` |
| Block3dPlayApp | 🧱️block / 🧊️3d | 23 | B, review surface edits for R |
| ArchitectPlayApp | 🏛️architect / 🏛️program | 21 | B + R import/report/analysis/search |
| LayoutPlayApp | 📏️layout / 📏️layout | 20 | B + R package/PDF/PNG/SVG export |
| DrawPlayApp | 🖍️draw / 🖍️draw | 20 | B + R canvas/boolean/commit work |
| Procedural2dPlayApp | 🌀️procedural / 🌀️procedural2d | 18 | S: eliminate `procedural2d_retained_reduce` |
| AnimatePresentPlayApp | 🎞️animate / 🎬️present | 18 | B + R video export |
| SequencePlayApp | 🎬️sequence / 🎬️sequence | 17 | B + R run/import-facing work |
| RasterPlayApp | 🖨️raster / 🖨️raster | 16 | B + R image payload work |
| HomeApp | 🪐️space / 🏠️home | 16 | B + R space import/share |
| Fem2dPlayApp | 🏗️fem / ◻2d | 15 | B, preserve fixed-operation registry boundary |
| Fem3dPlayApp | 🏗️fem / 🧊️3d | 15 | B, preserve fixed-operation registry boundary |
| Gis2dPlayApp | 🌍️gis / 🗺️gismap | 14 | B + R source opening if unbounded |
| SpaceIndexEditor | 🪐️space / 🪐️space | 14 | B + R artifact/open directory operations |
| DagPlayApp | 🕸️dag / 🕸️dag | 13 | B + R graph reorganization |
| ImperativePlayApp | 📜️imperative / 📜️imperative | 11 | B + R run |
| ReasoningWiresPlayApp | 💡️reasoning / 🔌️wires | 10 | B + R force layout |
| PlaybookPlayApp | 📖️playbook / 📖️playbook | 9 | B |
| TrinityJackPlayApp | 🔱️trinity / 🔌️jack | 9 | B + R completions |
| Block2dPlayApp | 🧱️block / ◻2d | 9 | B |
| SourcingCurateApp | 🪵️sourcing / 🗂️curate | 9 | S: eliminate `sourcing_curate_retained_reduce` |
| Puzzle5dPlayApp | 🧩️puzzle / 🖐️5d | 7 | B + R compose-kit import |
| Block5dPlayApp | 🧱️block / 🖐️5d | 7 | B |
| Puzzle3dPlayApp | 🧩️puzzle / 🧊️3d | 2 | B |
| Iso16757PlayApp | 📕️norm / 📓️iso16757 | 1 | B |
| Vdi3805PlayApp | 📕️norm / 📔️vdi3805 | 1 | B |
| Din4108PlayApp | 📕️norm / 📕️din4108 | 1 | B |
| Din16798PlayApp | 📕️norm / 📗️din16798 | 1 | B |
| En1990PlayApp | 📕️norm / 📘️en1990 | 1 | B |
| En1991PlayApp | 📕️norm / 📘️en1991 | 1 | B |
| En1992PlayApp | 📕️norm / 📘️en1992 | 1 | B |
| En1993PlayApp | 📕️norm / 📘️en1993 | 1 | B |
| En1994PlayApp | 📕️norm / 📘️en1994 | 1 | B |
| En1995PlayApp | 📕️norm / 📘️en1995 | 1 | B |
| En1996PlayApp | 📕️norm / 📘️en1996 | 1 | B |
| En1997PlayApp | 📕️norm / 📘️en1997 | 1 | B |
| En1998PlayApp | 📕️norm / 📘️en1998 | 1 | B |
| En1999PlayApp | 📕️norm / 📘️en1999 | 1 | B |
| Din18599PlayApp | 📕️norm / 📙️din18599 | 1 | B |

## Scan-then-monolith denial gate

| Owner | Helper | IDs | Required correction |
|---|---|---|---|
| Process3dPlayApp | `process3d_retained_reduce` | all 26 rows in its packet | Route-specific command handlers; checkpoints for any unbounded work; no final monolithic reducer call. |
| Procedural2dPlayApp | `procedural2d_retained_reduce` | all 18 rows in its packet | Same; split canvas/evaluation/generation state by operation and checkpoint actual iteration. |
| SourcingCurateApp | `sourcing_curate_retained_reduce` | all 9 rows in its packet | Same; bounded curate mutations may be direct, but no scan wrapper around a retained reducer. |

## Importer and process-global closure

All 35 canonical `import-media` owners are still explicitly `failClosedPendingFactory`: Procedural2d, Procedural3d, Gis3d, Gis2d, AnimatePresent, Shooting, Sequence, Fem2d, Fem3d, Process3d, Lowpoly, Layout, Cad, Iso16757, Vdi3805, Din4108, Din16798, En1990–En1999, Din18599, Playbook, Remodel, TrinityRewrite, Raster, Puzzle2d, Puzzle3d, and SourcingCurate. A generic factory elsewhere in any of those files is **not** importer proof.

The 29 process-global records contain 32 named owners: 17 child-content scratch owners, 8 fixed-operation registry owners (including `DRAW_MUTATION_ARENA_POOL`), 4 resizable-operation registry owners, 2 ABI bridge-retention owners, and 1 mutable payload (`RECONSTRUCTION_SESSIONS`). The highest-risk action is to move `RECONSTRUCTION_SESSIONS` and all resizable session/generation maps behind an operation key and persisted checkpoint; fixed registries and ABI bridges need an explicit static exemption, not silence.

## False-proof / factory assessment

- No false factory was found in the official snapshot itself: all 48 `factoryContracts` entries have `status: "explicit"`.
- Do not interpret that as whole-owner coverage: the 35 reserved importers remain fail-closed even where an owner has another explicit factory. Those are **coverage gaps**, not accepted factories.
- The only verified post-snapshot migration is Flow's two-route bounded-first-step chain. Its proof/factory/registration IDs agree, so it is not false. The snapshot's 37 Flow remaining rows are stale evidence and must be refreshed before using totals for closure.
- The 53 scan wrappers are not migration proofs. They remain explicit denials until their monolithic reducers disappear.
