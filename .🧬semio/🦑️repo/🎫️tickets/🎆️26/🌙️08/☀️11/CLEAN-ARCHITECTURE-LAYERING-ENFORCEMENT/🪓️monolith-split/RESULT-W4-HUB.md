# RESULT W4 HUB: `semio-hub` and `semio-hub-space` carved into producer crates

Status: **done structurally and verified statically; not compiled.** `bun repo cargo` refuses every invocation in this session (see section 8), so there is no `cargo check` or `cargo test` evidence. Verification is the full-resolution `rustc` differential per crate root, a definition scan with a seeded sensitivity check, rustfmt, TOML and JSON parse, the source-closure census on the written tree, manifest-graph checks and the TypeScript source tests that can run outside the launcher. Tooling and the plan live in `hub-carve-tools/` (python plan + engine, bun census), run logs were kept in `🗑️generated/hubcarve-*.log` and deleted after their numbers were copied here (the baseline snapshot used for the reader checker was a scratch archive outside the repo and is deleted too).

## 1. Targets before and after

| crate | files | bytes | lines | gate |
|---|---|---|---|---|
| `semio-hub` before | 62 | 2,088,285 | 33,835 | 8.46x the line limit |
| `semio-hub-space` before | 66 | 345,590 | 4,976 | 2.06x the file limit |
| after: largest of 58 crates (plain and test closures) | 22 | 223,199 | 3,482 | all under 28 files / 230,000 bytes / 3,500 lines |

No crate is in the warn band and no census row is over. The two rows with the highest line counts are `semio-hub-directory-postgres` (3,482 lines) and `semio-hub-artifact-authority` (3,371); the largest byte count is `semio-hub-space-app` test closure (223,199).

## 2. Crate graph (downward only, no cycles including dev edges)

Hub lib family (all new, `🌎️hub/<domain>/📦️packages/🦀️rust`): `semio-hub-testing`, `-refusal`, `-directory-contract` < `-directory` < `-directory-{sqlite,postgres,neo4j-rows < neo4j}` < `-directory-dispatch` < `-directory-service`; `-artifact-authority` < `-artifact-creation` < `-artifact-creation-service`, `-artifact-checkin`, `-native-openable`; `-trusted-catalog-contract` < `-trusted-catalog-descriptor` < `-trusted-catalog` (+ `-fixtures`); `-observability`, `-auth`, `-stores`, `-lag-rebootstrap`, `-local-bootstrap`; `-inference` < `-inference-ledger`, `-inference-catalog`, `-inference-runtime-contract` < `-inference-committer` < `-inference-runtime`.

Bin family: `-runtime` < `-ledgers` < `-route-models` < `-state` < `-session-routes` < `-document-routes` < `-routes` < `semio-hub` (bin `os-hub`: `main`, `serve`, the two command mounts).

Laws crates (test-only, `#[cfg(test)]` mounts, dev-dependencies only): `-directory-laws`, `-directory-backend-laws`, `-trusted-catalog-laws`, `-trusted-catalog-contract-laws`, `-component-codec-laws`, `-inference-laws`, `-inference-contract-laws`, and for the 157 former bin tests `-route-testkit` plus `-boot-laws`, `-open-plan-laws`, `-directory-route-laws`, `-presence-route-laws`, `-session-route-laws`, `-hostile-input-laws`.

Space: `semio-hub-space-foundation` (config, presence, terminology, shared ids) < `-operations` < `-commands-authoring` and `-commands-media` < `-app` < `-laws`; `semio-hub-space` keeps `plugin()` and `SpaceApps` (the component crate depends on it unchanged).

Per-crate census (files / bytes / active lines, canonical `readRustSourceClosureV1` on the written tree):

| package | lib files/bytes/lines | test files/bytes/lines |
|---|---|---|
| semio-hub | 3 / 40,466 / 650 | 7 / 46,922 / 733 |
| semio-hub-artifact-authority | 4 / 148,691 / 2,545 | 9 / 210,287 / 3,371 |
| semio-hub-artifact-checkin | 1 / 19,443 / 379 | 2 / 40,776 / 699 |
| semio-hub-artifact-creation | 4 / 39,146 / 620 | - |
| semio-hub-artifact-creation-service | 1 / 22,807 / 328 | 1 / 22,807 / 328 |
| semio-hub-auth | 4 / 58,927 / 1,328 | 10 / 134,156 / 2,093 |
| semio-hub-boot-laws | 1 / 192 / 4 | 5 / 153,363 / 1,384 |
| semio-hub-component-codec-laws | 1 / 191 / 4 | 1 / 191 / 4 |
| semio-hub-directory | 3 / 63,493 / 886 | 6 / 80,534 / 1,192 |
| semio-hub-directory-backend-laws | 1 / 434 / 8 | - |
| semio-hub-directory-contract | 4 / 71,873 / 1,653 | 4 / 71,873 / 1,653 |
| semio-hub-directory-dispatch | 1 / 49,049 / 811 | 1 / 49,049 / 811 |
| semio-hub-directory-laws | 1 / 599 / 10 | - |
| semio-hub-directory-neo4j | 1 / 196,368 / 2,890 | 2 / 197,735 / 2,949 |
| semio-hub-directory-neo4j-rows | 1 / 37,619 / 601 | 1 / 37,619 / 601 |
| semio-hub-directory-postgres | 1 / 211,837 / 3,417 | 2 / 213,290 / 3,482 |
| semio-hub-directory-route-laws | 1 / 170 / 4 | 5 / 165,798 / 2,101 |
| semio-hub-directory-service | 2 / 67,491 / 1,051 | 2 / 67,491 / 1,103 |
| semio-hub-directory-sqlite | 1 / 206,299 / 3,106 | 1 / 206,299 / 3,162 |
| semio-hub-document-routes | 10 / 200,347 / 3,152 | 10 / 200,347 / 3,250 |
| semio-hub-hostile-input-laws | 1 / 172 / 4 | 4 / 161,801 / 1,489 |
| semio-hub-inference | 7 / 141,154 / 1,394 | 7 / 141,154 / 1,418 |
| semio-hub-inference-catalog | 2 / 20,252 / 369 | 8 / 47,831 / 520 |
| semio-hub-inference-committer | 5 / 139,877 / 2,298 | 5 / 139,877 / 2,298 |
| semio-hub-inference-contract-laws | 1 / 731 / 12 | 19 / 153,917 / 1,252 |
| semio-hub-inference-laws | 1 / 183 / 4 | 5 / 126,430 / 1,617 |
| semio-hub-inference-ledger | 1 / 83,297 / 1,389 | 5 / 129,886 / 1,783 |
| semio-hub-inference-runtime | 3 / 47,993 / 796 | 3 / 47,993 / 796 |
| semio-hub-inference-runtime-contract | 4 / 35,005 / 690 | 4 / 35,005 / 731 |
| semio-hub-lag-rebootstrap | 1 / 32,886 / 685 | 5 / 67,607 / 950 |
| semio-hub-local-bootstrap | 1 / 45,327 / 1,055 | 4 / 75,029 / 1,401 |
| semio-hub-native-openable | 1 / 12,435 / 202 | 7 / 44,673 / 473 |
| semio-hub-observability | 2 / 18,844 / 233 | 4 / 74,190 / 421 |
| semio-hub-open-plan-laws | 1 / 189 / 4 | 4 / 164,130 / 1,846 |
| semio-hub-presence-route-laws | 1 / 166 / 4 | 3 / 48,079 / 558 |
| semio-hub-refusal | 2 / 12,062 / 112 | 3 / 16,556 / 179 |
| semio-hub-route-models | 11 / 150,871 / 2,717 | 11 / 150,871 / 2,897 |
| semio-hub-route-testkit | 2 / 78,118 / 1,235 | 2 / 78,118 / 1,235 |
| semio-hub-routes | 1 / 11,585 / 126 | 1 / 11,585 / 126 |
| semio-hub-runtime | 7 / 109,161 / 2,101 | 7 / 109,161 / 2,143 |
| semio-hub-session-route-laws | 1 / 285 / 6 | 5 / 113,141 / 1,458 |
| semio-hub-session-routes | 7 / 131,403 / 2,012 | 7 / 131,403 / 2,100 |
| semio-hub-space | 1 / 6,233 / 91 | 13 / 67,468 / 593 |
| semio-hub-space-app | 11 / 157,190 / 2,092 | 19 / 223,199 / 2,910 |
| semio-hub-space-commands-authoring | 20 / 42,111 / 706 | 20 / 42,111 / 706 |
| semio-hub-space-commands-media | 22 / 33,608 / 580 | 22 / 33,608 / 580 |
| semio-hub-space-foundation | 14 / 82,648 / 1,087 | 16 / 94,030 / 1,281 |
| semio-hub-space-laws | 1 / 1,886 / 32 | 13 / 62,037 / 869 |
| semio-hub-space-operations | 1 / 23,592 / 398 | 2 / 24,584 / 423 |
| semio-hub-state | 1 / 37,223 / 606 | 1 / 37,223 / 627 |
| semio-hub-stores | 1 / 42,006 / 928 | 2 / 66,826 / 1,374 |
| semio-hub-testing | 1 / 1,936 / 27 | 1 / 1,936 / 27 |
| semio-hub-trusted-catalog | 1 / 107,969 / 1,492 | 1 / 107,969 / 1,586 |
| semio-hub-trusted-catalog-contract | 6 / 103,545 / 1,306 | 7 / 105,084 / 1,353 |
| semio-hub-trusted-catalog-contract-laws | 1 / 487 / 8 | 7 / 145,680 / 324 |
| semio-hub-trusted-catalog-descriptor | 1 / 67,155 / 1,146 | 1 / 67,155 / 1,146 |
| semio-hub-trusted-catalog-fixtures | 2 / 20,376 / 238 | 2 / 20,376 / 238 |
| semio-hub-trusted-catalog-laws | 1 / 192 / 4 | 10 / 211,820 / 2,769 |


## 3. Facade decision and consumers

* **`semio-hub` lib disappears.** The package keeps only the `os-hub` binary and the `inference_client_contract` integration target; its `[lib]` and the stale lib root `📦️packages/🦀️rust/🦀️.rs` are gone. No other Cargo manifest depended on the lib (`git grep` over every `*.toml`), so no consumer needed re-pointing at the Cargo level.
* **`semio-hub-space` stays the plugin crate** (root `plugin()` and `SpaceApps`), the former inline `engine` module tree moved into the crates above.
* Embedded assets: production sources and fixtures above 6,000 bytes that laws only read are read at run time through `semio_hub_testing::hub_source(relative)` instead of `include_str!`; nothing embedded counts toward a laws crate.
* God-object rule: no inherent impl spans crates. Three structural consequences, all component decompositions rather than extension traits: `HubState` is built through `HubStateParts` plus `From`, the artifact creation service's `materialize_genesis` became a free function in the creation-service crate, and the `catalog_progress` method left `AuthorityOperationControl` in favour of explicit `load_current_observed` / `load_fixture_observed` taking the progress cell. `Neo4jDirectory` remains one 2,859-line inherent plus trait impl inside `semio-hub-directory-neo4j` (its 44 free row decoders and guards moved to `-neo4j-rows`); a further decomposition into component structs is not attempted without a compiler.

## 4. Registrations

57 new crates: `Cargo.toml` + `📋️project.json` each, 57 hub workspace members and dependencies, 57 owner rows in `📋️build-owners.json`, 342 launch routes in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (order bands 900.9xxx / 922.9xxx / 903.9xxx). `semio-hub` and `semio-hub-space` manifests were rewritten (no `id`, no boundary table, as before).

## 5. Test law paths moved

All 465 hub tests are accounted for (465 old, 465 new). The old-path to new-crate mapping is deterministic (`lawhomes.py`); examples: bin `tests::X` now lives in one of the six route laws crates with the same `tests::X` path (`quick`, `long` and `hostile_generative` stay below `tests::`), `directory::sqlite::tests::X` is `semio-hub-directory-backend-laws` `sqlite_tests::X`, `directory::tests::X` is `semio-hub-directory-laws` `unit::X`. Every `runRepositoryExactCargoLaws` group, `exactLaw`, `cargo test --bin os-hub` and `cargo test --lib` in `🌎️hub/📦️packages/🦀️rust/📜️script.ts` was re-pointed (literal groups through `tsrepoint.py`, variable-driven ones through `hubLawGroups([package, law] pairs)`); the space script runs the seven space packages.

## 6. TypeScript and JSON readers updated

* `📜️script.ts` (hub): new `readHubBinSource`, `readHubBinTestSource`, `readTrustedCatalogSource`, `readHubInferenceRuntimeSource`, `readNeo4jDirectorySource`, `HUB_TEST_PACKAGES`, `hubLawGroups`; `hubTestArtifactRoot` follows the artifact-root crate; native provider assertion follows the provider crate manifest.
* `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts` (credential oracle now read from the auth crate), `🧱️socket-grant-command-source` test + fixture, native law plan, ordered-append-broadcast reader, `🌳️unit-census` feature allowances (33 new test-only rows), `🌎️hub/📦️packages/🦀️rust/📋️project.json` named inputs.
* Descriptor-digest fixture `rustReceivers`: the two deleted files replaced by their three new homes (test passes, 8/8).
* Repo `📜️script.ts` interactivity audit reads the whole bin crate family; discovery `source-access` split-root list gained the seven roots; the dev local-hub test reads the three trusted-catalog roots.

## 7. Verification (real output)

* **Union differential** (all crates of one family mounted as modules of one sandbox crate, full-resolution `rustc --emit=metadata`, error multiset against the monolith, all features): native lib + bin + test crates `base 8 / split 8, only-split 0, only-base 0`; test variants for each of the 33 test-bearing hub crates `only-split 0`; space native and seven test variants `only-split 0`. Pre-existing errors identical on both sides: the baseline already fails on `component_sha256` in the trusted-catalog unit file.
* **Definition scan** (`defscan.py`): 3,093 items before, 3,095 after; lost 3 and gained 5, all explained (duplicate `test_artifact_root` mount of lib and bin merged, `Arc<VerifiedTrustedCatalog>` impl replaced by a blanket impl, inherent impl turned into the free `materialize_genesis`, new `HubStateParts`, new `hub_source`). Space: 434 before and after, 0 lost, 0 gained.
* **Seeded sensitivity**: removing `pub` from `role_to_wire` makes the union oracle report `E0603 function role_to_wire is private` in the three backends.
* **Orphan / inherent-impl scan**: no findings. **Cargo graph from the written manifests**: 59 manifests, normal and normal+dev graphs acyclic; every workspace dependency and member resolves.
* **Written tree**: TOML 61/61, JSON 63/63 (launch, seed, owners, project files), rustfmt parse 239/239 changed `.rs`, region markers balanced and empty region pairs removed, longest changed path 173 bytes (< 240), folder emoji admission 181/182 touched folder names (the one exception, the existing ZWJ folder `🧑‍🏭️service-v1`, predates this work), census above.
* **TypeScript**: `bun test` on descriptor-digest 8/8; socket-grant-command-source 6/7 (the failing case needs the stdio component schema module and fails on a peer-owned absent path); foundation-source 11/16 (three failures are the peer-side script.ts syntax error, two are stale fixtures that predate this work, see section 9). A static reader checker (`tscheck.py`) compares every literal a hub check asserts on a Rust source against the old and the new source set: 0 missing.

## 8. `bun repo cargo`

At the start of the session: `bun repo cargo --store=shared --configuration=prototype -- metadata --offline --format-version 1 --no-deps --manifest-path 🌎️hub/Cargo.toml` failed with `Finite original maintenance authority required`. At the end: `bun repo cargo --store=shared --configuration=prototype -- check --offline -p semio-hub-state` failed with `Live Original resource control issuance required`. No `RUSTC`, scratch workspace or private target dir was used; only `rustc --emit=metadata -o /dev/null` oracles.

## 9. Residual risks and things I did not touch

* **No compile.** Visibility widening, orphan rules, feature unification of `testkit` / `integration-fixtures` across 57 manifests and macro hygiene (`app_commands!`, `app_labels!` are external macros the union cannot expand) are checked by name resolution only. First `cargo check --workspace --all-features` and `cargo test` per laws crate are the next gate.
* **`Cargo.lock` is not refreshed** (no working launcher): the 57 new path packages and the changed `semio-hub` / `semio-hub-space` dependency lists will be added on the first resolving build; `--locked` builds fail until then.
* `semio-hub-directory-neo4j` is the largest remaining single impl (2,949 lines and 197,735 bytes in its test closure after the rows crate split).
* Pre-existing, unchanged and unrelated: creation tests include `📇️document-index-v1/🔣️.json` and `🌱️artifact-genesis-v1/🔣️.json`, which do not exist in the tree (census reports them as missing for `semio-hub-artifact-creation`, `-directory-laws`, `-directory-backend-laws`); the foundation-source fixture `🏗️consumer-contracts.json` points at a bin root that no longer owns `ensure_document`; the foundation fixture misses `readHubDirectorySource`.
* A peer's edit put two imports above the shebang of the hub `📜️script.ts` (a Bun syntax error); I left it.
* The default artifact root for laws moved to the `semio-hub-testing` crate directory; the `os-hub:test` verb exports the same directory.
* The sandbox is fixed to one tree: all runs read a snapshot taken before the flush; the flush itself was conflict-checked (no concurrent edit detected).
