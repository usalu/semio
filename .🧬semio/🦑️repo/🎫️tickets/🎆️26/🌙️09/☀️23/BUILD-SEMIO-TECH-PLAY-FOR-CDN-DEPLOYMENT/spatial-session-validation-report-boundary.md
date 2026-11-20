# Spatial session validation report source boundary

Live source preflight reproduced the supervisor's refusal at 12 files and 262,176 bytes for the spatial session crate. Its ceiling is 32 files and 262,144 bytes.

The B-Rep engine now owns the native validation-report cursor alongside its original geometry validation and reachability APIs. The cursor retains its source identity checks, staged validation, work and memory quotes, and JSON encoding. The session re-exports the report type and uses its existing guarded kernel query for admission, demand, and stepping. Both crates already use the validation dependency, and the split requires no manifest or lockfile edits.

The same source preflight then exited successfully with no over-budget rows or errors across the spatial kernel, B-Rep engine, and Flow B-Rep extension trees.

| Rust source owner | Before | After | Remaining bytes |
| --- | ---: | ---: | ---: |
| Spatial session | 12 files, 262,176 bytes | 12 files, 252,921 bytes | 9,223 |
| B-Rep engine | 22 files, 234,470 bytes | 23 files, 244,275 bytes | 17,869 |

The targeted Bun source scanner run passed 28 tests and 49 assertions, including the Ajv fixture corpus and independent SQLite condition evaluation. Rust compilation and the runtime validation tests remain with the release workflow.

The follow-up static review traced the report through the B-Rep engine facade, the session re-export and the Flow original-jobs consumer. It checked the visible construction, demand, stepping and retirement APIs and the separate tessellation resource methods. A Bun TOML check verified 17 dependency paths, package identities and corresponding Flow B-Rep lockfile edges across five consumers. The repeated source preflight exited successfully. The review found no confirmed defect and made no source edits.

## Changed files

- [Funded report cursor](../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🩺️validation-report/🦀️.rs)
- [B-Rep engine facade](../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🦀️.rs)
- [Session kernel custody](../../../../../../../✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🩺️validation-report/🦀️.rs)
- [Session re-export](../../../../../../../✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs)
