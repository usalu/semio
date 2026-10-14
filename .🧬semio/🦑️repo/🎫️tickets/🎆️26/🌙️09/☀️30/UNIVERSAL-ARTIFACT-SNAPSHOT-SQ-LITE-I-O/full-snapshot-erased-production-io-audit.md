# Full Snapshot Erased Production IO Audit

Read-only source audit, 2026-10-13. No builds, tests, production changes or runtime qualification.

## Proven Current Boundary

The full typed file APIs in `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs:2632` and `:2664` accept/return the actual `P: ArtifactSqliteSnapshot + RetireOwned + 'static`, supplied SQL control and original native owners. They need neither `ArtifactDsl` nor `ArtifactPack`. Their production declarations preserve a full snapshot boundary; existing migrated calls alone do not establish universal production routing.

The erased registry capability in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🦀️.rs:198`, `ArtifactSqliteSnapshotCodec`, instead exposes only `export(&IoPayload, ...) -> IoResult<SqliteDatabase>` and `import(...) -> IoResult<IoPayload>`. Its generic factory requires `ArtifactDsl + ArtifactPack` and invokes the native payload bridge. TypeId is a check, not a borrowed full snapshot callback. `IoEntry.run` likewise accepts/returns `IoPayload` (`🚪️io/🦀️.rs:2199`).

`snapshot_descriptor` and `supplemental_snapshot_entry` (`🚪️io/🦀️.rs:2301–2307`) mark any registered raw bridge `IoFidelity::Exact`. This descriptor does not prove full legal snapshot fidelity. TXT raw `to_body/from_body` normalization remains the counterexample documented in `txt-whole-snapshot-file-io-scope-audit.md`: arbitrary schema, embedded separators, and `[]` versus `[""]` cannot be carried through raw text bijectively. A raw-format IO route and a complete snapshot persistence route require separate semantic contracts.

## Existing Exact Borrow And Return Hooks

`🏪️store/🦀️.rs:4651`, `ArtifactStore<P, Mutation>::snapshot_ref()`, borrows the current immutable `P` without issuing/cloning snapshot custody. It selects the actual durable group root when present.

`🏪️store/🔗️read/🦀️.rs:865`, `ErasedSnapshotRead`, retains `Option<Arc<dyn Any + Send + Sync>>` plus its original `SnapshotReadLease`. Existing methods already provide:

- `get<T>() -> Option<&T>` and `typed<T>() -> Option<SnapshotReadRef<'_, T>>`: downcast a borrowed original root, with no payload clone.
- `try_into_typed<T>(self) -> Result<SnapshotRead<T>, Self>`: transfer the exact original Arc and lease; mismatch returns the original read.
- `commit_authority_matches(generation, revision)`: original issuer authority check.
- `try_duplicate()`: issues another original registry slot and Arc alias; this is an explicit additional lease, not necessary for a caller-owned borrow.

`SpaceMember::snapshot_read_erased_now` / async `snapshot_read_erased` (`🏪️store/🦀️.rs:10750`) already expose complete captured roots across type erasure. Its `as_any_mut` returns the concrete store rather than a snapshot, so downcasting the whole store is unnecessary for the registry callback.

Exact dynamic retirement exists: `🏪️store/🔗️read/🧾️return/🦀️.rs`, `impl RetireOwned for ErasedSnapshotRead`, owns the read in `ErasedSnapshotReadRetirement`, returns its original slot through `try_return_to_registry_witness`, then closes a retained `SnapshotReadReturn` witness. The final registry backing may require the original Store-owned retirement pump; a generic IO consumer must not bypass it with `Arc::try_unwrap`, an unrelated factory, or direct drops. `SpaceMember::retire_snapshot_read_erased`, returned-read demands and close methods remain available for issuer-specific custody.

The similarly named framework `ErasedReadLease` in `🌱️value/🔗️read/🪪️lease/🦀️.rs` is a different type with its own paid erased frame. Replacing Store reads with it would introduce a new ownership boundary; no such replacement is needed to borrow `P` for export. The composition owned-emission module erases mutation batches, not complete snapshots.

## Minimum Full-Scope Production Extension

Extend the actual registered snapshot capability with a full borrowed export thunk specialized by `P`. Accept a borrowed existing `ErasedSnapshotRead` (or an explicit borrowed `dyn Any + Send + Sync` view if registry provenance is separately supplied), check `get::<P>()` and original TypeId/schema/dialect, then call the existing typed file export under the caller's supplied SQL control and decode owner. Keep the read alive across the returned future. Do not copy `P`, serialize it to raw bytes, duplicate its Arc/lease merely to satisfy async erasure, or clone `ArtifactCodec`.

Import cannot return a borrowed read because it constructs a new `P` not yet issued by a Store registry. Add a repository-owned dynamic snapshot result capability whose generic implementation retains `Option<P>` and exposes borrowed Any/type identity plus explicit `RetireOwned` cursor/demand dispatch. Price its exact generic wrapper/box before ownership transfer using the same original encode owner. Prefer the existing owned-value retirement factory/cursor contracts; do not expose external types or synthesize a Store read lease without its actual issuer. A receiving factory may alternatively publish directly into an existing typed Store-owned candidate frame, provided refusal retains the exact candidate and diagnostics.

Keep imported `P` and `IoResult` diagnostics in the original receiving frame through publication/close. Erasure must occur inside that frame, with a priced move into the destination, not after successful typed API return followed by an unaccounted `Box::new`. Borrowed export needs no new erased snapshot wrapper. Dynamic import does need a paid concrete owner boundary unless the existing receiving Store factory takes ownership directly.

Schema-first law: a legal TXT snapshot with arbitrary schema, embedded NUL/Unicode/separators, empty list and one empty line exports through the global complete-snapshot interface; physical SQLite independent inspection preserves every row/metadata cell; import returns the exact full fields without invoking raw native `to_body/from_body`. Add mismatch-before-birth, cancellation, prior cumulative receipts, zero-close conservation and same-owner funded cleanup laws. Keep raw payload fidelity qualification distinct from full snapshot fidelity.

## Evidence And Policy Erratum

Parent reports Native64829 terminated with two stale Store fixture include errors, corrected afterward; fresh Native36879 is active. Neither run establishes selected runtime laws here. Earlier CSV/TSV policy census values are superseded: parent verified the actual relocated receiving fixtures use body capacity 4 MiB/depth 128 and close capacity 4 MiB/depth 256; four migrated strict caller literals were corrected. Historical 16 MiB/depth 4096 CSV/TSV assumptions must not be used as current authority.
