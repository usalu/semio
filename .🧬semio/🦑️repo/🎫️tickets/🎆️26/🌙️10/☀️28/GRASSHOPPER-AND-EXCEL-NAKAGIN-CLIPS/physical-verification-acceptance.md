# Current Physical Verification Acceptance

All **135 physical tests pass** on the corrected panel model. The full broad model/case/test AST/JSON map has identical start/end inputs. Hardened acceptance verifies exactly eleven domains, their exact counts, zero failures/errors, explicit success and current physical fingerprints. The model hash matches the fresh 52 structural checks, giving **187 passing physical/structural tests on one saved model**.

Model AST: 87255d8965e795fc9c5a00fed16fb8de67184b94552a72006f5d4959cdbaa084.

Physical input manifest: d1423bd15d7f3785fb515d710199611f86fe1aec0d9358e9709fb3c800768ffa.

| Domain | Tests | Seconds | Result |
|---|---:|---:|---|
| shared | 59 | 79.688 | PASS |
| 🚶️circulation | 18 | 21.781 | PASS |
| 📦️casing | 22 | 41.281 | PASS |
| 🧱️panel | 3 | 19.953 | PASS |
| 💊️detail | 9 | 10.813 | PASS |
| 🪟️window | 2 | 0.484 | PASS |
| 🧭️poses | 2 | 4.844 | PASS |
| 📍️survey | 6 | 0.000 | PASS |
| 🪟️unit | 8 | 44.578 | PASS |
| 🔧️mounts | 4 | 12.625 | PASS |
| 💡️light | 2 | 4.515 | PASS |

## Acceptance Contract

Every expected domain must appear exactly once with the exact authored test count. Empty, missing, duplicate, wrong-count, unknown and failed domains reject acceptance. Portable contract/schema inputs are retained beside accept-physical-evidence.py. The Python predicate and independent AJV validator agree on all eight positive/negative examples; the hardened recheck exits 0.

The model uses standard-library code only. Physical tests load that model and their own JSON; unit tests reuse mounts, mounts reuse circulation and light reuses mounts. Third-party oracles are NumPy, Rhino and NetworkX. No physical domain imports or dynamically reads the renderer or its visual/occlusion contracts.

The reusable scope filter names five exact audited rendering-only paths; it excludes no entire directory or future file. The 135-test run itself had unchanged broad endpoint maps. During subsequent renderer integration, the visual-case JSON changed and a strict recheck correctly rejected it because that fifth path was still conservatively included. The failed record is retained as physical-acceptance-missing-renderer-case.json until generated cleanup. Every physical domain’s actual JSON reads/imports were reread: none consumes the visual-case JSON or renderer. The contract now excludes that exact path too, and the current recheck passes with physical dependency maps unchanged. Any other unlisted input drift still fails. Canonical current physical map SHA256: 4307178a632167490a69cf730bb869333bb79ca47d63b79a7972f6206f4820af. Paths are serialized with forward slashes for Windows, macOS, Linux and containers; raw original endpoint records remain untouched.

## Historical Snapshots

The earlier 132 physical plus 52 structural result did not test missing intended panel material and was superseded by the confirmed corner slit. The first 134-test integration retained two lining-back manifold failures; that failed result is saved separately. Exact clipping vertex identities and a third all-lining regression corrected it. These historical artifacts are retained and never substituted for the current 135-test proof.

The separate ray/culling proof applies to its immutable pre-panel-correction export. Its source/raster hashes and preserved controls are documented in pixel-surface-attribution.md; current renderer and native acceptance remain separate.

## Admission

The final 135 run was already admitted and was retained under its original grant. The subsequent short acceptance recheck used official repo capacity run with 2,048 MB and two CPU cores, preserving queueing, the host floor and inherited live-grant validation. No admission bypass was used.

Native Excel replay remains on HOLD pending the final renderer/source GO.

## Seven Exact Unconsumed Renderer Inputs

The final sparse-overlap work adds dedicated occlusion schema and CJS schema-test inputs. Independent read-only audit of all eleven physical domains and their helpers confirms neither is consumed by the physical tests. The explicit unconsumed contract therefore lists seven exact files, without any directory or wildcard. These two new files were absent from the original135-run endpoint map, so adding their exact declarations does not remove an additional captured physical dependency or change the canonical physical hash. The original raw endpoint record remains untouched. All captured consumed physical inputs are rechecked against current sources.

The seven-path recheck passes with the original teaching Python3.12 interpreter:135 tests, consumed dependencies unchanged/current, canonical hash4307178a632167490a69cf730bb869333bb79ca47d63b79a7972f6206f4820af. An initial stdlib Python3.14 check rejected every Python AST fingerprint because AST serialization differs across interpreter versions; rerunning under the recorded validation runtime resolves that mismatch without any model or test source edit. Semantic Python fingerprint comparisons must use the same interpreter version as their recorded endpoints.
