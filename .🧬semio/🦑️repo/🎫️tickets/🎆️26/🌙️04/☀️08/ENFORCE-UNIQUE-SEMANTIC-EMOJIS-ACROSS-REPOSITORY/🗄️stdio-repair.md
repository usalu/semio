# Stdio Hand Review

This review starts with the plugin's scaffolding, packages and shared oracle families; format-specific artifact trees are not yet included. The physical pre-edit audit found 63 entries, 54 governed entries, two missing prefixes and one sibling collision.

Four exact moves are complete: the B-rep timing source is `🏃️benches/⏱️brep-kernel.rs` (performance runs and timing); the package specimen directory is `🧫️fixtures`, distinct from `🧪️tests`; its catalog authority case is `🌳️catalog-root`. The benchmark parent now matches the existing performance-run role instead of the old bug icon. The Cargo target identity `brep_kernel` stays unchanged. Reserved Cargo/package manifests and the already meaningful, distinct oracle-family emojis remain unchanged.

Pre-edit byte evidence:

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| B-rep benchmark | 7408 | `6a3a553ea4f1f1189424e5657a2f18aa4b6f2eb0ee8732c163b54c23e6c7fc5a` |
| Catalog specimen | 1013 | `1732e9fe60ab1218a17770e279c10cb6a0d06ec6f80c7e02d96a46399c5fb903` |
| Catalog schema | 1757 | `17bdae6acbcdbcf0c144fac074f05e858059e05bfdfc9b3c96c4d45c6e7aaad1` |

No whole-plugin completion is claimed. Artifact format siblings, mutation owners and before/after specimens still require individual review.

The three moved files retain all original bytes and hashes. The catalog-root contract ran successfully through the existing Stdio Nx test route, including its unchanged Ajv, WebCrypto and Binaryen checks. Two attempted benchmark verification invocations did not run a compiler: nextest rejected `--no-run`, and Nx exec inherited the wrapper's `npm_lifecycle_event=nx`. The corrected explicitly selected Nx exec `cargo check -p semio-s-plugin-stdio --bench brep_kernel` completed successfully in 25m21s, including its shared build-lock wait. This is a native compile check, not a benchmark execution or codec-test result. Evidence: `🗑️generated/stdio-benchmark-check-final.txt`.
