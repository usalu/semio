# WP-T2: Test-Case Layout, Fixture Resolution, Per-Plugin Catalog Content

Slice: T2 (session 10). Captures: `.tmp-ticket/wp-t2/generated/`. Ticket inputs (kept): `.tmp-ticket/wp-t2/*.py|*.ts`.
Inherits: R5 §4 (contract-3, 3434 rows). Siblings: T1 (rules/oracles/test placement), T3 (stub serializers).

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. Case layout and fixtures | **Done** except 9 `asset://🧬️schema/…` rows (§1.4): fixture-inside-case 338→0, example-io-in-test-folder 254→0, unexpected-dir-in-case 108→0, fixture-unresolved 217→9, fixture-dir-name 45→0, fixture-missing 36→0, tolerance `exact` 17→0 | `contract-0.txt` → `contract-1.txt` → `contract-2.txt`, `breach-delta.py` |
| 2. Catalog and content | **Done:**<br>• feature-tag-missing 188→0<br>• contribution-schema + catalog-invalid 126→0<br>• unknown catalog 27→0<br>• any-owned 25→0<br>• mutation-without-fixture 88→9 (the 9 left are gif89a, whose files a peer is editing)<br>• vocabulary-without-catalog 87→48<br>**Open, with reasons (§2.6):** runtime inventories, binary drift, gltf, unmet requirements, editor-layer vocabularies | `contract-3.txt` |
| 3. Parity of touched cases | Measured; table in §3. No fixture-resolution error remains except the 9 open `asset://🧬️schema` rows. Two failure causes are repo-wide and not from this slice: the Go test host cannot resolve `github.com/usalu/semio/...` ("downloaded zip file too large"), and oracle-only TS adapters are dispatched as TS subjects. Print is blocked by a missing export at HEAD (`compilePrintTexOnce`). | `parity-*.txt` |

## 1. Case layout and fixtures

### 1.1 Case-local fixture bundles

- **Root cause.** 101 cases kept a `🧫️fixtures/` directory inside the case and named its files `local://<file>`.
  - The resolver no longer knows `local://`. It resolves only `shared://` (owner `🧫️fixtures`), `asset://` (owner `🖼️assets`) and `schema://` (catalog export).
  - Result: every such case listed its fixtures as unresolved, and at runtime `ctx.fixture` threw "not part of this plan".
- **Fix.**
  - Each bundle moved to `<owner>/🧫️fixtures/<case-dir-name>/…`. File names are unchanged.
  - Every `local://X` in the feature and in all adapters (Rust, Go, TS) became `shared://<case-dir-name>/X`.
  - Three descriptive feature paths were updated.
  - Script: `case-fixtures-move.py`. Log: `generated/case-fixtures-move.txt`.
  - Owners: print (60 cases), presentation, and repo modules: cli, graphql, model, hooks, tickets, statutes, providers, languages, dashboard, coordinator.
- **Golden analyzer tree.** The statutes features and two Go package tests expected `📜️statutes/🧫️fixtures/📁️some/📁️folder`. The tree was at `💻️client/⌨️cli/🧫️fixtures/🔎️analyzer-paths/📁️some`.
  - Moved to the statutes owner. Updated: the cli component test (16 paths), the `tsconfig.json` exclude, and the taxonomy exact-path exception `repo-analyzer-path-fixture`.
  - Deleted the empty leftover `🦑️repo/🖼️assets/🧫️fixtures` directory tree.

### 1.2 Test-tree data (unit-test owners)

- **Root cause.** 87 JSON files sat inside `🧪️tests` trees in three layouts:
  - `🧪️tests/🧫️fixtures/<x>`, `🧪️tests/🧬️schema/<x>`
  - `🧪️tests/<case>/{🧫️fixtures,🧬️schema}`
  - bare `🧪️tests/<case>/🔣️.json` (caching)
- The legacy `🧪️fixtures` directories in ui, os engine, os Shell, print command and library are included here.
- **Fix** (`relocate.py`):
  - Each file moved to its owner bundle, `<owner>/🧫️fixtures/<name>/🔣️.json`.
  - Its shape schema moved to `<owner>/🧫️fixtures/<name>/📐️schema/🔣️.json`. This follows the taxonomy's `fixture-shape` kind, and the ui and os plugin-builder bundles already use this layout.
  - Every path literal that resolved to a moved file was rewritten in the same relative or repo-relative style: `include_str!`, `import … with {type:"json"}`, `join(import.meta.dir, …)`, `CARGO_MANIFEST_DIR` concat. That covers 67 sources; see `generated/relocate-1.txt` and `relocate-1-diff.txt`.
  - Two leftovers were fixed by hand: node-graph unit and renderer async-boundary.
- Duplicates and stale copies removed:
  - ui `🧪️fixtures/🔽️retained-select-overlay-raster` was a stale older copy; every consumer already read `🧫️`.
  - The codemod-corrupted `🧰️framework/🛐d️products` and `🧰️framework/🛠️products` trees were byte-identical duplicates of the real os engine files.
  - Empty legacy directories.
- Other moves:
  - TiledMapHost `🧪️tests/🧩️component/🧫️repaint.json` → `🧫️fixtures/🔁️repaint/🔣️.json`.
  - The normalization case's Cargo oracle crate → `🧹️normalization/🔮️oracles/📦️package-boundary-classification/`, with the case's `oracleRoot` updated.
- Stale `__pycache__` directories were deleted from 7 cases. The host already sets `PYTHONDONTWRITEBYTECODE`, so ad-hoc replays created them.

### 1.3 Manifest paths and tolerance

- **gltf ♾️any manifest.** The 36 `fixture-file-missing` rows are fixed. 18 manifests pointed at a pre-rename layout (`💎️material/🧫️fixtures/material/change-alpha/…`); they now point at `💎️material/🧫️fixtures/<id>/…`. Every sha256 and byte count matched the existing files. The generator's own path guard accepts the new paths.
- **Tolerance `exact`.** It is not a profile. The semio drawing subset defines `drawing-exact`, and 17 manifests, its generator and its `🧫️fixtures/🔣️.json` now name it.

### 1.4 Other fixture URIs

- print `asset://🖼️assets/…` doubled the directory; it is now `asset://…`.
- mcp `2️⃣g2-contract.json` moved to the `🔌️mcp` owner. The mcp Rust package test and the feature already expected it there. The client Go test now reads it via the same `../../🔌️mcp/🧫️fixtures` path its sibling fixture uses.
- **coordinator `🧬️g3-event-schema.json`.** The file was deleted on 09-08 when it became the scope contract `G3EventLogContract`.
  - The case now cites `schema://repo.server.coordinator/G3EventLogContract`.
  - Its Rust, Go and TS adapters read the declared constants from `$defs.G3EventLogContract.properties.*.const`. This is schema-first, with no fixture copy.
- **Open (9 rows).** `asset://🧬️schema/🔣️.json` in tickets, contributors, goals, todos, model, print and graphql (×2), plus mcp `🔣️descriptions.json`. The clean target is `schema://<scope>/<Export>`, but `🔣️schema-catalog.json` has no scope for these modules. `schema check` reports 368 `schema-owner-ineligible` repo-wide. This needs the schema-catalog owner; I did not regenerate the shared catalog.

## 2. Catalog and content

### 2.1 wfc: all five artifacts on one shape

The script is `wfc-normalize.py`. It derives everything from production source: `KINDS`, the `*Mutation` enum variants, the text facet's `*OperationDsl` order, and the committed fixture tree.

- **Features.**
  - wfc2d, grid2d and grid3d were rewritten on the bitmap/puzzle template: `@capability`, `@oracle`, `@comparison`, `@mutations` tags, and `@id-mutate` / `@id-inverse` outlines with `| id | vector |` over `shared://🧬️mutations/<vector>/…`.
  - wfc3d had Examples tables with no header row, so its scenarios expanded to `mutate-1…`; the header rows are added.
  - bitmap mount-contract: scenario tags plus the recorded decision `wfc-bitmap-mount-contract-statement`.
- **Oracles.**
  - wfc2d, grid2d and grid3d are now schema-valid `verified-native-second-implementation` entries: `engine`, `productionReachable`, `format`, typed survey candidates, languages, `specificationSource`, `capabilitiesCovered`.
  - The dotted ids are gone: `wfc-grid2d-python-independent`, `wfc-grid3d-python-independent`.
  - The contradictory capability-less no-oracle decisions are dropped.
- **Catalogs.** Each now carries `kinds` and kebab scenario ids. Directory names are bound to the fixture tree.
- **Mutation manifests** were added for wfc2d, grid2d and grid3d, with variants taken from the enums.
- **Subset policy.** `subsetPolicy: single` with an artifact-specific rationale for wfc2d, grid2d, bitmap and wfc3d (grid3d already had one). This closes the 25 any-owned rows.
- **Binary protocols** for grid2d, grid3d, bitmap and wfc3d now describe the real op frame that `dsl::variants_binary::encode_op` writes: `format u8`, `tag` = ordinal in `*OperationDsl`, one `record <kind> tag=<n>` per kind, opaque body. The existing `protocol` name lines are kept, which the bitmap unit test asserts. The previous content was the ArtifactPack container header, copied by mistake.
- **grid2d oracle ownership.** The case `🐍️.py` imported its reference from a ticket folder (`EXTRACT-WFC-PLUGIN/🐍️grid2d-oracle.py`). The reference is now inside the case, so it is self-contained.
- A bug in the wfc2d replay's kind detection (emoji directory names) is fixed.
- **Measured replays** (`python3 -B 🐍️.py`), all against the committed quintets: wfc2d 15/15, grid2d 14/14, grid3d 14/14, bitmap 10/10, wfc3d 15/15 (0 divergences).
- **Not done.**
  - The wfc Rust `🦀️.rs` files are crate-internal or `semio_repo_test_host` shapes, not host adapters, so these cases do not run in the parity host. That is unchanged.
  - The editor transient/config vocabularies (5 rows) still have no catalogs.

### 2.2 Catalog profiles (fem, equation, xml)

- **Root cause.** 23 catalogs declared `✳️any` / `✳️load` / `✳️analysis` / … where the owner directory is `🌐️any`, `🏋️load`, `➗️equation` and so on.
- `catalog profile does not match its contribution owner` invalidated each catalog, which is the cause of 25 `unknown mutation catalog` rows.
- **Fix.** Every catalog's standard/subset directory names are now bound to the owner path (`fix-catalog-profiles.py`).

### 2.3 Contribution-schema rows

- **16 capability-less no-oracle decisions.**
  - 13 are now bound to the `@capability-*` of the features that cite them (`fix-decision-capabilities.py`).
  - 3 were cited by no feature and are dropped: shooting, layout, remodeling.
- Invalid substitutes removed: `cross-owner-contract` (cli ×3) and `direct-production-trait-exercise` (gisterrain).
- tickets: `coversMutations` is set to the schema's `false`, and the duplicated `oracleHostPackages` is removed from the oracle.
- languages: `oracleHostPackages` moved to the contribution root.
- events, coordinator: the unused `entry` field is dropped (`package` already names the module).
- png: decision id `png-tIME-…` → `png-time-…`.
- note: 3 `productionDebt` records said "test-only everywhere" with an empty `reachableFrom`. The records are removed; the purity scan finds no production import of dxf, quick-xml or lopdf.
- **cad catalog.**
  - `_comment` moved to the manifest's `_comment`.
  - The 5 object-lifecycle kinds without vectors are declared as `deferredKinds`. This is a visible medium row, not hidden.
- **block 5d.** The kind directories `move-grip-2d`, `move-grip-3d`, `resize-grip-3d`, `update-part-2d`, `update-part-3d` (schema and fixtures) are renamed to the production kinds `move-grip2d` and so on. Also updated: the catalog, the feature, the leaf tests, the TS mirror, and the taxonomy `memberNames`. The text-DSL keywords (`📖️.grammar.semio`) are unchanged: they are wire spelling.
- **Peer fix, P6.** My block 5d kind-directory rename missed the 20 `#[path]` attributes in `🧱️block/🗿️artifacts/🖐️5d/🦀️.rs`. P6 repointed them. A `git grep` afterwards finds no remaining code reference to the old names (only prose).
- gisterrain config case: both Examples tables lacked an `id` column. The scenarios expanded to `mutate-1`, while the adapter registers `mutate-set-camera`. The `id` column is added.
- **Two feature-syntax rows.**
  - ui `@mode-regression` is not a mode; it is now `@mode-conformance`.
  - norm din16798 had a duplicated Examples row.

### 2.6 Open, with the reason

- **Runtime inventories: 175 rows.** A bridge is `<owner or ancestor>/🏭️bridge/📜️script.ts`, and only semio base has one. One bridge per artifact crate would need a standalone cargo workspace and a native build of every artifact crate, about 95 crates. That is a W1-scale build job, not a data fix.
- **Binary drift, stdio + norm + others: 93 rows.** The codecs use four different tag sources: derive ordinal, `OP_KEYWORDS`, a per-leaf `REGISTRY` tag, and hand-written ones (`protocol-survey2.json`). Writing records with guessed tags would be dishonest. They need per-codec derivation or a runtime probe.
- **block and cad vectors (mutation-without-fixture), and gltf scene/buffer/mesh (76 kinds).** Each needs new cases, oracle functions in `♾️any/🔮️oracles/🦀️.rs` and generated fixtures. That means Rust builds of the stdio host.
- **Oracle-requirement-unmet (27).** These need real qualifying oracles. A no-oracle decision cannot discharge a mutation requirement.
- **Editor-layer vocabularies (~45 rows)** have no catalogs. The only precedent, gisterrain, is itself incomplete.

## 3. Contract delta and parity

**Contract** (`contract-0.txt` → `contract-3.txt`): the total fell from 3434 to 655 (header of `contract-3.txt`). That total includes peers' work.

My classes, measured by breach id:

| Class | Before | After | Δ |
|---|---|---|---|
| fixture-inside-case | 338 | 0 | −338 |
| example-io-in-test-folder | 254 | 0 | −254 |
| unexpected-dir-in-case | 108 | 0 | −108 |
| fixture-unresolved | 217 | 9 | −208 |
| fixture-dir-name | 45 | 0 | −45 |
| fixture-missing | 36 | 0 | −36 |
| unknown tolerance `exact` | 17 | 0 | −17 |
| feature-tag-missing | 188 | 0 | −188 |
| contribution-schema + catalog-invalid | 126 | 0 | −126 |
| unknown catalog | 27 | 0 | −27 |
| any-owned | 25 | 0 | −25 |
| mutation-without-fixture | 88 | 9 | −79 |
| vocabulary-without-catalog | 87 | 48 | −39 |
| wfc NSI / no-scenarios / missing-oracle | 10 | 0 | −10 |
| binary-protocol-drift (now medium) | 101 | 97 | −4 |
| no-runtime-inventory | 172 | 175 | +3 |
| unmet requirement | 27 | 27 | 0 |
| gltf catalogs unclaimed | 3 | 3 | 0 |
| no-oracle-covers-mutation (new peer gate) | 0 | 13 | +13 |

Notes on the three increases:

- **no-runtime-inventory +3.** These are the three new wfc manifests (wfc2d, grid2d, grid3d). Before, those mutations were not declared at all.
- **no-oracle-covers-mutation +13.** These are the 13 decisions I bound to their citing features' capabilities. The new gate correctly says a decision cannot discharge a mutation capability. They are the same debt as `missing-external-oracle` and need real oracles.

**Parity** (exhaustive, `--owner`). Rust uses the private target `wp-t2/target`.

| Owner | Result | Notes |
|---|---|---|
| presentation | 14/14 passed, parity 7/7 | all 30 moved `.md` fixtures resolve |
| coordinator | 11/11 passed, parity 6/6 | includes `schema://repo.server.coordinator/G3EventLogContract`; the Go host fails (repo-wide) |
| print | 28/28 executed passed, parity 9/12, 102 not exercised | viz-probe imports `compilePrintTexOnce`, which `🖨️tectonic-template-compilation/🟦️.ts` does not export at HEAD (owner: print) |
| statutes | 11 passed, 2 errored, parity 2/6 | Go host; breach-cache TS "subject" is an oracle-only adapter |
| graphql | 18 passed, 11 errored, parity 8/24 | shared fixtures resolve; `asset://🧬️schema/🔣️schema.graphql` is open (the SDL lives at `💻️client/🔌️mcp/🧬️schema/🔗️.graphql`) |
| hooks | 35 passed, 13 errored, parity 13/39 | Go host ×5; rust~typescript diffs from oracle-only TS adapters |
| tickets | 33 passed, 10 errored, parity 9/27 | same, plus the open `asset://🧬️schema` row |
| cli | 0 executed, 8 not exercised | Go host ×8, nothing else runs |
| model | 7 passed, 2 errored, parity 1/3 | open `asset://🧬️schema` row; Go host |
| providers | 25 passed, 9 errored, parity 9/27 | Go host ×4 |
| languages | 14 passed, 2 errored, parity 2/6 | Go host ×5; moved `❌️*.ts` inputs resolve |
| dashboard | 0 executed, 1 not exercised | Rust-only case not dispatched at this level |
| mcp | 20 passed, 13 errored, parity 11/33 | `shared://2️⃣g2-contract.json` resolves; open `asset://🧬️schema/🔣️descriptions.json` |

Before the move, none of these cases could resolve a `local://` fixture: the plan dropped every one of them.

## Processes (pids)

- Contract runs: 24684, 15042 (detached, exited). Other contract runs were in the foreground.
- print parity: 35647 (detached, exited).
- statutes parity: 40896; coordinator parity: 60046 (exited).
- Repo-module parity loop: `wp-t2/parity-owners.sh`, pid 66328 (detached, finished).
- Rust private target: `.tmp-ticket/wp-t2/target`.

## Files changed (T2)

See §1–§2. The logs list every moved file: `generated/case-fixtures-move.txt`, `generated/relocate-1.txt`.
