# Result tax5: leaf case tables as the mutation fixture coverage source

`C` = this ticket folder. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Nothing was committed, staged, stashed or reverted. HARD FREEZE respected: no edit under `L`, to any `package.json`/`bun.lock`, to a `🔣️taxonomy.json` or to the root `📜️script.ts`. Cargo was not run. All library work is staged in `C/tax5-tools/staged/`; the 66 tables and the runner wiring are live (they are outside the frozen paths).

## 0. Verdict

Coordinator ruling implemented: a schema-defined, language-neutral LEAF CASE TABLE is the third `fixtureCoverage` source, read by the TypeScript validator and by the TypeScript, Rust and Python runners. No per-case directories.

Measured with the tax4 per-scope inventory (`sim-inventory.ts`) from a scratch copy of `L` with the tax4 staged patch (baseline) versus tax4 + tax5 staged patches, against the live tree, on the 9 scopes that carry the 165 findings:

| code | before (pre-change tree) | after |
| --- | ---: | ---: |
| `mutation-fixture-unpaired` | **165** | **0** |
| `projection-catalog-unrealized` | 143 | 21 (remaining: stdio 16, draw 4, mathematical 1; none of them table-covered) |
| `mutation-fixture-invalid` | 8 | 11 (**+3 draw, see 0.1**) |
| all other codes | 595 | 595 (identical, finding by finding) |
| all findings, 9 scopes | 911 | 627 |

Per scope the unpaired count goes architect 70, raster 39, norm 23, mathematical 13, stdio 9, draw 6, forms 2, cad 2, block 1 to 0 each. Four extra scopes (`🔋️energy`, `🏙️bim`, `🗒️note`, `🧩️puzzle`) are byte-identical before and after (4, 32, 5, 18 findings). No finding is created by the tables themselves in the patched engine.

0.1 **Three swaps, not regressions of the rule.** Once a bundle is covered the normalization validates it against `mutation-fixture-bundle-v1`; three draw bundles (`🎨️style`: `🌀️set-layer-fill-rule/🌀️evenodd`, `📝️update-text/📝️edit-caption`, `🧩️set-group-isolation/🧩️pass`) carry a fifth facet `↩️inverse/🔣️.json` (14 nodes, contract has 12), so they move from `projection-catalog-unrealized` to `mutation-fixture-invalid: bundle has 14 nodes, expected 12`. Only `update-text` reads that file (Rust `fixture_text!`); the other two are unreferenced. Fix is a contract decision (optional inverse facet in the bundle contract, which needs optional nodes in the descendant-contract machinery) or relocating the three files; I did not move owner data.

0.2 **Transient on the live tree until the staged patch is applied.** The 66 new files are `🔣️.cases.json`; the unpatched library resolves them as `semantic-stem-unresolved` (66 findings, measured live on `📋️forms`). They disappear with `apply.sh` (the patched run shows 83 = 83 `semantic-stem-unresolved`, the same as before the tables). If that is unwanted before the freeze lifts, delete the 66 files (`gen-tables.py write` recreates them) and revert the 9 raster runner files (`wire-raster.py` is not reversible by itself; `git diff` shows the exact hunks).

## 1. Design

- **Schema** (`🔣️taxonomy.json`, staged): `mutationCatalogProjection.fixtureCoverage.leafCaseTable = { fileKindId: "json-case-table", casesField: "cases" }`; new file kind `json-case-table` (emoji `🔣️`, extension chain `.cases.json`, role `test`) with resolution rule `physical-json-case-table-json`. Loader (`parseMutationFixtureCoverage`), type (`MutationFixtureCoverageContract`) and audit (`auditReferenceConsumersAndMutationCatalog`) know the new member; `coverage` text unchanged.
- **Table**: `<subset>/🧬️schema/🧬️mutations/<owner>/🧪️tests/🔣️.cases.json`, exactly `{ "cases": ["<fixture directory>", …] }` (`<owner>` may be domain-nested, e.g. `📋️register-row/🗑️delete`). It lists every fixture bundle of its leaf, a bundle being a child directory of `🧪️tests/🧫️fixtures/🧬️mutations/<owner>/` that holds all four facets (a non-bundle child such as din18599 `🧾️wire-witness` is not listed). Names compare with U+FE0F removed and NFC applied; the file stores the names as on disk.
- **Engine** (staged module `L/🔍️discovery/🗂️mutation-leaf-case-table/🟦️.ts`): `parseMutationLeafCaseTable` (object with exactly the cases member, non-empty array of plain unique directory names), `mutationLeafCaseTableProblems`, `foldMutationCaseName`.
  - `fixtureCoverageOf` (used by `validateMutationCasePairs` and `normalizeMutationCasePairs`): a fixture bundle is covered when its leaf table lists it (third source beside the exhaustive feature and the fixture runner). Normalization therefore realizes registered catalog vectors from table-covered bundles.
  - New law in `validateMutationCasePairs`: code `mutation-case-table-invalid` for an unreadable/invalid table, a listed case without a bundle, a bundle the table does not list, a table without a fixture leaf. Complete listing is deliberate: the table is the single place a runner enumerates the leaf from.
- **Language-agnostic test + third-party cross-check** (staged `L/🧪️tests/🗂️mutation-leaf-case-table/🟦️.ts` + fixture `L/🧪️tests/🧫️fixtures/🗂️mutation-leaf-case-table/🔣️.json`, which embeds the JSON Schema of the table): 15 parse cases, 4 disagreement cases, Ajv (strict) re-derives the schema verdict of every parse case; 7 engine cases run the real `validateMutationCasePairs` on a scratch tree (uncovered, listed, FE0F spelling, unlisted bundle, phantom case, unreadable table, non-bundle directory); one test per table beside committed fixture bundles checks parse, Ajv, completeness against the fixture directory, all four facets and valid JSON in every bundle file (the generic runner that every table-listed bundle goes through). 102 tests today: 59 of the 66 tables are found through the git index; 7 sit beside fixture directories that tax4 renamed and nobody committed yet (mathematical graph 3, jpg 3, raster `adjustment-parameter`), the test picks them up once committed. The inventory law covers all 66 regardless.

## 2. The 66 tables (live)

`C/tax5-tools/gen-tables.py` is deterministic: `discover <inventory dir>` reads the `mutation-fixture-unpaired` findings into `C/tax5-tools/leaves.json` (66 leaves); `write` / `check` render each table from the fixture directories (NFC sort, 2-space JSON, trailing newline). 66 tables, 381 cases (the 165 unpaired plus 216 already paired cases of the same leaves). `check`: drift 0 of 66. Per plugin: norm 23 leaves, mathematical 13, stdio 9, raster 7, draw 6, architect 3, forms 2, cad 2, block 1.

## 3. What executes the 165 cases (`C/tax5-tools/consumers.py`, heuristic for "literal")

| class | cases | where |
| --- | ---: | --- |
| iterates the table | 37 | raster `apply-filter` 8, `transform-image` 8, `fill-region` 8, `fill-selection` 7, `paint-stroke` 6: TS payload tests (4) read `./🔣️.cases.json` and assert it equals `readdirSync` of the fixture leaf; the 5 Rust leaf tests gain `the_case_table_names_every_scenario_of_the_leaf` (`include_str!` the table, compare with `cases()`, retire every snapshot) |
| exhaustive feature rows (wired here) | 20 | norm `en1997` 3, `en1992` 3, `vdi3805` 2, `iso16757` 8, `en1994` 3, `en1999` 1: `<kind>-middle` rows in the mutate outline of `🥒️.feature`, `VECTORS` of `🐍️.py`, `ROWS` + `fixture_text` of `🦀️.rs`, generated from the tables by `wire-norm.py` (the existing refusal-row mechanism of the shared norm engine) |
| named by a hand-written runner | 30 | mathematical 13 (`vector(kind)` / `🔬️unit` with literal fixture paths), stdio jpg/tiff 9, draw 5, cad 2, raster `🔒️protects` 1 (heuristic: a runner file names the leaf and the case) |
| only the generic table runner (structural) | 78 | architect 70, norm 3 (din18599 `update-climate/✅apply`, `replace-zones/🧭️middle`, `replace-elements/🧭️middle`), forms 2, draw `remove-image-asset`, raster `adjustment-parameter/🎚️preserves`, block `circle-to-rectangle` |

The coverage claim for the last row rests on the table plus the structural runner, not on a semantic consumer; see 4 and 6.

## 4. Independent-implementation evidence (third-party cross-check, ticket tools in `C/tax5-tools/oracle/`)

The host module `semio_repo_test` is stubbed; the repository's own Python references run offline on the committed bytes.

- **Norm** (`norm-check.py`): the shared Python norm engine replays all 47 bundles of the 7 norm tables through the mutate and inverse handlers: 46 pass, including all 22 `🧭️middle` bundles (position-faithful inverse). The one failure is `din18599 🌦️update-climate/✅apply`: the after-snapshot changes `climateTable.childId`, a content address no second implementation can compute. `norm-feature-check.py` replays every `Examples` row of all 15 norm features as the host registers them (mutate and inverse): 1,240 handler runs, 0 failing, including the 20 new rows. All 15 features also parse with the repository's `parseFeature` (`parse-features.ts`).
- **Architect** (`architect-check.py`): the architect Python reference replays 262 bundles through its real mutate and inverse handlers: 192 pass; the 70 unpaired fail. 62 `…-deletes-middle`: the reference's `create` verb has no `index`, so the inverse of a delete restores the row at the end (the payload schema has `index`; the docstring claiming otherwise is stale). 8 composed-child registers (`knowledgeRecord`, `benchmarkRecord`) whose committed outcome says `applied`: the reference answers `mutation.target-missing`. Adding the 70 rows to the architect feature would therefore turn the differential red, so they are left unwired on purpose (open item 1).

## 5. Staged patch (apply after the freeze lifts)

`C/tax5-tools/staged/apply.sh [--check] [--root <dir>] [--only <id>]` (anchored, assert-guarded, idempotent; same pattern as tax4). Order-independent with `C/tax4-tools/staged/apply.sh` (13 patched files byte-equal in both orders). `--check` against the live library: all anchors resolve.

- `leaf-case-table-schema`: `🔣️taxonomy.json` (file kind, resolution rule, `leafCaseTable`), `🔍️discovery/📜️taxonomy-contracts`, `🧹️normalization/🔣️taxonomy/🪞️projections-and-consumers`, `🔍️discovery/🗞️audit-reference-consumers-and-catalog`.
- `leaf-case-table-engine`: new `🔍️discovery/🗂️mutation-leaf-case-table/🟦️.ts`, `🧹️normalization/👥️mutation-projection/🟦️.ts` (import, `fixtureCoverageOf`, `validateMutationCasePairs`).
- `leaf-case-table-tests`: new test + fixture above; `🧪️tests/🔏️path-emoji-statutes/🟦️.ts` (harness support list, `fixtureCoverage` expectation).
- After applying: `bun test ./L/🧪️tests/🗂️mutation-leaf-case-table/🟦️.ts`, `🥒️mutation-fixture-coverage`, `🔏️path-emoji-statutes`, `🛂️mutation-owner-descriptor-roots`, `🔬️workspace-contract/📖️taxonomy-loading`, `bun C/tax-tools/validate.ts`.

## 6. Verification (exact commands, outcome)

| command | outcome |
| --- | --- |
| `C/tax5-tools/rebuild-sim.sh` (scratch `L` + tax4 + tax5 patches), `TAX5_SIM=tax5-sim C/tax5-tools/run.sh scopes-all.txt <dir> 3` for patched and `NO_TAX5=1 rebuild-sim.sh tax5-base-sim` for baseline | all 13 scopes rc 0; numbers in section 0 (`summarize.py <base> <after>`) |
| `bun test ./<scratch L>/🧪️tests/🗂️mutation-leaf-case-table/🟦️.ts` | 102 pass |
| `bun test ./<scratch L>/🧪️tests/🔏️path-emoji-statutes/🟦️.ts` | 31 pass, 6 fail; the same 6 fail in the baseline scratch (scratch workspace root, not the patch); the two tests touched by the patch (`normalization reads explicit catalogs…`, `mutation catalogs declare one canonical…`) pass |
| `bun test ./<scratch L>/🧪️tests/🥒️mutation-fixture-coverage/…`, `🛂️mutation-owner-descriptor-roots/…` | 14 pass, 15 pass |
| `bun test ./<raster leaf>/🧪️tests/🟦️.ts` for apply-filter, transform-image, fill-region, fill-selection (after wiring, live) | 2 pass each |
| `python3 C/tax5-tools/gen-tables.py check` | drift 0 of 66 tables |
| `python3 C/tax5-tools/oracle/{architect-check,norm-check,norm-feature-check}.py`, `bun C/tax5-tools/parse-features.ts` | section 4 |
| `python3 C/tax5-tools/rs-include-check.py` on the 25 norm and raster Rust adapters/tests | 1,355 `include_str!` literals, 0 missing |
| `rustfmt --edition 2021 --emit stdout` (syntax only) on the 11 edited Rust files | parses, 0 bad |
| `wire-raster.py --check`, `wire-norm.py --check` after applying | already wired (idempotent) |

Not run: cargo (every Rust edit is verified statically only: syntax by rustfmt, include paths by script, names by reading; the raster leaf tests and norm adapters were not compiled), the exhaustive host runs, the whole-repo `verify taxonomy`, `bun repo`. The raster Rust leaf tests have a stale `fixture_root()` (`…/🧫️fixtures/…` without `🧪️tests`, relative to a crate that no longer sits there): pre-existing, untouched, and unrelated to the new test, which only uses `include_str!` next to the file.

## 7. Files

Live: 66 × `<subset>/🧬️schema/🧬️mutations/<owner>/🧪️tests/🔣️.cases.json` (9 plugins); raster TS `🧬️mutations/{🌈️apply-filter,🔄️transform-image,🪣️fill-region,🫗️fill-selection}/🧪️tests/🟦️.ts` and Rust `…/🧪️tests/🦀️.rs` for those four and `🖌️paint-stroke`; norm `🧪️tests/<standard>mutate-<standard>-1/{🥒️.feature,🐍️.py,🦀️.rs}` for en1997, en1992, vdi3805, iso16757, en1994, en1999 (peers edited the same plugin trees concurrently; every edit is an anchored read-modify-write).
Tools (inputs, kept) in `C/tax5-tools/`: `staged/{apply.sh,apply.py,new/**}`, `gen-tables.py`, `leaves.json`, `wire-raster.py`, `wire-norm.py`, `consumers.py`, `make-case-table-fixture.py`, `oracle/{architect-check.py,norm-check.py,norm-feature-check.py}`, `parse-features.ts`, `rs-include-check.py`, `sim-inventory.ts`, `run.sh`, `rebuild-sim.sh`, `mk-sim-links.py`, `summarize.py`, `scopes*.txt`. Outputs (delete when the ticket closes): `C/🗑️generated/tax5-*` (inventories `tax5-inv-{base,after,live}`, replays, `tax5-consumers.json`; the scratch library copies were removed, `rebuild-sim.sh` recreates them in about 20 s).

## 8. Open items

1. **Architect 70**: wiring needs the Python reference to learn `index` on `create`/`connect` (62 bundles) and a position for the composed-child registers (8), then 70 rows (+2 `GUARD_VECTORS` updates) in feature, `🦀️.rs` and `🐍️.py`; the kind model of that adapter (`parts(kind)` parses the id) must be split into row id and base kind first. Owner decision; until then only the generic runner executes them.
2. The 3 draw swaps (0.1): optional `↩️inverse` facet in `mutation-fixture-bundle-v1`, or move the three files.
3. Structural-only bundles without any semantic consumer: forms `discard-response/🧪️reject`, `commit-response/🧪️retry`, draw `remove-image-asset/➖️removes`, raster `adjustment-parameter/🎚️preserves`, block `update-part2d/🧪️circle-to-rectangle`, din18599 (3, see 4).
4. Norm middle rows run `mutate-` only (the shared engine's row mechanism has no inverse); the Python inverse of the same bundles passes in section 4, the Rust inverse of a middle delete is not exercised.
5. The test framework's older law `mutationVectorRegistryBreaches` (`mutation-vector-missing`) still demands a per-case implementation for every registered scenario; it needs the third source (this table) in addition to the two from tax3.
6. tax4 open items 5.2 to 5.7 are unchanged.
