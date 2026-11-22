# Hub Cargo Lock Recovery

The managed Note producer logged a `--locked` rejection after reporting 703 compiled units. The next two lock messages recorded the repository launcher refreshing the Hub lock from its shared registry and confirming it was current. The same run then reported source-configuration checking and resource-directory admission activity. The producer log is `🗑️generated/native-call-managed-prototype-note-graph-ready-1.log`, lines 4720–4724.

The live Hub Cargo lock differs from the index by exactly two dependency entries. Each matches a dependency declaration in a live manifest outside the Note composition.

| Cargo package | Added local dependency |
| --- | --- |
| `semio-framework-3d-brep-native-registry` | `semio-framework-3d-brep-queries` |
| `semio-s-artifact-norm-en1998-mutations` | `semio-s-artifact-norm-en1998-schema-formulas` |

The declarations are in the B-rep engine native-registry manifest and the EN 1998 mutation-schema manifest. Neither edge adds or changes a registry package. The observed live lock SHA-256 is `d202dea6cb97ef83a641c05761051f88407195f8e93476b1f904bf73d13810c1`. The index is the comparison baseline for the diff.

Pinned Cargo `nightly-2026-07-20` returned exit code 0 for `metadata --locked --offline --format-version 1` with `🌎️hub/Cargo.toml`, for both `wasm32-wasip2` and `aarch64-apple-darwin`. These resolution checks used the managed Cargo registry directory and the pinned compiler for Cargo's target probes. The managed Cargo wrapper encountered directory admission during the metadata attempt, so the verification used the permitted pinned native fallback and compiled no units. The pinned Cargo rejected the temporary alternate-lockfile option before resolution, so the index copy served only as the diff baseline.

At the follow-up snapshot, the managed Note producer still reported resource-directory admission activity and 0/1 staged components. This audit made no source or lockfile edits.

## Final Managed Refusal and Exact Replay

The same managed run later exited with `SEMIO_CONCURRENT_EDIT`. Cargo exited 101 during the second warm-retry unit-graph observation, after the first observation and source admission. The classifier assigns that code to Cargo's stale-lock diagnostic. The run retained the first retry graph but produced no second retry graph or staged Note component. See producer log lines 5157–5182.

The first retry graph was recorded at 17:48:15 and matches the original graph byte for byte. The native-registry manifest was updated at 17:48:25, and the earlier automatic lock refresh followed at 17:48:42. The managed run ended at 17:56:07. The lock's mtime also advanced after the run while the content matched the previously audited hash. The solid-transaction manifest had an mtime of 17:56:57. Its live spline-construction dependency is already present in the staged lock comparison. The failed subprocess's manifest and lock bytes were not captured, so the exact drift at 17:56:07 is unresolved.

Pinned Cargo replayed the failed `build --locked --manifest-path … -p semio-hub-note-component --lib --target wasm32-wasip2 --profile prototype … -Z unstable-options --unit-graph` with all logged config assignments, the managed registry, and the managed `selected` feature resolver. Both online and offline resolution returned 895 units rooted at `semio-hub-note-component@0.1.0`, without starting compilers or updating the live lock. The fresh offline graph structurally matches both retained graphs across all units and dependency edges. The locked workspace metadata check also passed with the same resolver.

A fresh owner-managed Note invocation can establish a new lock and source admission and reuse compiler cache entries. The live lock requires no repair based on these checks. Managed staging after a fresh invocation remains unverified.
