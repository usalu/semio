# IFC SQLite Rich Port Audit

Read-only snapshot; no compiler started. Concurrent shared IO edits may alter signatures.

## Shared contract and ownership

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/📥️lifted/🚪️io/🦀️.rs` declares four rich methods: `to_sqlite_database_receiving` (decode owner), `from_sqlite_database_receiving` (encode owner), `decode_sqlite_snapshot_native` (decode owner), and `encode_sqlite_snapshot_native` (encode owner), all `RetainedWorkOutcome<T, NativeSnapshotFailure>`. The separate supertrait `ArtifactSqliteSnapshot` ordinary database methods still return Result; do not change their signatures or confuse them with the receiving overrides. Preflight remains Result.

`🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🫴️receiving/🦀️.rs:24,46` captures control.performed_work before a receiving operation and returns its checked monotone delta. The shared artifact receiving Port increments the same SQL control only for actual admitted actions, before their fallible body, at `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🦀️.rs:49-56`. Use its actual delta once, including errors; do not add an independently reconstructed item count or double-count nested work.

## IFC4 and IFC2x3

The two host impls are `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:47` and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:72`. Their native decode and encode overrides still return Result and delegate sibling `🚦️native/🦀️.rs`; receiving projection and reconstruction use trait defaults unless explicitly overridden. Ordinary `to_sqlite_database` and `from_sqlite_database` are supertrait methods and remain Result. IFC encoding first calls `admit_sqlite_values` then shared record-native encoder; decode calls shared decoder with a binding closure. Make preflight errors owning rich failures with actual performed-work delta, then forward the child outcome and work without selecting a single inner error. Capture and add only distinct preflight work if the child helper reports only its own delta; check helper contract after shared IO owner settles it. Preserve native output and error owners on refusal, and never drop a returned complete NativeSnapshotFailure to select its message.

Current shared record-native helpers are still old bare Result signatures and bare Result closure bodies: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs` and sibling `🛬️native-decoding/🦀️.rs`. They call now-rich `scoped_native` and `receive`; they must themselves propagate actual child work and complete failure before IFC wrappers can compile safely. Their original observer refusal and allocation settlement are additional fallible owners: retain both rather than replace a child failure, and keep actual pre/post owned byte settlement. Coordinate with shared owner.

## IFC4x3

No IFC4x3 `ArtifactSqliteSnapshotHost` override or direct record-native helper call exists under its subtree; its IFC graph is the IFC4 snapshot type. Its route has separate native owned translators: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/native-step-write.rs` and sibling `native-gltf-write.rs` declare `encode(...)->Result<Vec<u8>,ValueError>`, directly return `owner.receive` and use bare Result closures. Inspect the sibling `native-step-read.rs`, `native-gltf-receive.rs`, and route callers for the same change. Port their actual work and complete owner failure through the route boundary rather than synthesizing a work unit or flattening at `IoError`. These are distinct from SQL performed_work accounting.

## Finite native targets

- `semio-framework-job` target `original-native-progress`, feature `testkit`, entry `🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/📊️native-progress/🦀️.rs` (4537 bytes, 46 newline-terminated lines). It exercises actual completed 256 cancellation, zero-allocation callback, nesting and unwind restoration. This is entry size only; full transitive source cap was not measured, so no cap admission is claimed.
- `window_pair_receiving_outcomes`, declared in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📦️packages/🦀️rust/Cargo.toml`, entry `../../📐️owner-contract-and-mutation/🛬️receiving/🧪️tests/🦀️.rs`. Native caller exercises pair failures and five grant axes; pending semantic/native proof. Saved-window loader tests reside at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/📦️typed-pack-load/🛬️native/🧪️tests/🦀️.rs`; no separate integration target was found in this audit.
- IFC finite integration target and entry are recorded in `📥️runtime-coherence-audit.md`. Source-only inspection is not a semantic pass.
