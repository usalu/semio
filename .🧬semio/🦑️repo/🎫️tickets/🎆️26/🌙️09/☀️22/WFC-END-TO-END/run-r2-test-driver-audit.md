# Owned Test Driver Protocol

Read-only source audit. No test/build queue started.

Canonical current example: `🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🧪️tests/📦️capsule/🦀️.rs`, especially `normal_semantic_return_owns_its_receiving_witness_and_never_renews_quantum_one`, `actual_pending_work_keeps_its_original_claim_instead_of_charging_a_receiving_turn_every_quantum`, `source_fixture` and `close_origin`.

Keep one OriginalJobFixtureCaller and OriginalJobReturnOrigin::empty for the driver lifetime. On each caller.turn(grant, ...), nest cx.with_original_work_loan(business_fuel,cx.original_absolute_deadline_us(), ...). Inside, first origin.admit(cx); if ready call drive_step(&mut job,cx,origin.receiver()?,site,InteractiveStage::InteractiveStep,&mut verdict), otherwise treat as OriginalJobReturn::Preparing. The nested loan preserves the business step's actual tiny fuel while lifetime source fuel funds setup, returns, ACK and eventual close. The loan callback requires Copy output: return () and capture non-Copy observations outside the closure.

Match all four arms. Preparing did not call the business job. Pending keeps its unused original claim; do not reset/rebirth origin per poll. Outcome(reply) must convert reply.into_descriptor into a mutable Option and call origin.receive_outcome(cx,&mut input) on the genuine caller, retaining input if not received. Continuation likewise uses origin.receive_continuation. Do not silently drop an owned return arm. The normal semantic test proves business fuel remains zero after a quantum-one callback even while the held return credit pays reception.

For repeat semantic yields, inspect/borrow the received descriptor before repeated paid origin.acknowledge_outcome(cx) (origin implementation49/77 onward); ACK is not a replacement for receive. It is not necessary to close and re-admit origin after every Pending.

Close helper in canonical example loops at most4096 paid caller.turn(stock,|cx|{origin.close_step(cx)?;Ok(())}), asserts origin.terminal_is_empty(), then drops origin and caller.retire(stock). A real job additionally needs begin_close/close_step to terminal before dropping it; the example's jobs already assert terminal emptiness. Closing with an undelivered owned arm blocks until the same-source receive (explicit paid_cancel_close test).

Fixture lifetime stock from continuation/🧪️tests/🧫️fixtures/🔣️.json: 1,000,000 items and fuel; 4,194,304 copy and capacity bytes; 8,388,608 release bytes; depth128; constant clock Some(0), absolute deadline1,000,000. This is a reference for those small protocol tests, not evidence it covers every WFC fixture. Estimate WFC lifetime allocations/copies/steps plus return-frame/source birth and physical retirement; assert bounded completion and residual capacity. Caller constructor pays three roots from initial stock; stock/fuel cumulatively decrease across all turns. Caller Drop requires paid terminal closure. Never use business per-step fuel as the entire source lifetime fuel, never renew the caller per callback, and reserve enough release capacity/fuel for teardown.

Exact fixture caller implementation: `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/🦀️.rs`; admission9 onward, turn27 onward, retire_step50 onward, retire64 onward. Return API: continuation/🎟️origin/🦀️.rs34 admit,43 enter_body,45 receive_continuation,46 receive_outcome,47 outcome,49 acknowledge,50 close. These APIs are current source evidence; no native pass claimed.


## Proposed WFC Lifetime Contract Review

Read test-driver-contract.json with stock16,000,000 items,1GiB cumulative copy/capacity/release,depth256,fuel100,000,000,deadline decimal u64max. Finite lifetime grants are legitimate and do not eagerly allocate their maxima. They must remain one caller's cumulative stock; a per-turn1MiB grant only narrows remaining stock. This audit cannot prove adequacy without actual bounded WFC completion and teardown. Report exhaustion explicitly rather than increasing/renewing automatically.

All16 current manifest consumers of semio-s-plugin-wfc-engine-oracle are [dev-dependencies], verified by a WFC manifest walk plus global tracked exact-package search excluding ticket copies. No consumer outside WFC appeared. Thus internal semio-framework-job-testing-caller as oracle [dependencies] remains confined to test dependency graphs. The oracle package is not itself cfg(test), so placing the caller dependency in its own [dev-dependencies] would not make the public TestJobDriver library compile for downstream test consumers. Avoid reexporting unrelated framework test types unnecessarily. This is an internal repository dependency, not a newly introduced external runtime library.

Minimum authored source contract should include closed objects (additionalProperties:false), required stock/fuel/deadlineUs/clockUs/cases, stock integer fields with minimum1 and exact authored maxima, deadlineUs a decimal string const18446744073709551615 (do not parse via JS Number), clockUs integer const0, and a nonempty unique-id case roster. Each case needs explicit requested stepFuel, expected returned arm from preparing/pending/outcome/continuation, actual callbackVisits, expected acknowledgement for semantic outcomes, and terminal close expectations for job/origin/caller. Encode expected business fuel remaining/no renewal for the small-fuel case. Add a genuine ownedContinuation case independently of semantic Fault outcome: fault semantic metadata alone does not exercise failure ownership. Preparing should assert zero business callback visits; Pending should assert claim continuity rather than ACK.

Current JSON does not encode clock0, business per-step fuel, closure obligations or an ownedContinuation case. Those are coverage gaps in the fixture, not proof of a source implementation defect. The underlying framework source requires paid caller terminal close and same-source returned-arm receive before origin close. Native admission remains authoritative.


## Live Driver and Grid2d Migration Review

Read live oracle/🧰️step-context/🦀️.rs and grid2d fill/inference unit consumers. No concrete Rust lifetime/API defect found by source review; native compilation remains unverified. The generic visitor returns owned R through an outer Option and nested work loan returns (), satisfying Copy output. reply.original() is inspected while borrowed, then reply.into_descriptor() moves custody after that borrow ends. caller and origin are disjoint fields; callback borrows remain scoped within caller.turn.

The driver uses one admitted caller and origin, acknowledges the previous received descriptor before the next producer step, stages origin admission, matches all four current drive arms and receives an owned continuation before diagnostic panic. Preparing/Pending result visits correctly appear as None to existing bounded polling tests; tiny business fuel is actually applied through with_original_work_loan. No per-step source renewal appears.

Origin.close before job.close is appropriate for these direct driver consumers: origin.close_resources acknowledges/removes the descriptor before returning its own source loan. The original caller remains retained in driver, and subsequent job.close_step borrows that same live caller. Origin.close is not caller retirement. Source reference continuation/🎟️origin/🦀️.rs50–59; receiver close_resources acknowledges outcome before frame/credit cleanup. Final caller.retire occurs only after both origin and job terminal assertions. This ordering clears publication descriptor loans before producer page retirement. It does not replace the required job begin_close/close loop.

Grid2d fill drive_once copies PreviewReady payload to owned bytes before descriptor receive/ACK; inference likewise copies its25-byte preview and calls driver.close before returning. Both success and explicit bounded-exhaustion routes close. TestJobDriver close is idempotent only once the passed job is already empty, which is asserted. It intentionally cannot be reused for another nonempty job after source retirement.

Remaining evidentiary limits: visitor panic or owned continuation panic relies on test-unwind behavior rather than successful paid teardown; these paths must not be represented as successful lifecycle evidence. Test source budgets are finite cumulative ceilings and500k/2M iteration bounds alone do not prove sufficient stock; actual completion tests must pass. No source changes or queues were made by this audit.


## Portable Lifecycle Test and Remaining Migration Scan

Read current step-context/🧪️tests/🦀️.rs plus its contract.json/contract.schema.json. The Pending mode really calls the producer twice (first returns None after1fuel, second admits Yield); Yield and Cancelled use current semantic admissions. Continuation test catches driver panic only after asserting origin.continuation owns the received refusal, then invokes actual driver.close and checks all three terminal states. This is stronger than schema-only evidence, but not yet native execution evidence.

Refusal probe currently posts copied_items=usize::MAX through consume_retained. Exact context implementation248–252 first checked_adds reported effects to cumulative recipient, then records source, then returns grant refusal. With existing paid origin setup metadata, the MAX addition overflows before assignment, so the live recipient should stay unchanged; however the returned ValueError is decorated with retained_progress=MAX. If it were reached with zero cumulative items, the impossible receipt would instead mutate stock before rejection. This probe conflates callback refusal with impossible physical receipt accounting. Prefer canonical ReadOnlyFailureJob style explicit ValueError::literal(InvalidValue,...) converted to OriginalTurnFailure, zero effects; that exercises owned Continuation and clean close without claiming impossible physical effects. Source review alone does not prove current MAX probe cleanup passes.

Contract remaining coverage limitations: schema fixes array length4 but does not enforce unique IDs or required roster; four duplicate Yield rows validate. Native test does not assert clock/deadline exact correspondence, business fuel remainder/no-renewal, or intermediate Pending arm directly; businessSteps2 nevertheless proves the actual fixture's pending callback occurred. The finite lifetime stock is ample-looking for this tiny fixed4case fixture (<=128 polls each) but actual close remains the required verification. Do not claim the test covers the semantic Fault arm; current roster contains owned callback refusal instead.

Remaining old-helper scan (tracked WFC source): engine/💼️job/🧪️tests/🔬️unit/🦀️.rs14sites at62,75,89,167,205,245,279,285,313,325,351,358,399,447; engine/💼️job/📤️publication/🧪️tests/🔬️unit/🦀️.rs65; engine/🧪️tests/🔬️grid-job-drive/🦀️.rs29. These belong to native engine owner. No remaining artifact test_step_context hit. Sole current WFC drive_step call is oracle TestJobDriver's current6argument call; no old direct drive_step found. This is a point-in-time source scan during concurrent migrations.


## Static Integration Refresh

Concrete cross-component visibility issue: graph3d host inference factory-and-solve solve_with_job58 and solve_with_clock65 remain pub(crate). Their implementation is compiled inside the separate inference component crate; editor fill/preview and transient testing import through that component. They require pub visibility under cfg(testkit), not a production exposure. Parent notified despite the reported concurrent file write problem.

Concrete feature issue: graph3d editor manifest testkit forwards inferences/testkit only, and its dev-dependencies enable only inferences/testkit. Its normal transient component dependency has no testkit feature activation; editor and preview tests call transient::testing::solved_preview, which is gated in the separate transient crate. Forward transient/testkit from editor testkit AND activate transient/testkit in the editor devdependency (or its actual companion test owner). Transient own manifest correctly forwards inferences/testkit and its devdeps enable inference testkit.

Engine unit from_checkpoint call roster now supplies bytes,logical_now_us,test_headless_site at142,157,159,247,398; old test_step_context/root_cancel_token/StepBudget references no longer found in that unit file. The shared driver receives owned test outputs, and copied preview payloads do not escape borrowed outcome lifetimes. Five host unit suites now construct real inference parent jobs and close their owned drivers; no old solved-marker import appeared in the inspected suite headers.

Global WFC old helper scan now yields no test_step_context; only current6argument drive_step remains in oracle driver. No native compilation performed. Typed HeadlessFailure unwrap_err string equality was already migrated in bitmap host suite to an owned refusal variable; unrelated model/bridge unwrap_err equality remains legitimate.


## Current Final Static Revalidation

Graph3d solve helper visibility is now pub under testkit. Editor, schema companion and root package testkit forward transient/testkit; previous integration blockers resolved. Five host resumable_result helpers copy output bytes/fault text into owned values, close the same driver before decoding/return, and handle cancellation explicitly. No payload borrow escapes visitor scope; final descriptor is received by driver and ACKed by close. Engine restored checkpoint calls use current bytes/clock/site signature.

Engine cancellation tests call observe_original_cancellation then private step_value on the genuine caller, checking business metrics unchanged. These are direct body cancellation laws, not evidence that the public driver accepts an external cancellation handle or that browser controls were exercised. Their scope should be described accordingly.

Graph3d viewer now accepts cache,revision,locale,terminology and fences source before projection; authored fallback and native accessible status text are explicit. Source builder calls show no obvious mismatch; compilation remains required. No cold solve appears in reviewed viewer body.

Diagnostic refresh: enginejob3620lines/19files180114B still; proposed extra200test lines are not in this measured closure and would nominally produce3820. Fleet cfgtest:graph2dv13187,grid2dv12259,graph3deditor3802,grid3deditor3959. All are non-authoritative/not upper bounds; grid3d remains only41 nominal lines below4000 in this sample. Real native cfg must decide gate readiness. No queues started.

## Observer Boundary Follow-Up

At 2026-11-05T02:09:31.506Z, root read the current OriginalJobFixtureCaller::turn_original, StepContext::with_original_work_loan and drive_step implementation. Test completed_output visits/copies the borrowed answer after drive_step returns; drive_step qualifies the producer callback receipt before this caller-owned observer runs. The fixture turn checks explicit cumulative receipts and stock, and does not wrap all caller observation in a physical allocation measurement. Therefore test-observer bytes are distinct from runtime retained answer capture and are not evidence for it. Production headless has the separate state-owned paid PayloadCapture plus authored physical laws. No source change was justified by this concern; native execution is still pending.
