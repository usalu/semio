# Independent Base Complete Native-Matched Aggregate Authority

The 36 full and metadata-retaining empty authorities are independently authored from all 18 actual child SQLite Native fixtures. They are distinct from the existing 18 strict Native default authorities. No owning projector was invoked, and no production or test source was changed.

The actual Bun run validated 108 SQLite databases: semantic, public Binary, and public Text for each case, including integrity and foreign-key checks. Every semantic database retains 183 tables, 56,726 schema bytes, and maximum width 54. Public databases retain 184 tables and 57,019 schema bytes. The real Base root row selects exactly one child; its value-byte cost is 27 plus the UTF-8 subset tag length. Measured public metadata adds one row and 38 Binary / 36 Text bytes.

| Subset | Full rows / bytes | Empty rows / bytes |
|---|---:|---:|
| brep | 80 / 5180 | 2 / 75 |
| mesh | 101 / 7681 | 2 / 55 |
| model | 100 / 6528 | 2 / 57 |
| value | 23 / 626 | 3 / 77 |
| document | 53 / 1309 | 3 / 81 |
| cad | 28 / 2026 | 2 / 53 |
| drawing | 33 / 2016 | 2 / 193 |
| image | 22 / 867 | 4 / 163 |
| video | 11 / 543 | 2 / 57 |
| audio | 15 / 546 | 4 / 146 |
| animation | 185 / 10116 | 2 / 67 |
| presentation | 53 / 2259 | 2 / 73 |
| flow | 7 / 351 | 2 / 55 |
| text | 9 / 288 | 2 / 57 |
| table | 21 / 627 | 8 / 265 |
| graph | 20 / 830 | 2 / 59 |
| object | 8 / 495 | 2 / 279 |
| kit | 22 / 1372 | 2 / 53 |

Seven retained manual child scripts were executed behind a SQLite cell-capture boundary. Only their original handwritten SQL statements ran; file writes were suppressed and public metadata tables excluded. Eleven additional constructors hand-insert canonical semantic rows into the actual SQL schemas. The four children without neutral Source snapshots were constructed from their actual Native fixture definition and its SQLite JSON asset. Infinity real cells are represented explicitly as `realSpecial` in the neutral cell authority; Binary64 integers retain signed decimal words, Binary32 bits retain positive integer words, and blobs retain literal byte arrays.

Object preserves the actual Native fixture targets and literal child identities `brep-owned`, `mesh-owned`, and `value-owned`; it does not substitute the child identity into the target artifact ID. Its child census is full 7 / 462 and empty 1 / 246. Drawing captures the corrected canonical SQL segment tags and actual child census 32 / 1,982.

Strict Ajv 2020 closed-const contract validation passed. The emitted Source hydration transpiled and its actual framework admissions validated 100 Binary32 words and 867 Binary64 words. Six available `parseSemio<Subset>Snapshot` exports were exercised successfully; this does not claim parsers for the interface-only child snapshots. Video signed rate and unsigned timestamp decimal texts hydrate to BigInt, Object null optional children are omitted, and the retained per-child hydration preserves existing optional-field rules.

Inputs: `complete-corpus/📜️script.ts`, `handcrafted-native-complete-contract.json`, `handcrafted-native-complete-contract-schema.json`, `handcrafted-native-complete-semantic-cells.json`, and `handcrafted-native-complete-demand-specifications.json`.

The proposed test-only reuse roster has 37 exact guards: expose each of the 18 existing `#[cfg(test)]` SQLite test modules as `pub(crate)`, expose each existing `fixture()` as `pub(crate)`, and add one Base test helper dispatching to those fixtures and clearing only each metadata-empty collection. The fixture bodies are not copied. There is no production API. Guards and helper are `test-only-fixture-reuse-narrowed-guards.json` and `test-only-fixture-reuse-held.rs`. Read-only rustfmt accepted helper syntax. This is not Rust type or Native runtime credit; helper integration and original owner demands remain Root-owned.

Nx/launch registration for this dedicated input validation was requested from Root and is pending at report creation. The measured results above are from the direct Bun input-script run; no duplicate whole-owner command was run.

Root subsequently registered the dedicated complete-corpus validation and reported actual Nx97608 exit0: the 108 manual SQLite authorities, strict Ajv, framework word admissions, and exported domain parser checks ran through the registered owner wrapper. This qualifies the input oracle run; it does not claim a Native provider or complete Base owner pass.

The earlier chat statement that Object differs from the old Source shortcut was inaccurate: the current fixture asset explicitly gives target artifact IDs equal to those child IDs, so that shortcut presently produces identical values. The new independent corpus still reads original target fields separately and does not infer their equality.

Actual dedicated registered Nx replay38828 completed exit0 after the added comparisons. Its final receipt confirms all18 mounted child comparisons, the 108 Base semantic/public databases, strict Ajv, 100 Binary32/867 Binary64 admissions, six exported domain parsers, and paths=0 (no mounts). This remains input-oracle scope only.
