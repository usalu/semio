# Artifact Reader Lifetime Checkpoint

2026-10-15. Goal/ticket active. This is an actual Source/private-image checkpoint, not canonical product/store/full-repository acceptance.

## API And Ownership

`artifacts/👓️reading/🟦️.ts` now accepts `{workspace,invocation,storage,control,phaseControl?,request}` and a callback receiving an opaque reader. `withRepositoryArtifactTaskV1` synchronously enrolls a consumer against that exact live reader and invocation. It passes a real canonical child control and context; an escaped/copied reader cannot start work. Active accepted consumers are bounded32 and settled entries are removed; a first failure closes births and retains Error/undefined. The exact artifact SH remains outside the inner canonical scope and accepted drainage. Both actual reservation base and carried phase receive enrollment. The generic API opens no substitute products store and cannot adopt a reservation from paths, capabilities or receipt DATA.

The Core read-only `requireIssuedResourceControlRefinementV1` validates exact private control identity and same invocation. It accepts actual inherited control ancestry; unmatched base ancestors must have requested0 and exactly the invocation signal. Thus a store base with no extra restriction can accept a genuinely authored sibling phase, while a finite/custom-signal base cannot be replaced by an unrelated longer sibling. No new issuer, timestamp sampling/rebasing, control copying or registration exists. Storage has adopted the same guard for scratch/publication/binding after its genuine hostile-phase RED97032.

## Bounded Physical Reads

The manifest is read from a real descriptor into a pre-sized buffer bounded by min(16MiB, request maximumBytes), using64KiB reads, exact extent and named/descriptor pre/post identity checks. Extra-byte growth and replacement refuse. The parsed manifest fingerprint is tied to the initial full inventory and every later recheck. Full file hashing reports the exact descriptor size used in the aggregate byte account, avoiding separate-stat growth undercount. Directory identity is captured and checked after traversal. Inventory bounds are262144entries,64depth,256pathunits, request file/byte limits and32active consumers; files/hardlinks/symlinks must satisfy the actual physical ownership checks. All descriptors/directories close on completion/error.

The independent audit first found longer-sibling phase widening, unbounded manifest readFile race, then parsed-manifest/snapshot binding and stat/digest aggregate races. Actual Source now fixes these. Further independent runtime race/hostile qualification is still needed; do not infer it from the positive barrier law.

## Callers And Routes

The Vite artifact-reader wrapper, Next standalone start and Next runtime verifier now use actual supplied stores and opaque consumer tasks. The existing reader command now runs manifest/AST laws and the private publication/reader proof in separate Bun children (one received Original per Bun process), under one authored60s control. It adds no executable command/editor route. The old unqualified canonical-products reader test is replaced by an actual private proof helper; its mutation/over-budget/negative failure vectors remain in the adjacent language-agnostic corpus but need separate genuine child-ledger gates before claiming that coverage.

## Evidence

- API Source RED6099 correctly saw the actual old `(workspace,request,invocation,body)` signature. Earlier21187 targeted the wrong artifact file and is a fixture error.
- Guard attempts91453/94317 and22431 hit fixture import/inside-test registration errors, not a genuine missing-export runtime RED. The helper now contains no test registration and executes inside the existing suite's already received Original. Storage's actual hostile-phase97032 supplies the genuine physical widening RED; no historical failure is relabeled.
- Current guard/reader Source successor50432 prototype:15passed/222assertions,1.97s.
- Physical positive75194 prototype:1passed/20assertions,2.49s;99741 release:1/20,3.12s. Each used the existing real32MiB private image plus funded scratch, exact shared artifact lease and genuine Node child. Independent Python SQLite EX stayed blocked until the detached accepted child completed; escaped/copied/foreign custody refused; successor publication value8 succeeded. Exact own scratch/image retirement completed, all handles collected. This proof deliberately awaits the outer storage fixture; it does not qualify a detached repository-store callback.
- Owned strict89455 prototype exited0;24885 release exited0. Broader types5355 remains RED with the newly traversed coordinator/library/browser old or in-flight diagnostics; this is not a whole-graph pass.
- Latest combined neutral87587 release:19passed/269assertions,5.60s; zero failures. This includes peers' current office/renderer/font Source laws as well as the reader.
- The standalone Native fixture32accepted/33rd-refusal successor61413 completed prototype1passed/22assertions,5.12s,exit0. Exact own image/scratch retirement completed. Its release22-assertion counterpart remains pending; earlier release99741 was20assertions.

## Required Next Boundaries

Historical bare-store lifetime gap is now corrected by Nx's privately retained store/frame enrollment and accepted drainage. The generic reader outer operation and accepted task both enter Storage's authenticated store consumer bridge; every wait/read/recheck uses its returned actual phase. Native non-store reservations receive the distinct actual-base/phase fallback; known store mismatch/closure remains fatal. Root API Source RED13315 (18pass/1fail) was missing the actual helper call; successor79183 passed19/270. Nx joint prototype72250 passed5laws/60assertions,13.81s: actual producer EX blocked the detached reader SH until its outer callback had returned; its subsequently accepted task ran a genuine Node read and SHA recheck, while independent Python SQLite proved SH held/free. Exact32MiB image retired only after settlement. Nx release counterpart is still pending at this Root checkpoint. This qualifies the real small-image reader/store boundary; no canonical products or maintenance claim follows.

Reader Error/undefined/cancel/manifest mutation/growth and hostile inventory proofs, actual Vite/Next/Font/Print consumption, maintenance/cross-alias semantics, canonical products publication and Linux/Windows runtime remain unqualified. No normal product, Rust, SDK, network, maintenance, old-format retirement or global namespace operation was performed here.

## Retained Root Logs

- `root-artifact-reader-red.log`: 5848B; SHA256 `d1f7ce2dff67d7d790d525d7840a7740a63fac73ed2dd1690538e5671931bcbe`.
- `root-artifact-reader-red2.log`: 5891B; SHA256 `cca3482b64e132b7801a78412eb32b1c6190320ef06597437fa411e008f23d39`.
- `root-artifact-reader-prototype.log`: 7754B; SHA256 `31bee9482cc34b970f942585bee0544b87a2599f7662db4068acc35575843952`.
- `root-artifact-reader-types-prototype.log`: 29628B; SHA256 `d01b7c5be733f5d0d60415565ca70a1795621aac4857317bb0bbbbbb17d96068`.
- `root-reader-control-red.log`: 6122B; SHA256 `fdaea01d78850e35879a5baa0025c83d0578e66e8f368fbabcfa6b11a52f3ab6`.
- `root-reader-control-red2.log`: 6410B; SHA256 `0556e21b76ebfe0db663c617ef1b94052bb2b6045f5a27705b7e3cf8ff18fa52`.
- `root-reader-control-prototype.log`: 6412B; SHA256 `b1dc950915cdd87414242df8eae7c6f1159e5dc5ae0627eccb02dd951346c307`.
- `root-reader-control-prototype2.log`: 4420B; SHA256 `9cd906ae8314f95280a5c540919802b5a74423fc8f8e79c2c12112dccca8170c`.
- `root-artifact-reader-physical-prototype.log`: 1151B; SHA256 `c6272495889df6741d42d02f08b145ce60b0414c82b8c471f559991ee65d1374`.
- `root-artifact-reader-physical-release.log`: 1151B; SHA256 `3a24ec81cc2193bbf986efad610d23a88600a9c901524394b830bb9899702222`.
- `root-artifact-reader-types-owned-prototype.log`: 0B; SHA256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
- `root-reader-final-neutral-release.log`: 5324B; SHA256 `1a74d8cbc50d6fa5f7bdc9769cce22907cfdb9eb0d49fcceb5a304dfc6697e40`.
- `root-reader-final-types-owned-release.log`: 0B; SHA256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.


## Closed Coupled Store Successor

Nx joint reader/store/Vite-input fixture is now BOTH66028/80697 GREEN5/92 with expanded strict12801/74195 BOTH0, all own scopes/images closed. See cache-store-consumer-lifetime-current.md for exact child-error/undefined ledger and image-retirement evidence. This supersedes the earlier pending release note without promoting generic Source to actual Vite compiler or canonical product acceptance. A later separate Storage release cancellation probe failed before its private fixture callback; that unrelated partial open has no retirement proof and is preserved pending attribution.
