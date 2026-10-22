# Current Multipart Optional Retirement Policy Review

Read-only static review, no Source evaluation, native operation or cleanup.

## Actual Current Formats

Build input storage optionalInventory/retireOptionalGraphHeadroom walks inference and unlinks individually selected optional leaves. It retains device/inode/size, but not multipart reachability. Scoped retirement also needs the same format policy rather than a divergent selector.

Caching records owner (caching/🗃️records/🟦️.ts) publishes pack first, then closed index by rename; index tuples bind contiguous offset/length/digest and packBytes. Record readers treat damaged/vanished records as misses. Compaction also removes old idx/pack individually. A pack/index pair is one publication unit; a retained valid index must never outlive its pack. Existing partial state cannot be declared safe merely from matching filenames.

Observation owner (build/👁️observation/🟨️.mjs) writes hash-named Brotli observation blocks before a header; header includes block digests plus inline rows, result digest and authority observations. Blocks may be shared by multiple headers, so header→block reachability determines retention. Header removal alone is a cache miss; removing a block while a retained header names it creates a partial publication. Current expandInferenceBlocks verifies physical block only on cache miss and retains blocksByDigest. persistInferenceBlockV1 line726 trusts blocksWritten; after physical retirement a surviving process can report persistence true and publish a new dangling header. Physical block revalidation or genuine owner invalidation is required before claiming successful block persistence. The new inventory reuse guard alone does not repair this distinct memo.

## Minimal Safe Current-Format Policy

Keep complete proof refusal for unknown, corrupt and unexplained partial groups. Do not add partial-name ignores. Compute groups using existing validated pack/index and authenticated header/block links. Shared blocks survive while any retained header needs them. Hold genuine image EX and the actual namespace writer for any group mutation; image EX must not be mistaken for a fabricated namespace writer.

For interruption safety choose an owner-defined atomic invalidation boundary before leaf reclamation. A whole independently owned record-store or header/block namespace can be renamed to a fixed same-image retirement location only after complete membership/reference proof and no active reader/writer, with a closed durable group manifest binding original coordinate, exact member hashes and retained closure references. Preserve all unknown members; do not sweep directories merely by regex. Namespace choice must avoid enclosing unrelated scopes (for example do not rename the whole inference root containing contexts/selections).

Complete-proof reader must explicitly recognize this new closed retirement format, authenticate its exact survivors/absence states and retain its conservative closure-reference union until terminal group deletion. Restart/cancel resumes only that authenticated group, not arbitrary partial files. Delete one already-invalidated group incrementally, account actual closed-descriptor blocks/statfs, then remove its intent last. A group already committed to invalidation should not be left visible in its old live coordinate midway through deletion. Atomic rename alone without a recognized manifest leaves the next proof unable to distinguish safe interrupted deletion from corrupt publications.

If instead using durable intent without directory move, publish/bind complete group/reference intent before the first unlink and make ordinary readers plus proof understand the invalidated group; this is a larger live-format change. Never allow index/header to remain apparently valid while required children are absent. Existing unmarked partials stay a refusal; only the original publication owner may establish a genuine recomputation/invalidation transition. Pack publication and compaction themselves have interrupted multipart windows, so the new policy must cover those paths or honestly refuse preexisting unexplained partials.

## Required Regression

Schema-first rows: shared block retained by second header; pack/index single group; unrelated/unknown member retained; writer busy/reader active; cancellation/crash after invalidation and after every member removal; restart resumes authenticated group; altered manifest/member/ancestor refuses; stale blocksWritten publisher rechecks/recreates before fresh header; no available floor until descriptor close/statfs. Independent SQLite edges/reachability and state transition tables determine groups/references, independent hashes validate bytes, and real image tests exercise actual namespace writer plus canonical EX. Ordinary producer restart/cache miss must recompute; exact required task reads remain strict.

This is a concrete design recommendation, not a claim that the current implementation already has a durable group protocol or can safely reclaim unexplained partials. Root/High own scope decision and schema-first implementation.

## High Prewrite Plan Review

Read6539-byte plan b46308469e3811403ed6710e51b112038b78c38a871beb95ee6dfe9ec84dfc7e. State-machine design addresses interruption without partial regex ignores. Leaving unindexed pack unresolved/fail-closed is acceptable honest availability limit; no reason to silently widen feature. Requested explicit distinction between both-coordinate conflict and legitimate recreated original generation after commit: exact directory identity and committed intent must decide; new original is freshly enumerated and never retired by old intent, prepared replacement refuses. Also cover empty-retired-directory removed/intent-last terminal window and authenticated intent/catalog/root epochs during resume. Observation physical memo repair is prerequisite. No additional protocol executed or final candidate approved.
