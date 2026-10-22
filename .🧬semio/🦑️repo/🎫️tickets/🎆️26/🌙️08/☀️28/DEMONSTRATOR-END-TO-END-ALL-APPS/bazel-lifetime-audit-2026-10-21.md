# Bazel Graph Maintenance Lifetime Audit

Date: 2026-10-21. Bounded current-source reads only. No production edit, Native job, process signal, heavy parser import or mounted cache traversal.

## Finding

**The proposed same-image self-drain hypothesis is not confirmed by the current call graph.** Current graph artifact preparation settles its own reader callback before storage preparation; the Bazel preparation branch opens other partitions, not graph maintenance. Genuine Native maintenance refuses same-owner accepted-operation close instead of awaiting that operation's own drain.

This is source evidence, not an explanation for an older process's observed wait. It does not establish what revision a long-lived process loaded. Disappearance of an older reader is not self-drain proof. The later Source9977 worker-format failure is separate.

## CLI and artifact ordering

Paths relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`:

- `🏗️build/📜️script.ts:193–208`: graph/show routes directly await `loadBuildGraphV1`, render the result and return. They do not enter immutable artifact/backend execution.
- Run routes call `prepareBuildToolStorageV1(...,snapshotBytes=0)` before baseline graph load (210–211), then `withRepositoryBuildGraphArtifactV1` (212–254).
- `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts:39–46`: the artifact loop enters the graph store, acquires snapshot-store writer, authenticates the desired snapshot and returns ready:false with exact additionalBytes when funding is needed. Its writer finally and outer store callback settle before line46 awaits the prepare callback. Artifact file-reader acquisition/backend body occur only on the ready path. The prepare callback is not awaited inside that same retained graph-store callback.
- `🏗️build/🔧️tools/💾️storage/🟦️.ts:6–7`: Nx preparation invokes its graph-headroom owner. Bazel preparation calls `withBazelStorageV1` and rechecks tools/dependencies/products only.
- `🏗️build/🔧️tools/🏗️bazel/💾️storage/🟦️.ts:30–31`: Bazel storage nests exactly tools, dependencies and products. It does not open or maintain the graph partition.
- The Bazel backend runs inside the artifact consumer, deliberately retaining its graph pin. Current backend entry `🏗️build/🔧️tools/🏗️bazel/📜️script.ts:812` uses the same three Bazel stores. Managed source-closure reads are graph consumers; no graph maintenance call was found in this specific backend chain.

Bazel and Nx storage-preparation branches differ. Persistent Bazel graph SH therefore does not by itself imply Bazel entered Nx's mandatory graph-EX recovery.

## Graph writer and bridge ordering

- `🏗️build/📥️inputs/💾️storage/🟨️.mjs:149–150`: optional writer preflight opens a separate graph partition receiver, rechecks and, if needed, attempts nonwaiting maintenance before entering its fact-store body. That receiver closes in finally. Nested existing fact-writer scopes do not repeat the top-level maintenance preflight.
- Optional scope recovery (218–219) validates the canonical receiver, performs its recheck, then directly calls nonwaiting maintenance. It measures space inside the genuine kernel-EX callback and returns consumed:true. There is no outer `withNativePlatformImageOperationV1` wrapping that maintenance request.
- Partition opening owner `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts:80,102` enrolls the private facade operation/control and unwraps the actual image. The partition frame itself is not a Native borrower operation wrapping the callback. Its child work settles before the operation resolves. `closePartition` (78) rejects closing the same partition from its accepted operation.
- Selected recovery's genuine Native operation (graph/headroom 75–85) completes before Nx invokes optional maintenance. Nx preparation (nx/graph 70–74) closes the original consumed facade in finally, then opens fresh coarse recovery; it does not queue coarse recovery while retaining that facade.
- Ordinary queued cache maintenance explicitly rejects an existing live same-partition reader scope (opening/store158). Inherited Nx graph-CLI preparation selects nonwaiting admission (nx/graph116).

## Actual Native self-drain guards

Paths below relative to `🧰️framework/🔨️modules/🏃️process`.

Persistent backend `📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts:142` does drain its own registered readers before registering the kernel EX acquisition. That ordering alone does not prove a cycle:

- `closePersistentMaintenanceReaders` (123) calls `stopNativeImageBorrowerAdmissionV1` and then `closeNativeImageBorrowerV1`.
- Both functions invoke `requireOutsideBorrower` in `.../storage/📥️borrower/🟦️.ts:28–31`. It walks the actual private AsyncLocalStorage frame chain and throws “Cannot close Native image guard from its accepted operation” for a live same-owner frame. This happens before closeBorrower's pending-work drain (32).
- Retained cleanup readers have an independent same-receiver refusal (persistent command124); their release similarly refuses inside their accepted reader (122).
- Fixed backend follows the same borrower guard and cleanup-reader refusal (`.../storage/🧱️fixed/🟦️.ts:83–84,98`).

Thus maintenance invoked from its own genuine Native accepted operation is refused before it can wait for that operation to complete. Separate receiver/ancestor readers can still cause a real kernel admission wait or nonwaiting deferral; that is a different lifetime and is not evidence of the hypothesized unregistered self-drain.

## Disposition

No current call-site/guard evidence justifies a self-drain repair or explains ownership of the historical Bazel routes. Keep current KernelEX APIs, Source guards and close ordering unchanged. Prepared unit/catalog/architecture and E2E dispatches remain ready but unlaunched.
