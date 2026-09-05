# Typed Child Publication Native Lifecycle Driver Blueprint

Status: **RED only for the new law’s driver.** Current source was inspected; no build or native test was run in this audit.

## Root cause of the 100,000-turn spin

`PluginApp::advance_typed_operation_publication` is deliberately a **Publishing-only** turn. Its inner selector returns immediately unless the chosen operation is in `MountedTypedCommandFullOperationStage::Publishing`; it never advances a `Worker` operation. [`plugin/🦀️.rs:22325`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22325) [..:22344](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22344)

An admitted typed command starts as `Worker`; dispatch gives it one initial worker opportunity but may leave the mounted session pending. [`plugin/🦀️.rs:23113`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23113) [..:23150](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23150) Repeating only the publisher therefore leaves the operation at `Worker`, produces no page, and says nothing about production liveness. This is a harness fault, not current evidence of a publisher failure.

## Actual production route

The reactor calls both halves of the protocol:

1. The live reactor schedules `plugin_step_live_cleanup` on every pass. [`reactor/🦀️.rs:1369`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1369)
2. That function submits a bounded maintenance job; its pump calls `PluginApp::maintenance_step`. [`plugin/🦀️.rs:30119`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30119) [..:30150](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30150)
3. Maintenance stage `0` selects one typed operation and calls `drive_worker_step` for `Worker`, or `retirement_step` for `Retiring`. [`plugin/🦀️.rs:24153`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24153) [..:24170](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24170) It is one of 21 maintenance stages, so a direct test must not assume every maintenance turn is a worker turn.
4. Only once the worker/session has reached publication does `plugin_continue_typed_operations` call `advance_typed_operation_output`, which runs the Publisher and obtains one ACK-owned result page. [`plugin/🦀️.rs:31235`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31235) [..:31280](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31280)
5. The reactor routes that page to the shell. The shell’s result-token message calls `plugin_acknowledge_typed_operation_result`; this is the real ACK ingress, not a test-only stage change. [`reactor/🦀️.rs:1496`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1496)

`Child` ACK specifically requires the retained child publication to still be `Committed`; it transitions it to `Acknowledged`, removes the exact page, and returns the mounted operation to `Publishing`. [`plugin/🦀️.rs:16381`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16381) [..:16403](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16403) The next publisher turns close that child owner, emit the terminal page, and only the **Terminal ACK** changes the operation to `Retiring`. Maintenance then owns terminal disposal. [`plugin/🦀️.rs:16415`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16415)

## Exact native-law driver

Base the new Child law on the existing real runtime driver, not on direct `advance_typed_operation_publication`:

- The existing `retained_operation_continues_after_command_admission_until_publication_and_retirement` creates a `PluginRuntime`, installs a real `RuntimeAppCell`, calls live cleanup + continuation, ACKs actual page tokens, then destroys and closes the app. [`plugin/🦀️.rs:34031`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34031) [..:34080](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34080)
- The existing direct child test already proves the minimum app-level pairing of `maintenance_step`, publisher, page take, and ACK. [`plugin/🦀️.rs:34088`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34088) [..:34111](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34111) It is not a runtime scheduler proof, so the new law should combine its real child command with the former runtime driver.

### Required driver sequence

1. Create the real `VcsArtifactApp<KeyedTestApp, TestMembers>` using its registered factory; bind a real receiver id; register the real child with the normal public app API; and admit the typed composite command through `dispatch_typed`/the registered command path. Do not build `MountedTypedCommandFullOperation`, force its stage, call `drive_worker_step` directly, or set `maintenance_stage`.
2. Put that app in a real `RuntimeAppCell` in a `PluginRuntime`, exactly as the existing runtime continuation law does.
3. On each bounded host turn:
   - call `plugin_step_live_cleanup(&runtime)` and propagate a runtime fault;
   - call `plugin_continue_typed_operations(&runtime).await` exactly once;
   - if output contains a result page, reject `Fault`, record its lane and token, and ACK **that returned token** through `plugin_acknowledge_typed_operation_result`;
   - use `std::thread::yield_now()` as the existing native driver does, allowing the admitted maintenance job and worker pool to run.
4. Require the ordered semantic milestones `Child` before `Terminal`; ACK the Child page before expecting the terminal page. The page is retransmittable, so always ACK the newest returned token—`take_result_page` changes its attempt number on redelivery. [`plugin/🦀️.rs:16363`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16363)
5. Stop only when continuation reports no further work. At that point require both milestones, no `Fault` lane, `!app.has_pending_typed_operations()`, and real parent/child snapshot values. The Child page must be observed before terminal, while its group remains ACK-owned; the terminal page must be observed before claiming retirement.
6. Release the cell only after live terminal proof. Call `plugin_destroy_app` and drive `plugin_step_close_cleanup` until its quarantine entry disappears, as the existing runtime law does. This proves no retained child publication, worker session, cancellation lease, or displaced root reaches `Drop` nonterminal.

The loop may use the existing fixture-owned turn cap, but it must fail with its collected observed lanes, `more` value, and runtime maintenance status—not silently spin. No direct app `close_step` belongs in the normal-success loop: it changes the operation into close/cancel semantics rather than proving live `Retiring` progress.

## Required exact assertions and hostile cases

The normal law should prove:

| Boundary | Required evidence |
| --- | --- |
| Worker | At least one runtime live-cleanup turn was scheduled; no direct private worker pump was used. |
| Child result | Exactly a non-fault `Child` page is observed, with a token accepted once; parent and target child show the committed expected values. |
| Child ACK | A wrong/stale token returns `false` and preserves the pending operation; the current returned token returns `true`. No terminal is credited before that ACK. |
| Terminal | A non-fault `Terminal` page follows the Child ACK and is ACKed through the runtime API. |
| Retirement | Subsequent live-cleanup turns remove the operation; continuation reports no work; destruction reaches exact terminal-empty closure. |

Add a separate hostile runtime law for withheld/duplicate Child ACK. It must observe bounded page retry and then the documented fault page/retry behavior; it must not claim completion merely because the worker stopped. This is where an actual ACK-liveness defect, if any, belongs—not in the success driver.

## Scheduler/publisher findings the corrected harness can expose

1. **No current source defect is proven by the publisher-only spin.** Publisher-only service is explicitly not a worker scheduler by contract.
2. **Normal external ACK is presently counted as zero progress by maintenance.** Stage `0` calls `drive_worker_step` for every non-retiring operation, including `AwaitingAck`; that returns `Blocked`. The maintenance watchdog faults after 256 zero-progress maintenance callbacks. [`plugin/🦀️.rs:24161`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24161) [`plugin/🦀️.rs:29460`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29460) [..:29474](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29474) Immediate ACK hides this. A withheld-ACK law must establish whether the intended outcome is the page retry/fault protocol or an unrelated `RUNTIME_MAINTENANCE_FAULT`; the latter would be a production scheduler RED.
3. **Pool submission contention is recoverable, not a successful step.** `plugin_step_live_cleanup` returns `false` after `try_submit` contention and restores `READY`; a correct driver keeps calling it while work remains. [`plugin/🦀️.rs:30140`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30140) [..:30150](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30150) A test must not convert a `false` submission into either completion or permanent failure.
4. **The real Child publisher is still legacy group dispatch.** `publish_mounted_typed_child_operation_unit` invokes `dispatch_emit_group`; this driver proves lifecycle/ACK/retirement only, not the separate retained atomic parent+child transaction currently filed as RED. [`plugin/🦀️.rs:22478`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22478)

## Acceptance

Credit the Child native law only after the runtime-path law reaches `Worker → Publishing → Child ACK → Publishing → Terminal ACK → Retiring → terminal-empty`, with actual host ACK calls and app destruction cleanup. The existing direct child lifecycle law is useful support, but it cannot replace this runtime driver. No browser/socket or atomic-composition claim follows from it.
