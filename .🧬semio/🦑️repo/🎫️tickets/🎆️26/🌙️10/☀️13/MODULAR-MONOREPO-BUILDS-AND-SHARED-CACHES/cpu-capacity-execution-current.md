# CPU Capacity Execution Proof

2026-10-15. Source regression and design boundary stage. Production marker borrowing is still unfixed; no CPU-sharing completion claim is made.

## Implemented Gate

`🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/` now contains a closed neutral observation/result schema, a JSON scenario corpus, an owned inert child script and a falsifiable regression. Ajv and the repository validator admit the same expected observation and reject the same malformed results. The parent acquires the actual all-host exclusive grant; independent Python SQLite transactions observe the real gate busy before and after the separately owned child. The child receives the same original process envelope through the actual owned-capture path and obtains its own privately issued Source Original control. It authors only the old boolean/count capacity markers and requests one core. There is no CPU workload.

The new process-script suite is `test compiler-capacity-inheritance`; the Nx target is `@semio-tech/framework-process:test-compiler-capacity-inheritance`. The existing honest capacity suite is unchanged. Launch and seed contain the new Nx suite and direct strict/runtime harness commands. Ticket-local `cpu/📜️script.ts strict|runtime` supplies an actual privately issued Source root with existing `createScriptProcessEnvelope`/`withScriptProcessEnvelope` APIs and follows the actual Source owned-capture path. It neither copies a structural invocation into private custody nor bridges Source to frozen Original.

## Executed Evidence

- Strict TypeScript (`--strict --noUncheckedIndexedAccess`, the test and child graphs) completed with exit 0, including through the ticket-owned Source invocation harness.
- The Source runtime gate completed with exit 1 at the intended assertion. Actual observation: `parentExclusiveBefore=true`, `parentExclusiveAfter=true`, `childEntered=true`, `childCoreDatabases=0`. Expected `childEntered=false`. Python independently observed SQLite gate contention while the unpaid marker admitted the inert child. Bun reported 20 expectation calls, including the deliberately failing comparison and the successful release check in `finally`. The actual parent lease was released in `finally`, independently observed free afterwards, and all owned children were awaited to close.
- The Nx route was attempted once and stopped before the test at `Active graph readers retain required Nx pool headroom` in `prepareNxGraphCacheStorageV1`. No recovery, storage rewrite, setup or Cargo run was attempted. The direct Source root avoids that unrelated graph-storage prerequisite while preserving genuine private Source issuance.
- A first harness run stopped before admission because adding a `cpu` output subdirectory made the full lease queue-ticket path exceed 256 UTF-16 units. The harness and launch now use the ticket's generated root directly; no guard was weakened.

Launch/project JSON parsing succeeded and new Source paths were measured below 256 UTF-16 units. The read-only `git diff --check` attempt returned `error: daemon terminated`; no successful diff-check claim is made. All test-owned generated directories were removed without touching other lanes' generated output.

No compiled/native frozen Original proof was run or claimed. Positive inherited subgrants, over-allocation, replay, drain, cancellation, expiry and crash custody remain required before a production implementation is complete.

## Required Production Boundary

The inspected actual gate/core leases are SQLite transactions owned by the process executing `acquireResourceLease`. Owned capture joins its direct child and can request tree termination; it does not transfer an actual SQLite transaction to a live child. A private local registry plus an inherited IPC channel can narrow honest delegation but does not, by itself, retain the parent's actual gate after parent death. An unpaid IPC parent could also reproduce arbitrary handshake data unless admission is rooted in the actual paid issuer.

The production change must place actual gate/core ownership in a custodian whose lifetime includes every accepted compiler tree. Child work must be started only through that custodian's actual owned-spawn boundary, receive bounded subgrants from the paid issuer, and settle before that issuer releases its SQLite transactions. Parent EOF may trigger cancellation but cannot release the reservation until actual work death has been established. Existing source/frozen issuers must stay exact; the fix must not authorize copied control objects, marker data, PIDs, heartbeats or another global namespace as capacity.

No production marker removal was shipped: independently reacquiring while an honest enclosing all-host reservation remains held would deadlock nested compiler paths. This stage supplies the executable failing proof and preserves that defect visibly for the subsequent custodian implementation.

## Files Changed By This Lane

- `🧰️framework/🔨️modules/🏃️process/📜️script.ts`: isolated test suite registration.
- `🧰️framework/🔨️modules/🏃️process/📋️project.json`: matching Nx target.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/🟦️.ts`: actual exclusive owner and unpaid child regression.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/👷️child/📜️script.ts`: inert owned child.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/🧬️schema/🔣️.json`: closed neutral result and observation schema.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/🧫️fixtures/🔣️.json`: forged marker case, expected refusal and malformed results.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: Nx, strict and runtime launches.
- Ticket `cpu/📜️script.ts`: genuine Source root harness.
- This report.

No live terminal handles remain from this lane. Sessions 33254, 43256, 61185, 67957 and 9121 completed and were collected. Both successful final strict and deliberately failing final runtime runs were collected. Ticket/global generated directories belonging to other lanes were preserved.
