STATUS: tool-run-tick-wire and tool-run-presentation LIBS GREEN native (tickwire-5 rc 0) and wasm32-wasip2 (tickwire-6 rc 0). Tests and the plugin tick native are UNVERIFIED (not compiled).

# tickwire: tool-run tick/receive port

## Decision (agreed with runtime)
No resident NativeLocalAuthority: no phase of ToolRunTickNativeCursor needs an effect primitive (it fills plain Vecs, pack parser admit/retire stay grant-based and pure). Only ControlledRetirement::step and PagedList reserve/adopt/release_empty_page need the effect, and those sit in the leaf steppers. advance() stays on NativeDecodeControl::receive_construction_step (plugin debits once).

## Edited (all under 🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/)
- 🏗️native: shared `pure(effect)` (one unit, false = no work left, nothing moved) and `release_page`; initialize_header_step / receive_identity_step / receive_sequence / publish_into / close_step take the effect; close_demands/close_frame return Result (no expect).
- 🔢️entities (ToolRunEntityColumnReceiving): reserve_step (reserve_capacity_one_funded), commit_step (adopt_reserved), receive, close_step take the effect; list primitives record their own work, Denied = Ok(None), allocation failure keeps its receipt and progress.
- 📊️progress: initialize_header_step, reserve_step, receive, advance_child_step (adopt phases via adopt_step), publish_into, close_step (both types), close_content take the effect; identity/step children stay grant-based under the parent's single unit.
- 📼️trace: close_step(effect) with ControlledRetirement::step(effect); advance-time parser drain is `close_parser_step(grant)` (page already moved); no unwrap.
- 🦀️ root: ToolRunTickNativeCursor::close_step(effect) -> Result<RetainedCloneStep,_> via `settled` (keeps primary/work/receipt cause); all unwrap/expect/unreachable removed (accessors + `le`). 📝️step: one unwrap removed.
- Plugin 🎞️tick/🛬️native: close via cursor.close_step(effect) and close_original_effect_step; phase 1 ported to the 2-arg with_native_receiving with `outcome` + `stepped` custody settled next turn (settle_receiving_scope, fuel = copied_items, source refusal / failed call become typed errors); unwraps removed.

## Follow-up (coordinator)
- Mid-flight `outcome`/`stepped` retirement routed: next_close_demands quotes it first, close_with settles the receiving scope, close_effect_step drops it with one recorded unit and surfaces a refused outcome or failed step as a typed error carrying its progress.
- The six test files: limits/identity/step use only unchanged grant APIs; root, native, progress and entities now run on the effect fixture (`effected` / `observe_effect!` in tick/🦀️.rs over value owner/🧪️work/effect.rs, heap observation kept outside the fixture's own work action); close_demands of frames now `.unwrap()`. UNCOMPILED, so assertion fallout (heap counts, Option/Result shapes) is unknown.
- No new language-agnostic law for the effect-recorded work equality yet.

## R6 (runtime2 ruling)
`ToolRunTickNativeCursor::advance` now enters through `control.receive_effect(demand,1,&mut frame,..)` with a resident `NativeReceivingEffect` frame (terminal_is_empty requires it Empty); it records its one transition via `effect.performed(1)`, sets the allocation marker from the step progress, reports `effect.performed_work()`, maps Pending to a typed refusal and returns `OriginalReceivingPrimary`. Plugin `stepped` carries that type. UNCOMPILED. Open: an unadmitted work quote (`Pending{work:0}`) now refuses typed instead of retrying; a frame left non-Empty by a refusal has no close route.

## Backpressure and frame close (coordinator)
- `Pending{work:0}` from receive_effect now returns an empty Progress step (retry next turn); a Granted frame is cancelled with the control first so the retry starts with a fresh grant. Other Pending stays a typed refusal. The "quote can never fit a whole turn" refusal is NOT implemented (no whole-turn bound is visible to the cursor); open.
- Non-empty frame close: cursor.frame_pending()/settle_frame(control) (Granted -> cancel_unstarted, otherwise finish_receiving with its refusal kept); the plugin close_with settles it inside with_native_receiving after the outcome is dropped, and cursor.close_step refuses typed while a frame is pending.
- Compile: tickwire-3 (native) rc 1 stopped in `semio-framework-value-retirement` (not mine, tick-wire not reached). Log 🗑️generated/1105b/tickwire-3.txt. wasm32-wasip2 not run.

## Compile round (main-148 follow-up)
- tick-wire: pack parser storage became effect-based (admit_record_storage/retire_record_storage(effect), they record their own work); advance_admitted/trace.advance/close_parser_step now take the effect, tally one unit on every non-primitive branch (`tally`), the allocation marker is set per branch; PackLimits import dropped.
- presentation: merged the duplicated `impl ToolRunPresentation` (old grant block removed, new effect block kept); update.rs payload child advance/close_step take `effect`.
- Claimed+fixed value/✨️derive/🧬️retained-clone/🦀️.rs:105 (boxed_original_retirement signature takes the effect), which was the E0425/E0053 in presentation's derive.
- Proof: tickwire-5 native rc 0, tickwire-6 wasm32-wasip2 rc 0 (`-p tick-wire -p presentation`, libs only).
- Still open: test targets uncompiled; presentation update/seed/slot still contain unwrap/expect/unreachable (pre-existing, not swept); quote-never-fits refusal.

## Assignment 2 (os-inference, infinite-board-schema, tool-typing-edit): libs GREEN native+wasm
- Proof: native tickwire-15 / tickwire-12 / tickwire-10, wasm tickwire-16 / tickwire-17 / tickwire-18, all rc 0 (libs only).
- tool-typing-edit: `OriginalTypingEdit::admit` runs through `admit_original_construction` with a caller-owned `OriginalConstructionAdmission` + failure slot (Ready -> reset_step, Failed -> take_failure_into_step), `admit_clone` inside `cx.with_original_effect`, decoder identity typed `NativeAllocationIdentity`.
- infinite-board-schema: four manual RetireOwned impls replaced by derives (Selection, DagNodeEvaluationStatus, DagInputFacts, DagInputCursor, DagInputKind); `DagInputCursor::step` takes the effect (grammar step and shared-owner admits record their work; other transitions one unit).
- os-inference: values/plan page reserve+release through `cx.with_original_effect` (typed Work/Allocation failures kept), close ladders through `close_original_effect_step`, source gets admission/failure/failure_close custody with settled status handling and staging advance through effects, `Box<dyn InferenceOwnedValue>` retired by an `InferenceValueRetirement` cursor (window-mutation pattern).
- NOT ported (callers/tests, other crates): plugin typing/edit (`built`/admit callers), flow wasm dag-input `step(grant)`, tests of the three crates.
