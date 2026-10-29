# Repository Search Readiness Blocker

Root observed repo search linux fail with index limit exceeded: tree documents >250000. Read-only source diagnosis confirms that query selectivity cannot avoid this limit: the full tree is indexed before query matching. No failing search rerun by this audit.

## Exact Pipeline

- Tree Go package 🔍️search/🐹️.go SearchMonorepoTreeWithCache calls ensureCacheIndexed before processing the query. Afterwards countSearchableTreeNodes independently enforces MaxDocuments again.
- Tree Go 🎊️query-cache/🐹️.go ensureCacheIndexed338 computes fingerprint, obtains cache lock, then collects the full tree at378. collectSearchIndexNodes507–539 allocates a complete node pointer slice and rejects the250001st noncategory node at526–527. All kinds count, including source/ticket/code metadata nodes; this is not merely a file count.
- Every collected document is encoded with own fields and immediate child fields, then appended to the staged index391–408.
- Search Go 🏛️core/🐹️.go20–27 hardcodes document1MiB, documents250000, traversal500000, pending events500000, persisted256MiB and query limits. ownedIndex holds all text in a map plus pending events. SearchContext materializes/sorts all IDs and accumulates every match, then sorts and truncates only at request.Size. A requested result limit therefore does not bound matching memory.
- CLI Go 🎊️query-cache and 🔍️tree-search mirror this pipeline. Fix the domain owner and remove/align repeated implementation rather than leaving CLI behavior divergent.

## Clean Bounded Design

Define schema-first search traversal/index/query budgets and language-neutral page/shard/result contracts. Stream bounded documents into independently committed shards with per-shard document/byte caps; retain a bounded manifest/cursor rather than one all-document map. Query shards sequentially with cancellation/progress and a bounded top-k heap or explicitly paged matches. Deterministic score/ID ordering must be maintained across shards and continuation tokens bound to the same fingerprint. Distinguish total repository size from resident batch/index capacity: current cap is valid for one resident shard but invalid as a global usability ceiling.

Push structural kind/path filters into traversal when semantically authorized; text query matching still requires all eligible documents unless a persistent index can identify candidates. Do not drop historical content silently to make the count pass. Do not switch to unbounded recursive SearchTreeInMemory or raise limits without a memory model. Result tree pruning can expand whole matched subtrees via ancestorMatched; bound result serialization separately or expose paged flat hits plus ancestor context.

## Regression Requirements

Create a neutral corpus above250000 eligible documents with sparse matches beyond the first shard. Compare scored ordering and totals against an existing independent search library oracle for a smaller equivalent corpus, and assert bounded resident documents/events/result heap on the large corpus. Test same query across cold/warm cache, cancellation during collection/query/persistence, corrupt shard rejection, atomic publication recovery and fingerprint changes between pages. Existing caps must remain enforced per bounded unit.

The observed failure does not prove accidental node_modules traversal: the tree contains semantic entities beyond216k tracked files. Instrument counts by node kind/path domain before changing selection. No source changes made.

## Ranking and Caller Preservation Review

Existing neutral ranking-vectors corpus contains18cases for conjunction, ID tie breaks, full total despite truncation, zero-size total, negative-size all hits, case folding, punctuation tokenization, substring match, fuzziness, deletion and replacement. Native Go unit suite mirrors these semantics; ranked-search adapter emits ID@score strings for cross-implementation comparison. Preserve score descending then ID ascending, full Total, and index update/delete behavior.

CLI stream-filters owns additional direct search consumers at109 and148. queryCacheIndex sets Size to its supplied limit; tree search supplies total searchable count, so bounded top-k alone does not bound tree matching. Tree pruning also retains all descendants of a matched ancestor. A streamed match visitor/page API is needed for tree caller, alongside preserved finite index SearchRequest semantics. Findings delivered directly to search repair owner.

## Adversarial Review of In-progress Sharded Implementation

No heavy tests executed while capacity peers queued. Implementation was actively changing during this read-only review; these are review findings, not final defects after repair.

- Cursor currently binds only manifest SHA256 and score/ID, not query/fuzziness. A cursor from another query in the same index is accepted and silently changes which hits are omitted. Bind normalized query contract digest and test same-generation changed-query rejection.
- OpenSharded retains shard os.Open descriptors to keep a generation stable. Windows default syscall.Open uses FILE_SHARE_READ|FILE_SHARE_WRITE, without FILE_SHARE_DELETE ([Go primary source](https://raw.githubusercontent.com/golang/go/master/src/syscall/syscall_windows.go)). Therefore concurrent outer directory publication/retirement can fail while readers hold descriptors. Resolve with an owned platform handle interface or explicit reader/writer lifetime synchronization; test Windows replacement and cleanup.
- Same document ID hashes deterministically into the same shard; sequential replay preserves global last-write-wins and delete/reindex semantics. Manifest checks enforce sorted unique shard names, bounded counts/bytes, SHA256 shape and replayed SHA256/count/event parity.
- Core top-k heap preserves score descending/ID ascending and full Total while limiting retained hits. Current positive Size cap10000 changes previous legal larger sizes; tree caller still requests full searchable count and stream filter requests len(items). These callers must migrate before declaring readiness. Negative−1 remains finite resident-index all-hits behavior; earlier other negative sizes also behaved unlimited but corpus specifies−1 only.
- Tree result pruning still includes whole matched subtrees in inspected version; finite hit lists alone cannot bound returned tree memory/bytes/depth. Declared MaxResultTreeNodes/Bytes/Depth constants require actual enforcement before recursive projection.
- Cancellation checks are present during event replay, shard verification and hit scanning; progress is exposed. Exact zero-shard/open continuation and publication rollback tests remain useful.

All actionable findings sent directly to repair owner and coordinator. Container shared-config/node_modules publication runtime checks remain pending; this audit does not claim them complete.

## Integration and Kernel Lease Follow-up

Production scan after stream-filter migration found two remaining unbounded direct callers: modular CLI package 🌳️tree Go268 calls treepkg.SearchTreeInMemory; duplicated client 🔎️query-commands Go270 calls searchTreeInMemory. Public SearchMonorepoTree wrappers also delegate to unbounded in-memory search. These references were sent to repair owner for bounded route migration. All found production matchesQuery call sites now pass context and propagate bool/error; no old signature caller found in this scan. Compilation remains required.

New persistent-inode kernel lease uses flock on Linux/macOS and LockFileEx on Windows; OS ownership releases when process exits, avoiding unsafe age-based lock stealing. Reader wrappers retain the lease until shard descriptors close, so publication cannot swap Windows-open generation files. No prior cross-platform owned Go file-lock abstraction was found outside the new search ingress.

Two review points sent to owner: hardcoded60polls×200ms gives12second timeout, potentially shorter than legitimate cold indexing; make wait policy schema-owned/context-governed without stealing a live writer. Recheck lock named path identity after acquiring the kernel lock, so a substituted inode fails closed rather than admitting a detached lock. No heavy tests or source edits by audit.
