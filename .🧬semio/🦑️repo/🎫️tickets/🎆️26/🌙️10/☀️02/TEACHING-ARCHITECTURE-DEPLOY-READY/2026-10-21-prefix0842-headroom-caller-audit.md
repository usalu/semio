# Native0842 Graph Headroom Refusal: Caller and Recovery Scope

Actual owned96421 exited1 without cancellation before prefix entry; raw553845B/c8fd78a3956388d65b638de69fd98e69afe42efd2c2c2cdb60f0126f9bd1347b. All101 control pins remained unchanged after terminal, all22 future outputs absent, all5 gate/effect markers0. No retry/cleanup/Source edit was performed.

The actual exception is `Authenticated graph cache cannot fund required physical headroom` at `⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts`57: available973414400B versus1073741824B required, retiredBytes0/retiredFiles0. The shortfall is100327424B. This is an enforced graph image headroom result, not proof of global host disk scarcity. Exact physical-image root/available source is independently reviewed by Lowprogress.

## Existing Caller Already Tried Recovery

Actual `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts`70–76 prepareNxGraphCacheStorageV1 computes floor through nxGraphRequiredHeadroomV1; underfloor it first tries authenticated selected pairs, then per-writer scoped optional reclamation, then nonwaiting coarse optional reclamation. It closes its shared graph reader before queued full-image recovery, preserving Original phase control and ordinary maintenance deadline. The final recovery invokes retireBuildGraphOptionalHeadroomV1 with genuine maintenance custody. The actual line57 failure therefore follows the existing guarded fallback rather than an omitted generic cleanup call.

## Existing Retirement Authority

`recoverBuildGraphImageHeadroomV1` opens only the declared Native fixed partition. Its available() delegates nativeImageSpaceV1(image.root,image.filesystemBytes), then multiplies bavail*bsize with safe-integer/nonnegative checks. Full-image maintenance queues behind readers when wait=true; inherited/nonwaiting callers do not fabricate exclusive custody.

Eligible graph publications are only hash-named snapshots whose full digest agrees with filename; complete selected .json/.authority.json pairs whose receipt and full graph digest agree; and complete canonical.json/authority.json pair. Leaves must be ordinary same-device single-link bounded files, exact descriptor/name epochs are retained/rechecked, and receipt parsing runs as an owned bounded child. Oldest valid publications retire first with actual progress/cancellation. Unknown files, incomplete pairs, foreign data, symlinks and unproved live namespaces receive no generic deletion authority. The actual retired0/0 is a final recovery result; it does not alone prove all directories empty or justify broad deletion.

The optional callback requires genuine canonical Native image writer custody and only the existing optional path predicate/inventory with full file identity checks. Isolated inference/selections/<digest> and inference/contexts/<digest> have a separate scoped API that also acquires their exact writer lease and durable optional-ledger lease before retiring admissible leaves, skips busy scopes, and preserves directories/ancestor identity. That scoped stage is invoked before the coarse queue. The coarse callback delegates ordinary optionalInventoryV1. Whether any currently remaining isolated namespace was busy then or is now eligible requires actual owned evidence; no such assumption is made here.

No separate supported recovery CLI was identified in this bounded caller slice; Root is reviewing the build skill/frontend. No helper was executed from Node, no private cache function was invoked, and no manual rm/resize/guard change or peer cleanup is proposed. The mandatory physical floor and all image/lease/receipt guards remain authoritative.
