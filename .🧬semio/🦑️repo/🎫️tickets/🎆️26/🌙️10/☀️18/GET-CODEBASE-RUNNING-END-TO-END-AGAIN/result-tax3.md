# Result tax3: path policy, mutation fixture drift, projection registry, package-form grammar

`C` = this ticket folder. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Nothing was committed, staged, stashed or reverted.

## 0. Verdict

| item | before (aud-tax-s, 69 `✏️s` scopes) | after (patched engine, same 69 scopes) |
| --- | ---: | ---: |
| all findings | 22,393 | 1,944 |
| `path-too-long` | 15,742 | 0 |
| `mutation-pair-path-budget` | 1,163 | 0 |
| `mutation-fixture-unpaired` | 3,148 | 274 |
| `projection-catalog-unrealized` | 821 | 260 |
| `projection-catalog-coverage` | 223 | 108 |
| `projection-member-unresolved` | 109 | 371 (stdio 302: see 3) |
| `package-role-unresolved` (✏️s / framework) | 13 / 20 | 10 / 10 |
| `package-implementation-file` (framework, warnings) | 602 | 357 |

"After" was measured with `C/tax3-tools/sim-inventory.ts`: the inventory engine from a patched scratch copy of `L` (`C/🗑️generated/tax3-sim/`) against the live tree, because of the library freeze (see 5). The scratch copy contains every patch listed below, so the "after" column is exactly what the live tree gives once the staged patches in 5 are applied.

## 1. Path policy (applied, live)

Coordinator ruling implemented exactly: a tracked path relative to the repo root may have at most **256 UTF-16 code units** and every component at most **255 UTF-8 bytes**.

- Schema: `L/🔣️taxonomy.json` `collisionPolicy`: `maxPathBytes: 240` replaced by `maxPathUtf16Units: 256` and `maxComponentUtf8Bytes: 255`. The path budget reserves of `semanticDescendantContracts` (`pathBudgetReserve.bytes`) are now `utf16Units` (catalog 133 bytes to 117 units, impl case 11 to 7, fixture bundle 42 to 30; the Draw scenario fixture 72 to 52).
- Code: new `L/🔍️discovery/📏️path-budget/🟦️.ts` (`pathUtf16Units`, `pathLongestComponentUtf8Bytes`, `pathBudgetCodes`, `pathBudgetFitsWithReserve`, `pathBudgetStatement`). Consumers: `pathPolicyViolations` (new code `path-component-too-long` beside `path-too-long`), `taxonomyPlatformPathViolationCodes(path, policy)` (policy now a required argument, no default constant), the mutation pair budget in `normalizeMutationCasePairs` and in the owned-file projection, projected destinations (`path-projection-contracts`, result field `maxPathBytes` renamed `maxPathUtf16Units`), the nested Cargo adapter check, the audit (`🛑️audit-path-policies-and-areas`, `🛤️audit-member-and-descendant-contracts`, `🗞️audit-reference-consumers-and-catalog`), loader and both type copies.
- Test (language-agnostic): `L/🧪️tests/🧮️path-budget/🟦️.ts` + `L/🧪️tests/🧫️fixtures/🧮️path-budget/🔣️.json` (18 path cases and 4 reserve cases at the 255/256/257 and 254/255/256 boundaries; U+FE0F counted as one unit, astral emoji as two, backslash splits components, the previous 240-byte regression case). Third-party cross-check: `iconv-lite` re-measures every case (UTF-16LE and UTF-8 encodes); the fixture was generated with Python's encoders, a third implementation. 45 tests pass (`bun test ./L/🧪️tests/🧮️path-budget/🟦️.ts`).
- Fixtures/tests updated: `cad-draw-path-projection` golden (237 to 189 and 207 to 151 units), `draw-source-scenario`, `artifact-path-projection` test (15 pass; its 7 other failures are pre-existing Draw/launch items), `transaction-sentinel-policy` (17 pass), `generator-preview-and-dispositions`, `taxonomy-loading` (54 pass, 2 pre-existing failures), `path-emoji-statutes`.
- Measured: `🔋️energy` 4,270 findings to 43; `🏙️bim` 5,892 to 16; `📕️norm` 1,386 to 68.

## 2. Mutation fixture drift

### 2.1 Empty leftovers (done)
820 empty directories under `✏️s/**/🧬️schema/🧬️mutations/**` removed (747 `🧪️tests/<case>` plus 73 emptied `🧪️tests` parents and scaffolds), all older than 60 minutes (`C/🗑️generated/tax3-run/empty-dirs-removed.json`). They were untracked scaffolding left by the Oct 15-17 deletion of the per-case Rust implementation cases.

### 2.2 Is the pairing rule stale or are implementations missing? (evidence)
Both, by pattern:

- **bim (1,576)**: commit `7880cc36f78` (Oct 17, #693) deleted 1,595 per-case `🧪️tests/<case>/🦀️.rs` and replaced them by one fixture-driven runner `L...🧬️schema/🧬️mutations/🧪️tests/🗂️fixture-cases/🦀️.rs` ("a leaf is covered by adding its case folders"). Rule stale.
- **norm (~650), parts of block/mathematical/stdio (~260)**: per-case cases replaced by the subset's exhaustive mutation case `🧪️tests/<emoji>mutate-<artifact>/🥒️.feature` (+ Rust adapter, Python oracle) whose step URIs `shared://🧬️mutations/<dir>/<fixture>/…` declare each bundle (`Examples` rows). Rule stale.
- **126 "stub-only" cases** (125 stdio, 1 mathematical) were complete hand-written Rust cases. The engine rejected them because `projectionSourceAt` demanded the structural kinds `standard`/`subset` for the profile directories, but `9️⃣89a`, `7️⃣87a`, `✉️base`, `🧱️base`, `➗️equation` legitimately carry other emoji (`profileDirectoryIsCanonical` in `🧪️test/🧮️mutation-catalog` judges only the shape). Rule bug; fixed (`standardVersion`/`subsetId` captured by shape, same as `scenarioId` already was).
- Everything else is genuinely missing: see `C/tax3-missing-implementations.md` (274 bundles in 21 subsets, listed per subset; implementations are not invented).

### 2.3 Rule fix (schema-first)
`mutationCatalogProjection.fixtureCoverage` in `L/🔣️taxonomy.json` (loader, audit and both type copies updated; `coverage` is now `every-catalog-vector-has-one-fixture-bundle-with-declared-coverage`):

- `implementationCase`: the existing per-case descendant contract (unchanged).
- `exhaustiveCase`: `featureFileKindId gherkin-feature`, `stepUriPrefix shared://🧬️mutations/`, `bundleFacetDirectoryNames [📸️snapshot, 🦠️mutation, 🔺️diff, 🎯️outcome]`.
- `fixtureRunner.relativePath`: `🧬️schema/🧬️mutations/🧪️tests/🗂️fixture-cases/🦀️.rs`.

Engine: `L/🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts` (`mutationFixturesDeclaredByFeature`: expands `Scenario Outline` placeholders like `parseFeature`); `fixtureCoverageOf` in `L/🧹️normalization/👥️mutation-projection/🟦️.ts` is used by `validateMutationCasePairs` (no more `mutation-fixture-unpaired` for covered bundles) and by `normalizeMutationCasePairs` (a registered vector is realized by a covered fixture bundle). Test: `🥒️mutation-fixture-coverage` (7 cases; every case is also expanded by the repository's own scenario planner `parseFeature` and must match; no third-party Gherkin parser is installed and `package.json` is frozen, so that is the independent reference).

Caveat: the test framework has its own older law `mutationVectorRegistryBreaches` (`🧪️test/📃️case-contract`) that still demands a per-case implementation for every registered scenario (`mutation-vector-missing`). It is a different validator and was not changed; it needs the same two coverage sources if the exhaustive-case model is the intended one.

### 2.4 Orphan fixtures (198 + 16)
- **Puzzle 2d/3d (182)**: not orphans of removed implementations. The schema side was nested by domain (`🧬️mutations/🧊️object/⚓change-object-anchor`), the fixtures still sat flat. Re-homed 74 fixture directories to the nested mirror and rewrote 206 referencing files (201 Rust `include_str!`/`concat!` paths, 2 features, the `🔣️.json` fixture manifest re-sorted, 1 TS test); static check `C/tax3-tools/ref-check.py` (every fixture-path literal resolves to an existing path) stayed at 0 missing before and after (710 + 321 literals). Cargo not run. 2d additionally needs its `mutationDomainOwners` row (staged, 5).
- **energy (14)**: kept. The coarse fixtures (`☔️change-humidistat`, `🎈️change-mechanical`, `🔳️change-ideal-loads`, ...) are superseded by per-field leaves, but the feature, the catalog and the schema still disagree on names; deleting would lose hand-authored expectations. Needs the energy owner.
- **gltf (16)**: kept. Schema leaves were renamed (`⚖️change` to `⚖️change-weights`, `📝️change-extras`, `🧩️change-extensions`, `🌫️change-alpha`, `🪞️change-sides`), fixtures and the scene feature rows still use the old leaf names and the impl cases use hashed names. Mechanical rename by emoji within domain is possible but crosses feature rows owned by the gltf executor.

### 2.5 Projection registry (applied, live)
No generator exists: the `mutationCatalogs[].vectors` in `🔮️oracles/🔣️.json` are hand authored. Realigned from disk with the one-off `C/tax3-tools/realign-catalogs.py` (text-level replacement of the `vectors` array only, verified to be a byte-exact no-op on 224 catalogs): registers physical leaves that have a fixture bundle and an implementation case or declared coverage, adds missing scenarios, keeps existing ids, drops vectors whose leaf and fixtures are gone; never renames. 18 catalogs rewritten (bim +11, layout +3, procedural, shooting, trinity, remodel, lowpoly, cad, draw, raster, ...); all validate with the test framework's `mutationCatalogProblems` and introduce no new problem (`check-catalogs2.ts`). 10 catalogs skipped (gltf source-owner mapping, forms with two catalogs). Remaining drift: `projection-catalog-unrealized` 260 (stdio 116 and architect 70 are registered scenarios whose fixture exists but has no coverage; raster 38), `projection-catalog-coverage` 108 (fem 34, stdio 56), `projection-member-unresolved` 371 (stdio 302: 232 leaves have implementation cases but no fixture bundles at all, 70 have fixtures whose scenario names or ids are not registrable; missing data).

## 3. Package-form grammar (staged, see 5)

Findings: 712 package-form findings were four things.
1. **Analyzer defects (fixed in the patch)**: the Rust glue analyzer matched `use`/`mod`/`type` inside identifiers (`house`, `reuse`, `bimodal`), so mount trees such as the bim `🪞️mirror` were "unsupported Rust package form"; the Go lexer treated a backslash in a raw string (`` `/\` ``) as an escape ("unbalanced lexical structure"); Go methods and aliased single imports fell through to "unsupported Go package form". All gofmt-parseable Go is now classified (declaration, bootstrap, implementation); `package-role-unresolved` 20 to 10 in the framework (the rest are TS test harnesses, `.csproj`, `.cjs`, and five Rust files that are macro registrations).
2. **Policy**: `packageBoundaryRules[*].implementationAdmission` (new, optional, schema-first): `nested-source-unit` (Go: a source file of kind `go-source` in a directory strictly below the package root is a Go package; the toolchain addresses code by directory, and two platform packages cannot be merged into one owner file) and `nested-manifest-unit` (Rust: a directory below the package root that carries its own `cargo-manifest` is a crate). Admitted files get the new role `admitted-implementation`. The package root and module directories without their own manifest stay under the glue-only law (they are parts of the root crate; move them to the owner). Framework `package-implementation-file` 602 to 357: Go sub-packages 245 admitted; the remaining 357 are Rust module files without a manifest (142 `📦️packages/🦀️rust/📚️library/🦀️.rs`-style, 96 test harness `.ts` inside crates, 40 crate roots), 23 Go root files, TS harness files and assets.
3. Test (language-agnostic): `🧱️package-implementation-admission` with 7 admission cases and 6 source cases (every source is also parsed by `gofmt -e` / `rustfmt` as the third-party cross-check; the role is asserted). 19 pass in the scratch run (the 20th, the live-schema assertion, needs the applied taxonomy).
4. Not policy, left: tracked runtime state `🌎️hub/📦️packages/🦀️rust/.semio/hub/directory.db` and `db/catalog/root.bin`, bespoke tooling files (13 destination-unresolved).

## 4. Files

Applied (live):
- `L/🔣️taxonomy.json` (collisionPolicy, 5 reserves, `fixtureCoverage`), `L/🔍️discovery/📏️path-budget/🟦️.ts` (new), `L/🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts` (new), loader/audit/type files (`🧹️normalization/🔣️taxonomy/**`, `🔍️discovery/{📑️taxonomy-document,📜️taxonomy-contracts,🛑️audit-path-policies-and-areas,🛤️audit-member-and-descendant-contracts,🗞️audit-reference-consumers-and-catalog,🧱️path-projection-contracts,🪞️path-projection,🗃️owned-file-projection,🎁️package-projection}`), `🧹️normalization/👥️mutation-projection/🟦️.ts`, `🧹️normalization/🗺️artifact-projection-catalogs/🟦️.ts`.
- Tests/fixtures: `🧪️tests/🧮️path-budget` (+fixture), `🚨️transaction-sentinel-policy`, `🔬️workspace-contract/{🧾️generator-preview-and-dispositions,🎨️artifact-path-projection,📖️taxonomy-loading}`, `🧫️fixtures/{📐️cad-draw-path-projection,🖍️draw-source-scenario}`, `🔏️path-emoji-statutes` (one assertion updated; its normalization harness test is broken until `apply-4`, see 5).
- `✏️s`: 820 empty dirs removed; puzzle 2d/3d fixtures re-homed (74 dirs, 206 files rewritten); 18 `🔮️oracles/🔣️.json` catalogs.
- Tools (inputs, kept): `C/tax3-tools/` (`inventory.ts`, `scopes.txt`, `sim-inventory.ts`, `sim-run.sh`, `summarize.py`, `classify-unpaired.py`, `classify-unrealized.py`, `feature-coverage.py`, `plan-rehome.py`, `rehome-apply.py`, `rehome-plan.json`, `ref-check.py`, `realign-catalogs.py`, `check-catalogs*.ts`, `classify-package*.ts`, `derive-domain-owner.py`, `apply-{1,2,3,4}-*.py`, `new/` staged modules, tests and fixtures). Report: `C/tax3-missing-implementations.md`.

## 5. Staged, not applied: library freeze (19:45 HARD FREEZE: no edits under `L`, `package.json`, `bun.lock`, `🔣️taxonomy.json`)

The freeze arrived mid-change. The live tree is consistent (`bun C/tax-tools/validate.ts`: 0 problems; `bun build` of the mutation projection OK) but one live test is red because of my pre-freeze edit: `🔏️path-emoji-statutes` "normalization reads explicit catalogs and canonicalizes both mutation pair roots" (its harness mocks the engine's dependencies by name). All remaining edits are prepared as anchored, assert-guarded scripts, checked against the live files (`--check`), applied to the scratch copy and measured:

1. `python3 C/tax3-tools/apply-1-coverage.py` (coverage in `validateMutationCasePairs`/`normalizeMutationCasePairs`).
2. `python3 C/tax3-tools/apply-2-puzzle2d-registry.py` (registers the 36 puzzle 2d domain-nested leaves in `mutationDomainOwners`; the descriptors already carry the nested owners).
3. `python3 C/tax3-tools/apply-3-package-admission.py [--check]` (schema, loader, audit, types, `classifyPackageRole`, discovery scan, analyzer fixes, new module `🔍️discovery/🧱️package-implementation-admission`).
4. `python3 C/tax3-tools/apply-4-tests.py` (copies `🥒️mutation-fixture-coverage` and `🧱️package-implementation-admission` tests + fixtures; fixes the `path-emoji-statutes` harness; asserts `fixtureCoverage`).
Order 1, 3, 2, 4 (any order works). Then run, with explicit files only: the two new tests, `🧮️path-budget`, `🔏️path-emoji-statutes`, `🔬️workspace-contract/📖️taxonomy-loading`, `🔬️workspace-contract/📦️package-language-handoff`, `🌳️kind-only-basename`, and `bun C/tax-tools/validate.ts`.

## 6. Verification

- `bun C/tax-tools/validate.ts`: `{"problems":0}` after the live edits.
- `bun test ./L/🧪️tests/🧮️path-budget/🟦️.ts`: 45 pass. `🚨️transaction-sentinel-policy`: 17 pass. `taxonomy-loading`: 54 pass, 2 fail (both pre-existing). `artifact-path-projection` (`SEMIO_TEST_LEVEL=long`): 15 pass, 7 fail (Draw `launchName`, producer import authority, Cargo mount; pre-existing). `path-emoji-statutes`: 36 pass, 1 fail (see 5).
- Scratch runs (patched engine copy): coverage test 14 pass; package admission test 19 pass, 1 fail (live schema, expected before apply); per-scope inventories of all 69 `✏️s` scopes, rc 0 each (`C/🗑️generated/tax3-run/sim2/`), framework scope before 1,215 / after 960.
- Not run: cargo (puzzle fixture move verified statically only), `verify taxonomy` whole-repo, the test framework's `mutationVectorRegistryBreaches`, tests of the staged patches in the live tree.

## 7. Open items

1. Lift of the freeze, then apply 5 and re-run the verification list (exact numbers in 0 should reproduce).
2. `C/tax3-missing-implementations.md`: 274 bundles (stdio 104, architect 70, raster 39, norm 23, energy 14, mathematical 13, draw 6, cad 2, forms 2, block 1) need a per-case implementation, a feature declaration, or deletion by their owners. stdio has 232 mutation leaves (pdf/text families) with implementation cases and no fixture bundles at all.
3. The test framework's `mutationVectorRegistryBreaches` has the old per-case law (2.3 caveat).
4. energy and gltf fixture/feature/schema renames (2.4); `mutation-payload-schema-authority-invalid` 549 (stdio 391 missing descriptors) untouched.
5. Remaining package-form warnings: 357 framework Rust module files and harnesses (move to owner or declare as units), hub runtime state files.
6. `C/🗑️generated/tax3-sim/` (scratch copy of the library, 39 MB) and `C/🗑️generated/tax3-run/` are tool output: delete when done.
