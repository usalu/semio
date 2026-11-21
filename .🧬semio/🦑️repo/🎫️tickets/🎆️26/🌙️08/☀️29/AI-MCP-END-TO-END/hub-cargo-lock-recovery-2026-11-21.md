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
