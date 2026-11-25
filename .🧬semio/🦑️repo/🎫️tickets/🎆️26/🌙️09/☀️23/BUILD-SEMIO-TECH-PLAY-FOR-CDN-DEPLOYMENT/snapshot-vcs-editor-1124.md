# Snapshot VCS Editor Fix (`semio-s-artifact-vcs-vcs-editor`, 9 errors)

Tree edited: `/Users/ueli/.sinc/play` (snapshot). **Nothing was compiled.** No cargo or rustc ran. The release build is the only verifier. No framework, derive or macro files were touched. No files were renamed. No dependencies were added, so no Cargo.lock was edited.

Source log: `.cache/b/r/m/components/runs/wVzfDkM/l/cargo-release-00-b1.log`, the lines before 258489.

## Errors

| # | Error | Cause | Fix | Model |
|---|---|---|---|---|
| 1 | E0027: `ArtifactCommandInputs` pattern is missing `snapshot_read` (`✍️edit-command-work` :179) | The snapshot's `ArtifactCommandInputs` has a new `snapshot_read: ArtifactCommandSnapshotRead` field. | Added `snapshot_read: _` to the pattern. The edit work does not use it. | Framework struct `🔌️plugin/🧵️retained-command/🔖️checkpoint-and-work-contract` |
| 2, 6 | E0308: `return Ok(..)` / `Ok(match ..)` where `RetainedWorkOutcome<InteractiveJobCloseStep, _>` is expected (:237, :257) | `close_original_granted_step` takes a closure that returns `RetainedWorkOutcome { result, work }`, not a `Result`. | Each branch of the granted closure now returns `RetainedWorkOutcome`: `work: 0` for Blocked, Complete and idle Pending; `work: 1` when the remnant owner is created; `Err(error)` with `work: 0` when it is refused. The refusal still restores the workspace. | Framework `🧵️job/♻️retirement/🎟️context::close_original_granted_step`; the energy engine tail birth (`energy_construct` counts one unit of work for a successful construction) |
| 3 | E0061: `ControlledRetirement::step` takes `(body, effect)` (:240) | The method is now `step(&mut self, body: usize, effect: &mut RetainedAllocationEffect) -> OriginalRetirementPoll`. | Stepping the remnant moved out of the granted closure: `body = cx.retained_grant().maximum_copy_bytes`, `demand = owner.next_demand(body)?`, then `close_original_effect_step(cx, demand, \|effect\| owner.step(body, effect))?`. The remnant is dropped once `terminal_is_empty()`. | gis `🏔️gisterrain/…/🎮️commands/🌍️geojson` and `📥️import-elevation/⏩️work`: `close_step`, the same three lines |
| 4, 5, 7 | E0308: `Ok(..)` / `Err(..)` patterns matched against `RetainedWorkPoll` (:245–247) | These follow from #3: the poll is no longer a `Result`. | Removed by #3. `close_original_effect_step` takes the `OriginalRetirementPoll` directly. | same as #3 |
| 8, 9 | E0624: `bounded_first_step_tool_proofs` is private (`🧭️editor-and-manifest` :85) | The snapshot's `bounded_first_step_tool_proofs!` macro emits a private `fn` inside the impl in `📬️store-preparation-and-proofs`. The sibling module `🧭️editor-and-manifest` cannot call it. This was **not** caused by the earlier store-preparation port: live has the same code and the same private macro. | Each of `impl VcsBoundedProofs` and `impl VcsResumableProofs` gained `pub(crate) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> { Self::bounded_first_step_tool_proofs() }`. `ArtifactEditor::bounded_first_step_tool_proofs` now chains `VcsBoundedProofs::proofs()` and `VcsResumableProofs::proofs()`. All proofs are kept. | gis `🏔️gisterrain` `LocalFileImportJobFactory::proofs` / `LocalFileExportJobFactory::proofs`, which the gisterrain editor's `bounded_first_step_tool_proofs` chains |

One behaviour detail changed. `close_original_effect_step` always reports `Pending`. So when the remnant finishes, the next close turn reports `Complete`: the remnant is gone, the workspace is empty and `closing` is set. This is the same as the gisterrain model. Before, the same turn reported `Complete`.

Extra cleanup: removed the unused `use protocol::ColdMutation as _;` from `VcsOneItemPreparation::advance_step`. The log warned about it, and the earlier port had added it. The rest of the earlier port (`VcsOneItemPreparation*`) raised no errors and was left unchanged.

## Files changed
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/✍️edit-command-work/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📬️store-preparation-and-proofs/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️editor-and-manifest/🦀️.rs`

## Sweep
I searched the whole `🌿️vcs` plugin for these stale items: `ArtifactCommandInputs {` patterns, `.step(grant)`, `close_step(grant)`, `InteractiveJobCloseStep`, and calls to `bounded_first_step_tool_proofs`. No other library uses remain.

## Unresolved
- Nothing was compiled.
- The test targets are not part of the release lib build and were not ported. As the earlier pass already reported, `✏️editor/🧪️tests/🔬️unit` still uses the old preparation shape.
- One warning remains as it was: unused `request` in `🧭️editor-and-manifest` :109.
- Live's `history::render(doc.history, view_state.locale)` was not ported. The snapshot's history window has the one-argument form, and the log shows no error for it.
