# Canonical Guest Close and Binding Copy Review

## Exact Guest Close Boundary

The coordinator directly read Plugin `🚪️lifetime/🦀️component.rs`, the reactor close admission, the canonical WIT event/result and shared actor-export macro, the generated bridge, and ShardClient close capture/receipt matching.

The native API is `plugin_capture_instance_close(runtime, instance_id) -> PluginInstanceCloseLease`. Capture stores the exact weak allocation identity. `begin_close(runtime)` checks that identity and admits at most once; `close_generation()` returns the checked native generation only after admission. `is_retired()` requires runtime COMPLETE and empty app cell, worker session, rejected owner and outcome, with the pump complete and not terminal. It never treats generic idle or quarantine removal as retirement.

The current guest exposure is absent. WIT `instance-close-event` contains only an instance; `turn-result` contains no close receipt. Reactor `poll_kernel` calls `plugin_destroy_app`, discarding captured close authority. The shared actor macro exposes only `poll` on the reactor interface; the generated JS API exposes no captured guest lease. A generated `closeInstance` branch cannot honestly synthesize accepted/retired receipts from the current ABI.

The native/Dag lane owns the canonical captured guest close integration. The demonstrator owns generated forwarding and missing-capability tests in its already-reserved materializer. Same-activation numeric-instance reuse must be included in capture design; checking a late close request against the current numeric ID is insufficient. No second ABI, synthetic generation, idle fallback, or host-effect-cancellation retirement credit is approved.

Independent actor R8 now actually passes78/78 across4files,1.29seconds,exit0,start18:11:36, using `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-actor:test --skip-nx-cache`.

Independent generated producer R1 passes10tests,49skipped,59discovered,3.23seconds,exit0,start18:11:57. Exact command: `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx run @semio-tech/framework-os-dev:test --skip-nx-cache --testNamePattern='pluginComponentBridgeSource |rewriteJcoComponentAssetUrls'`. This includes the newly added missing-native-close-authority refusal: no poll call, no receipt, and the original actor remains owned. Neither run is fresh Wasm.

The canonical integration decision is now recorded in `📓️guest-captured-close-boundary-2026-08-27.md`: runtime-issued guestLifetime at immediate open capture, retained Captured/Accepted/Retired receipt and exact ACK over the existing poll protocol. Dag owns canonical schema/Rust codec/Kernel/WIT/native mounting; Demonstrator owns TS codec/generated mapping/ShardClient/PluginRuntime create-close transport. Wire tags0..7 and44-byte maximum are agreed; no old-codec alias will remain. Implementation/acceptance are still open.

Further direct reactor review found additional terminal-authority joins: the native lease does not itself certify reactor request/task/resume/timer/metadata, pending patches/turn handback, patch tracker, job/render bindings or UI handbacks. `step_reactor_close` currently takes its cursor out of the fixed registry while advancing; fault/unwind recovery must keep the exact root structurally owned. Pending patch close registration currently silently omits a new closing instance when its fixed roster is full. These are assigned native integration findings, not grounds for returning a fabricated guest retired receipt.

## Binding Copy and Native Width Checkpoint

The coordinator read the new `🖱️ui/🧬️contract/🔗️bindings/📋️copy/🦀️component.rs`. It retains source, candidate and pending binding separately. Metadata/payload allocation, exact alias clone and placement occur in distinct advances; cancellation visits candidate, pending and source with the typed retirement cursor. The runtime's enclosing fault/unwind ownership and all remaining live whole-field operations still require their own proof.

Actual retained output reviewed:

- `🧪️surface-binding-clone-green-r15-native-2026-08-27.txt`: one test passed,89 filtered,0.016seconds. DEBUG:132turns,79,744allocated bytes,66,304initialized bytes, maximum allocation2,072 and maximum placement2,072 per advance.
- `🧪️member-ui-paged-copy-full-r31-native-2026-08-27.txt`:130tests passed,none skipped,3.384seconds.
- `🧪️member-ui-paged-copy-wasm-r32-2026-08-27.txt`: canonical check-wasm completed all three compilation stages in3.59/3.38/4.07seconds. This includes current generic paging and binding copy. It is compilation proof, not consumed guest/runtime/browser execution.

The coordinator subsequently read the actual R16/R17/R18 outputs. `🧪️surface-paged-ownership-r16-native-2026-08-27.txt` ran1PASS/1FAIL before the children-phase fallthrough repair. `🧪️surface-paged-ownership-r17-native-2026-08-27.txt` passes7/7,84filtered,0.208seconds; DEBUG covers ten binding cancellation frontiers with79,744..159,488owned bytes, exact terminal closure and zero allocation during close. `🧪️surface-runtime-regression-r18-native-2026-08-27.txt` passes90tests in1.835seconds with the single intentional inline-census RED explicitly excluded. This is not a full unexcluded runtime-green claim.

Process resident accounting, pre-producer admission, remaining Component/record field copy and deep comparison, and the all-app hard timing gate remain open; no byte quota or timing budget is raised.

## Preservation

No cleanup, deletion, relocation, git mutation, source restoration or output publication was performed by this coordinator. Two guessed source paths were absent; canonical locations were subsequently resolved with `rg --files`. Those lookup mistakes are not evidence-loss findings. The existing master ticket remains open; repo ticket tools are still not callable in this session.
