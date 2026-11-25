# Work-Limit Cap Reverted (2026-11-25, from ALL-PUZZLE-ARTIFACT-EDITORS-END-TO-END)

The candidate `maximum=fuel_remaining - ORIGINAL_INVOCATION_WORK` in `StepContext::with_original_effect_at` (`🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`) is reverted to `maximum=fuel`.

The cap made a worker step's last unit, the one where `fuel_remaining == work`, exceed its own ceiling, and both 3D `editor-action-producer` tests failed with `interactive-job.worker-continuation` "WorkLimit". `with_original_invocation` charges no handoff unit. The handoff unit exists only on the `NativeDecodeControl` path, which already widens the ceiling by one.

Analysis: `ALL-PUZZLE-ARTIFACT-EDITORS-END-TO-END/l52-action-producer-worklimit.md`. The Koordinator/Bearbeiten refusal is still open on your side (`first_work_refusal` probe).
