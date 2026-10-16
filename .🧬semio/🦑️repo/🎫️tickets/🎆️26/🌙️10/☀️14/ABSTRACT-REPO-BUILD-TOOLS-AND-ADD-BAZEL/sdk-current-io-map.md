# Current SDK Capture, Copy and Verification IO Map

Read-only Source analysis while High owns independent retirement TDD. No tools-image contents, SDK runtime, compiler, metadata benchmark or storage operation was opened. Counts use retained completed same-snapshot clone receipts and the actual read-only5803 index identity; they do not establish current Source/SDK acceptance or elapsed cost.

## Retained Dataset and Limits

Actual completed clone receipts (`public-nx-sdk-one-census-glob-reuse.log:493` and `public-nx-sdk-funded-reader100.log:972`) for snapshot6847b0…adce report N11701 rows = F10189 files + D1478 directories + L34 links. Capture receipts report B168402658 file bytes; clone copies18982856 owned Nx bytes and shares other dependency inodes. The retained metadata receipt reports index2016560B/SHA4a884ca6…86ad8. No full row-size/depth array is retained in the ticket, so exact per-file chunk and ancestor sums are not asserted. The4096-row benchmark had1287 distinct parents, but its common parent checks were synchronous for BOTH variants; it did not compare the production asynchronous ancestor chains.

Independent system SQLite verified the arithmetic below against these literal retained counts. These are Source-derived formulas, not syscall tracing or CPU measurements.

## Current Hash Backend

`derivative/🔏️index/🟦️.ts:36` exports hashNxSdkSourceFileV1, forwarding to hashSource:37. Current hashSource uses fs/promises.open(O_RDONLY|O_NOFOLLOW), file.stat,64KiB file.read, file.stat, file.close. It yields after each chunk and checks genuine control before open, after open/stat/read/yield/final-stat/close. A failed read/hash body preserves its arbitrary primary over close failure. There is one full content read per hash invocation; hashSource does not replay the complete byte stream.

The earlier bounded synchronous-descriptor qualification describes historical bytes, not another currently selected production implementation. Current production index module imports no openSync/fstatSync/readSync/closeSync. Current parallel capture/verify tests explicitly expect descriptor synchronous count0. The current synchronous filesystem operations in this algorithm are the short rooted prefix checks in index/tree; they do not make file hashing synchronous. Attribution to any peer/author of this change is not inferred.

For one full B-byte hash pass: exactly F opens,2F fstats,F closes =40756 fixed awaited descriptor calls, plus C read awaits and C continuation awaits, where C=sum(ceil(fileBytes/65536)). Without retained per-file lengths, only bounds2570≤C≤12758 can be inferred from B,F (zero-length files included). Exactly20378 fstats occur per full pass. The existing16-worker depth owner bounds concurrency and drains all accepted workers on failure; these await counts are not wall milliseconds.

## Capture and Source Verification

captureNxSdkSourceIndexV1:48 discovers depth layers using streamed directory membership. Each row obtains current rooted/relative-parent proof, then lstat. Each file executes the hash backend; each directory has one opendir enumeration; links readlink+realpath. It compares deterministic complete index bytes to the held sealed index, retains private issued base SH, and the SourceTree owner completes a fresh final rooted directory check. The preliminary separate full verification was removed earlier; current capture itself authenticates the full tree once.

verifyNxSdkSharedSourcesV1:56 independently processes all N rows through the same16-worker layer helper. Each file is fully hashed again and compared to the index; each directory is registered for final current-tree proof; each link target is checked. Exact readonly modes, byte sizes, executable modes and known inode identities remain checked. This is a full B-byte pass per invocation, not a metadata-only index check.

Let A be sum(path segment count−1) across N rows and S be the short enforcing store-root→base prefix length. One SourceTree-owned N-row pass entails N+A awaited row/relative-parent lstats, one initial root lstat and D+2 final directory/root lstats: N+A+D+3 =13182+A, excluding link/hash/metadata work. It also reobserves synchronous prefix nodes before row use and final boundaries. Actual A is not recoverable from the retained aggregate; no guessed exact depth is reported.

Capture progress has a directory early-return branch, so counting floor(N/128) store rechecks from aggregate N alone would be wrong. Verify increments every row, so its continuation points are floor(11701/128)=91. Each continuation publishes, yields and rechecks the enforcing store; await/control semantics remain per current Source.

## Clone and Post-Copy Rechecks

copyNxSdkDerivativeV1:47 processes N rows once through the depth helper. Every row checks current source parent+row lstat against captured facts, obtains current destination parent, performs its exact mkdir/typed symlink/private Nx copy+chmod/shared hardlink action, then destination.created rechecks current parents and actual created inode. The destination owner retains mutable directory identity/mode without freezing legitimate directory timestamps, exact final membership and initial input bytes; accepted operations drain before release.

The SourceTree owner adds its complete final readonly directory proof. The destination owner adds full final row/membership proof. copyNxSdkDerivativeV1 then calls projection recheck before reporting completed clone. Thus its emitted elapsed phase includes final metadata proofs, not only native copy/link duration. Copy bytes18982856 differ from the full168402658 hash bytes; shared hardlinks are not recopied content.

For each destination row, parent() checks root+all relative parents, and created() checks that chain again plus the newly created inode. This gives2(N+A)+N metadata checks before final destination membership, plus separate fixed initial-input/root work. Each destination check currently rechecks enforcing prefix before its async lstat. These currentness checks straddle awaits and must not be declared redundant merely from static repetition.

A full projection recheck (derivative:32) rereads pinned dependency inputs and marker, traverses current N rows and link targets, compares captured readonly file stamps while allowing hardlink-induced ctime changes, completes a fresh SourceTree proof and rereads the sealed index. It hashes no module file contents during this metadata pass. Current production normal successful publication contains recheck boundaries after copy, after admitted publication pending work drains, in derivative quote finally, and at outer projection success. These are separate lifetime boundaries, not four interchangeable cached-success calls. Branches/failures change which boundaries complete; a single static total is not an observed trace.

## Metadata Byte Replay and Installed Readers

readNxSdkSourceMetadataV1 forwards to the generic readPhysicalCargoSourceV1. That actual reader opens a no-follow descriptor, walks host ancestry and verifies descriptor/path identity; it reads the bounded body then replays its bytes for equality, with repeated ancestry/path/fstat checks around chunks. For one2016560-byte index read, K31 chunks per pass means62 file.read awaits and4033120 bytes consumed. More generally, its successful recheck loop is2K+3 invocations; each reobserves all retained host ancestors plus path lstat and fstat. This is concrete repeated metadata IO inside one read, distinct from module hashSource's single byte pass. It is required by that reader's current contract; no skip or weakened ancestry is proposed.

Each withNxSdkDerivativeReaderV1 first takes base SH and calls full verifyNxSdkSharedSourcesV1, then checks complete derived namespace membership/readonly/inode/link rules. Installed(staging), installed(destination), and activation each use this reader on a normal newly published path. If all three complete, they consume3B module bytes in base verification, in addition to capture's B (673610632 total across four full pass invocations), plus their metadata/patch/binary/authorizer work. These are conditional call-structure totals, not a claim that the latest120-second failed command reached them. Latest50239 had only capture21.123s completed and no completed clone.

No private Nx content hash beyond existing patch/version/receipt checks is invented: the derived reader's private-Nx branch itself checks writable/shared inode isolation and modes; whole base verification hashes source content. Earlier six-input/install proofs do not eliminate these actual current readers.

## Interpretation

Concrete repeated IO exists, especially metadata byte replay and separate full base hash passes at distinct admitted boundaries. This map is not permission to cache filesystem facts, Original validation, current namespace membership, store rechecks or authority across awaits. No cost is assigned from count alone. Root will obtain a fresh built-in live90 profile with actual process.cpuUsage/wall sidecar after current Source holds; the older profile predates keyset and prefix fixes. No further microbenchmark or production Source change was made in this lane.

## Exact Inspected Source Snapshot

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts`: `c8b9fee47068209cb0a48179c5cafd654281bb701f7b110bac9999364755b0a4`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts`: `01a290658bd36f54b1084dd4b04a61e49b86e73d6531fe9c0489ebd731440e86`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/📂️tree/🟦️.ts`: `b11f2db334d0075a566b44438bea7208df14fedf7f16da700756ddda2011d5b3`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🟦️.ts`: `07ee0e0c601609eb4f143410b4a54fa90ed9bcdba472c0b9b1555050c53cd432`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts`: `441ccfb3c54685b8c3d404decd050d95833a0a235946c3fc7c1ecabfde30a456`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📚️layers/🟦️.ts`: `432c1c7c1a447f48073b6ddc18eab2eb942b6501f1fddbc0f41424c509399141`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts`: `a8f3e8646759a783651d8da962ff3bcfebde12d6743790a99a1d44d0829fdced`

## Function Links

- [async function hashSource](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts:37)
- [export async function captureNxSdkSourceIndexV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts:47)
- [export async function verifyNxSdkSharedSourcesV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts:56)
- [export async function withNxSdkSourceTreeV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🔏️index/📂️tree/🟦️.ts:18)
- [export async function withNxSdkDestinationTreeV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📤️copy/🟦️.ts:17)
- [async function recheck(state](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/🟦️.ts:32)
- [export async function withNxSdkDerivativeReaderV1](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/🔗️derivative/📥️reader/🟦️.ts:18)
- [export async function readPhysicalCargoSourceV1](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts:12)
