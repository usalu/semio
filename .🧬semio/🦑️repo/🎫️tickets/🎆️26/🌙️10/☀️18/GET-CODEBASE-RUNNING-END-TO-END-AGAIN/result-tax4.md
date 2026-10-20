# Result tax4: per-scope taxonomy re-inventory and root-cause repairs

`C` = this ticket folder. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Nothing was committed, staged, stashed or reverted. No edit under `L`, to any `package.json`/`bun.lock`, to a `🔣️taxonomy.json` or to the root `📜️script.ts` (HARD FREEZE respected). Cargo was not run.

## 0. Verdict

| scope group | start (tax3 patches live) | now | after the staged patch |
| --- | ---: | ---: | ---: |
| `✏️s` (69 sub-scopes) | 1,946 | **1,528** | **1,119** |
| `🧰️framework` | 971 | 971 | 971 |
| `♻️mit-bestand` / `🌎️hub` / `🏢️semio-tech` / `👴️leutwiler` / `🧫️fixtures` | 101 / 22 / 6 / 1 / 1 | same | same |
| `🎓️teaching`, `🧪️tests`, `🧬️schema` | 0 | 0 | 0 |
| all scopes run (78) | 3,048 | 2,630 | 2,221 |

The start value for `✏️s` reproduces tax3's simulated 1,944 (+2, not reconciled). "After the staged patch" was measured with the patched engine copy against the live tree (section 3). `projection-catalog-invalid` and `path-emoji-*` are now 0.

`✏️s` by validator code:

| code | start | now | after staged |
| --- | ---: | ---: | ---: |
| `mutation-payload-schema-authority-invalid` | 549 | 507 | 98 |
| `projection-member-unresolved` | 371 | 329 | 329 |
| `mutation-fixture-unpaired` | 274 | 165 | 165 |
| `projection-catalog-unrealized` | 260 | 156 | 156 |
| `projection-catalog-coverage` | 108 | 72 | 72 |
| `semantic-stem-unresolved` | 96 | 89 | 89 |
| `mutation-fixture-invalid` | 83 | 17 | 17 |
| `package-implementation-file` | 51 | 51 | 51 |
| `fixed-source-disposition-unresolved` | 40 | 40 | 40 |
| `projection-authority-invalid` | 35 | 35 | 35 |
| `mutation-implementation-invalid` | 22 | 25 | 25 |
| `file-kind-unresolved` | 15 | 14 | 14 |
| `opaque-reference-target` | 10 | 6 | 6 |
| `package-role-unresolved` | 10 | 10 | 10 |
| `taxonomy/kind-only-basename` | 6 | 5 | 5 |
| `directory-kind-unresolved` | 5 | 4 | 4 |
| `path-emoji-multiple` / `path-emoji-presentation` / `projection-catalog-invalid` | 6 / 2 / 1 | 0 | 0 |
| `package-implementation-destination-unresolved` | 2 | 2 | 2 |
| `symlink-absolute-target` | 0 | 1 | 1 |

`mutation-implementation-invalid` +3 is not a regression: three gltf cases were hidden behind the old hashed names and now reach the node-count check. `symlink-absolute-target` is a transient peer file (`gltf/📦️packages/🟦️typescript/.cfb3defcde0`).

Outside `✏️s` (1,102 findings, unchanged): `package-implementation-file` 371, `semantic-stem-unresolved` 353, `fixed-source-disposition-unresolved` 117, `taxonomy/kind-only-basename` 92, `opaque-reference-target` 39, `directory-kind-unresolved` 36, `mutation-fixture-unpaired` 29 (all `💻️os/🎚️config` fixtures), `mutation-payload-schema-authority-invalid` 26, and 12 smaller rows. 112 of them sit inside `L` (frozen).

## 1. Method

- Driver `C/tax4-tools/run.sh <list> <from> <to> <parallel>` runs `C/tax4-tools/inventory.ts` (the live `inventoryTaxonomy`, no reference planning) once per scope under `perl alarm 570`. Scope list `C/tax4-tools/all-scopes.txt` (69 `✏️s` + framework, hub, teaching, semio-tech, mit-bestand + leutwiler, tests, fixtures, schema = 78), all rc 0.
- Output: `C/🗑️generated/tax4-findings.json` (all findings with `scope`, final), `tax4-findings-initial.json` (before any fix), per-scope files in `tax4-inv-final/`. Aggregation `C/tax4-tools/aggregate.py`.
- The root scope `.` (186,827 files) did not finish in a 570 s call; its content is the union of the scopes above plus a handful of root files, so it is not separately counted.

## 2. Applied (live, outside frozen paths)

Every script is in `C/tax4-tools/` (input scripts, kept).

1. **Energy orphans (14 findings, 7 `projection-member-unresolved`).** The feature, the Python map, the fixture directory and the schema leaf disagreed on three names per leaf (`☔️change-humidistat` flat, `☔️change-humidistat-humidifying-setpoint-schedule` in the feature, `💧️humidistat/☔️change-humidifying-setpoint-schedule` in the schema). Seven fixture leaves moved to the domain-nested schema path, 19 files re-pointed (feature rows, `🐍️.py` maps, `include_str!` paths), and 29 leaf-test `directory: "…"` literals that pointed at long-gone flat names now equal the mirrored fixture path (`energy-rehome.py`, `energy-directory-literals.py`). Catalog: 7 vectors / 14 scenarios added (`realign-catalogs.py`, `check-catalogs2.ts`: no problem). `🔋️energy` 27 to 4 (the rest: 4 `.epJSON` file kinds).
2. **gltf pairing (95 unpaired bundles, 17 catalog and invalid findings).** Root cause: 88 implementation cases carried truncated-hash names (`🚫️removes-the-root-516263`) while the corpus law (`🧪️fixture-corpus` doc) requires `<entity>/<verb>/🧪️tests/<case>`; 16 fixture leaves still had pre-rename names (`🌳️node/⚖️change` vs schema `⚖️change-weights`). `gltf-plan.py` derives both renames from each case's own `assert_case("…")`; `gltf-apply.py` renamed 88 case directories and 16 fixture leaves and re-pointed 24 files (8 mirror mounts and workspace members with `case_<slug>` module names, 16 `assert_case` literals). `rs-path-check.py`: 890 `#[path]`/Cargo path literals, 0 missing before and after; 0 remaining plan items. Catalog family (♾️any plus 8 sibling subsets): leaf names set from the domain registry and 21 physical cases registered (`realign-gltf.py`, original scenario ids restored from HEAD). `check-catalogs2.ts`: no problem.
3. **Descriptor identity (42 `mutation-payload-schema-authority-invalid`, plus 85 catalog `does not render mutationId` problems).** Root cause: 41 mutation owner directories use a slug shorter than the descriptor `semanticKind` (`🏷️change-node` vs `change-node-label`, `☀️change-scene-sun` vs `change-scene-sun-enabled`). `identity-rename.py` renamed them to `<emoji><semanticKind>` in 15 artifacts: 93 directories (schema leaf, fixture mirror, io mirrors) and 176 files re-pointed (including each descriptor's `owner`). Static check: `#[path]`/Cargo literals unchanged before/after for all 15 artifacts; old names no longer occur anywhere in the tracked tree (`git grep`). Outside the artifacts: `💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json` (3 paths) and `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts`. The two generated schema catalogs in `L` are staged (section 3). Catalog problems in the 27 touched catalogs: 107 to 22 (the 22 left are the same defect for owners without a flat descriptor, section 5.6).
4. **Architect (3 `mutation-fixture-invalid`).** Three implementation cases renamed to the catalog scenario directory (`🧲️reception-waiting` to `🧲️reception`, `🫷️reception-waiting`, `✂️requirement-decision`); the one mount file re-pointed.
5. **pdf (4 catalog scenarios).** Four fixture directories `🔄️round-trips-the-concrete-inverse` renamed to the registered `🔄️round` (6 files re-pointed), keeping the registered scenarios.
6. **procedural `🚀️original-field` (6 `path-emoji-multiple`).** `X.json`/`X🧬️.json` pairs moved to `Xslug/🧫️.json` and `Xslug/🧬️.json`; TS imports and Rust `include_str!` re-pointed. `bun build` of the bridge TS modules OK.
7. **Small leaf fixes.** `🏗compliant`/`🌡fill` gained U+FE0F (2 directories, 6 files); 7 `🏭️bridge/🔣️collector.json` renamed to `🔣️.json` (kind-only leaf, imports updated, `bun build` OK); `bmp ... 🎨️paint-region/🧪️tests/🟦️.test.ts` merged into the occupied `🟦️.ts` (`bun test`: 1 pass); 4 comments no longer name the retired `compose` tree (zip feature, dwg feature and Rust, puzzle 3d `flatten`); 2 tracked `.DS_Store` removed; the stray `✏️s/🔌️plugins/🗄️stdio/🗟️artifacts/` (typo duplicate of `🗿️artifacts/📰️xml`, one superseded fixture) removed.
8. **Catalog hygiene.** 3 stale scenarios (no bundle on disk) dropped (generation2d, remodeling, semio kit); 2 duplicated scenario ids in the bim catalog suffixed `-2`. An earlier over-eager drop of 10 scenarios (source-owner-mapped gltf catalogs, 2 pdf catalogs) was detected by re-diffing against HEAD and restored; `prune-stale-scenarios.py` now resolves `mutationCatalogSourceOwners`.

## 3. Staged library patch (apply after the freeze lifts)

`C/tax4-tools/staged/apply.sh [--check] [--root <dir>] [--only <id>]` (anchored, assert-guarded, idempotent; `--check` against the live library: all anchors resolve):

- `io-owner-descriptor-roots`: `validateMutationPayloadSchemas` demanded a canonical descriptor from every direct child of every `🧬️mutations` root, including codec projections `🚪️io/<representation>/🧬️mutations/<op>` (0 of 804 ever had one; the schema-side owner holds it). New exported predicate `mutationOwnerRootHoldsDescriptors(root, taxonomy)` in `L/🧹️normalization/🗺️artifact-projection-catalogs/🟦️.ts` (schema-first: uses `testFixturesDirName` and `representationDirs`, no new key). New test `L/🧪️tests/🛂️mutation-owner-descriptor-roots/🟦️.ts` with fixture `L/🧪️tests/🧫️fixtures/🛂️mutation-owner-descriptor-roots/🔣️.json` (7 cases; `picomatch` re-derives every classification).
- `schema-catalog-identity-paths`: re-points three paths in `L/📓️schema-catalog.md` and `L/🔣️schema-catalog.json` that follow the renames in 2.3.

Measured on a patched scratch copy of `L` (since deleted) against the live tree: `mutation-payload-schema-authority-invalid` 387 to 24 in `🗄️stdio`, 6 to 0 in `✒️writer`, 20 to 0 in `🔱️trinity`, and all other plugins; total 409 fewer findings, no new finding in any of the 20 scopes that carry the code. New test in the scratch copy: 15 pass.

## 4. Verification

| command | outcome |
| --- | --- |
| `bun C/tax4-tools/inventory.ts <scope>` for 78 scopes (`run.sh`) | rc 0 each; totals in section 0 |
| `python3 C/tax4-tools/rs-path-check.py <artifact>` before/after (gltf, 14 identity artifacts, wfc, norm) | missing counts identical (0; energy 12 and Cargo glob lines are pre-existing/false positives) |
| `bun C/tax3-tools/check-catalogs2.ts` on 27 modified catalogs vs their HEAD versions | 107 to 22 problem lines, none new |
| `bun test ./<bmp paint-region>/🧪️tests/🟦️.ts` | 1 pass |
| `bun test ./<scratch L>/🧪️tests/🛂️mutation-owner-descriptor-roots/🟦️.ts` | 15 pass |
| `bun test ./L/🧪️tests/🧪️mutation-leaf-identity/🟦️.ts` | 9 pass, 1 fail (`descriptorless-helper-module`: `💻️os/…/🔁️workflow/…/🧬️mutations/⚡️apply` no longer exists, a peer change; not touched here) |
| `git grep` for all renamed old names (energy 7, gltf 88, identity 41) | no remaining reference outside `.🧬semio` |
| `C/tax4-tools/staged/apply.sh --check` (live) | both patches resolve |

Not run: cargo (every Rust move verified statically only), whole-repo `verify taxonomy`, `bun build` of the root script (untouched), the exhaustive mutation cases.

## 5. Open items (classified; none is invented)

1. **165 `mutation-fixture-unpaired` (every bundle has an owner decision, no renamed implementation is left).** architect 70: bundles `…-deletes-middle`, `…-renames`, `…-replaces` are registered in the catalog (70 `unrealized`) but not declared by `🏛️mutate-program-1/🥒️.feature`; the adapter is feature-table-driven (`KINDS` in `🦀️.rs` and `🐍️.py` mirror the table), so the fix is 70 rows plus the two lists and a run. raster 39: each leaf keeps one `🧪️tests/🦀️.rs` that lists its cases by directory name (implementation exists, not per case). mathematical 13, draw 6, forms 2, cad 2: same shape (`🔬️unit` aggregate or exhaustive-adapter `include_str!`). norm 23: `🧭️middle` bundles and `🌦️update-climate/✅apply` have no consumer anywhere (missing implementation). stdio 9 (`jpg`, `tiff` `🎯️direct`), block 1. Decision: split into per-case directories, or add a third coverage source (`leafCaseTable`) to `mutationCatalogProjection.fixtureCoverage` for tests that name their case directories.
2. **Per-case cargo crates.** 8-node `mutation-implementation-invalid` (gltf 6, puzzle 8, wfc 3, shooting 3, note, trinity, stdio) and trinity `bundle has 16 nodes, expected 12` (extra `🪆️child` snapshot) conflict with `mutation-implementation-case-v1`/`mutation-fixture-bundle-v1`; either extend the contracts (optional `📦️packages`, optional child snapshot) or move the crates.
3. **`projection-member-unresolved` 329:** pdf 144 and semio 119 leaves have only `🔬️unit` aggregate tests and no fixture bundles (fixtures must be authored with the leaf generators, which needs cargo); layout 18, svg 9, xml/txt 12, shooting 8.
4. **Aggregate shard directories in `🧬️mutations` (95 of the remaining payload findings).** `🧭️edit-rules`, `🪜️mutations-2`, bim `🔙️cascade-restore`/`🦉️wall-geometry`, … are Rust module shards that match the owner name pattern. Canonical home is `🧬️schema/🔨️modules/` (needs Rust mount re-points and a cargo check).
5. **Library work (frozen):** semantic stems of authored formats (cad `modelDefinitions` 72 distinct stems: fixed contracts need one literal basename each, so the examples need directory-per-document or a loader change; hub/stdio fixtures `policy`, `answers`, `contributions/*`), directory kinds (`protocol`, `component`, `profile`, `bins`), file kinds (`.epJSON` 4, `.icc` 2, `.digest` 3, `.scan` 2, `.tif` 2; each trades for a stem finding unless fixed contracts are added), Rust glue grammar (`mirror/🦀️.rs` files that contain real `static`/`const` code: 40 stdio plus gis, wfc, fem, puzzle; 10 unregistered macros in `🧿️semio`).
6. **22 catalog "does not render mutationId" lines** (generation2d, remodeling, drawing `🔀️transform`, block 3d): same defect as 2.3 for owners resolved by registry; either register them in `mutationDomainOwners` or rename by `identity-rename.py` after adding descriptors.
7. **Renames not done on purpose:** `semio-tech/🎡️play` root files (`⚛️play-card.tsx`, `🎨️globals.css`, `🪧️brand.ts`: product :6033, imports across Next entries), sequence `🌉️wasm/📡️protocol.rs`/`🧩️component.rs` (need two directory kinds), `🧪parse-en1990-artifact.bun.ts`, hub tracked runtime state `.semio/hub/{directory.db,db/catalog/root.bin}`, puzzle oracle `productionDebt.reachableFrom` strings (5 `opaque-reference-target`), `puzzle/…/5d-react/package.json` description (package.json is frozen).
8. `C/🗑️generated/tax4-*` are tool outputs (`tax4-findings*.json`, `tax4-inv-final/`, plans); delete when the ticket closes. Earlier scratch copies were removed.
