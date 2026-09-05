# Retained Result ACK Maintenance

## Outcome

The retained typed-operation runtime now treats a presented result page as an external-input wait instead of runnable maintenance work. Result pages are one-shot until an exact ACK arrives, a fixed 64-slot maintenance scan skips presented ACK waiters without starving another runnable operation, and quiet worker/scheduler handback states no longer consume the structural-stall watchdog.

The runtime receipt drives a real parent-plus-child command through the production continuation and live-cleanup callbacks. It retains the original Child result token across 512 renderer polls, observes no duplicate delivery, processes an `AwaitingInput` maintenance transition, ACKs the original token, receives and ACKs the terminal page, verifies parent/child apply plus undo/redo, and retires the instance.

## Production changes

- `PluginCloseStep::AwaitingInput` distinguishes legitimate host or scheduler waits from structural `Blocked` progress.
- `PluginApp::has_runnable_typed_operations` and per-operation `has_runnable_work` exclude an already-presented `AwaitingAck` operation.
- `take_result_page` is one-shot; ACK remains the only transition that releases the retained result.
- The maintenance selector scans all 64 operation slots and advances its cursor only to a selected runnable owner.
- Worker handback, ordinary empty pump turns, transient pool contention/saturation, and pending take outcomes map to `AwaitingInput`; structural faults remain `Blocked` or faulted.
- The live-cleanup watchdog resets its zero-progress credit on a processed `AwaitingInput`. Test-only probes count actual app maintenance callbacks and processed input-wait transitions rather than queue attempts.

## Language-neutral receipt

The exact Bun/Ajv/source oracle passed:

```text
bun -e 'import { toolJobLatestWinsSelfTests } from "./📜️script.ts"; const checks = toolJobLatestWinsSelfTests(); console.log(`[DEBUG] tool-job-latest-wins-self-tests checks=${checks}`);'
[DEBUG] tool-job-latest-wins-self-tests checks=92
```

The fixture pins 512 pre-ACK polls, exactly one result delivery, and attempt `1`. Hostile source mutations reject a one-slot selector, reusable result delivery, worker-wait misclassification, and transient scheduler-wait misclassification.

## Native receipts

```text
env NX_ISOLATE_PLUGINS=false NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false RUST_MIN_STACK=268435456 bun nx run @semio-tech/framework-plugin:test --skip-nx-cache -- retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close
Nextest: 1 test run, 1 passed, 541 skipped
```

```text
env NX_ISOLATE_PLUGINS=false NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false RUST_MIN_STACK=268435456 bun nx run @semio-tech/framework-plugin:check --skip-nx-cache -- --tests
Finished dev profile in 1m 37s; exit 0
```

The first native attempt compiled successfully but used unsupported libtest-only arguments with a nextest target; nextest rejected those arguments before executing an assertion. It is not counted as a test receipt.

## Explicit nonclaims

This packet does not make the current sequential `dispatch_emit_group` path atomic, add Flow `addWidget` to the app-owned retained factory registry, or repair generic retained-command pre-publication `ChildEmit` retirement. Those are separate active packets. Existing compiler warnings were not introduced or treated as failures.
