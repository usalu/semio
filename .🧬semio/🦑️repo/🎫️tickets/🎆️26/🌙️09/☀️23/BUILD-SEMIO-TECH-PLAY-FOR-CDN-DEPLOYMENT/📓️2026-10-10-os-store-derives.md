# 2026-10-10 os-store-derives

## Findings
- `CanonicalJsonTree` + `RetireOwned` already existed for `ArtifactChild<S>`, `ArtifactLink`, `LinkPin`, `BlobRef`, `ArtifactRef`, `ArtifactDialect` (store `🔗️link/🧬️schema`, `♻️retirement`, artifact-reference crate) and for `Viewport2d` (`🖱️ui/🪟️viewport/◻️2d/🧬️schema`, hand-written tree + derive `RetireOwned`; the OS kernel only re-exports it). Nothing duplicated or changed there.
- Dependency direction is fine (store -> value, pack-json, artifact-reference).

## Added (the only remaining gap: `RetainedClone` for `ArtifactChild<S>`)
- `🏪️store/🪆️child/🧬️retained-clone/🦀️.rs` (new): `ArtifactChildCloneCursor<S>` (field cursors for `child_id`, `target`, then an alias step for `local_owner`, then assembly; close ladder mirrors the derive's), `ArtifactChildLocalOwnerAlias` (+ one-step `RetireOwned`), `impl<S: Send+Sync+'static> RetainedClone for ArtifactChild<S>`. Hand-written because the derive treats `PhantomData<S>` as a field and cannot add `S: 'static`.
- `🏪️store/🦀️.rs`: one `#[path] mod artifact_child_retained_clone;` line after `impl Clone for ArtifactChild`.
- `🧬️schema/🗿️artifact-reference/🦀️.rs`: `semio_framework_value::RetainedClone` added to the `ArtifactDialect` and `ArtifactRef` derives.
- Tests (`🏪️store/🧪️tests/🧬️retained-clone/🦀️.rs`, appended): `artifact_child_retained_clone_matches_clone_and_value_oracles_and_aliases_the_local_owner` (Clone + `ToValue` oracles, `Arc::ptr_eq`, strong count returns to 1) and `artifact_child_retained_clone_retires_every_partial_cursor_under_cancellation`.

## Verification (slot-gated, private dirs `play-fleet/os-store-derives`)
- `cargo check -p semio-framework-os-kernel --lib`: Finished, 0 errors.
- `--target wasm32-wasip2 --lib`: Finished, 0 errors.
- `cargo test --lib artifact_child_retained_clone`: NOT RUN to completion; the lib-test target fails to parse a peer file (`🏪️store/🧪️tests/🔬️unit/🦀️.rs:11031`, mismatched delimiter in a `with_fixture_identity!` migration in progress, not mine). Log: `play-fleet/os-store-derives/test.log`.
