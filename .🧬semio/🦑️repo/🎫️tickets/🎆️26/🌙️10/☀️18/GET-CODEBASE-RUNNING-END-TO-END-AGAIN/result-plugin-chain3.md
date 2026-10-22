# result-plugin-chain3 (executor plugin-chain3, wave 1022a)

Tools: `C/plugin-chain3-tools/{run.sh,chain.sh}` (`sh chain.sh TAG [--update] cargo-args`). Logs `C/🗑️generated/plugin-chain3-1022/`.

## 00:25 status
k1 (check of typed-ops, reactor-core, tool-run-turns, host-guest-runtime, dag-host, native): reactor-core, tool-run-turns, dag-host GREEN after the edits below; typed-ops + host-guest-runtime red (visibility, fixed after k1, k2 running).

### Edits
- reactor-core: `tasks_task_lane::TaskRecord` -> `pub` (iterated through `TASK_RECORDS` in `checkpoint_now`).
- tool-run-turns: `publication_ingress` -> `pub mod`, `ToolRunPublicationIngress`, `ToolRunRefold`, `ToolRunTraceWindow` -> `pub` (ledger fields of the instance crate).
- typed-ops: dep `semio-framework-value-native-owned`; `ToolPublicationScopePermit` (plugin-operation), `MountedCommandEntryRefusal`, `MountedGroupReceiptOutput` (publication-mounted) -> `pub`.
- host-operation: `Received` pub + re-exported (snapshot call signatures); `OperationHostStore::operation_clock_us(clock_binding)` (WIT `operation-clock-us(clock-binding)`): trait, wasmtime func_wrap, imports host, describe host, test State.
- host component-codec: `GUEST_CODEC_BUDGET` ported to the new `kernel::Budget` (retained work, policy, work_fuel, deadline_us).
- dag-host: `web_sys::HtmlCanvasElement` / `wasm_bindgen::prelude::*` imports gated like their only users (browser wasm).
- Owned original job protocol (guest + host), was a half-port (WIT/wasmtime and trait already new):
  - guest `semio_owned_start_job_v1` now takes `owned_abi::StartJobInput{source,budget}` and returns the fixed `SJH2` receipt (`exports::start_job_owned`); dead `StepJobInput/CancelJobInput` removed; `owned_abi::take_actor_start_job_input`.
  - interpreter runtime: `OwnedOperation::{BorrowJobOutput,AcknowledgeJobOutputChunk}`, `begin_owned_scalar_operation`, `OwnedRuntime::{start,step,read_output,acknowledge_output,cancel,close}_actor_job` (read = read receipt -> borrow page -> copy -> acknowledge chunk inside one crossing, one item + one fuel unit reserved for the handback).
  - host adapter `GuestRuntime for OwnedRuntime` delegates to those.

## 01:15 status
- k2 (`check -p instance-typed-ops reactor-core tool-run-turns host-guest-runtime os-infinite-board-dag-host --keep-going`, native): rc 0, 0 errors (13m38s, mostly cold third-party).
- k5 (`check -p semio-framework-plugin-instance-app --keep-going`): rc 0, 0 errors. Extra fixes on the way: instance-ladders (`ReferenceSpaceMember` import in `lifecycle-stores`, `full-operation`), `PeerRosterFault` + `ToolLatestWinsScope` -> `pub`.
- Test doubles ported to the original job protocol (only compiled under `testkit`/`test`, UNVERIFIED): `MockGuestRuntime` (scripted `JobStep` -> `OriginalJobOutcome` + one offered output page, `MockJobOutput`), `RecordingRuntime` (`last_job_budget: Option<OriginalJobBudget>`), the poisoned-instance owned-runtime law now calls `start_actor_job(None, budget, execution)`.
- k6 running: gismap workspace check.
