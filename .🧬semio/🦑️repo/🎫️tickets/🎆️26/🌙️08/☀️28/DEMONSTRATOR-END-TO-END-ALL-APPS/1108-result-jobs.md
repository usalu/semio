# jobs lane (1108)
## Step 1: lib compile, FIXED
- job-context: close_ledger (lib + testkit) takes &mut RetainedAllocationEffect (native W-jobs-2).
- job-return (continuation): import close_original_effect_step; frame.close_step(body=retained_grant().maximum_copy_bytes, effect) in capsule+origin.
- job-context source: into_source_loan/into_borrowed_parent pub (cross-crate use by park/return).
- job-park: OriginalSourceTurnWitness imported from job-context.
- Verified native W-jobs-5 rc 0, wasm32-wasip2 W-jobs-6 rc 0 (park, return, context).
## Next: tests via framework Cargo.toml
## Step 2: tests
- --tests compile (framework Cargo.toml) W-jobs-7 rc 0 for park + return. park has 0 tests; job-return has 2 required-features=testkit integration tests.
- Ported both job-return tests (acknowledge, view) off the HeapWitness global_allocator (conflicts with value-testing-allocator pulled by the testing caller) to observe_retirement_allocations (heap tuple). LOCK NEEDED: job-return Cargo.toml dev-dep semio-framework-value-testing-allocator.
- Run: tst.sh W-jobs-12 🧰️framework/Cargo.toml native -p semio-framework-job-return --features testkit -> rc 1, blocked upstream: semio-framework-value-testing-effect-bank 🎟️effect/🦀️.rs new 'pub mod drive' (repl, in progress) uses OriginalRetirementFailure/OriginalRetirementPoll without importing them (7 E0425).
- Rerun after repl fixes it: "$C/1105b-tools/runtime/tst.sh" W-jobs-13 🧰️framework/Cargo.toml native -p semio-framework-job-return --features testkit
STATUS: BLOCKED tests wait on repl effect-bank drive-mod imports (libs FIXED-verified W-jobs-5 native, W-jobs-6 wasm32-wasip2)
## Step 3: replication-retained join
- diff/sequence close_step: macros turn!/native!/poll! had lost $value/$call specifiers (16 errors) -> restored; replication-retained lib native W-jobs-14 rc 0, wasm W-jobs-15 rc 0.
- job-return tests rerun W-jobs-16: compile + run, both FAIL 'assertion failed: ready' (origin.admit never ready): return table is never born until the native-turn birth driver lands (lane driver; see 1108-result-birth.md). Not a jobs bug.
- Retained tests (claimed 🧪️tests/**): W-jobs-17 (--tests) now shows the lib itself red again (16 errors) because lane retained is mid-change making decide/inverse/close reply-shaped (RetainedMutationPreparation/LeafPreparation gain decide_reply/inverse_reply/close_reply; set-field, patch, 🧪️tests/🦀️.rs impls still old). Test port NOT started: it would target a moving trait API. Start once retained posts FIXED-verified lib, then port impls in 🧪️tests/🦀️.rs, 🩹️patch/🧪️tests/🎟️demand, set-field, diagnostic, span, diff, field tests; then check retained_row_patch! consumers (spr, replication re-export, paged-one-item) native+wasm.
STATUS: BLOCKED retained tests wait on lane retained's reply-shaped trait change; job-return tests wait on birth driver
## Step 4: receivers2 debt (denial + cancel-cut matrices, real closes)
- Shared driver value/🧪️testing/📥️admission/receiver-driver.rs now has: receive_cut (each advance turn first denied on every demanded axis: items 0, copy-1, capacity-1, release-1, depth-1 -> work 0, heap (0,0), default progress, never parks, quote unchanged; then funded; cut cancels after N turns), close (same denial matrix on begin_close_into and every close_step), retire (ControlledRetirement.step to terminal-empty), close_source (RetainedCloneSource.close_into to terminal-empty), lifecycle (receive/cancel, inspect, retire output, close receiver, drop, close source).
- Rewritten on it, no mem::forget: graph manifest (cancelCuts), ui text (cancelPrefixes), ui presentation run<T> (needs T: RetireOwned), expanded paths (cancelCuts). Paged retained_work_tests (value paged receive) not touched: it drives EffectBank directly, not this driver.
- UNVERIFIED: W-jobs-20 (graph-manifest --tests) stops upstream in semio-framework-replication-retained (retained lane mid reply-shaped trait change: unresolved retained_routed/settle_advance/RetainedMutationAdvance). Rerun: chk.sh W-jobs-21 🧰️framework/Cargo.toml native --tests -p semio-framework-graph-manifest (then ui contract, expanded paths) and tst.sh for the four tests.
STATUS: BLOCKED matrices written, unverified until replication-retained lib compiles
## Step 5: replication-retained tests (IN PROGRESS, not green)
- Fixed macro: retained_row_patch! expansion lacked RetainedCloneStep import (🩹️patch/🦀️.rs line 61).
- Harness landed: clone-source-testing drive::drive_reply<T,E> (generic reply+recover producer under a real original action; re-enters as recover after each Pending; asserts heap==progress, posts to the bank) in value/🧬️retained-clone/🔗️source/🧪️testing/🦀️.rs (unverified, W-jobs-23 queued); retained 🧪️tests/reply_drive.rs (step/denied/grant helpers) mounted as #[cfg(test)] crate::reply_drive; LOCK NEEDED retained Cargo.toml dev-dep semio-framework-value-clone-source-testing (added).
- Test port NOT done. --tests shows 73 errors in 8 files (W-jobs-22): tests/🦀️.rs (25), diagnostic tests (16+7), diff tests (7) and receiving (8), patch tests (4+4, need retained_empty_option_patch! + RetireOwned retirement_seed/RetirementCursor close_into doubles). The doubles in tests (Preparation, QuoteCursor, issuers) implement pre-733 traits: decide/inverse/close_step -> decide_reply/inverse_reply/close_reply (use retained::conclude with child poll Pending -> RetirementPollExit::Pending), mutations context is &Arc<dyn Factory> now, FactoryVectorRetirement::admit(original,output,effect) is poll-shaped. No already-ported RetirementCursor test double exists in the tree to copy (all reference doubles are pre-733), so each file is a first port. ~178 KB of dense tests over 17 files.
STATUS: BLOCKED retained tests unported (harness only); matrices unverified
## Step 6: return_original_cancel_alias_step effect-shaped (priority)
- Libs job-worker-authority + job-worker FIXED-verified native W-jobs-29, wasm W-jobs-30 (hub manifest). Signature now (witness, effect); NativeWorkerAuthority::close_original_worker_params(cx) runs cancel.close_step inside cx.with_original_effect and consumes its work.
- ~35 callers moved (see claims 00:58). Unverified callers: preview/headless (framework manifest build W-jobs-28 stops upstream in value-native-receiving-frame 'is_unexplained' on OriginalReceivingPrimary, not mine), renderer wgpu x6, plugin x3, services x2, tests x7. Rerun: chk.sh W-jobs-31 🧰️framework/Cargo.toml native -p semio-framework-job-preview -p semio-framework-job-headless (then plugin/renderer/services crates, then --tests).
STATUS: FIXED-verified libs W-jobs-29/30; callers unverified
## Step 7: caller verification attempt (00:20 11/09)
- W-jobs-33 (framework manifest: job-worker, job-preview, job-headless): no error on any alias-step site (preview 91, headless 262 type-check); remaining errors are unrelated API drift: preview drive 59 (no method step on S), headless 167/306-309/caller 74-76,131 (RetainedWorkPoll .result/.work on source close_step: Source close is poll-shaped now). Not mine, not fixed.
- W-jobs-34 (hub-wide): refused before compiler by a peer's source: semio-s-artifact-norm-en1991-inferences 'Unbalanced Rust source delimiter'. Renderer/plugin/services/ui callers therefore still UNVERIFIED.
- Retained tests / receiver-driver matrices: not run (framework graph-manifest build blocked by the same upstream churn; lock-124 dev-deps pending).
- Rerun when green: chk.sh W-jobs-35 🌎️hub/Cargo.toml native (then wasm), then tst.sh for graph-manifest/ui/expanded-paths matrices.
STATUS: BLOCKED callers unverified (upstream: norm-en1991 delimiter, headless/preview poll drift)
## Step 8 (00:50 11/09)
- preview drive: stale session.step(grant) -> session.step_on_original_caller(cx)? (OriginalPreviewSession has no step(grant)); headless 288 return_alias_step now effect-shaped via cx.with_original_effect. Other headless debt NOT ported: caller 🎟️caller admit_root(grant)/parent.close_step(grant)/authority.close_step(grant) (self-funded caller pays by grant), 167/306-309 cursor.step(grant) (ControlledRetirement::step(body,effect)), capture 111. Needs an effect source for the self-funded caller (design decision), so deferred.
- Many OTHER grant-based CancelTokenRetirement::return_alias_step callers remain (async cancel tests 10-46, ui tests 173/179/624/630/656, layout-input 110, candidate-baseline 199, frame-job 197/518/594, frame-preparation 148, host-base asset-decode 321, frame-actions 571, plugin session 56 / post-turn-relay 98 / router-effect 439, services compute-pool tests 39 / parameters 26, scene-intents 276, worker turn tests 46). Not converted yet.
- Build for these crates in the framework manifest: -p semio-ui-wgpu-engine semio-ui-host semio-renderer-wgpu-host-base semio-renderer-wgpu semio-plugin-job semio-plugin-retained-command (W-jobs-39 pending when written); semio-os-services and plugin-host-post-turn-relay are not in that manifest.
STATUS: BLOCKED callers partially ported; W-jobs-39 result pending
## Step 9 (01:50 11/09)
- job-headless FIXED-verified native W-jobs-40 (with job-preview, job-worker): HeadlessCaller takes every effect from OriginalRetirementPump turns: staged OriginalCallerSourceAdmission prepare/admit_backing/publish for the Source parent, parent/authority/cancel-retirement/failure-cursor closes via pump.with_effect, pay(work:u128), close_step returns OriginalTurnFailure; capture/main closures annotated RetainedWorkOutcome::<_,ValueError>. wasm not run.
- Product crates (root Cargo.toml, -p semio-plugin-job -p semio-plugin-retained-command -p semio-plugin-host-post-turn-relay) W-jobs-42 rc 1: blocked upstream in os store one-item publication (close_step arity on RetirementCursor/ErasedSnapshotRetirement, RetainedCloneSourceTake import, project_owned): not mine. My plugin call sites unverified. semio-ui-host / ui-wgpu-engine / renderer-wgpu / os-services are not in the root manifest; framework manifest rejects plugin-job: they need their own manifest (unknown) .
- Remaining grant-based return_alias_step callers (~30, list in Step 8) NOT converted: shapes are mixed (outcome vs Result), converting blind would only add unverified edits.
STATUS: BLOCKED callers unverified (store one-item publication red upstream)
- ui-host metrics-cursor-and-clipboard:188 fault close now close_original_effect_step(self.fault.close_step(effect)) (edited, W-jobs-56 pending)
- compute original-caller: cancel_birth (publish backing->close parent, return_inputs for roots, retire inputs) on every refused/failed birth exit + law cancelled_birth_closes_partially_staged_roots (unit test in 🦀️.rs; admission/authority Drop assert proves emptiness). EDITED, not compiled.
- job-worker 🧪️testing: OriginalFixtureSession::return_alias now effect-shaped; close_owner uses cx.with_original_effect (W-jobs-57 pending: services lib test cancelled_birth)
## Step 10 (02:55 11/09)
- Cancel path for partially staged roots: FIXED-verified. cancel_birth law passes (W-jobs-57, 1 passed). services lib W-jobs-55 rc 0.
- Open: ui-host close_step_granted port (edited, unbuilt); ~28 grant-based return_alias_step callers; headless/preview wasm; receiver-driver matrices (unrun); retained tests (unported); job-return tests wait on birth driver.
STATUS: FIXED-verified compute caller cancel law; rest open (see Open)

## FINAL HANDOVER (lane jobs, 11/09)
Pattern used everywhere (compiled, W-jobs-29/40/55): `cx.with_original_effect(G,|cx,effect|X.return_alias_step(cx.original_cancel_token(),effect))?.map_or(RetainedWorkOutcome{result:Ok(<default>),work:0},|(t,_)|t)`; owner-less closes: `OriginalRetirementPump::new().turn(fuel,deadline,clock,grant)` + `turn.with_effect(|effect|...)` (see headless `🎟️caller/🦀️.rs` and services compute `🎟️original-caller/🦀️.rs`). Inside `close_original_retained_step(cx,..,|grant|..)` closures use `close_original_effect_step(cx,demand,|effect|RetainedWorkPoll::Complete(..))` with a pre-borrowed `witness`.
### A. Remaining grant-based callers of `CancelTokenRetirement/CancelToken::return_alias_step(witness,grant)` (29 sites; convert against real compile errors using each crate's own manifest, `chk.sh W-x-N "<manifest>" native [--tests] -p <crate>`; paths relative to 🧰️framework)
| crate | manifest | site |
|---|---|---|
| semio-framework-async | `🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/⏳️async/⛔️cancel-token/🎟️alias/🧪️tests/🦀️.rs`:12 |
| semio-framework-async | `🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/⏳️async/🛑️cancel/♻️retirement/🎟️work/🧪️tests/🦀️.rs`:10 |
| semio-framework-async | `🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/⏳️async/🛑️cancel/♻️retirement/🎟️work/🧪️tests/🦀️.rs`:11 |
| semio-framework-async | `🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/⏳️async/🛑️cancel/♻️retirement/🧪️tests/🦀️.rs`:29 |
| semio-framework-async | `🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/⏳️async/🛑️cancel/♻️retirement/🧪️tests/🦀️.rs`:30 |
| semio-framework-async | `🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/⏳️async/🛑️cancel/♻️retirement/🧪️tests/🦀️.rs`:40 |
| semio-framework-async | `🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/⏳️async/🛑️cancel/♻️retirement/🧪️tests/🦀️.rs`:41 |
| semio-framework-async | `🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/⏳️async/🛑️cancel/♻️retirement/🧪️tests/🦀️.rs`:46 |
| semio-framework-async | `🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/⏳️async/🛑️cancel/🎟️admission/🧪️tests/🦀️.rs`:25 |
| semio-framework-job-worker | `🔨️modules/🧵️job/👷️worker/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/🧵️job/👷️worker/🎟️turn/🧪️tests/🦀️.rs`:46 |
| semio-os-kernel | `🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🎬️scene-intents-and-clipboard/🦀️.rs`:276 |
| semio-os-services | `🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🧰️compute-pool/🦀️.rs`:39 |
| semio-plugin-host-effects | `🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🌉️router-effect-handler-and-jobs/🦀️.rs`:439 |
| semio-plugin-host-post-turn-relay | `🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🧾️mounted-session-types-and-registry-struct/🦀️.rs`:98 |
| semio-plugin-job | `🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🚪️session/🦀️.rs`:56 |
| semio-plugin-operation | `🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/📦️lease/🦀️.rs`:62 |
| semio-renderer-wgpu | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs`:571 |
| semio-renderer-wgpu | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/📋️frame-preparation/🦀️.rs`:148 |
| semio-renderer-wgpu | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs`:197 |
| semio-renderer-wgpu | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs`:518 |
| semio-renderer-wgpu | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs`:594 |
| semio-renderer-wgpu-host-base | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏛️host-base/📦️packages/🦀️rust/Cargo.toml` | `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏛️host-base/📥️lifted/🔁️asset-decode-jobs/🦀️.rs`:321 |
| semio-ui | `🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/🖱️ui/🧪️tests/♻️physical-job-close/🦀️.rs`:173 |
| semio-ui | `🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/🖱️ui/🧪️tests/♻️physical-job-close/🦀️.rs`:179 |
| semio-ui-host | `🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/🖱️ui/🖥️host/🧪️tests/🔬️window-unit/🦀️.rs`:624 |
| semio-ui-host | `🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/🖱️ui/🖥️host/🧪️tests/🔬️window-unit/🦀️.rs`:630 |
| semio-ui-host | `🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/🖱️ui/🖥️host/🧪️tests/🔬️window-unit/🦀️.rs`:656 |
| semio-ui-wgpu-engine-surface | `🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🪟️surfaces/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🪟️surfaces/🧯️candidate-baseline-close-and-layout-worker/🦀️.rs`:199 |
| semio-ui-wgpu-mounted-layout | `🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/📦️packages/🦀️rust/Cargo.toml` | `🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🔤️layout-input-and-text-measurement/🦀️.rs`:110 |

Notes: async rows are tests of the leaf API (already effect-shaped upstream: pass an effect via the framework test driver `clone-source-testing drive`). Crates NOT in root/framework workspaces use their own manifest. Many of these crates are still red upstream (store snapshot-clone RetainedCloneSourceTake/close_step arity: storewire; replication-history: osvcs/repl; native-turn birth: driver), so verification waits on those.
### B. Receiver-driver matrices (written, never compiled/run)
`value/🧪️testing/📥️admission/receiver-driver.rs` (receive_cut/close denial matrices, retire, close_source, lifecycle; ownedtests added take_admitted) and the four tests: graph manifest `🕸️graph/🛂️manifest/🛬️receive/🎟️work/🧪️tests/🦀️.rs` (pkg semio-framework-graph-manifest), ui text `🖱️ui/🧬️contract/🔤️text/🛬️receive/🧪️tests/🦀️.rs`, ui presentation `🖱️ui/🧬️contract/🖼️presentation/🛬️receive/🦀️.rs`, expanded paths `🛍️products/💻️os/..🗂️expanded/🛬️receive/🎟️work/🧪️tests/🦀️.rs`; paged `retained_work_tests` untouched. Run: `chk.sh W-x-N 🧰️framework/Cargo.toml native --tests -p semio-framework-graph-manifest`, then `runtime/tst.sh`. Blocker then: replication-retained lib (retained: green) and owned crates.
### C. replication-retained tests (unported; 73 errors in 8 files, W-jobs-22)
Harness ready: `drive_reply<T,E>` in `value/🧬️retained-clone/🔗️source/🧪️testing/🦀️.rs` (verified W-jobs-24), `🎮️mutation/🧳️retained/🧪️tests/reply_drive.rs` (mounted `crate::reply_drive`), dev-dep semio-framework-value-clone-source-testing (lock-124). Port doubles to decide_reply/inverse_reply/close_reply (use `retained::conclude`, child Pending -> RetirementPollExit::Pending), `RetainedMutationCloseContext{mutations:&Arc<dyn Factory>}`, RetireOwned::retirement_seed + RetirementCursor::close_into for QuoteCursor, `retained_empty_option_patch!` for patch types. Files: 🧪️tests/🦀️.rs (25), ⚠️diagnostic tests (16+7), 🔺️diff tests + 🫴️receiving (7+8), 🩹️patch tests + 🎟️demand (4+4), plus span/key/field/set-field/change tests.
### D. Other open
- job-return tests (original-receiver-acknowledgement, original-continuation-view) fail `assertion failed: ready` until the native-turn birth driver (lane driver) lands.
- ui-host `close_step_granted` port (metrics-cursor-and-clipboard:188) built in W-jobs-60 (see below).
- headless/preview wasm: W-jobs-61 (see below).
### E. Final verifications
- ui-host lib native W-jobs-60 rc 0 (close_step_granted port verified).
- job-preview + job-headless wasm32-wasip2 W-jobs-61 rc 0.
STATUS: HANDOVER; FIXED-verified W-jobs-29/30 (worker libs), 40/61 (headless+preview native+wasm), 55/57 (services + cancel law), 60 (ui-host); open items in sections A-D
