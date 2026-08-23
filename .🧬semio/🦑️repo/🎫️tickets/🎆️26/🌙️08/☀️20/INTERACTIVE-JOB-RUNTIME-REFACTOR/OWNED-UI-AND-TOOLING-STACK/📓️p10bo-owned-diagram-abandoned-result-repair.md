# P10bo Owned Diagram Abandoned-Result Repair

## Outcome

The remaining P10bn Diagram-owned rejection is repaired. `useDiagramLayout` now distinguishes a produced candidate from the last committed publication, retains both while a concurrent successor can still commit, and retires an abandoned candidate only after a newer lifecycle has committed an ownership reset. The authority ledger remains bounded, the displayed proxy is never retired by its own terminal callback, and unmount drains every retained authority. Dagre remains installed. No shared Port/browser-worker source, Rust/Cargo source, manifest, or lockfile was edited.

## Owned Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️component.test.tsx`
- This report.

## Commit-Acknowledged Ownership Protocol

The hook owns three distinct witnesses:

1. `activeGenerationRef` is the current job-generation guard. A stale terminal cannot publish after replacement or cleanup.
2. `candidate` is a result produced by the matching live generation but not yet acknowledged as displayed by a React commit.
3. `committed` is the last candidate promoted by a layout effect after the render displaying that exact authority commits.

The terminal callback only adds the new result to the owned ledger and requests candidate state. It does not close either the prior committed result or the candidate. A suspended transition therefore keeps both authorities readable and possibly committable.

When a newer source, option, or process-port lifecycle commits, its passive effect compares the small authority ledger with the authority actually displayed by that commit. If the commit abandons another authority, the effect queues an ownership-reset state update. Only the subsequent commit of that reset changes the lifecycle revision used by the retirement effect. This is the durable abandonment acknowledgement: the old suspended state has been superseded before its authority is removed from the ledger and scheduled for close.

Retirement invokes exactly one published-result `closeStep` per macrotask until completion. Successful replacement preserves only the newly displayed authority; source fallback preserves no stale proxy; unmount schedules every remaining authority. Equivalent inline option objects are tracked through scalar option values so an ownership acknowledgement cannot create a resubmission/render loop.

## Regression Evidence

- A committed result remains readable while a transition-produced successor suspends, then retires only after that successor later commits.
- An interrupted suspended successor is captured as a real proxy, remains retained while committable, and is drained after a third generation commits its lifecycle without displaying it.
- Releasing the abandoned suspension later cannot resurrect its retired result.
- A duplicate stale terminal cannot acquire ownership.
- Repeated first/suspended/successor replacements retain the currently displayed result, drain the abandoned result, then drain the former display after the successor commits.
- A source-identity fallback commits source data, drains the former displayed result, and cannot resurrect it.
- Unmount drains a produced-but-suspended successor.
- Every involved interactive consumer is closed through its cursorized `closeStep`, and `terminalIsEmpty()` is asserted after close.
- Existing hostile ingress, exact coverage, zero-node, count/byte boundary, cancellation, and publication-close regressions remain green.

## Final Gates

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run '🧱️elements/📊️Diagram/🧪️component.test.tsx'` | PASS — 1 file, 42 tests |
| `bun nx run @semio-tech/ui-react:test-long --skip-nx-cache -- --run '🧱️elements/📊️Diagram/🧪️component.test.tsx'` | PASS — 1 file, 42 tests |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS — Nx emitted its flaky-task notice |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS |
| `bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache` | PASS — 2 files, 32 tests |
| `bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker --skip-nx-cache` | PASS — boot and frame-worker bundles |

The Worker gate remains fake/static protocol coverage. No live browser Worker, Wasm, or OffscreenCanvas lifecycle was executed or claimed.

## Safe Boundary

This packet changes only Diagram-owned React lifecycle code, its Diagram test file, and this ticket report. It consumes the shared `readInputPage` / `onOutputPage` / `onTerminal` / `closeStep` / `terminalIsEmpty` contract as landed by P3 and makes no claim over shared Ports, the browser-worker scheduler, or Rust. `dagre@0.8.5` remains present in the React package manifest and Bun lockfile.
