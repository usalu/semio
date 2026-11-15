# 1112 floweditor (lane floweditor, 11/12)
## Step 1 semio-s-artifact-flow-flow-editor-retained GREEN (W-floweditor-2 hub wasm rc 0; baseline W-floweditor-1 93 errors / 13 files)
Shared helpers in retained root `🦀️.rs` (pub(crate), imported as `crate::retained::{...}`): `poll_outcome`, `close_with_effect`, `begin_close_owner`, `effected`, `replied`; `Retirement::step(body,effect)->OriginalRetirementPoll` via `ControlledRetirement::retain`.
Per-file checklist:
- [x] 🦀️.rs (root Retirement + helpers)
- [x] 📥️original (intake: admission advance/take_output/return_inputs under `effected`; take_command via `take_authority_into` + `replied`, `recover` flag so a suspended handback resumes with `recover_take_authority_into`; `close_step(g,effect)`)
- [x] 📬️resolve root / 🧵️job / ⏱️effect / 🌱️origin / 📡️resume (close ladders via close_with_effect; origin lease via `SharedOwner::admit_from` under `effected`; resume NativeEncodeControl resume/pause tuple errors; job phase 40 reserve under `effected`; encoding via close_original_effect_step)
- [x] ✏️authoring, ➕️descriptor, 🧮️evaluation root + 📤️job
- [x] 🏠️instance-owner (maintenance via `original_lifecycle_effect_step`; scene cache `advance(cx)` funded by `with_original_retirement`+`close_with_effect`)
Rerun: "$C/1105b-tools/chk.sh" W-floweditor-<n> "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-flow-flow-editor-retained
Step 2: flow editor crate semio-s-artifact-flow-flow-editor GREEN W-floweditor-5 (terminal_frame_bound removed x4, close ladders effect-shaped via lifecycle effect step, host effect payload ArcLease, settled host construction, registries step under plugin_effect_close)
Step 3: semio-hub-flow-component GREEN (W-floweditor-6 hub wasm rc 0) before debts. Debts in progress: (a) FlowTextOrderingWork::close_step(body,effect)->OriginalRetirementPoll (host evaluation-source ordering + pending origin + child-publication leaves + ordering laws); (b) composed-source assembly layout entry advance/close suspension -> retry via recover_advance_into/recover_close_into, `recover` flag, advance output into owners.layout directly.
## Final (11/12)
Verified (hub wasm, "$C/1105b-tools/chk.sh" W-floweditor-<n> "🌎️hub/Cargo.toml" wasm ...):
- W-floweditor-2 rc 0: semio-s-artifact-flow-flow-editor-retained (baseline 93 errors / 13 files).
- W-floweditor-5 rc 0: semio-s-artifact-flow-flow-editor (40 errors / 6 files: terminal_frame_bound x4 removed, close ladders, host effect payload ArcLease, settled host construction, registry retirement step).
- W-floweditor-6 rc 0: semio-hub-flow-component (before the debt edits).
- W-floweditor-7 rc 0: semio-hub-flow-component after the debt edits (a)+(b).
Debt (a) done: FlowTextOrderingWork::close_step(body,effect)->OriginalRetirementPoll (framework flow host 🔤️ordering, via close_by_grant so the work is charged to the effect); callers: child-publication leaves (poll_outcome), host pending 🔎️origin.rs, ordering laws test.
Debt (b) done: composed-source assembly layout entry advance/close: a suspended callee keeps its state and returns zero progress; next turn uses recover_advance_into / recover_close_into via a `recover` flag; advance writes straight into owners.layout (no per-turn local output slot).
Not verified / debts: no --tests, no native check, no behaviour run (check only). cfg(test) code in editor-retained (🧪️ files: envelope.rs, donation.rs, origin.rs, routes.rs, scene-cache 🧪️.rs, tests/🔬️unit) still on old APIs. `retained::{poll_outcome,close_with_effect,begin_close_owner,effected,replied}` are now pub (editor crate imports them). Retirement::step charges one unit for the owner birth. Local close events in host-only job use plugin_pay_unit; ordering `OriginalTextOrdering::close_step(g)` (raw inner comparator) stays grant-shaped (used by procedural fold).
Rerun: "$C/1105b-tools/chk.sh" W-floweditor-<n> "🌎️hub/Cargo.toml" wasm -p semio-hub-flow-component
STATUS: FIXED-verified W-floweditor-7
