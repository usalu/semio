# Cache Platform Law Review

Budget suite library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🧪️tests/🟦️.ts has unconditional Darwin assumptions in the real Nx partition test:

- Line57 resolves declared directory against process.cwd instead of the private repository.root actually passed to withRepositoryCacheStoreV1 at56.
- Line58 requires fixed.dmg, while directory backends use their own schema-declared filename.
- Line59 requires root filesystem device to differ from capsule parent; valid directory-backed Linux partitions intentionally live on the same filesystem.

Do not skip the test on Linux or remove custody checks. Derive the backing filename through nativeStorageBackingFilenameV1(nativePlatformImageBackendV1(store.image)), with authenticated image identity from the existing platform facade. Use nativeStorageMountedAtV1(directory) to select the mounted-volume device-separation assertion; directory backends should assert same-device placement plus actual registered platform image/opening binding, retained storage reservation and recheck. Keep exact root path, artifact-envelope bytes, receipt owner, allocation charge and post-body storage recheck. Review block allocation assertions against backend backing type rather than assuming every backing is a fully preallocated disk image.

Interfaces already exist in framework process native storage 🔌️backends/📋️contract/🟦️.ts: nativeStorageBackingFilenameV1, nativeStorageMountedAtV1, nativeStorageBackendOfDirectoryV1 and nativeStorageMountedVolumeV1. Platform facade exports nativePlatformImageBackendV1 and requireNativePlatformImageBindingV1. These preserve the authored backend capability policy and honor actual selected provider/test override.

## Devcontainer Dependency Endpoint Volume

Current devcontainer config no longer mounts node_modules as a named volume. Runtime-bootstrap law94 forbids named mount targets with the source dependency endpoint basename. This is a sound ownership correction if the JavaScript dependency installer atomically replaces the node_modules endpoint with a generation projection: a mountpoint cannot be replaced via rename/unlink like an ordinary child of its parent filesystem. Cache partitions stay in checkout-private named volumes and volume ownership law112 still exactly tracks those retained volumes.

Preserve dependency generations in the managed dependencies partition instead of making Source projection endpoints persistence containers. Verify actual cold creation, projection publication, repeated setup and warm restart; static assertions alone do not prove Docker mount behavior. Update the README if it still says node_modules is a named volume. No source edits by this audit.

## Corrected Platform Observation

Nx cache runtime law now resolves its private repository root and uses the declared backend backing filename and mounted-volume capability. It retains allocated backing bytes, physical capacity, root location, correct device relation, exact artifact receipt/allocation and store recheck assertions. Neutral two-backend corpus agrees with independent Ajv admission and SQLite capability observation; added explicit process public barrel exports for owned backing/capability declarations. Pure contract passes; actual Nx image test requires its authenticated parent Script invocation and remains pending full suite (direct tools invocation refused that missing parent, not claimed as runtime proof).

## Independent Correction Review

Reviewed current correction: backing expectations come from a closed neutral imageBackendCase corpus; exact IDs equal the authored backend registry, and Ajv plus independent SQLite query validate filename and mounted-volume capability. Runtime test now resolves private repository.root, chooses the actual authenticated image backend, retains allocated-block/physical-capacity checks, requires correct mounted versus directory same-device relation, exact receipt owner/artifact allocation and final store.recheck. No platform skip or unconditional success substituted. Actual authenticated runtime execution remains pending coordinator suite; direct unissued invocation refusal is an admission result, not a runtime pass.

## Prospective Source Path Law Review

currentCheckoutSourcePathsV1 now unions cached and nonignored untracked git candidates, excludes absent entries via lstat and deduplicates paths. This correctly models current checkout Source after unstaged renames/deletions, including newly authored files. Existing stock Windows213 file/201 directory limits and independent UTF-16 oracle remain unchanged. It does not prove the current index can create a safe fresh clone while removed long paths remain indexed; that separate index/publication condition must be satisfied before committing/publishing, and the law description should distinguish current checkout from index clone semantics. No test-limit weakening was observed.
