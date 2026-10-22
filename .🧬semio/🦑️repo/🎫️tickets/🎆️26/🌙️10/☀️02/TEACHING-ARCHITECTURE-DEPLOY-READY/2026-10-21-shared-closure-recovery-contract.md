# Shared Closure Recovery Contract Proposal

Read-only design before Source mutation. No Native operation, cache effect, or Source helper executed. Scope: the real top-level build-input-closures/build-lock-closures gap recorded by graph-inventory-recovery-gap.md.

## Lifetime Findings

Actual writer storage137–169 retains the canonical image and writer lease through publication and accepted children. Managed exact reads retain the image through their batch; active artifacts retain the image through backend consumers. Genuine kernel-exclusive image maintenance excludes these admitted readers and writers. It does not prove every Original invocation has ended or erase inert process-local values.

The all-workspace generation loader expands compact inventory tokens through portableInputs before returning a backend graph (generations/load114–116); selected/cold discovery does the same unless explicitly producing compact internal fragments (graph276–277). Portable expansion requires hash-valid inventories. The generation fragments and inference caches can still contain compact tokens and need conservative retention. Exact task hashing also consumes token plans and refuses absent records.

There is a concrete volatile-cache complication: inference reuse has a ten-minute validInventoryRecords shortcut, while observation publication may be declined by optional quota and its process-local project image may survive. Therefore merely retaining references found on disk cannot justify reuse of a previously cached in-memory token whose record has since been retired. Proposed narrow companion change: revalidate physical inventory presence/identity before any reuse shortcut, treat only genuine missing immutable inventory as an inference cache miss so its producer recomputes, and retain corruption/identity/cancellation failures as refusals. Exact task readers remain strict. This needs an actual production reuse regression, not only a planner case.

## Proposed Conservative Policy

Use reference retention, not broad graph invalidation. Add a dedicated first-party closure recovery owner adjacent to graph/headroom; integrate it only into existing full image-exclusive recovery. Never broaden optionalPath. Before considering closure retirement, authenticate and enumerate every retained publication format that can carry compact refs: canonical/selected/snapshot graph JSON; generation manifests/fragments and current pointer with existing hash-named Brotli fragment formats; inference project/test JSON records, their complete header/result digest joins, observation blocks, and packed-record index/value joins. Reuse existing first-party schema/record validators where safe; no caller-selected paths, limits or effect ports.

A complete reference set is mandatory. Unknown publication shape, partial pair, missing reachable fragment, malformed pack/index, invalid hash, foreign/symlink ancestor, exceeded entry/byte/decompression/work bound, or unstable identity makes closure reclamation fail closed before any closure unlink. Conservative false-positive references are retained. No inference from age, filename alone, a missing authority, an empty directory visit, or an unrecognized format means unreachable. Persistent ignored platform metadata must have an explicit closed rule rather than be guessed harmless.

Only after that complete reference proof, candidates are exact lowercase64hex.json leaves in the two fixed closure directories. Validate their existing closed Source or lock contract and content SHA256 against filename through bounded nofollow descriptors. Retain ordinary single-link same-image leaves plus full ancestor dev/ino/mode and full named/descriptor epochs. Before each unlink, require the genuine still-live image maintenance writer, recheck ancestors and descriptor/path epochs immediately, and check cancellation/deadline. Oldest eligible unreferenced candidates retire only until the existing requested physical floor; account allocated blocks, not logical length. Unknown/corrupt candidates are never deleted. Preserve the existing strict floor and result shape.

The complete durable-reference reader is the main implementation surface. If an existing format cannot be safely enumerated via its current owner, the implementation must refuse reclamation rather than invent a lossy decoder. This is an explicit completeness requirement for review, not a claim that all current image bytes are eligible or that enough headroom will be recovered.

## Schema/Test-First Plan

Add closed recovery contract and language-neutral fixtures beside the headroom owner, then author actual consumer tests before runtime edits. Keep existing storage/headroom/scoped tests intact. A new exact owner-local test title is proposed: shared graph closure recovery retains every authenticated reference before reclaiming orphan inventories. Root owns phase20 integration; existing19/control/guest/prefix remain untouched by this work.

1. Independent SQLite reference closure oracle over canonical/selected/snapshot/generation/inference references, including shared chains, duplicate refs, referenced source/lock and orphan source/lock leaves; independent SHA256 and allocated block accounting.
2. Production planner and complete reference-reader paths against real closed serialized formats, including Brotli and actual record-index/value constructor; malformed/partial/unknown/escaped references fail closed.
3. Genuine canonical image maintenance callback, actual writer/reader custody, authenticated orphan reclamation and referenced retention; busy reader/writer yields no closure deletion.
4. Symlink/foreign device/hardlink/oversize/path limit/hash mismatch/contract mismatch/leaf replacement/ancestor replacement, cancellation during scan and immediately before unlink, bounded scan/decompression, and insufficient eligible bytes.
5. Actual managed Source/task digest still consumes retained refs; actual inference reuse recomputes a truly absent record even inside its prior validity window and does not swallow corruption or cancellation.
6. Production recovery callback invocation must prove it reaches the new owner under genuine exclusive custody; a detached pure planner pass is insufficient.

Potential Source paths: new graph/headroom/closure schema, fixtures, test and implementation; existing headroom owner callback; reuse runtime plus existing/new owned test corpus. Exact additions and literal titles will be reported after schema/test design is reviewed, before Root integrates closed Source rosters. No public cleanup script, new capability or maintenance bypass is proposed.

## Independent Prewrite Review Addendum

Lowcourse requires retaining the complete reference-proof read set, not just candidate epochs: every publication/pointer/index/value descriptor, every enumerated directory and exact absence observation, plus all ancestor identities must hold before first unlink and across retirement. A changed/new reference or replacement pointer/pack after planning preserves all closures by refusing retirement. Only ENOENT of the exact inventory leaf beneath revalidated admitted ancestors qualifies as an inference miss; missing ancestors, partial publication, permission failures, and corruption remain refusals. The actual ten-minute shortcut plus declined optional publication/process-local image scenario must be exercised through production reuse, not a planner proxy. These requirements are incorporated before Source edits.

## Approved Ordering Refinement

Root authorized deterministic largest allocated authenticated unreferenced orphan first, then oldest and stable identity, to reduce repeated complete-proof checks. Eligibility, full proof fences and physical floor are unchanged. The effect loop considers all eligible rows in that order and observes actual statfs after descriptor close; estimated allocated sums never establish headroom. SQLite window ordering and closed corpus follow the same data rule. A finite work/deadline refusal after partial safe retirement preserves actual counters and remains a failure.
