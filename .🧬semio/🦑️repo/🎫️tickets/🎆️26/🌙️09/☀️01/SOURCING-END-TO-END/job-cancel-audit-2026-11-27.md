# Job Cancellation Audit (2026-11-27)

Read-only audit of `InteractiveJob` implementations against the new typed-`Cancelled` return path described in `surface-fixture-final-2026-11-27.md`. No source was edited.

## Scope and method

- Search: `impl[^{]*InteractiveJob\s*for` over all `*.rs` (excluding `target`, `node_modules`, `.cache`, and ticket copies under `.🧬semio`). 149 raw hits; 9 are ticket copies (excluded). **140 impl sites remain: 77 production, 63 test/testing/tests.**
- Trait: `🧰️framework/🔨️modules/🧵️job/🚦️clock-stride-and-job-protocol/🦀️.rs:31` (`InteractiveJob::step`). Cancel signal: `StepContext::is_cancelled()` at `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:640`, documented as "jobs check on entry and after every bounded unit".
- Per impl, the `step` body was followed through up to 4 call levels (same-file helpers first, then repo-wide name index). The chain column shows where the first cancel check was found. Chains are a triage signal, not proof of entry ordering; the rows marked "verify" and the findings below were read by hand.
- Path abbreviations used in the table: `FM/` = `🧰️framework/🔨️modules/🧵️job/`-adjacent framework modules (`🧰️framework/🔨️modules/`); `FP/` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`. Plugin prefix `✏️s/🔌️plugins/` and FEM prefix `✏️s/🔨️modules/🏗️fem/` are shown in full.

## Priority areas

- **Sourcing plugin (`✏️s/🔌️plugins/🪵️sourcing`)**: no `InteractiveJob` impl of its own. Its only job is `ArtifactRetainedCommandJob<EditorApp<SourcingCurationApp>>` (`retained-commands/🦀️.rs:90`, factory is `SourcingCurationBoundedCommandJobFactory`). `ArtifactRetainedCommandJob::advance_original` checks `cx.is_cancelled()` first (`🧵️retained-command-job/🦀️.rs:463`), so cancel stays a no-op-plus-Cancelled. Verified: **ok**. `NaturalFileImportJob` (`📥️natural-file/🦀️.rs:275`) checks on entry too (`step_original:306`): **ok**.
- **Hub (`🌎️hub`)**: no production `InteractiveJob` impl. The hub's cancel-polling traits (`AuthorityOperationControl`, `is_cancelled` at `🗿️artifact-authority/🦀️.rs:162`, implemented e.g. by `DocumentCheckInJob`) are not driven by the framework job return path. Out of scope for this change.
- **Plugin runtime (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`)**: most impls check on entry. Exceptions are listed under "Needs a fix".
- **Store / history**: `ArtifactEnvelopeDecodeAuthority` (`📄️decode-page-state-and-authority/🦀️.rs:474`) has no entry check but `advance_decode` (`:359`) checks cancel first and only performs release/cleanup under cancel: **ok**. `ArtifactStoreInitializationJob` (`♻️lifecycle/🦀️.rs:543`) delegates to the replay authority, which checks cancel at `🏗️initialization/🔁️replay/🦀️.rs:342`: **ok (verify)**. The `RetainedHistoryTurn::is_cancelled` trait (`📡️spr/📜️history/🦀️.rs:63`) is not an `InteractiveJob`; not audited further.

## Result summary (77 production impls)

- ok: 64 (includes delegating and verified ones)
- ok (verify): 8 (ordering not fully read; see "Verify-only")
- does-extra-work (small, bounded): 3 (`GuestRelayLifecycleProbeJob`, `SearchJob`, `EchoJob`)
- mutates-state-under-cancel: 1 (`ArtifactReservedToolJob`)
- test-only: 1 (`Stub`, specimen in a test module, to confirm)
- may-not-terminate: 0 found. No job loops under cancel without a cancel check; loops in the FEM and snapshot jobs are bounded by `units_per_turn` / `should_yield` / `fuel_remaining`.

## Needs a fix

1. **`ArtifactReservedToolJob`** (`FP/📜️declaration/👷️job/🧱️ownership/📦️producer/🦀️.rs:65`, `step` at :66): the step never checks `cx.is_cancelled()`. It takes the inner job out of the shared `state` (`state.lent = true`, `held = inner`) and steps it, so a cancelled job mutates shared state and does one unit of inner work. Fix: `if cx.is_cancelled() { return cancelled(cx); }` at the top of `step`, before the `source.try_lock()` branch. This contradicts the fixture note's claim that reserved tool jobs "check on entry".
2. **`SearchJob`** (`✏️s/🔌️plugins/🀄️wfc/⚙️engine/🔍️search/🏃️batch/🦀️.rs:14`, `step` :15): when `yield_next` is set it emits `admit_yield` before delegating to the inner `WfcJob`, which checks cancel only afterwards. Under cancel the caller sees one extra `Yield`, then `Cancelled`. Fix: check `cx.is_cancelled()` before the `yield_next` branch and return `admit_cancelled`.
3. **`GuestRelayLifecycleProbeJob`** (`FP/🖥️host/🔀️post-turn-relay/🔬️lifecycle-probe-and-refused-source/🦀️.rs:67`, `step_callback` :53): no cancel check; under cancel it still runs the publication/yield path. Probe-only. Fix: entry check, same as the cold-relay job.
4. **`EchoJob`** (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs/🦀️.rs:706`, `step_value` :692 → `advance_from_source`): `advance_demands` runs before the `cx.is_cancelled()` check (`:278`). Move the check ahead of the demand computation. Low impact.
5. **`LayoutExportJob`** (`✏️s/🔌️plugins/📏️layout/.../⏭️advance-commit-and-codec/🦀️.rs:199`): cancel check is inside a helper reached after `slot.settle` and publication work. Verify ordering; if work precedes it, move the check to `step_with_drawing_read` entry.

## Verify-only

- `MountedLayoutJob` (`worker_one`, `close_requested || is_cancelled`): confirm the check precedes work.
- `FemJobGraph`, `AssemblyJob`, `LdltJob`, `SubspaceIterationJob`, `PcgJob`: `step_plain` gates retirement on `!is_cancelled()` and then calls `run()`. `MeshJob::run` checks cancel on entry (verified, returns `MeshRun::Cancelled`). The other five use the same shape; confirm their `run()`.
- `ArtifactStoreInitializationJob`: confirm `replay.rs:342` runs before any replay work.
- `Stub` (`FP/.../📦️producer/🦀️.rs:130`, risk: test-only): test specimen in a test module, admits yield under cancel. Confirm `cfg(test)` and ignore.

## Test impls (63, not in the table)

Test/testing/tests impls are unchanged by the framework edit but their probes can now observe one extra step under cancel. The probes that assert old behavior are `EntryProbe`/`CancelProbe` in `🧵️job/⏱️budget/🧪️tests/⏱️budget/🦀️.rs` and the `🧪️tests/🔬️retained-ownership` fixtures. Not audited beyond the scan.

## Caveats

- The scanner resolved helpers up to 4 levels deep; about 70 production impls came back as "check reachable in helper chain" rather than read line by line. Entry ordering was hand-checked for about 25 of them, and the rest rely on the chain result.
- No source was changed. Ticket generated files live in `🗑️generated/` (`scan*.py`, `scan*.tsv`, `table-rows.md`).

## Full table (production impls)

| # | Impl (file:line) | Job | Risk | Cancel result | Note |
|---|---|---|---|---|---|
| 1 | `FM/🎯️action-bus/🦀️.rs:348` | ErasedToolJob | ok | inherits inner | delegates to inner job, which checks cancel itself |
| 2 | `FM/🕸️graph/⏯️layout-run/🏃️job/🦀️.rs:887` | LayoutRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_outcome |
| 3 | `FM/🖱️ui/🖥️host/🪟️window/🖱️metrics-cursor-and-clipboard/🦀️.rs:160` | NativeClipboardJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_outcome |
| 4 | `FM/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🏁️collect-publish-and-close/🦀️.rs:275` | MountedLayoutJob | ok (verify) | Cancelled | close_requested || is_cancelled inside worker_one; confirm entry |
| 5 | `FM/🖱️ui/🎯️targets/🧊️wgpu/🏃️host/🦀️.rs:128` | ClipboardIoJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_outcome |
| 6 | `FM/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🏁️job-completion-and-interactive-job/🦀️.rs:450` | PreparedRenderJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_callback |
| 7 | `FP/🔌️plugin/🧵️retained-command/🧵️retained-command-job/🦀️.rs:673` | ArtifactRetainedCommandJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original>advance_original |
| 8 | `FP/🔌️plugin/⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs:203` | RuntimeLiveCleanupJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original |
| 9 | `FP/🔌️plugin/⚛️reactor/🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs:465` | RuntimeCloseCleanupJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original |
| 10 | `FP/🔌️plugin/🖥️host/🔀️post-turn-relay/🥶️cold-relay-job/🦀️.rs:154` | GuestColdRelayJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_callback |
| 11 | `FP/🔌️plugin/🖥️host/🔀️post-turn-relay/🔬️lifecycle-probe-and-refused-source/🦀️.rs:67` | GuestRelayLifecycleProbeJob | does-extra-work | yield/publication (no Cancelled) | no cx.is_cancelled() on entry; probe publication path runs once under cancel |
| 12 | `FP/🔌️plugin/🖥️host/⚡️effects/🌉️router-effect-handler-and-jobs/🦀️.rs:60` | DynRouterEffectJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step>step_callback |
| 13 | `FP/🔌️plugin/🖥️host/⚡️effects/🌉️router-effect-handler-and-jobs/🦀️.rs:158` | FaultRouterEffectJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_callback |
| 14 | `FP/🔌️plugin/🖥️host/⚡️effects/🌉️router-effect-handler-and-jobs/🦀️.rs:219` | CompleteRouterEffectJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_callback |
| 15 | `FP/🔌️plugin/♻️lifecycle/🦀️.rs:543` | ArtifactStoreInitializationJob | ok (verify) | authority | no own check; delegates to authority.step, replay checks is_cancelled at replay.rs:342 |
| 16 | `FP/🔌️plugin/👷️job/🦀️.rs:585` | ChildEmissionPreviewJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original |
| 17 | `FP/🔌️plugin/👷️job/🏛️reserved/🦀️.rs:154` | ? | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original |
| 18 | `FP/🔌️plugin/👷️job/🏛️reserved/🦀️.rs:443` | FrameworkConfigurationBinaryJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original |
| 19 | `FP/🔌️plugin/👷️job/🛬️lifted/🔄️full-operation/🦀️.rs:86` | TypedCommandFullOperationJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original |
| 20 | `FP/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️producer/🦀️.rs:65` | ArtifactReservedToolJob | mutates-state-under-cancel | only via held inner's own check | no entry check: takes inner out of shared state (state.lent=true, held=inner) and steps it under cancel |
| 21 | `FP/🔌️plugin/📜️declaration/👷️job/🧱️ownership/📦️producer/🦀️.rs:130` | Stub | test-only (verify cfg(test)) | Yield | specimen in test module; admits yield under cancel |
| 22 | `FP/🔌️plugin/📜️declaration/👷️job/🎟️request/🏭️reserved/🦀️.rs:27` | ArtifactReservedRequestJob | ok | admit_cancelled / Cancelled | direct check in step |
| 23 | `FP/🔌️plugin/📜️declaration/📥️natural-file/🦀️.rs:275` | NaturalFileImportJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original |
| 24 | `FP/🔌️plugin/🎞️media/📤️structured/🦀️.rs:197` | StructuredMediaExport | ok | admit_cancelled / Cancelled | cancel check in helper chain: advance |
| 25 | `FP/🔌️plugin/🎞️media/📤️snapshot/🦀️.rs:118` | NativeSnapshotPackExportJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_turn |
| 26 | `FP/🏪️store/📦️codec/🧬️owned-schema/📄️decode-page-state-and-authority/🦀️.rs:474` | ArtifactEnvelopeDecodeAuthority | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original |
| 27 | `FP/🛎️services/🚪️native-io/🦀️.rs:437` | NativeIoJob | ok | admit_cancelled | retired file/dir cleanup runs before cancel check; cleanup only |
| 28 | `FP/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill/🏁️job-close-and-interactive/🦀️.rs:179` | BoardFillJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_original |
| 29 | `FP/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs:332` | RendererAssetDecodeJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_turn |
| 30 | `FP/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🐣️create-app/🧩️compile/🦀️.rs:51` | CompileJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: poll |
| 31 | `FP/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs:142` | SurfaceResizeJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_turn |
| 32 | `FP/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:65` | FrameBuildJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_turn |
| 33 | `FP/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:530` | ActiveFrameBuild | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_turn |
| 34 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs:271` | FemJobGraph | ok (verify) | Cancelled | retire_due gated on !is_cancelled; run() expected to check entry (same pattern as MeshJob) |
| 35 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🔶️csr-build-and-run/🦀️.rs:190` | AssemblyJob | ok (verify) | Cancelled | same FEM pattern as MeshJob |
| 36 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🕹️mesh-job-run/🦀️.rs:334` | MeshJob | ok | MeshRun::Cancelled | step_plain runs run() first; run() checks is_cancelled on entry (verified) |
| 37 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🚀️run-and-close/🦀️.rs:156` | LdltJob | ok (verify) | Cancelled | same FEM pattern as MeshJob |
| 38 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🎵️subspace/🏃️run-and-publish/🦀️.rs:301` | SubspaceIterationJob | ok (verify) | Cancelled | same FEM pattern as MeshJob |
| 39 | `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🏁️run-and-entry/🦀️.rs:97` | PcgJob | ok (verify) | Cancelled | same FEM pattern as MeshJob |
| 40 | `PL/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️mesh-export/🦀️.rs:421` | MeshExportJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_turn |
| 41 | `PL/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️reconstruction-session/🏃️run-job/🦀️.rs:549` | ReconstructionRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 42 | `PL/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️reconstruction-session/🔍️revalidate-job/🦀️.rs:84` | ReconstructionRevalidateJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 43 | `PL/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📤️export/🦀️.rs:244` | RasterImageExportJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 44 | `PL/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📤️host-only-retained-route/🦀️.rs:120` | FlowHostEffectJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 45 | `PL/🗄️stdio/📇️registry/🧬️contract/🎬️media-export/🦀️.rs:143` | IncrementalMediaExportJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_owned |
| 46 | `PL/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🎚️tuned-encode/🦀️.rs:133` | TunedDeflateEncodeJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 47 | `PL/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🧮️zlib/🚚️publication-and-job-steps/🦀️.rs:85` | DeflateEncodeJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 48 | `PL/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/💾️native-save/🧾️job.rs:235` | PdfGraphSaveJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_turn |
| 49 | `PL/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏯️tool-run/🦀️.rs:717` | PreviewEvalRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 50 | `PL/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏯️tool-run/🦀️.rs:648` | PreviewEvalRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 51 | `PL/🀄️wfc/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/💡️inferences/🔁️job-lifecycle/🦀️.rs:184` | Wfc3dInferenceJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 52 | `PL/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/📑️fill/🦀️.rs:867` | Wfc3dFillRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 53 | `PL/🀄️wfc/🗿️artifacts/🧱️grid3d/🔨️modules/🏠️host/💡️inferences/🦀️.rs:648` | Grid3dInferenceJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 54 | `PL/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:820` | Grid3dFillRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 55 | `PL/🀄️wfc/🗿️artifacts/🖼️bitmap/🔨️modules/🏠️host/💡️inferences/🦀️.rs:731` | BitmapInferenceJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 56 | `PL/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🌡️fill/🦀️.rs:686` | BitmapFillRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 57 | `PL/🀄️wfc/🗿️artifacts/◻️2d/🔨️modules/🏠️host/💡️inferences/🦀️.rs:667` | Wfc2dInferenceJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 58 | `PL/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill/🦀️.rs:847` | Wfc2dFillRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 59 | `PL/🀄️wfc/🗿️artifacts/🔲️grid2d/🔨️modules/🏠️host/💡️inferences/🦀️.rs:680` | Grid2dInferenceJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 60 | `PL/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣fill/🦀️.rs:839` | Grid2dFillRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 61 | `PL/🀄️wfc/⚙️engine/🔍️search/🏃️batch/🦀️.rs:14` | SearchJob | does-extra-work | Yield, then Cancelled | yield_next branch emits admit_yield before delegating; inner WfcJob checks cancel after one Yield |
| 62 | `PL/🀄️wfc/⚙️engine/💼️job/⏯️restore-job-run-and-interactive-job/🦀️.rs:109` | WfcRestore | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 63 | `PL/🀄️wfc/⚙️engine/💼️job/🚦️job-run-and-interactive-job/🦀️.rs:118` | WfcJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
| 64 | `PL/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📤️export-tool-job/🦀️.rs:168` | LayoutExportToolJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_turn |
| 65 | `PL/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎥️media-export-job-and-factories/🦀️.rs:126` | LayoutMediaExportJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_turn>complete_original |
| 66 | `PL/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/⏭️advance-commit-and-codec/🦀️.rs:199` | LayoutExportJob | ok (verify) | admit_cancelled | cancel check sits in a helper after settle/publication; confirm ordering |
| 67 | `PL/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/⏯️suggestions-run/🦀️.rs:487` | BrushSuggestionsRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value>turn |
| 68 | `PL/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🔍️fill-revalidation/🦀️.rs:313` | FillRevalidateJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value>turn |
| 69 | `PL/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/⏯️fill-run/🦀️.rs:656` | FillRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value>turn |
| 70 | `PL/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs:299` | Puzzle3dFillToolRunJob | ok | admit_cancelled / Cancelled | direct check in step |
| 71 | `PL/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️.rs:597` | Puzzle5dPlannerToolRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value>turn |
| 72 | `PL/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🔍️revalidate-job/🦀️.rs:197` | Puzzle2dFillRevalidateJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value>turn |
| 73 | `PL/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🏃️run-job-stepping-and-finalize/🦀️.rs:115` | Puzzle2dFillRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value>turn |
| 74 | `PL/🧩️puzzle/🎮️commands/📤️jobs/🦀️.rs:706` | EchoJob | does-extra-work | admit_cancelled | advance_demands runs before the is_cancelled check in advance_from_source |
| 75 | `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/📦️owned/🦀️.rs:189` | DrawingOwnedSerializerJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value>turn |
| 76 | `PL/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️gesture-jobs/🦀️.rs:196` | DrawingGestureOperationJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value>turn |
| 77 | `PL/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🧵️run-job/🦀️.rs:539` | EnergySimulationRunJob | ok | admit_cancelled / Cancelled | cancel check in helper chain: step_value |
