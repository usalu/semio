# SDK Layered Metadata Independent Audit

Plan-only independent review, 2026-10-15. Implementation not yet inspected. No tests, runtime, probes, or image reads.

No concrete race or lifetime omission established in the proposed read-only bounded layer pool. Parallel metadata reads under the same retained immutable projection custody do not themselves add mutation authority. Keeping capture hashing and copy writers unchanged preserves their existing accounting and serialization responsibilities.

Required implementation review points:

- Validate a positive schema-owned concurrency limit before spawning workers; a zero/invalid limit must not silently skip all rows.
- Normalize and validate paths before scheduling; compute depth by actual path components, not string length, and finish every shallower layer before a deeper read starts. Preserve exact direct-parent, no-follow, entry and stamp checks inside each scheduled operation.
- Maintain an explicit failed boolean separately from the error value, so `throw undefined` is not treated as success. On first observed failure stop dequeuing new jobs, settle all already accepted worker promises, then throw the original first error. Never return an unresolved worker promise or release projection/base custody while work remains.
- Per-worker catch must record failure synchronously before another dequeue from that worker. The pool must not use Promise.all's immediate rejection as scope completion. Cancellation is a failure with the same accepted-work drain requirement.
- Deduplicating directory postrecensus must retain one actual check for every required directory, including root, and must not deduplicate distinct identities merely because a basename matches. Keep the final root identity check after all accepted entry and directory work settles.
- Pure layer/concurrency tests should gate reads with promises and assert maximum simultaneous accepted reads, no deeper-layer start before all shallower completions, no births after first observed failure, all accepted reads settled before rejection, and exact thrown undefined. Independent Ajv/SQLite corpus can validate authored bounds and depth/order expectations; it does not substitute for runtime invocation of the production pool.

The intended 16-read bound limits simultaneous metadata operations, not total file count or allocated bytes. Existing bounded namespace and actual byte/stamp checks must remain. No deadline extension, new Original, skipped hashes or fabricated runtime evidence is justified by this plan.

Await Root's Source-ready notification for final review; this document does not certify an uninspected implementation.

## Implementation Re-Review

The current `derivative/📚️layers/🟦️.ts:7-11` validates unique closed paths, total rows and positive safe schema concurrency (16), orders component-depth layers, and awaits all workers in each layer before proceeding. Each worker catches read failure, records an explicit failed boolean and the first thrown value, stops dequeuing, and settles normally. Promise.all therefore drains every accepted read before the outer failed branch rethrows, including thrown undefined. A deeper layer cannot start after failure. No concrete worker lifetime defect established.

`derivative/🟦️.ts:30` retains cancellation checks and actual parent/lstat/stamp/link checks inside each scheduled read. Directory registration occurs before that read resolves, so all ancestor registrations finish before child layers begin. Paths are unique and the captured rows preserve their existing bounded Source facts.

`derivative/🔏️index/📂️tree/🟦️.ts:16-22` retains unique exact-path directory identities, tracks accepted parent/recheck calls, stops new calls when its body retires, and drains accepted work before retiring live state. Its final directory census uses the bounded layer pool and performs an additional actual root check afterward. Failure arrays use length, so thrown undefined is preserved. Parent checks still require actual no-follow directory identity against the anchored device/inode/mode/timestamp record. The successful final root check follows all accepted directory reads.

Source qualification: no new actionable race or lifetime omission established in the inspected implementation. This does not assert runtime success, measured performance, test results, or complete SDK publication. No tests/probes were run by this audit.

## Full Descriptor Capture Parallelization Re-Review

The new `derivative/🔏️index/🟦️.ts:42-49` holds an authenticated actual snapshot SH before entering the rooted tree scope, and rechecks it before publication. It discovers children by actual opendir in breadth-first layers. A synchronous shared claimed set reserves each path and checks maximum entries/duplicates before enqueue; no await separates those admission decisions. Each next layer only starts after the previous production layer walker drains.

Each accepted walk still performs control/path admission, actual anchored direct-parent verification, actual lstat/device/type/hardlink checks, and one actual no-follow descriptor hash for each file. Byte totals are charged synchronously before hash await. Directory identity is registered before its children become eligible in a deeper layer. The existing descriptor hash retains pre/post descriptor checks, chunk cancellation/continuation and finally-close. At most 16 complete walk jobs run, so at most 16 file hashes/descriptors are admitted concurrently by this path.

On a read failure the previously reviewed layer pool stops new worker dequeues, drains accepted reads including descriptor closes, and rethrows Error or undefined. Accepted directory walkers may finish discovering bounded next-layer paths, but those paths are never executed after failure. Final successful publication requires rows count equal claimed namespace, canonical byte sorting, exact sealed-index byte equality/re-read or refusal of an appeared index, and the rooted tree's final directory/root postcensus. Private rows/bytes remain captured defensively.

No new actionable reservation, parent-order, fault-drain, or custody omission established in this full-capture correction by Source review. Capture remains a full actual hash pass; no previous validation pass is adopted as a substitute. Runtime speed and prototype/release qualification are unclaimed by this independent audit.
