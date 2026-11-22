# Crate Split and Immutable Registry View

## Task 1: semio-plugin-instance-app source split

- State found: `maxSourceBytes` was already back at the standard 262144 (the natural-source agent had extracted `semio-plugin-instance-natural-publication` and `🪢️members`). Reachable crate source measured 216,715 bytes in 13 files, so admission passes with the standard cap and no override remains.
- Split of the 92,608-byte `PluginApp` root (`🔌️plugin-app-impl/🦀️.rs`, now 80,496 bytes): a trait impl cannot span files, so cohesive method bodies moved into inherent `pub(super)` methods in per-component modules wired by `#[path]`; the trait methods delegate by the same name (inherent methods win resolution).
  - `🎞️media-transfer/🦀️.rs`: `prepare_media_import`, `advance_original_media_import`, `submit_media_export`, `submit_media_export_checked`, `poll_media_export`, `import_media`, `media_fingerprint`.
  - `🔖️reserved-mount/🦀️.rs`: the reserved mount and permit ownership probes plus `selected_typed_result_page`.
- A concurrent writer restored the root file once (last-writer-wins); the split was reapplied on top of the natural-publication extraction.
- Evidence: `repo cargo check -p semio-plugin-instance-app --lib` (prototype, shared store) exited 0 (admission passed at 262144). Remaining: further components of the root (typed-operation publication, backbone, transactions) can follow the same pattern.
- `retire_born_candidate_step` now exists in `🔨️modules/🧵️job/🏭️construction/🦀️.rs` (delegates to `close_step`); not touched.

## Task 2: G9c immutable registry view

Root cause (see `cargo-registry-drift-audit.md`): the fetched receipt folded path, device, inode and mtime of locked `.crate` archives, so any touch, rewrite, rename or fleet restoration of an identical archive invalidated artifact consumption.

Fix in `🏃️process/🦀️cargo/📥️dependencies/🧾️fetched/🟦️.ts`:
- The registry view is content-addressed by the lockfile: `fetchedLockedV1` yields sorted `[name-version, checksum]` rows (every registry package must carry a SHA-256 checksum, else the lock is refused).
- `fetchedRegistryViewV1` requires each row to exist in any registry index directory with content SHA-256 equal to the checksum; the view digest folds only name and checksum. Inode, mtime, index placement and unrelated packages cannot change it; a missing or different archive refuses.
- `FetchedPortV1.stamp` replaced by `digest(path)`. The storage port (`🟢️ready/💾️storage/🟦️.ts`) hashes regular non-symlink files, streamed, memoized by path/dev/inode/size/mtime, with a stable-stat check around the read.

Tests (`🧾️fetched/🧪️tests/🟦️.ts`, fixture `🔣️.json`, language-agnostic sequence plus Python oracle recomputing the digest independently): touch, recreate (new inode), move across index directories, unrelated crate and source additions keep the receipt holding; corrupt content, removal and lock change refuse; capture refuses corrupt, missing, git, path and broken locks. 15 pass, 0 fail (run with the cached bun directly because the governed `repo exec` was blocked by an unresolved Nx engine import `🧊️stamps/🟦️.ts` from other agents' edits).

Limits: the unpacked `registry/src` tree (the live `tempfile-3.27.0` missing source file symptom) is not part of the view; Cargo re-extracts it from the verified archive only when its `.cargo-ok` marker is absent, so a deleted single file in `src` remains outside this proof. The `ready` receipt (other stamp consumers) was not changed.

## Follow-up: extracted registry/src repair and Nx import

- `fetchedSourcesV1` (fetched module) compares every locked package's `registry/src/<index>/<name>` directory with the files of its checksum-verified `.crate` (own gzip/ustar/pax/GNU tar reader, `node:zlib` only) plus the `.cargo-ok` marker; with `repair` it re-extracts a missing, partial or altered directory (stage, move aside, rename, remove). Extra files are tolerated; a corrupt or absent archive refuses.
- Hooked in `🟢️ready/💾️storage/🟦️.ts`: every ready/fetched receipt hit must also pass a verify-only source check (else it falls to the writer path); the exclusive-lease writer repairs before and after `cargo fetch`, so cargo never sees a partial source tree (the `tempfile-3.27.0` symptom). Lockfiles with git/path sources are skipped.
- Test: `🧾️fetched/🧪️tests` generates a real crate with Python `tarfile` (independent of our reader) and walks missing, deleted-file, tampered-file, missing-marker, extra-file and corrupt-archive cases. 16 pass.
- Nx import: `./🧊️stamps/🟦️.ts` was an in-flight edit by another agent; it has landed and governed `repo exec` runs again. It then ran the fetched and dependency tests: 27 pass, 1 fail (`wasmCompilerSelectionArgumentsV1 is not defined` in `📥️consumption`, another agent's in-flight wasm helper, not this change).
