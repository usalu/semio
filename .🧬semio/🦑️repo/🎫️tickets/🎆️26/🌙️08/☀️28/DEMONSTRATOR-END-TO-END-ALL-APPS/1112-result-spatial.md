# spatial (11/12)
Step 1: semio-s-spatial-kernel-semio-session (39 errs) ported to the effect-shaped contract.
- GeometryPort trait (flow/geometry): retain_step/cancel_step/close_step and GeometryPortRetirement::close_step take `&mut RetainedAllocationEffect`; GeometryPortRetirement::retire_cold removed (no wallet in geometry). 4 flow-host callers pass their existing effect (host-close, eval-session close-and-retirement, preview cancellation, preview retention).
- spatial session: SessionState/SessionCapture close_original_step -> OriginalRetirementPoll (events charged via spend_event), close_step/close_mesh_import(_original)/cancel_step/close_retired_tessellation_step take the effect; SessionRetention::step takes the effect; construction close runs the three raw owners and the ready capture inside cx.with_original_effect; tessellate_step/cancel_tessellation_step run in cx.with_original_effect.
- Cold paths (SessionCapture::retire_cold, Session::close, retire_tessellations_cold) use neural_engine::retirement::cold_effect (debt: it uses an AuthoringWallet internally; pre-existing helper).
Verify: chk.sh W-spatial-2 hub wasm -p semio-s-spatial-kernel-semio-session (pending)
Debts: cfg(test) code and 🧪️tests/* of the session crate and geometry crate remain grant-shaped.
Step 1 verified: W-spatial-2 hub wasm session crate rc 0.
mesh ported (operator-core, mesh-operator-io, macro); W-spatial-4 = playbook procedural component
Step 2: W-spatial-3 (flow-host session+host) rc0. W-spatial-4: mesh 92 errs gone, 44 remained in brep ext (booleans, tessellate, inferences/geometry); ported -> W-spatial-5
Step 3: W-spatial-11 playbook procedural component rc0 (mesh 92 errs + brep ext booleans/tessellate/inferences + procedural mutations/terminal_frame/learning-program/embedded-eval). Next W-spatial-12 brep ext component.
Step 4: W-spatial-12 brep extension component (wasm, -p semio-s-plugin-flow-brep-component) rc 0. stdio-brep-viewer / txt-tests were not red in this closure (owned by stdio lanes anyway).
Rerun: chk.sh W-spatial-<n> "🌎️hub/Cargo.toml" wasm -p semio-s-spatial-kernel-semio-session ; chk.sh ... "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧩️component/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-s-plugin-flow-brep-component ; playbook: "✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml" -p semio-s-plugin-playbook-procedural-component
Verified rc0: W-spatial-2 (session), W-spatial-3 (flow-host-session+flow-host), W-spatial-11 (playbook procedural component), W-spatial-12 (brep ext component). Check only; no tests run.
Debts: cfg(test)/🧪️tests of session crate, flow geometry crate (ownership tests), mesh operator-core, tessellate tests remain grant-shaped (not compiled by check); GeometryPortRetirement::retire_cold removed (callers in session tests); cold paths use neural cold_effect (AuthoringWallet inside, pre-existing); mesh job input retained in pending_input until close (deferred push, since Operator::step_plan is grant-shaped); native events charge 1 unit via effect.performed(1) only in session close, not in retention.
Debt: with_original_effect does not debit the turn: every wrapper calls cx.consume_work(performed).
STATUS: FIXED-verified W-spatial-12
