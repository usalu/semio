# Graph Writer and Image Custody — 2026-10-21

## Actual Failure

Exact Source-demand native handle 46731 terminated exit 1 before Bun assertions. The untruncated generated/sourcedemand-quota.log records enforced SEMIO_LEASE_ORDER hold-and-wait: PID58126 retained graph-facts shard13 EX while requesting shared native-fixed-image .cache/b/g, blocked by57763 for15466ms. Graph rank3, image rank6; hold-and-wait/cycle/self-deadlock enforcement remains authored and unchanged. This is distinct from earlier memory admission and quota snapshot failures.

## Source Cause and Owner Change

The existing private withRepositoryCacheStoreV1 already opens or retains the genuine graph image before the writer callback. Opening that image earlier was not the missing step. Inside withGraphFactWriterV1, both initial optional maintenance and forced low-water retry maintenance ran while the graph-facts EX lease remained held. Optional headroom maintenance calls image.withExclusiveMaintenance(wait:false); a deferred upgrade can still require restoration of the shared reader. The actual terminal request was that shared image wait under the graph writer.

The narrow owner change keeps the genuine Store/partition lifetime and moves initial maintenance before graph-facts acquisition. Each writer attempt acquires graph-facts, opens/rechecks the original ledger and closes it in its existing finally. A low-water again result then releases graph-facts in the existing lease finally before forced maintenance; the next attempt reacquires graph-facts and observes the ledger anew. Body work and optional publication remain inside live writer custody. Initial maintenance failure/cancellation occurs without a graph-facts lease.

Nested current-writer invocation/refinement handling is unchanged. Existing !current conditions suppress both maintenance entry and again return for nested work, so the forced retry path cannot run while retaining an outer writer. Original store recheck and ledger opening use the retained actual image, not a new image or caller supplied custody. Primary error and cleanup precedence are retained.

## Schema-First Regression

Before the owner edit, saved storage schema sibling GraphWriterImageCase, graphWriterImageCases fixture and actual-function AST regression, with owner bytes verified unchanged at that boundary. Ten language-neutral cases cover no pressure; initial/retained pressure with acquired/deferred maintenance; initial/retained maintenance failure/cancellation; body failure.

The test extracts actual withGraphFactWriterV1 into controlled image/lease ports. Maintenance simulates reader release and asynchronous reader reacquisition; either step under held graph EX fails. Body requires the writer. Ajv independently validates each closed case, while SQLite independently counts acquisitions/maintenance/body events and checks no maintenance/reacquisition held the writer and no body lacked it. Existing twelve writer lease/cleanup cases and twelve quota snapshot cases remain intact.

Exact changed paths, all under build/📥️inputs/💾️storage:

- 🧬️schema/🔣️.json
- 🧪️tests/🔣️.json
- 🧪️tests/🟦️.ts
- 🟨️.mjs (only withGraphFactWriterV1 for this repair)

The prior physical-floor constructor repair, used<=maximum validation, optional reservation denial, mandatory publication floor, external optionalGraphQuotaObservationV1 diagnostic helper, and its regression extraction adaptation were preserved. No lease policy, guard, floor, native flag or private receipt changed. No foreign process or agent was stopped.

## Review and Execution State

Independent source review confirmed both maintenance paths outside graph EX, ledger/lease closure before retry maintenance, nested suppression and cleanup precedence. No concrete source issue was found. The exact existing owning target is repo-graph-maintenance:test, whose explicit Bun suite includes the changed test. No native command or dry run was launched for this repair pending parent combined review. Actual red/green assertion results remain pending; earlier quota8489 and Source-demand46731 failures occurred before assertions.

## Concurrent Writer Integration

After the preceding saved review, another author changed withGraphFactWriterV1 to give maintenance its own opened/closed graph partition, attempt graph EX nonblockingly with timeoutMs=0 while a Store reader is live, close that Store before a queued graph wait, and retry from a fresh Store. Nested namespace contention is explicitly refused. Parent and independent reviewer confirmed this current source; no attribution or restoration of the earlier implementation was attempted.

The regression fixture/schema/actual-AST ports now model this saved implementation with seventeen language-neutral cases. In addition to the original ten pressure/cancel/body cases, it checks timeout-zero contention followed by an image-free queued wait/retry, wait cancellation, unexpected acquisition failure, valid nested namespace reader reuse/refusal, maintenance image close failure and primary-error precedence. Ports distinguish actual maintenance image handles from retained Store image readers. SQLite checks no graph wait retains images, no maintenance/reacquisition retains graph EX, and body retains both writer and image. Native queued lease source was read to confirm timeoutMs=0 is an authored supported path. Existing twelve writer cleanup and twelve quota cases remain intact.

Only the existing test/schema/fixture were adapted for this external change; storage.mjs was not changed again by this agent. No native invocation or dry run occurred. Current combined source/test review is required before the next exact owning test launch.

## Fresh Exact Verification

Parent confirmed combined review clean for current seventeen-case harness, current external writer and all app contracts. Fresh exact Source-demand launched once as native handle 42083, prototype/trace, one job, workers off and ticket artifact environment, with stdout/stderr redirected directly to generated/sourcedemand-custody.log. Parent owns fresh quota71477 and runtime_execution owns fresh app unit. No further source changes are planned except in response to actual failures. Assertion execution and terminal success are not yet claimed.

The retained Source-demand42083 passed host-capacity admission with one job,3584MB reservation,10946MB available and6553MB floor. It then waited for exclusive native-fixed-image .cache/b/g behind an existing shared reader PID8301, position5of5. This is a different resource stage from its preceding memory admission wait; no Bun header, assertions, terminal result or lease-order error has appeared at this observation. The same process and direct log remain retained.

## Live Post-Publication Custody Observation

At 2026-10-21T04:02:31.544Z supported leases status exited0. Current own holder/waiter image projection: `[{"pid":19334,"held":[{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/o","mode":"shared"},{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d","mode":"shared"}],"waiting":[{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared","position":4,"queued":6,"waitedSeconds":270}]},{"pid":19644,"held":[{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/o","mode":"shared"},{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d","mode":"shared"},{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared"},{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:40","mode":"exclusive"},{"resource":"build-graph-state:/Users/ueli/Documents/semio/.cache/b/g/m:selected:40","mode":"exclusive"}],"waiting":[]},{"pid":19742,"held":[{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/o","mode":"shared"},{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d","mode":"shared"}],"waiting":[{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"exclusive","position":2,"queued":6,"waitedSeconds":1329}]}]`. Wait cycles=0; retained diagnostic enforced violations=0. This is live queue/holder evidence for these specific jobs; no Bun assertion pass is implied. Raw status in generated/leases-after-publication.json. No process or lease was modified.

## Retained Source-demand42083 Terminal

The retained native process returned exit1 normally. It advanced out of the exclusive image queue and performed authenticated graph headroom recovery, then refused `Authenticated graph cache cannot fund required physical headroom` at caching/storage/enforcement/budget/graph/headroom/🟦️.ts:48. The exact observation was availableBytes971608064, minimumAvailableBytes1073741824, retiredBytes38432768 and retiredFiles2. The deficit was102133760bytes after authenticated retirement. This was a physical image headroom refusal before test assertions, not a lease timeout, enforced lease-order failure or assertion failure. Complete stdout/stderr is retained directly in generated/sourcedemand-custody.log. No caller-birth/publication command was launched after this failed prerequisite.

Read-only source inspection confirms recovery only removes content-authenticated bounded graph snapshots/publication under the actual image exclusive lifetime, then observes nativeImageSpaceV1 and preserves the required floor. No safety floor, custody guard or source implementation was changed in response. Parent is coordinating the physical-space cause before any fresh invocation.

## Conditional Recovery Source Audit — 2026-10-21

Independent low-effort read-only audit traced prepareNxGraphCacheStorageV1 and recoverNxGraphCachePhysicalHeadroomV1: whole-image EX is conditional on actual physical bytes falling short of the unchanged 1 GiB floor plus rounded serialized snapshot bytes and 16 MiB allowance. Funded preparation leaves recovery false. Before queued recovery the shared reader closes, and recovery rechecks funding before acquiring EX. Funded snapshot publication instead uses the separate graph-store writer and snapshot reader resources. This does not prove runtime success or identify cold mount/create/repair branches. Current graph mount statfs separately measured 952258560 available bytes, consistent with necessary recovery.
