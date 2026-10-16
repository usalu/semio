# Windows State Metadata Design Review

Read-only review of windows-state-metadata-port-proposal.md; no Windows/system/native/store execution. External documentation verification remains separately recorded by Root/executor.

Direction is conditionally sound: metadata token is privately issued from the actual platform image/retained receiver and descendant control, not JSON mapping or caller volume text. Exact image→disk→partition→NTFS volume and target directory volume must agree before/after the local CIM operation; accept numeric ReturnValue0 only and bounded closed receipt after full child/stream closure. No generic host path or temporary directory can authorize a host-volume flush.

Metadata lifetime must close births and drain accepted sync/progress work before returning; State operations then leave metadata callback before image.close to avoid self-drain cycles. One pending child requires bounded serialization or explicit busy refusal, not an unbounded promise queue. Nested borrower operation must preserve actual image/control refinement and support its intended reentry without requiring an EX upgrade while holding SH.

POSIX directory-descriptor barrier remains distinct; Windows implementation must not widen the directory-sync allowlist. Failed barrier retains the linked/intent transaction for exact recovery; error return does not imply namespace rollback. Arbitrary undefined/frozen primary must survive secondary close/child failures. Native interfaces expose only repository-owned DATA/types and opaque issued capability, no CIM/Stats/handle or repository-domain types.

Privilege/provider/NTFS/locality assumptions must fail closed on actual admitted image. Existing hardcoded Windows root and Hyper-V availability constraints remain unresolved independent provisioning limits; this metadata change cannot claim zero-touch all-edition/unprivileged Windows support. Models using platform strings/PowerShell text/ordinary temp paths are not Windows physical evidence. Actual fixed-image and nested child image qualification, cancellation/drain and controlled crash durability remain separate.

Final approval requires actual schema, private token/borrower/drain Source and Windows mapping command review. No physical release follows from this design review.

## Early Draft Source Review

Opaque token issuance uses actual platform image operation and private WeakMap; one pending operation explicitly refuses overlap, accepted work drains before token invalidation. Windows command uses retained receiver, exact local disk/partition/volume and directory volume mapping, numeric zero Flush result, closed pre/post mapping receipt and parent rechecks. No physical Windows proof follows.

Concrete phase gap reported: State/🟦️.ts child withNativeImageMetadataV1 callback receives metadata only, dropping actual callback phase and using outer actual control for openData/rechecks/scope/body. Carry child callback phase explicitly; token is issued against the platform-returned actual phase. Parent metadata callback already receives its actual phase. Final review remains pending correction/models/strict.

## Mandatory Native Port Addendum Review

Mandatory syncMetadata(backing,directory,actualControl) on neutral NativeStoragePort keeps generic storage free of platform runtime imports and external/repository types. Real ports authenticate retained borrower/backing; ordinary port refuses. Journal and receipt await one actual barrier before success/cache publication/unlock while gate/borrower remain live. This design direction is clean pending Source/fixture census.

Root’s stronger requirement governs stale addendum wording: mark dirty and invalidate cached projection BEFORE potential append or checkpoint rename, not only after successful mutation. Partial write/fsync/rename failures may already alter bytes. Barrier or callback failure is not rollback, and recovery must reread actual journal rather than retain prior successful cache authority. Explicit modeled fixture barriers remain Source evidence, not physical Windows proof.

## Failed Scope Follow-Up

Root correctly identified current rejected barrier handler overwrites scope.failure and admission lacks failedscope refusal. Final contract must retain first arbitrary failure (including undefined), refuse later births with zero effects once failed, and preserve bodyprimary precedence. Existing onepending check rejects overlapping calls before admission; it is not a queued serialization implementation. Require explicit fault law for undefinedfirst/frozensecond and no second dispatch.

## Integration Draft Reread

Child State metadata callback now receives actual phase; failed scope retains first arbitrary failure and closes later births. Mandatory actual Port barrier is used, and journal invalidates dirty cache before potential mutation, awaits barrier/postchecks before publication/unlock and preserves primary.

Concrete remaining Windows identity gap reported: fixed metadata verify checks directory device only, without retained target inode/nonsymlink/directory/ancestry identities across system child. Initial requireNativeStoragePath checks current path once; final verify does not repeat that path proof. Same-volume replacement can pass despite declared changed-directory refusal. Require bounded pathchain snapshot/recheck and actual-function race law before final clean verdict.

## Target Identity Correction And Directory API Decision

Current fixedMetadataDirectoryStamp retains bounded root-to-target directory/non-symlink/same-device dev/ino identities and verifies them before/after the owned child; prior target-identity gap is corrected in Source.

Root’s proposed removal of unverified Get-Volume -FilePath(directory) is sound at the Source-contract level: actual retained VHD→disk→partition exact root AccessPath→NTFS volume establishes the owned root, and physical no-alias/same-device/devino ancestry keeps target inside it. Flush acts on that exact owned volume. No file probe is needed for an empty newly admitted child root, and querying the backing VHD file would identify its parent volume incorrectly.

Keep unique exact mapping and ancestry pre/post checks. Root mount lstat/reparse and Windows st_dev behavior remain genuine Windows physical qualification, not established by text or modeled ports. This supersedes earlier directory-GetVolume requirement without relaxing actual image authority.

## Final Held Production Narrow Verdict

Actual platform registry retains real leaseDirectory/policy; child State carries metadata callback actual phase. First arbitrary failure is retained, subsequent births refuse, overlapping operation is explicit preadmission refusal, and accepted metadata work drains before image close. Fixed Windows uses exact retained root mapping plus bounded physical directory chain pre/post, numeric zero Flush result and bounded closed receipt; undocumented directory GetVolume/file probe is removed. Mandatory real Port preserves generic injection boundary and ordinary port refuses. Final narrow Source review is clean; active final strict/model terminal evidence remains separately pending at this observation. No Windows physical, privilege, power-loss or all-edition zero-touch qualification is claimed.

Executor final terminal report: Source3346 actual0/9tests167expects10.24s; bounded Source23185 actual0/1focused12expects1.063s under genuine Original; strict62686 actual0/8roots10.335s. Portable receipt/POSIX fault/AST Windows race/capability/lifetime models remain modeled-system evidence, not Windows/native physical acceptance.
