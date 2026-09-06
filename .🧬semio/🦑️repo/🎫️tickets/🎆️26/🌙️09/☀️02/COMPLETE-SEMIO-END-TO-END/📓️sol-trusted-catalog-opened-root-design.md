# Trusted Catalog Opened-Root / No-Link Design

Status: design boundary fixed before production edits on 2026-09-06. This packet is not a runtime verdict.

## Authority model

The trusted catalog will stop accepting an arbitrary bundle `Path`. Its only production entry will be the Hub-owned `OS_HUB_DATA` directory, opened once without following any path component. A crate-private `TrustedCatalogDataRoot` retains that platform directory descriptor. From it, the loader opens only the literal `trusted-catalog/current.json`, validates one canonical bounded pointer, then opens the fixed `trusted-catalog/generations/<generationId>` directory into a retained `TrustedCatalogGenerationRoot`.

The current pointer has exactly `profileId`, `generationId`, and `bundleSha256`. Both digests are lowercase 64-byte hex identities, the profile is bounded, and the serialized bytes must be canonical. `trusted-catalog.json` is opened from the retained generation root, its SHA-256 must equal the pointer before parsing, and the selected profile must repeat the exact pointer profile and generation. A missing current pointer means artifact authority is unconfigured. A present but malformed, linked, substituted, or unreadable catalog fails Hub startup closed.

No compatibility `load(&Path)` remains. The production shape is:

```rust
struct TrustedCatalogDataRoot { /* private platform directory descriptor */ }
struct TrustedCatalogGenerationRoot { /* private child directory descriptor */ }
struct TrustedCatalogRelativePath { /* bounded parsed UTF-8 segments */ }
struct TrustedCatalogOpenedFile { /* already-opened regular file */ }

impl TrustedCatalogDataRoot {
    fn open_server_owned(data_root: &Path) -> Result<Self, AuthorityError>;
    async fn open_current(&self, context: &OperationContext<'_>) -> Result<Option<TrustedCatalogSelectionRoot>, AuthorityError>;
}

impl TrustedCatalogGenerationRoot {
    fn open_regular(&self, path: &TrustedCatalogRelativePath) -> Result<TrustedCatalogOpenedFile, AuthorityError>;
    async fn read_regular(&self, path: &TrustedCatalogRelativePath, maximum: u64, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError>;
}

impl TrustedCatalogLoader {
    async fn load_current(data_root: TrustedCatalogDataRoot, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<Option<VerifiedTrustedCatalog>, AuthorityError>;
}
```

The relative-path value accepts only non-empty UTF-8 `/`-separated segments, with no empty segment, `.`, `..`, root, prefix, backslash, or NUL. Total bytes and segment count are bounded. Duplicate closure identity is a `BTreeSet<TrustedCatalogRelativePath>`; resolved filesystem paths never become authority or identity.

## Same-handle read contract

Every segment is opened relative to the retained descriptor while rejecting links/reparse points. The final handle is inspected after open and must be a non-empty regular file no larger than the declared maximum. The same handle is converted to the async file and read in fixed 64 KiB chunks with the current operation checkpoints. The read uses a maximum-plus-one fence so post-open growth is rejected without unbounded allocation. Cancellation, error, and success drop every transient handle. Hashing and decoding consume only the immutable returned bytes.

On Unix, the first data root is walked from an opened `/` descriptor for absolute paths or an opened current-directory descriptor for relative paths. Each component uses `openat` with `O_NOFOLLOW | O_CLOEXEC`, and ancestors add `O_DIRECTORY`; the final object is verified using `fstat`. The implementation lives behind the first-party owner and may use a target-private `libc` binding.

On Windows, the first root is opened from its volume/root authority and descendants use rooted `NtCreateFile` with `OBJECT_ATTRIBUTES.RootDirectory`, `OBJ_DONT_REPARSE`, and non-directory final-object semantics. The returned handle is checked with attribute-tag information; any reparse point or directory is rejected. Target-private `windows-sys` bindings remain wholly behind the first-party owner and do not escape in APIs.

## Startup and materializer migration

`configured_artifact_authority` will receive `&data_dir` and the linked provider set only. It will not read `OS_HUB_TRUSTED_CATALOG_BUNDLE` or `OS_HUB_TRUSTED_CATALOG_PROFILE`; authority selection comes exclusively from the canonical current pointer below the opened data root.

The normal materializer already writes the immutable generation and current pointer below its data root. Its isolated candidate currently launches with `candidate-data` while passing an external absolute bundle path. Before the Rust startup migration can land, the candidate path must instead stage the exact verified immutable generation and canonical current pointer below `candidate-data/trusted-catalog`, fsync them, and launch with no bundle/profile path environment. This is a coordinated edit in the Hub script owner region; it must not be replaced with a generic trusted-root environment variable.

## Test-first packet

A neutral fixture and JSON Schema will describe literal current/generation paths, bounded relative paths, the opened-handle phases, and denials. Its Bun/AJV oracle will agree with the Rust owner on valid segments and hostile empty, dot, parent, backslash, prefix, NUL, oversized, and duplicate paths.

Native laws use the existing `FixtureDirectory`, `TestControl`, and `FixtureProviderSource`:

1. an in-root same-byte component leaf symlink is denied before provider preview;
2. an in-root same-byte intermediate directory symlink is denied before preview;
3. an in-root same-byte browser-actor symlink is denied before actor publication;
4. a file opened normally and then path-swapped still reads the original handle bytes, while a digest mismatch publishes nothing;
5. zero length, maximum-plus-one, post-open expansion, and cancellation after one chunk are bounded and publish nothing;
6. a linked initial data root, linked current pointer, and linked generation root are all denied;
7. Windows runs the same rows with a real first-party reparse/junction fixture and treats provisioning failure as test infrastructure failure.

The registered source/neutral and exact native targets will be `trusted-catalog-opened-root-check` and `trusted-catalog-opened-root-native-check`. Source success alone will not be represented as runtime qualification; macOS, Linux, and Windows each require their platform native row.

## Non-goals

This change does not add arbitrary path loading, legacy environment aliases, public file-descriptor APIs, actor activation, Map mutation authority, or a claim that digest verification alone proves filesystem identity.
