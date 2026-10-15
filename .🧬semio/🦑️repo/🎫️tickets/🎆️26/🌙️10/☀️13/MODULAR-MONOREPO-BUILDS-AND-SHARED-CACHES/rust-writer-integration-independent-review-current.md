# Rust Writer Integration Independent Review

Read-only Source follow-up, 2026-10-15. No gates, Originals, compilers, edits or physical deletion. Root supplied private 32 MiB prototype 2/28; not rerun here. Contract remains hard global Rust16 plus initial required physical headroom, never a per-mode physical quota. Exact remaining child seam sent to Nx.

Paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Prior generic-store self-wait removed

Generic compiler store no longer owns global writer through arbitrary callbacks. It retains real backing/toolchain/reservation as a reader scope. Thus the prior selected unit fixture holding a generic store while spawning a sealed compiler child no longer forces child writer to wait on its parent merely from that enclosing store.

`⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts` now requires actual issued NativeStorageReservation of compiler-workspace kind for `withCompilerWorkspaceWriteV1`. New outer writer holds the exact root EX, then observes actual available bytes and retains EX through awaited body/final checks. Same-process nesting reuses only private live AsyncLocalStorage root plus exact signal, not an inherited environment flag; each workspace wrapper still authenticates actual storage before reuse. Generic reader stores remain concurrent.

## P1: Preparation wrapper still holds writer across a writer child

`⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts` 24 invokes `withRepositoryCargoPreparationV1`, then inside its callback calls `prepareCargoWorkspaceInvocation` before actual cargo execution. Preparation owner `⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts` 34 wraps that complete callback in global writer custody. Workspace Cargo `🗂️workspaces/🦀️cargo/🟦️.ts` 387–388 spawns the synchronize child. Child preparation `📜️script.ts` 45 again enters `withRepositoryCargoPreparationV1` and requests the same writer. Parent waits for child while holding writer; the child cannot reuse main-process AsyncLocalStorage.

Narrow this actual seam to the child/in-process owner that performs preparation writes rather than parent launch scope. Do not propagate a writer-held environment boolean or bypass actual admission. This is a Source-proven wait cycle when preparation is required, not a runtime result from the supplied 32 MiB law.

## Writer call and lock placement

- Dependencies fetch owner `🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` 42 takes registry EX through ready owner, then writer only for real fetch. Resolution at 48 similarly takes registry EX then writer through actual producer. Warm ready SH body itself remains outside writer until actual compilation callback.
- Launcher `🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` 120–135 takes writer **before** Cargo build/store lease and CPU/scheduler resources, retaining through execute/consume. This matches maintenance writer-before-Cargo-store order.
- Test admission, executable preparation, exact Cargo body, native orchestration and native artifact build now explicitly wrap their real compiler callbacks. Their enclosing dependency registry scopes precede writer; preserve this order. Any nested callback that attempts a new registry writer while already holding global writer can invert with registry EX → writer, so broad callbacks must not be described as arbitrary safe composition.
- Direct unit cold path `⚡️caching/🦀️cargo/📦️units/📜️script.ts` 31/36 still takes output EX before global writer. Maintenance does not currently take unit output EX, so no concrete inversion with the **current** scanner is introduced. A future writer-held unit-publication reclaimer must use nonblocking output acquisition or change producer order consistently.
- Same-process nested writer reuse checks signal/root/live but does not independently enroll/drain a detached callback. Awaited callbacks are the supported lifetime; do not extend the 32 MiB law to arbitrary detached writers. A closing outer scope makes later nested calls refuse, but already accepted detached bodies would need real pending drainage to retain EX.

## Compact maintenance and preservation

`⚡️caching/🧹️pruning/🏪️budget/🟦️.ts` 53–54 now opens actual Rust image and selects compact mode layout, then takes writer without requiring initial growth floor. This permits maintenance on a full image. It retains cache-prune/store EX, real physical locks, process and descriptor refusal, recensus and per-operation deletion cap (37–46). Scan now combines recognized build and incremental units (35), improving actual eligible candidates while keeping unknown roots outside selection. Unique maintenance transcript turn directories avoid overwriting previous evidence.

It still does not claim every byte in b/p, b/r, old cargo/prototype/build or Nx is reclaimable. Existing old/unknown roots and independent readers remain charged unless separately authenticated. Actual before/after statfs and unmet target remain necessary; logical deleted-byte totals do not prove initial floor funded. Global writer protects only participating writers and cannot replace old physical process/descriptor checks.

## Failed outputs and publication

Unit cold writer surrounds scratch creation, actual compilation, receipt validation, bounded artifact copy/publication, rollback and final scratch retirement. Existing publication reservation refusal and previous-output restoration remain in unit script 37–39; the new headroom check must not bypass them. Existing compiler failure catches still preserve primary error while swallowing cleanup refusal; failed physical scratch is not proved absent after cancellation/full filesystem without separate cleanup evidence. Initial floor is finite admission, not a physical quota or success guarantee.

## Result

The generic-reader correction and current launcher/maintenance lock order address the previous broad store wrap problem. The preparation parent→synchronize-child cycle remains a concrete blocker before broad integration qualification. Preserve original guards and unknown roots; no Native success beyond supplied private prototype is inferred here.
