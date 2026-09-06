# Trusted Catalog Opened-Root / No-Link Boundary

## Verdict

The current loader has a concrete no-link and opened-handle gap. It is not repaired by the component, descriptor, or browser-actor digests:

1. `contained_path` canonicalizes a child path and merely checks that its final resolved path begins with `root`. An **in-root** symbolic link (including an intermediate directory link) therefore passes. [trusted-catalog](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1109)
2. `read_bounded` then performs `metadata(path)` and a later `File::open(path)`. A path can change type or name between those two operations. It never checks metadata on the opened handle. [trusted-catalog](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1125)
3. The loader repeats that pair for component, descriptor, and browser actor. [trusted-catalog](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:485) [trusted-catalog](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:520)

An altered target normally fails a digest, but an in-root linked object holding the expected bytes is admitted despite the generation's promised regular/no-link closure. More seriously, a post-`metadata` type replacement can turn the later open into a different object or a blocking special file before the byte-bound enforcement starts. A digest is a content binding, not an opened-object or no-reparse assertion.

There is no existing first-party cross-platform descriptor-relative no-link reader to reuse. The DB filesystem owner has useful durable-directory/fsync ordering, but it is path-based; it does not retain a directory descriptor or forbid link/reparse traversal. [FsStorage replacement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7651) No repository Rust source uses `openat`, `O_NOFOLLOW`, `NtCreateFile`, `OBJ_DONT_REPARSE`, or `FILE_FLAG_OPEN_REPARSE_POINT` for this purpose.

`cap-std`/`cap-primitives` are present only transitively in the local Cargo cache, not a direct Hub dependency or a Semio-owned interface. They are not a drop-in answer: their documented `FollowSymlinks` setting is a final-component policy, while this boundary must reject *every* intermediate link/reparse component. Do not make an unwrapped third-party ambient-directory API the trust root.

## Smallest coherent owner

Replace the path pair with one crate-private, native-only owner in the trusted-catalog module, for example:

```rust
pub struct TrustedCatalogGenerationRoot { /* owned platform directory descriptor */ }
struct TrustedCatalogRelativePath(/* canonical slash-separated segments */);
struct TrustedCatalogOpenedFile { /* opened regular file + length/identity */ }

impl TrustedCatalogGenerationRoot {
    fn open_server_owned(/* fixed generation root authority */) -> Result<Self, AuthorityError>;
    async fn read_regular(
        &self,
        relative: &TrustedCatalogRelativePath,
        maximum: u64,
        context: &OperationContext<'_>,
    ) -> Result<Vec<u8>, AuthorityError>;
}
```

It must own the generation directory descriptor for the whole load. It parses a relative path once (non-empty UTF-8, `/` separator only, no `.`, `..`, root/prefix, NUL, or empty segment; bounded total bytes and bounded segment count) and opens every requested file *relative to that descriptor*. `read_regular` must:

1. reject link/reparse points during resolution;
2. inspect the **opened handle's** metadata before allocation, requiring regular, non-empty, bounded length;
3. read that same handle in 64 KiB chunks with the current cancellation/deadline checkpoints and a fixed `len + 1` capacity/read fence;
4. retain no path as a post-open authority; hash/decode the returned immutable bytes; and
5. close all transient descriptors on success, cancellation, and error.

`TrustedCatalogLoader::load` should consume `TrustedCatalogGenerationRoot`, not `&Path`. It must open the literal `trusted-catalog.json` through that owner, parse the bundle, then call `read_regular` for component/descriptor/actor. The duplicate-file set becomes normalized `TrustedCatalogRelativePath` values; canonical paths are neither needed nor valid identity after no-link admission.

The raw startup path is also a boundary: `configured_artifact_authority` currently forwards `OS_HUB_TRUSTED_CATALOG_BUNDLE`'s `PathBuf` straight into `TrustedCatalogLoader::load`. [Hub startup](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:403) The minimal complete API is a server-owned generation-root constructor at this callsite—anchored in the configured Hub data/catalog owner, then opened with the fixed generation id and literal bundle name—not a public `load(&Path)` compatibility overload. Otherwise the caller still supplies the root that the descriptor-relative capability purports to constrain.

The test fixture and test-support should obtain the same root owner from their ticket-owned fixture directory before loading; the existing `FixtureDirectory` already owns the root and bundle path. [FixtureDirectory](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1350) The Hub binary's synthetic native-openable bundle call is the only external production caller. [Hub binary fixture](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6978)

## Platform implementation boundary

Keep platform calls private behind `TrustedCatalogGenerationRoot`; do not expose file descriptors, `libc`, or Windows types.

| Target | Required descriptor-relative operation | Non-negotiable check |
| --- | --- | --- |
| Linux | Prefer `openat2` rooted at the retained directory with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_MAGICLINKS`; on unavailable kernels, walk each parsed segment with `openat`, `O_NOFOLLOW | O_CLOEXEC`, adding `O_DIRECTORY` for parent segments. | `fstat` the final fd; accept only `S_IFREG`, with bounded `st_size`. Do not fall back to path canonicalization. |
| macOS/BSD | The same descriptor-by-descriptor `openat` walk with `O_NOFOLLOW | O_DIRECTORY` for ancestors and `O_NOFOLLOW` for the final leaf. | `fstat` only after the final open. Every intermediate fd is retained until its child opens, then closed. |
| Windows | Retain a directory `HANDLE`; call rooted `NtCreateFile` with `OBJECT_ATTRIBUTES.RootDirectory` and `OBJ_DONT_REPARSE`, opening the final object with reparse-point semantics and no directory allowance. | Query attributes on the resulting handle (`FileAttributeTagInformation` / `GetFileInformationByHandleEx`); reject `FILE_ATTRIBUTE_REPARSE_POINT`, directory, and non-regular forms before converting the handle to `std::fs::File`. `FILE_FLAG_OPEN_REPARSE_POINT` by itself is insufficient because it does not prove every intermediate component was not reparsed. |

The pinned local `windows-sys` source exposes the rooted `NtCreateFile`, `OBJECT_ATTRIBUTES.RootDirectory`, `OBJ_DONT_REPARSE`, and attribute-tag structures, but Hub does not presently declare it directly. If the implementation uses those system bindings (and Unix `libc`/equivalent), make them target-only private dependencies behind this owner—never a public API. The source already has no direct `cap-std`, `libc`, or `windows-sys` declaration in the Hub package.

Opening the *initial* generation root must itself start from a Hub-owned data-root descriptor. A generic absolute `PathBuf` constructor merely relocates the unchecked trust root; it cannot guarantee that a configured generation path did not traverse a link before the handle existed. This is why changing only `contained_path` cannot close the gap.

## Executable law packet

Use the existing `FixtureDirectory`, `TestControl`, and `FixtureProviderSource.calls` (there is no current `CountingProvider` symbol in this module) rather than a synthetic loader. Every denial below must leave `calls` empty and `document_codec(schema)` absent, proving no preview or registration occurs.

1. **Leaf link, same bytes:** replace a selected `components/*.wasm` with a link to an in-root regular file containing the exact original bytes. The current loader admits it; the new owner must reject before `FixtureProviderSource::preview`.
2. **Intermediate link/reparse:** move `components/` to an in-root sibling and replace `components` with a directory link/junction. The bundle's lexical path remains unchanged and all bytes/digests remain valid. Reject before the provider. This is the case a final-component-only solution misses.
3. **Actor link:** in `trusted_browser_actor_loader_verifies_retains_and_cancels_before_publication`, replace `browser/closed-actor.mjs` with an in-root same-byte link. Reject before publication; do not silently accept because its SHA matches. [existing actor law](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:2015)
4. **Opened-handle swap:** directly open a normal component through `TrustedCatalogGenerationRoot`, rename/replace its path with a link or a different file, then read the already opened handle. It must return the original bounded bytes; no subsequent path lookup is permitted. A changed original handle payload must fail its recorded digest and still make zero provider calls.
5. **Cancellation/bounds:** use a component over one 64 KiB chunk, set `TestControl.cancelled` at a checkpoint, and assert cancellation closes the handle/no codec publication. Also test zero, `maximum + 1`, and a post-open length expansion; all must fail without growing retained allocation beyond the fixed maximum.
6. **Platform link fixture:** Unix creates leaf/directory symlinks. Windows creates a real directory junction or another real reparse point through a test-only first-party OS helper; inability to provision it is an infrastructure failure, not a skipped no-reparse qualification. Run the same semantic rows on macOS, Linux, and Windows.

These laws belong beside the current native trusted-catalog tests, whose existing hostile-path test only covers lexical `../` and therefore cannot certify this boundary. [current lexical-only law](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:2175)

