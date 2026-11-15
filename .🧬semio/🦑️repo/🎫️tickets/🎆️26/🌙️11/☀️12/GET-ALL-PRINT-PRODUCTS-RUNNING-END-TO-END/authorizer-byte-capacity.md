# Captured authorizer byte capacity exceeded

Root cause: legitimate growth, not a stray file. Census (censusNxAuthorizerSourcesV1) of both tooling entries:
- authorizers/📦️runtime/🟦️.ts: 1960 files, 17,271,179 source bytes (import 12.08 MB, discovered-data 4.98 MB, declared-companion 0.18 MB), snapshot JSON 24,560,986 bytes.
- authorizers/📜️script.ts: 1906 files, 17,049,058 bytes, snapshot 24,224,854.
- Largest single file 387 KB (bazel scopes json); no generated/duplicated/huge file, average ~8.8 KB.
- readNxAuthorizerSnapshotV1 used snapshot/🔣️policy.json (16 MiB source, 23 MiB snapshot) while the capture-side closure Policy (build/📥️source/🔏️closure schema) already admits 20 MiB / 32 MiB / 2048 files. Capture passed, snapshot read refused at 16,777,216; the snapshot cap (24,117,248) would have refused next.

Fix: align the snapshot policy to the closure policy.
- 📦️snapshot/🔣️policy.json: maximumSourceBytes 16777216 -> 20971520, maximumSnapshotBytes 24117248 -> 33554432 (maximumFileBytes, maximumOutputBytes unchanged).
- 🧪️tests/🧫️fixtures/🔣️.json capacity.byteBudgets: same two values.
- 📦️snapshot/🧪️tests/🟦️.ts pinned policy: same two values.
- 📦️snapshot/🧪️tests/📋️faults.json: fault "five files of 3.5 MB" -> six files (21,000,000 B) so it still exceeds the source maximum.
- Untouched: SourceCensusPolicy (64 MiB), RuntimeDataDiscoveryPolicy maximumDeclaredBytes (16 MiB discovery bound), closure Policy.

Headroom: source 17.27 of 20 MiB (~21%), snapshot 24.56 of 32 MiB (~30%). File count 1960 of 2048 (only ~4%): next growth will hit maximumFiles in the closure Policy and the authorizer schema (sources.maxItems). maximumOutputBytes (8 MiB) is unmeasured here; the minified bundle must be checked in the next heavy run.

Tests (light, SEMIO_TEST_ARTIFACT_DIR set): snapshot unit 5/5 pass; 📏️capacity 10/10 pass; 🧊️memo 3/3 pass. Not passing and unrelated to byte caps: 🧪️tests/🟦️.ts (4 fail: need a parent Script process invocation / concurrent edit of bun workspace ownership policy), 🧭️closure-law (1 fail: "computed sites outside the declared table"); 🔬️script-commands has no tests.
