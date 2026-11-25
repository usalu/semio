# Snapshot Sequence and Space Fixes (2026-11-24)

Tree edited: `/Users/ueli/.sinc/play` (snapshot). Log: `.cache/b/r/m/components/runs/wrMkAas/l/cargo-release-00-b2.log`.
**Nothing was compiled** (no cargo/rustc); release build is the verifier.

## Fixes

| Crate | File (under `✏️s/🔌️plugins`) | Change | Live had it? |
|---|---|---|---|
| `semio-s-artifact-sequence-sequence-schema` | `🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` | `RetireOwned for StepParams`: removed `fn retirement` (+`unsafe` `ManuallyDrop`/`ptr::read`); added `type RetirementCursor = <Dictionary as RetireOwned>::RetirementCursor`, `retirement_ready` delegating to `.0`, `retirement_seed(mut self)` = `std::mem::take(&mut self.0).retirement_seed()`. | No: live file is byte-identical and still has the old shape (live framework differs). |
| `semio-s-artifact-space-home-examples` | `🪐️space/🗿️artifacts/🏠️home/…/✏️editor/🎚️config/🦀️.rs` | Added `begin_copy_byte_demand` to `HomeConfigPreparationFactory` (copied from live). File is now byte-identical to live. | Yes, ported exactly. |
| `semio-s-artifact-space-home-editor` | `🪐️space/🗿️artifacts/🏠️home/…/✏️editor/🫧️transient/🦀️.rs` | Replaced the whole `//#region 🔖️Retirement` with live's: `RetireOwned for HomeTransient` + `HomeTransientRetirement` cursor (`native_retirement_reply!`, `close_step(body, effect)`); factory impls delegate to `SharedValueRetirementFactory` / `OwnedValueRetirementFactory` over `SharedOriginalStorage`. Removes old `HomeTransientRootRetirement` (ManuallyDrop + `unsafe` header release). File is now byte-identical to live. | Yes, ported exactly; each API checked against the snapshot framework. |
| (test only) puzzle commands jobs | `🧩️puzzle/🎮️commands/📤️jobs/🧪️tests/♻️retirement/🦀️.rs` | Ported from live: `Probe` gets `RetirementCursor` + `retirement_seed`, and the test uses `admit_original_construction_settled`. Both match the snapshot framework. | Yes. |

### Why `StepParams` is hand-written, not derived
`StepParams` has its own `Drop` (cold-retires the dictionary), so a derive that destructures fields can't move `.0` out. `mem::take` moves the dictionary out without `unsafe`. The `Drop` then sees an empty `Dictionary` and does nothing. `retirement_birth_bytes` delegates to `.0`, and the cursor type is the same, so `retirement_cursor_birth_demand`'s layout check still holds.

## Sweep (`✏️s/🔌️plugins` + `🌎️hub`; script in the session scratchpad)
- `impl … RetireOwned for …` checked: **77**. 4 still have the old shape (`fn retirement`):
  - flow `…/composed-source/🧵️assembly/🧪️scene.rs:28`
  - block 3d `…/🧬️schema/📸️snapshot/🦀️.rs:347`
  - forms `…/▶️try/🫧️transient/🦀️.rs:143,154`
  - puzzle test (fixed above).
- `impl … ArtifactStoreOneItemPreparationFactory / ArtifactStoreBatchItemAuthority` checked: **39**. 10 lack `begin_copy_byte_demand`:
  - home config (fixed)
  - block 3d ×2, 5d, 2d
  - writer
  - vcs
  - forms
  - equation (mathematical)
  - lowpoly ×2
- `impl … ErasedSnapshotRetirement for …` checked: **3** remaining. 1 still has the old grant-only `close_step`: writer `…/✒️main/🫧️transient/📢️publication/🦀️.rs:71`.
- `impl … SnapshotRetirementFactory / ArtifactOwnedValueRetirementFactory` checked: **14**. 3 still have the old shape:
  - writer unit test
  - gismap ×2
- The rest of the 🪐️space plugin has no other old-shape `close_step` / `retire` / `retire_owned` or `RetireOwned` impls.

### Not fixed (deliberately)
These defects remain in 8 plugins: 🧱️block, ✒️writer, 🌿️vcs, 🌍️gis, 📋️forms, ➗️mathematical, 💠️lowpoly and 🌊️flow. They have **zero crates in this build log**, and every affected file is **byte-identical in live**, so live has no fix to port. These plugins are stale throughout. For example, block 3d still uses the old `sequence(vec![deferred(..)])` boxed-cursor composition. Patching one method in each would not make them build. Each needs its own port ticket if it joins the play build.

## Open risks
- `StepParams` is the only hand-written change; live has no version of it, so it is unverified until the build runs.
- The home transient port relies on `HomeTransient: RetireOwned` being implemented only once in the snapshot; grep confirms this.
- The build log ran without keep-going semantics, so other crates may still fail later in the build (raster-model, shooting-mutations, stdio dwg/step editors are outside this scope).

## space-home editor port
Log: `.cache/b/r/m/components/runs/wVzfDkM/l/cargo-release-00-b1.log` (`semio-s-artifact-space-home-kinds-conformance`, 11 errors, lines ~260390-260540). **Nothing was compiled** (no cargo/rustc).

| Error | File (under `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any`) | Fix | From live? |
|---|---|---|---|
| E0308/E0061 `close_step` (grant-shaped `close_original_granted_step`, `owner.step(grant)`) | `✏️editor/🦀️.rs` | `close_step` now uses `close_original_effect_step(cx, close_demand()?, \|effect\| …)`; pays the retirement birth with `effect.performed(1)`, steps `ControlledRetirement::step(body, effect)` (returns `OriginalRetirementPoll`), drops the owner once `terminal_is_empty()`. | Yes, the section is byte-identical to live. Checked against the snapshot: `close_original_effect_step`, `RetainedAllocationEffect::{performed, grant}`, `ControlledRetirement::{step, terminal_is_empty}`, `semio_framework_value::{OriginalRetirementFailure, RetainedWorkOutcome, RetainedWorkPoll}`, `From<ValueError>` for `OriginalRetirementFailure`/`OriginalTurnFailure`. |
| E0609 `files.pack` / `files.spr` | `✏️editor/🎮️commands/🗃️apply-local-catalog-document/🦀️.rs` | `files.files.pack` / `files.files.spr` (`export_backbone_pack` returns `BackboneExport { name, files }`). | Yes; the file is now byte-identical to live. |
| E0277 `HomeTransientMutation: RetainedMutation<HomeTransient>`, `HomeTransient: SnapshotProjection`, `HomeTransientMutation: StoreMutationFor` | `✏️editor/🦀️.rs`, `👁️viewer/🦀️.rs`, `✏️editor/🫧️transient/🦀️.rs`, `✏️editor/🫧️transient/🪜️fold-stepper/🦀️.rs` | Hand port (below). | No. Live calls `semio_plugin::cold_transient_preparation_factory`, which live's framework added at `a6e34dbae6f` (11-23). The snapshot framework (native file blob = `01324745f80`) has only `bounded_transient_preparation_factory`, which needs `SnapshotProjection<Output=P>` + `RetainedMutation` + `OpBinaryReceiving`. The Home transient never had these: the editor has called the bounded factory, without compiling, since 11-05. |

### Transient preparation (hand port)
`bounded_transient_preparation_factory` would need a full retained port. `HomeTransient` would have to become a by-value `SnapshotProjection` (an `Arc` field is not projectable). `ApplyDirectoryPage` would need a `RetainedMutationLeaf` whose decision reads the base, but the `retained_set_*` macros only build base-free patches. Instead, the Home lane now uses the snapshot's public, domain-task route `store::ArtifactEphemeralTaskPreparationFactory`:
- `✏️editor/🫧️transient/🦀️.rs` adds a new `//#region 🔖️Publication` containing:
  - `pub fn home_transient_preparation_factory()`: `ArtifactEphemeralTaskPreparationFactory::<HomeTransient, HomeTransientMutation>::new(preflight, task, HomeTransientRetirementFactory, OwnedValueRetirementFactory::<HomeTransientMutation>)`.
  - preflight: `ArtifactStoreOneItemFootprint::for_ephemeral_item(page_json.len())`. The item still costs its page, never the projection.
  - task creation: parses the canonical page and calls `admit_page` against the captured root. A refusal (not canonical, or a frontier/authority mismatch) refuses the preparation, as the cold diff's fatal/error messages did. `Held` republishes the captured root. `Fold` starts the existing grant-driven `HomeDirectoryFold`, which had been unused outside a test.
  - task `HomeTransientPublication` (`ArtifactEphemeralPreparationTask` + `ErasedSnapshotRetirement`):
    - Each `advance` runs whole fold units within `min(copy, capacity)` of the grant and reports them as `Progress`. The checkpoint bytes grow by the charged bytes.
    - When the fold is done, `advance` builds the root (`Arc<HomeTransient>` + projection `Arc` + users `Arc`, priced with `shared_retirement_allocation_bytes`) and returns `Prepared` with `completed_items: 1`.
    - Close releases the unfinished fold in 4 KiB slices of its `retained_bytes`, following the `HomeTransientRetirement` pattern.
- `🪜️fold-stepper/🦀️.rs`: adds `HomeDirectoryFold::retained_bytes` (copied rows + users + page events, in the stepper's own byte units).
- `✏️editor/🦀️.rs` and `👁️viewer/🦀️.rs`: `build_transient_store_one_item_preparation_factory` returns `home_transient_preparation_factory()`.
- Not needed under this route: live's `DslReceiving` derives and `original_tagged_operation_receiving!` (+ `📡️.protocol.semio`) on the transient mutations. The app traits only require `ColdMutation + OpBinary + OpText`, and the task factory requires only `Send`.

### Other crates compiling these sources
- kinds-conformance, `semio-s-artifact-space-home` and `semio-s-artifact-space-home-viewer` all re-export `semio_s_artifact_space_home_editor::editor::home::transient` whole, so the new function reaches every one of them.
- The `🪐️space` artifact crates (`space-space*`) compiled cleanly in this run. Their only difference from live is extra document-store owner wiring in live, which is not needed here.
- No other old `close_original_granted_step`, `owner.step(grant)`, `files.pack` or bounded-factory shapes remain in the plugin outside tests.

### Tests (cfg(test) only, not in the release build)
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs` `kept_pair`: `semio_os::export_backbone_pack` + `files.files.*`, as in live.
- `🫧️transient/🧪️tests/🔬️unit/🦀️.rs`: both factory calls now use `home_transient_preparation_factory()`. The `publish` helper is still stale (one-argument `advance(grant)`, `ArtifactStoreOneItemGrant { maximum_bytes }`). That test file needs its own port to the snapshot's `advance(grant, receiving)` contract.

### Open risks
- The task route is hand-written and unverified until the release build runs.
- A single fold unit whose quote exceeds every grant (an event row larger than the grant's copy/capacity) blocks the page. This is inherent to `HomeDirectoryFold`'s whole-unit quotes.
- `begin_close` measures the unfinished fold once, which is O(rows). `HomeTransient::retirement_seed` does the same.
