# Core Compiler Cache Custody Audit — Current WIP

Read-only Source inspection on 2026-10-16; ongoing extraction is not frozen. No runtime, Native fixture, physical effect, new Original/control, or git mutation was performed. AGENTS.md and the extraction plan including its authoritative factory correction were read. This report remains open for Storage’s coherent handoff.

## Concrete Findings Sent to Storage

1. Core `💾️cache/📋️model/📨️receiver/🟦️.ts`, `run`: `Promise.resolve(body())` invokes caller work before `pending.add(work)`. A synchronous body can call `close()` while the pending set is empty. That close returns without joining the accepted operation. Enroll before executing caller work and refuse same-frame close to avoid self-join deadlock. The model transition does reserve a bounded accepted entry before body invocation, but the draining promise set does not yet reflect it.
2. Repository `💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts`, final opener catch: `try { await image?.close(); } finally { await leases.close(); } throw error` allows either cleanup rejection to replace the original admission failure, including thrown `undefined`. Its frozen wrapper close similarly allows lease close to replace actual-image close failure. Preserve an explicit failed boolean and ordered error collection through both owners; always attempt both closes.

## Verified Source Limits

The newly appearing Core files are policy/schema/model/receiver/inventory/descriptors/layout. The actual volume lifecycle and closed handoff were not present at this read. This is known WIP, not an execution blocker by itself. The canonical platform opener currently directly returns the real host backend result. The partition factory currently returns a frozen spread of that result with dependency-aware close. Compiler factories currently create distinct frozen base and toolchain-bearing full stores. None can safely be authenticated by reconstructing an image from root DATA or by enrolling its spread as an image alias.

Core policy currently validates a closed schema, freezes captured primitive fields, requires charge = backing + parent metadata, filesystem + 1 MiB = backing, MiB alignment, and minimum available <= filesystem. Schema enforces the existing 33 MiB backing and 32 MiB filesystem floors and bounded maxima (including 32 accepted receiver operations). These are Source facts only; no physical capacity qualification was run. The lifecycle must still enforce parent floor, actual reservation identity and exact same backing envelope.

The receiver ledger stores a separate `failed` boolean, so an accepted rejection with `undefined` is retained as failure; `close()` checks failure-array length rather than truthiness. This does not establish body-priority or physical receiver-close ordering; those belong to the still-changing lifecycle.

## Pending Handoff Review

Check exact factory-only identity maps for canonical real image, public partition wrapper, both compiler store objects; reject copied/mutated/foreign/after-close objects before effects. Check synchronous enrollment, own-frame close refusal, descendants drainage, actual child receivers closed before retained parent operation settles, real image before lease dependency close, body/accepted/close error order including undefined, original allocation/control refinement, parent floor and complete backing+metadata charge. Re-read actual patches rather than promoting plan text to implementation evidence.

## First Moving Source Re-read

The Core ledger now pre-enrolls its exact promise and runs caller work inside a private AsyncLocalStorage frame. `close` refuses its own accepted frame before returning any closing promise. This resolves finding 1 at Source level; no tests were run by this auditor.

The canonical platform opener now owns a private WeakMap populated only by the actual canonical factory. The helper compares captured root/image/envelope/port/recheck/close/maintenance and dispatches the captured port’s genuine `withOperation` after private control-refinement validation. Copied image wrappers have no map key and refuse. Retired-image refusal delegates to the real port owner; physical behavior has not been tested.

New moving-patch issue sent to Storage: `openNativePlatformStorageV1` reads `options.control` for its map after awaiting backend open. Backends capture opening input separately, so caller mutation during that await can associate the issued object with a different control. Capture opening options/control synchronously before dispatch and use that captured value in the map. This is an exact-opening-authority requirement, not permission to adopt structural wrappers.

The next platform re-read shows `options=Object.freeze({...options})` before host dispatch, resolving the asynchronous opening-control snapshot issue at Source level. The repository bridges are still WIP at this checkpoint.

## First Repository/Core Lifecycle Handoff Read

The moving patch now has both repository bridges and Core volume lifecycle. Factory-only partition and base/full compiler-store maps are present; partition operation hands the private actual image to Core rather than its public spread. Partition work is pre-enrolled in owner/frame promise sets; close refuses its own live frame, stops top births, drains accepted work, then attempts actual image before lease dependency. Ordered outcome records preserve undefined and admission cleanup now retains the initial error and all distinct cleanup errors. This resolves finding 2 at Source level.

Actionable moving-patch gaps sent to Storage:

1. Core initial admission checks reservation membership but does not call the supplied allocation’s real `recheck()` before physical effects. It also dropped the old explicit parent maximumWorkspaceBytes / initial floor equality. The passed parent policy is not tied to the supplied reservation’s privately captured original policy, so a generic caller may present altered policy DATA. Preserve floor/envelope/charge relations against actual issued owner facts, not caller assertions.
2. The existing reservation checker stores only AbortSignal in its private map; it intentionally does not prove exact allocationControl identity or captured policy. Checking that helper plus a caller-selected refinement is insufficient evidence for the plan’s exact allocation-control/policy claim. Do not widen its public semantics silently. The compiler owner does retain exact storage/control/policy; generic Core admission still needs an appropriate existing genuine owner relation.
3. Both repository bridge entry paths shallow-copy request DATA and defer Core policy capture to an asynchronous child/write callback. Caller mutation of policy fields before that callback can change the accepted policy. Capture closed primitive policy/parent policy/key/content at bridge admission before yielding.

These are observations of the first moving patch, not frozen final findings; Storage is actively correcting Source. No effect or test was executed by the auditor.

## Frozen Audit Handoff Checkpoint

Root requested this audit slot return while implementation continues. A final bounded re-read found actual corrections now present: heldReservation privately records exact issuance control plus frozen captured policy; a separate read-only accessor checks exact control and returns those facts without changing the existing signal-only checker semantics. Core compares complete parent policy keys/values against those issued facts, restores the initialParentFloorBytes equality, and awaits the supplied allocation’s real recheck immediately inside retained parent operation before subsequent lifecycle work. Compiler and partition bridge entry points now synchronously capture closed policy/key/content, with partition additionally freezing parent policy before its child callback. These specific moving-patch findings are addressed at Source level.

Remaining status: WIP, not final qualification. This auditor did not execute tests, inspect final Source hashes or review a coherent completed handoff test cohort. A successor should review returned-phase lifetime and nested partition/frame drainage across final code, copied/mutated/foreign/after-close negative tests, exact floor/envelope and reservation-control cases, and error/undefined ordering in the final Native-backed lifecycle. Physical image interleaving remains untested and unauthorized here. No unresolved concrete defect from this audit is asserted after the final bounded re-read; that is not a completion claim for the extraction.
