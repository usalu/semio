# Build Operation Bridge Independent Review

Read-only current draft review; no test or compiler launched. Actual GREEN76849 is pending according to Runtime's handoff.

The native-build `📥️resources/🦀️.rs` helper records real read/read_dir internal source+line/name separately from output paths. File reads retain returned original bytes in an OUT_DIR snapshot. Directory reads retain complete typed entries and raw symlink targets, sorted by UTF-8 filename bytes. Immediate read/listing/entry-metadata/link failures record explicit failed rows and preserve the caller's I/O error. Failed rows are unresolved proof, not evidence of complete empty input.

Runtime `runtimeBuildResourceReadOwnersV1` first validates the same retained JSONL/resource snapshots, then requires one actual production custom-build unit and one build-script package/OUT_DIR record in the same observation. It checks current unit inputs, actual rustc-consumed checksums, and surviving selected artifact bytes. Internal operation source must be a file input in that same unit; exact name/line resolves to one token offset, otherwise unresolved. Per-operation original file or directory inputs populate core ownership. Failed/incomplete proof emits findings; no blanket helper waiver exists.

Relative file!() operation paths are resolved by the build helper against actual process cwd. If that does not match the original compiled source path, the bridge's source membership/unique-token checks refuse; do not assume a Windows or foreign-cwd success until actual receipt covers it. Resource enumeration is terminal directory metadata and does not imply reading child bytes. Copy rows retain original/output identity but are not reinterpreted as a read operation token.

No new concrete unsafe promotion was found in this bounded draft. Currentness of actual production WGPU receipts, actual operation mapper GREEN, directory proof and final host graph remain required separately. The owner reports no current Canvas/Infinite builder at those named manifests; this audit makes no inferred original resource reader claim for absent builders.

## Retained Actual Neutral Receipt

Independently parsed `🗑️generated/oct8-runtime-graph-artifacts/build-read-cargo-70657325-d82c-4e86-b17a-ab1072b94f7f/verified-receipt.json`: successful uncancelled actual Cargo check targeting wasm32-unknown-unknown, two compiler units (host custom-build and target library), one retained buildResources envelope, two mapped helper operations, mapper and graph findings both empty. Rehashed all retained file inputs at this audit checkpoint: 0 drift. The source-specific helper is present in raw .d and rustc consumed-byte checksum records. This is genuine neutral Cargo/host producer proof, not the real WGPU guest/publication. Owner reports full registered GREEN76849 95/0/1095 in12.7s; the complete test log was not independently parsed here.

## Typed Directory Schema Follow-up

Fresh source confirms RuntimeCargoBuildResourceSnapshotV1.observedEntries references the genuine RuntimeCompilerResourceEntryV1 leaf. Original JSONL typed path/kind/raw-link roster is compared with current typed enumeration, direct-child membership, retained observedEntries and canonical basename/kind/link digest. Same package/OUT_DIR/unit/consumed source and exact operation token prerequisites remain. Runtime reports schema RED12759 then GREEN88641 95/0/1096; no fresh production build inferred.
