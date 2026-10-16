# Paid Direct-Child Custodian Foundation

2026-10-15. Implemented Source foundation; the production marker bypass remains deliberately unchanged. This is a direct-child implementation and does not establish general descendant-tree custody.

## Actual Ownership And Narrow API

The new `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🛟️custodian/` module implements `withPaidProcessCustodianV1(original, plan, callback)`. The input Original must pass the actual existing Source private issuer. The callback receives a frozen request/result port exposing only `run(index)`. Each port request is registered with the actual `withIssuedResourceOperationV1` before its first send. The parent client cannot spawn a declared job locally in response to any broker message. A result is captured data and never creates a CPU grant.

The closed schema requires explicit `coverage:"direct-child"` and fixes a finite plan before physical admission: at most 32 declared programs, bounded arguments, program/working-directory paths of at most 256 UTF-16 units, an exact serialized UTF-8 plan cap of 1 MiB, bounded separated outputs and per-job worker counts no greater than the reserved pool. The actual custodian is spawned with an inherited private IPC channel and the same serialized Original deadline. It receives its own actual Source Original through the existing process receiver. Only that process calls the existing real capacity API, with `environment:{}`. It therefore acquires the real shared host gate and exclusive per-core SQLite transactions instead of interpreting the old inherited marker. No additional global namespace, inode replacement or Core issuer is introduced.

The custodian accepts each declared index at most once, refuses concurrent requested counts that exceed its actually paid pool, and sends only bounded result/refusal data. The worker environment strips the old capacity boolean/count. The declaration, channel metadata, result schema and private parent port never authorize an unpaid local compiler callback.

Callback return sends drain, stops births and awaits the custodian's close. Caller cancellation sends cancel and still joins the custodian; it does not kill the custodian's process group. Actual parent death closes IPC, stops births and aborts accepted captures. The custodian drains every accepted owned capture and only then releases its actual gate/core transactions. This retains the real grant while direct-child work is settling after parent death. It does not rely on eventual child EOF alone.

## Executed Source Proof

Ticket `cpu/📜️script.ts custodian-strict` runs `--strict --noUncheckedIndexedAccess` over the production module, server entry, test, cancellation/crash actor, forged-wire actor and inert worker. The actual Source root is privately issued using existing process APIs. The strict run succeeded. `custodian-runtime` runs the registered process suite under that same actual Source root; the final corrected runtime passed 88 expectations in 11.12 seconds with exit 0. Final strict also passed with exit 0 after the server-Original failure correction.

Actual runtime checks include inert paid child execution, Python SQLite core contention while that child is live, concurrent over-allocation refusal, callback-return drain with the paid core still unavailable, index replay refusal, escaped port refusal and cancellation through an actual derived Source Original. The canceled child realm is asserted outside its actual receiver: its Original fails after settlement, its observation process reports that fact, and the observing original parent remains live and independently verifies the core is free. Caught and detached refusal are likewise asserted outside a separate genuine receiver, so no failure ledger is cleared.

The macOS crash test starts a genuine separate caller, waits for its inert direct worker, kernel-stops the custodian, then kills the caller itself with SIGKILL. Independent Python SQLite still observes the custodian's actual paid core locked after the launch parent has died. Resuming the custodian allows its actual IPC-disconnect cancellation path to join its owned direct child and release the core. Test-only PIDs address SIGSTOP/SIGCONT; production capacity authorization and death settlement use no PID inventory or PID liveness check.

The forged-wire actor waits for a real paid custodian, sends a syntactically valid result in the request direction, and verifies settlement without the declared worker ever writing its startup marker. Independent Python observes the real paid core before the forged request and a free core after settlement. Ajv and the repository schema admission cover the same closed plan/result direction cases.

Hostile laws kill an actual idle custodian after its worker has closed, then throw an Error or `undefined` from the callback. The client retains the exact primary failure using an explicit boolean even when settlement also fails. A real missing-executable capture failure now proves the actual custodian Original fails after physical settlement: bounded refusal is sent, the same caught failure is rethrown inside its accepted operation, pending work drains and the paid core is released before the server exits with status 1. Nonzero command status remains captured result data, as required by the existing capture contract.

Finite no-activity laws cover both an idle paid custodian and a custodian blocked awaiting capacity. The deadline actor is an explicit authored test entry that refuses an inherited `SEMIO_SCRIPT_PROCESS_INVOCATION`. Its observer deliberately clears that field and only captures evidence. The actor authors one real 1-second Source envelope before actual custodian child birth; the production client transfers that exact Original and absolute expiry to the custodian receiver. Reported facts prove expiry includes startup and ready/preparation: deadline minus start is 1000 ms, readiness precedes expiry when admitted, and final failure follows absolute expiry. No timer is reborn after readiness and no fresh root is created inside an inherited receiver.

The initial foundation runtime exposed expected cancellation being caught inside the same Original realm: the Original correctly retained that child failure and refused final success. The test now places the cancellation case in a separate genuinely issued child realm, checks its actual failure status and checks the observing parent's final successful settlement. No structural Original alias or failure suppression was added.

No Cargo/compiler cache/store setup, Nx graph recovery, canonical frozen rebuild or compiled/native frozen Original proof was run. The old inherited-marker regression remains a separate deliberately failing production gate.

## Explicit Direct-Child Limit

Existing owned capture awaits a native direct-child `close` event. Its current tree termination helper uses a `ps` snapshot with process-group/PID signals on POSIX and `taskkill` on Windows. Those operations are useful termination attempts but do not prove that a daemonized or escaped descendant has died. This foundation cannot become the production compiler admission route until a validated native owner joins every accepted descendant. A custodian crash itself is also outside the qualified guarantee; killing the process that owns SQLite can release its transactions while a worker remains alive.

The existing physical cleanup reserve is specifically closed over `kind:"native-image"`. This lane did not widen that unrelated Core enrollment or manufacture cleanup authority from its structural ports. Current direct-child capture joins after cancellation with the existing actual operation lifetime. A future native tree provider needs an explicitly authored cleanup/lifetime contract reviewed at the proper owner.

## Native Owned-Tree Provider Design

The proposed own-code interface must return a privately issued live owner from actual native creation, expose bounded job birth, stop births, request cancellation and asynchronously join actual kernel-owned work. Its final empty/dead result must be created only by the provider holding those native handles. Serialized results are diagnostics; callers cannot recreate ownership from a handle number, PID, process name, receipt or schema-valid object. Framework/OS ABI types remain behind the implementation interface.

Linux candidate: create a dedicated custodian as a subreaper before any job birth. The Linux manual confirms orphaned descendants are adopted by the nearest living subreaper, which can wait for their termination; the setting survives exec but is not inherited by children. A native owner must stop new births and reap both direct and adopted children until the kernel reports no owned children, while preserving the actual capacity reservation. This is a candidate design, not a Linux runtime claim. Cancellation also needs a real ownership-preserving signaling strategy; `ps` snapshots cannot supply that authority. [Linux PR_SET_CHILD_SUBREAPER manual](https://man7.org/linux/man-pages/man2/PR_SET_CHILD_SUBREAPER.2const.html)

Windows candidate: create each process suspended, associate it with an unnamed owned Job Object before resuming, forbid breakaway and retain the actual job/process handles until confirmed empty. Microsoft documents descendant association, job termination and the fact that ordinary completion-port messages can be lost. Absence of a notification is therefore insufficient. Query actual job accounting through the owned native handle and check all API failures. Suspended creation/assignment ordering and ABI layouts still require platform validation. No Windows runtime test was run. [Microsoft Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects), [QueryInformationJobObject](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-queryinformationjobobject)

macOS remains open. The actual installed SDK `usr/include/sys/event.h:355-361` explicitly states that `NOTE_TRACK`, `NOTE_TRACKERR` and `NOTE_CHILD` have not been supported since 10.5. The suggested kqueue descendant-tracking implementation is rejected. `usr/include/sys/spawn.h:60` defines `POSIX_SPAWN_START_SUSPENDED=0x0080`; that flag alone does not establish descendant ownership. No general tree-death guarantee follows from start-suspended plus ordinary NOTE_EXIT. A supported durable ownership primitive or an explicitly restricted process model is needed before integration.

The local process module search found no existing subreaper/Job Object/native descendant owner. No ABI/runtime claims for unexecuted platforms are made, and no speculative native provider was added to production.

## Changed Files

- New custodian `🟦️.ts`, `📜️script.ts` and closed `🧬️schema/🔣️.json`.
- New custodian test `🧪️tests/🟦️.ts`, neutral fixtures and worker/cancellation/forged-wire/explicit-deadline `📜️script.ts` entries.
- Process `📜️script.ts` and `📋️project.json`: isolated custodian suite and Nx route.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: Nx and ticket-owned strict/runtime commands.
- Ticket `cpu/📜️script.ts`: additional custodian strict/runtime selections.
- This report and the earlier CPU execution report.

All generated test inputs/SQLite outputs are isolated under a unique directory at this ticket's generated root and removed by the test. Other lanes' generated output remains untouched.

Final validation sessions 61959 (strict) and 13675 (runtime) completed and were collected; no live terminal handles remain. Earlier completed sessions were also collected. No CPU test generated directories remain.
