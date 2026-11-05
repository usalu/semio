# Remodel Source Bounds — 2026-11-04

Executed retained `🐍️remodel-source-bounds.mjs` with `repo tools exec --light node -- <ticket>/🐍️remodel-source-bounds.mjs`, exit 0. This invokes the actual `readRustSourceClosureV1` parser from framework process/native-build/source, with fs byte reads, manifest-relative ownership and no invented generated inputs. Only six changed owner crates and their actual selected module/input closures were audited, plus the external artifact editor integration target containing photo/JPEG tests. No compiler was run.

| Owner | Ordinary Files / Bytes / Active Lines | Test Files / Bytes / Active Lines |
|---|---:|---:|
| Engine Bridge | 2 / 24901 / 428 | 11 / 127009 / 2224 |
| Reconstruction Session | 7 / 85718 / 1694 | 11 / 147240 / 2393 |
| Command Ingest | 12 / 44569 / 666 | 12 / 44569 / 668 |
| Editor | 7 / 104786 / 1361 | 7 / 104786 / 1361 |
| Reconstruct | 8 / 125013 / 2244 | 9 / 125408 / 2341 |
| Mesh | 6 / 66165 / 1374 | 7 / 110435 / 2401 |
| Artifact Editor Integration | 1 / 7417 / 123 | 25 / 222884 / 3094 |

The supplied cfg sets were empty, `test`, and `test` plus `feature=testkit` plus `feature=artifact-app-testing`. The last two have identical closure counts for every audited owner. Physical successful read counts/bytes equal returned closure files/bytes in every case. The session test source alone is 42386 bytes; artifact editor unit source including JPEG coverage is 62738 bytes. No module-resolution or generated-input refusal occurred.

All seven exact Cargo manifests lack maxSourceBytes/maxSourceFiles/maxActiveLines declarations. They therefore do not tighten the selected source-boundary policy; authoritative effective limits must be read from the actual compilation unit and execution policy. The admission owner explicitly uses policy when unit.boundary is absent, and otherwise takes per-axis minima. Its schema has positive-integer constraints, no authored default ceiling. As a comparison only, every measured closure fits 262144 bytes / 32 files / 4000 active lines, the values used in the native-build Cargo fixture and earlier framework instance manifest. The largest audited integration closure leaves 39260 bytes, 7 files and 906 lines against that comparison. This is not evidence of the currently selected compiler policy.

No concrete source-bound blocker was found in these provisional cfg closures. This does not establish rustc type checking, runtime behavior, full actual compiler cfg/feature selection, generated output behavior, or formal precompiler admission. Shared edits can change the measured bytes. Native compilation remains authoritative. No source ceilings or production code were modified; the helper is cross-platform and resolves its parser from repository root.
