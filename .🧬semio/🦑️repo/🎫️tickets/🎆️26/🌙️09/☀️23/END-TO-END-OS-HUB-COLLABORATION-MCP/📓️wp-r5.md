# WP-R5: Test-Platform Suite, Subset Schema Drift, Mesh TS Adapter, Contract Debt

Slice: R5 (session 10). Captures: `.tmp-ticket/wp-r5/generated/`. Ticket inputs (kept): `.tmp-ticket/wp-r5/*.ts|*.py`.
Inherits: R3 §3/§4.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. test-platform suite | **19 → 11 fail: 93/19 → 102/11**, measured (`test-platform-3.txt`, 113 tests incl. my new `🔬️ subject selection`). The 11 left are live-repository debt the tests correctly report; each has an owner in §1/§4 | `test-platform-0.txt` → `test-platform-2.txt` → `test-platform-3.txt` |
| 2. subset schemas vs wire | **DONE.** 18/18 base arm vectors validate (was 12/18). Every subset's own snapshot/mutation/diff fixtures validate. 449/449 semio schemas compile (was 445). The TS second implementation now **reproduces 22/22** vectors (was 4/22) and fails exactly the tampered vector | `ajv-arms-3.txt`, `ajv-subsets-2.txt`, `carrier-reproduce-1.json`, `carrier-reproduce-tamper.json` |
| 3. mesh TS adapter errors 52/52 | **DONE.** TS subject run: 52 errored → not dispatched. `parity exhaustive`: **51/52**, 103/104 passed. The one red is `identity-round-trip`, which the feature documents as deliberately red (§3) | `mesh-subject-ts-1.txt` → `mesh-subject-ts-2.txt`, `mesh-parity-1.txt` (17/52) → `mesh-parity-3.txt` |
| 4. contract findings | Classified, table below. 3703 at my first run → **3434** after my fixes. R3's 2900 is not comparable: peers' new or renamed cases added ~800 rows in between (§4) | `contract-1.txt`, `contract-3.txt`, `contract-3-classes.txt`, `contract-delta-2.txt` |

## 1. Test-platform suite: the root causes

Suite: `bun test ./🧪️tests/🧪️test-platform/🟦️.ts` (in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`).

**Fixed (8 tests now pass):**

1. **Contributions were cast without being validated.** Registry records reached the gates as `undefined`: 13 oracles had no `comparisonProfiles`, and one decision used `capability` where the schema says `capabilities`. That crashed 3 tests.
   - Root fix, schema-first: `readContribution` now validates every record of every contribution array against its protocol `$defs` entry. The new function is `contributionSchemaProblems` in `🧪️test/🟦️.ts`. Its findings surface as the existing `contribution-manifest-invalid` contract breach.
   - The repo's own validator (`📚️library/🧬️schema/✅️validation/🟦️.ts`) could not do this: it had no `$ref`. I added `$ref`, `allOf`, `oneOf`, `patternProperties`, `propertyNames`, schema-valued `additionalProperties` and type arrays.
   - **Third-party cross-check:** over all 2247 records, Ajv 8.20 and our validator disagree on 0 verdicts (`contribution-problems.ts`).
2. **The protocol schema was stale against the code.** Mutation manifests required `✳️`/`🔖️` prefixes, the same bug R3 fixed in TS. Real, meaningful fields were also missing from the schema:
   - fixture `comparisonPipeline`/`reproducibilityDiffs`/`invariants`
   - generator `exportEngine`
   - requirement `oracle`
   - mutation `carriers`/`notes`/`comparisonPipeline`
   - probe `packages`/`productionDebt`
   - host `rationale`
   - `svg-user-unit`, `-y`, `unitless` angle

   All of these are now in `🧬️schema/🔣️.json`, as a new `ProfileDirectoryName` and mirrored TS types. Invalid records (Ajv): 614 under the old schema → 298 once the schema matched the code → 39 after the data fixes.
3. **Data drift fixed at its cause:**
   - 24 fixture generators recorded `platform: process.platform` (`"darwin"`, not a platform id). They now use `currentPlatform()`, and the 202 committed manifests were corrected.
   - Mechanical data fixes (`fix-contributions.ts`, 46 files):
     - 33 `handcrafted`→`authored`
     - 44 `none` units
     - 7 `+` families
     - 25 wfc outcomes that were diagnostic severities
     - 13 oracle profiles, derived from the fixtures each oracle generates
     - 5 engine families, taken from the generating oracle's registered engine
     - 2 decision shapes
     - `_comment`→`rationale`
     - `text: true`
     - `ecosystem: typescript`
4. **The oracle-purity scan disagreed with the dependency classifier about `🧪️tests/`.** The classifier treats a `🧪️tests/` declaration as test-owned. The scan reported unit tests (`cfg(test)`, vitest) as "production source". `testsDirName` is now test-owned in `oracleImportsInProduction`: 299 → 133 hits before the other fixes.
5. **26 owners kept their contribution in `🔮️oracle/` (singular).** Discovery never saw them, so they surfaced as "unknown oracle"/"unknown profile"/"unknown decision", 310 contract rows. The oracle-coverage test also had 212 unbacked cases.
   - Moved to `🔮️oracles/`: print, presentation and 24 repo modules. READMEs and the one feature that named the path are updated.
   - Unbacked cases 212 → 4. Contract: −123 unknown oracle, −97 unknown profile, −89 unknown decision.
6. **Stale live expectations updated to the repository as it now is:**
   - The C4 obj case now sits in its subset (expect `[]`).
   - `compose/` became `🌎️hub/compose.yaml`.
   - The bachelor-thesis asset moved to `🧱️base/🖼️assets/🎓️bachelor-thesis/`.
   - The dependency functions moved out of the root `📜️script.ts`. The test now imports them from `📇️inventory`; `dependencyDiscoverContributionManifests` is now exported.
   - A projected-storage fixture case needed an emoji name.
   - External host packages are compared per ecosystem. `jsonschema` is both js and python.
7. **134 case adapters could not compile.** A 09-08 sweep rewrote `semio_s_plugin_stdio::artifacts::…` to `crate::…`. In a generated host, `crate` is the host, not the subject.
   - Fixed with `crate-paths-fix.ts`, which resolves each owner's subject crate with the platform's own `rustSubjectPackage`. It only rewrites a root that the crate's lib actually declares.
   - The two wfc adapters that are `#[path]`-included in their own crate were deliberately kept on `crate::`.
   - List: `adapters-crate-path-fixed.txt`.
8. Other fixes:
   - `⚙️generator` → `🏭️generator` in step cc6. It was drifted from the taxonomy name, so brepjs looked like production.
   - The 5 `three` oracles' `productionDebt` now has the schema shape: `reachableFrom` = the 9 measured production paths, and `plan` = the owners' own reason + consequence.
   - wfc `exact-json` → `ordered-json-v1`.

**Still failing (11), with the live reason:**

| Test | Why it still fails | Owner |
|------|--------------------|-------|
| every committed case satisfies the frozen contract | asserts all 3434 contract rows are gone (§4) | all owners |
| dependency ratchet / purity ×2 / recorded debt ×2 | `serde_json` is registered as a **third-party oracle** by 9 carrier readers, and production imports it (121 files). This covers equation, fem2d/3d, drawing, and semio cad/document/mesh/drawing/brep. **Fix plan:** re-implement the 9 generator/reader workspaces on `json` (json-rust), as R3 did for base. Their fixtures would be regenerated, because float spelling changes. Also newly visible: `node-crypto-repo-events` (66, `📡️events` registers `node:crypto`), `ajv` (22), `typescript-compiler` (5), `d3-force`, `manifold-3d` (in `♾️infinite/🌍️world/🎨️r3f`, which breaks fem's "manifold is test-only" claim), `image` (os world) | equation/fem/draw/stdio-semio; repo `📡️events`; os renderer |
| every oracle names … a profile | `energyplus-25-2-0-via-honeybee-openstudio` names the pipeline id `energy-model-1-bestest-compare-v1` as a profile. Its feature names the unregistered `@oracle-energyplus-ashrae-140` | energy |
| oracle coverage | 4 unbacked: wfc `mutate-grid2d-1`, `mount-contract`, `mutate-wfc-grid3d-1` declare neither oracle nor decision; energy bestest names an unknown oracle | wfc, energy |
| native second implementation earned | 3 wfc NSI entries: they have no `format`, the wrong survey shape, and their contributions own no mutation manifest to bind to | wfc |
| mutation without fixture | ~60 declared mutations without vectors: block 5d (41), cad (19), equation graph/geometry (9, visible now that their `subsetDirectoryName` is bound) | block, cad, equation |
| external host package is test-only | python `deepdiff` classifies `test-runner`: the classifier only excuses oracle *packages*, not host packages. `pypdf`/`simplejson` are declared host packages but absent from `pyproject.toml`/baseline. This needs the dependency classifier to excuse host packages and a `uv lock` for the two; not done (no network lock taken) | stdio, test platform |

## 2. Subset schemas vs wire (the six R3 drifts, plus what the sweep found)

- **Root cause 1: restated vocabularies.** The 18 base arm payload schemas were generated inline copies, each up to 127 KB, of every arm's union. They were stale: audio still required `timelineIndex`, and brep was the wrong tagging.
  - Each is now a 0.5 KB schema whose `mutation` `$ref`s the arm's own `mutations.json`. That fixed audio and brep.
  - In the same way, 92+55 leaf `$defs` copies of snapshot types now `$ref` the subset's snapshot schema. Presentation's `DocBlock`/`DocRun`/`RunStyle` now reference document's (`leaf-defs-to-refs*.py`).
- **Root cause 2: types.**
  - image `rgba8`/`icc` and video `data` were typed as hex strings. The JSON wire is `Vec<u8>` → an integer array, the same as mesh's `bytes`. Fixed in the snapshot and diff schemas and the TS mirrors.
  - document `Option` fields are nullable on the wire.
  - `DocBlock` struct-variant fields are `style_id`/`image_id` on the wire, because `rename_all` on a tagged enum renames tags only.
- **Dangling refs:**
  - cad/value diff (`#/$defs/layer`→`CadLayer`, `valueId`→`ValueId`)
  - brep `artifact.json` (`brepVertex`→`BrepVertex`)
  - base diff's malformed presentation `$id`
  - two animation text mirrors
- **TS second implementation** (`✉️base/🔬️probes/📜️script.ts carrier-reproduce`):
  - It now applies each arm verb the vectors exercise, written from the arm leaf schemas. There are 18 laws, e.g. `insertChannel` = positional insert, `EditCell` = cell by column name.
  - Result: 22/22 reproduced, 22/22 conforming, status ok. Tamper of one committed after → 21/22, `apply-table-applied` named.
  - Its pipeline `semio-v1-base-carrier-reproduce-v1` now gates all 22 fixture manifests, and the qualification evidence was updated to these measurements.
- **Finding for the document owner (not changed):** the wire mixes `styleId` (struct fields) and `style_id` (`DocBlock` variant fields). The clean fix is `rename_all_fields = "camelCase"` on `DocBlock`/`DocPathSegment`, plus a rewrite of ~100 fixtures, the Python oracles and the fixture digests. I documented the wire instead of rewriting 100 files.

## 3. Mesh TypeScript adapter

- **Root cause.** `ownerShipsImplementation` treats any `📦️packages/🟦️typescript` at an ancestor as "ships an implementation". `🧿️semio`'s TS package exports only the artifact definition. The mesh TS adapter IS the registered oracle `semio-mesh-typescript-three-independent`, a hand-written second implementation, and it registers no subject.
- **Fix.** New `subjectImplementations` in `⚖️parity/📋️orchestration/🟦️.ts` excludes the language whose adapter hosts the case's declared oracle. This is declared data: `hostPath === caseDir`, carried as `hostedByCase` on `oracleDecision`. A package that ships a language and forgets a subject still fails loudly.
  - The mesh oracle's `hostPath` was a pre-rename path. It is corrected, and `ecosystem` is now `javascript`.
  - New live test: `🔬️ subject selection`.
- **Two more real breaks surfaced once the TS noise was gone:**
  - The Rust adapter used `crate::standards` (§1.7).
  - Both adapters read spec vectors via `asset://`, while the feature moved them to `shared://🧬️mutations/…`.
  - Both are fixed. Parity went 17/52 → 34/52 → **51/52**.
- **Left red on purpose:** `identity-round-trip`. Rust prints `…063` where JS and Python print `…062` for the same double. The feature records that the grammar must name a canonical FLOAT spelling. That fix is a format decision for the mesh/semio owner.

## 4. Contract findings (3434 high rows at `contract-3.txt`)

**Why the total rose since R3.** Rows are up versus R3's 2900. Between R3's run and mine, peers made print (381), wfc (282), repo modules (211) and presentation cases discoverable. My schema gate made 69 existing record errors visible. My fixes removed 269 rows net between `contract-1` and `contract-3`; the dependency class is 296 → 217 vs R3 even with the newly visible repo-module oracles.

| Rows | Class | Root cause | Owner(s) | State |
|------|-------|-----------|----------|-------|
| 638 | go-test-in-package | Go `_test.go` must live in the package dir; the layout rule demands canonical test dirs. Rule ↔ language conflict | repo modules (cli 143, model 93, hooks 80, graphql 56, …) + test-platform owner (rule) | listed |
| 338 + 254 + 108 | fixture-inside-case / example-io-in-test-folder / unexpected-dir-in-case | case dirs carry `🧫️fixtures`/inputs instead of owner fixtures | print 344, presentation 62, fem, repo modules | listed |
| 218 + 30 + 24 + 13 + 11 | rust-test-outside-canonical / inline test mods / depth / filename / registration | test code outside canonical impl dirs | os, repo modules, ui | listed |
| 217 | fixture-unresolved | `local://` fixtures missing (same case-layout debt) | print 114, presentation 30, repo | listed |
| 217 | oracle-in-production | serde_json ×9 oracles (121); node-crypto (66); ajv 22; typescript 5; three recorded as debt now | see §1 table | 166 removed; rest listed with plan |
| 189 | feature-tag-missing | wfc scenario outlines without `@id/@level/@mode` | wfc | listed |
| 172 | no-runtime-inventory | subsets without a production bridge inventory | stdio 98, norm 15, note 8 | listed |
| 138 | legacy-test-filename | pre-taxonomy names | ui 64, os 23 | listed |
| 101 | binary-protocol-drift | `📡️.protocol.semio` lacks per-kind records (base fixed by R3) | stdio 56, norm 15 | listed |
| 88 | mutation-without-fixture | declared kinds without vectors | block 41, cad 19, wfc, equation | listed |
| 87 | vocabulary-without-catalog | vocabularies with no catalog | fem, wfc, equation, trinity | listed |
| 82 + ~40 | stub-serializer | serializers emit internal DSL / never read input | puzzle, gis, lowpoly, procedural, shooting, block, … | listed |
| 69 | contribution-schema (new gate) | wfc 38 (NSI shape, dotted ids), note 3 (empty `reachableFrom`), 16 empty-capability no-oracle decisions (equation graph, shooting, sequence, gis, fem, layout, remodel, draw, os config), `entry`/`oracleHostPackages` misplaced in oracles | wfc, note, the 9 decision owners, os | 575 of 614 invalid records fixed; rest listed |
| 57 | catalog-invalid | scenario ids not kebab, bad vector dir names | wfc, fem, block, cad | listed |
| 45 | fixture-dir-name | non-canonical fixture dir | ui, os | listed |
| 37 | obsolete-test-category | legacy categories | stdio 12, others | 52 removed by the dir renames |
| 36 + 17 | fixture-missing / unknown tolerance `exact` | stdio | stdio | listed |
| 27 + 27 + 25 | unknown catalog / unmet requirement / any-owned | fem, equation (now correctly bound), wfc | respective | listed |
| 10 | the R3-surfaced rows | 7 × `subsetDirectoryName` → **fixed** (bound to the owner path, e.g. `➗️equation`, `🌐️any`, `✅️valid`); 3 × gltf `scene/buffer/mesh` catalogs claimed by no case: needs a case plus 76 kinds implemented in the json oracle | gltf | 7 fixed, 3 listed |

## Processes (pids)

All my runs finished or are recorded: test-platform 53254, 79879, 2401, 15293 (detached `bun test`); contract runs (detached, exited); parity runs foreground. No servers, no cargo outside the platform's own host builds.

## Files changed (R5)

- Test platform:
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts`
  - `…/🧪️test/🧬️schema/🔣️.json`
  - `…/🧪️test/⚖️parity/📋️orchestration/🟦️.ts`
  - `…/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts`
  - `…/📚️library/🧬️schema/✅️validation/🟦️.ts`
  - `…/📚️library/🕸️dependencies/📇️inventory/🟦️.ts`
- Case adapters: 134 × `🧪️tests/<case>/🦀️.rs` (`generated/adapters-crate-path-fixed.txt`); mesh `🦀️.rs` + `🟦️.ts` spec-vector URIs.
- Generators: 24 × `🏭️generator/📜️script.ts` (`currentPlatform()`); step cc6 `⚙️generator/` → `🏭️generator/`, plus its `🔮️oracles`/`🧫️fixtures` manifests.
- Contributions:
  - 46 via `fix-contributions.ts` (`generated/fix-contributions.txt`)
  - 26 dirs `🔮️oracle/` → `🔮️oracles/`, plus their READMEs and the print `🏭️domain-families` feature
  - 7 profile-dir manifests (equation ×3, fem ×2, xml ×2)
  - 5 `three` debts (fem ×2, obj, gltf, semio mesh)
  - wfc grid2d/grid3d, fem2d decision, stdio frozen-hound decision
  - semio base `🔮️oracles/🔣️.json` (pipeline on 22, qualification)
  - semio mesh `🔮️oracles/🔣️.json` (hostPath, ecosystem)
- semio schemas (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/`):
  - 18 × `✉️base/🧬️schema/🧬️mutations/*apply-*/🧬️schema/🔣️.json`
  - ~55 leaf schemas (`generated/leaf-refs-2.txt`)
  - `📑️document`, `🖼️image`, `🎬️video`, `📽️presentation` snapshot schemas
  - `🖼️image`, `🎬️video`, `📐️cad`, `🔢️value` diff schemas
  - `🎞️animation` text ×2, `🧊️brep/🧬️schema/🔣️.json`, `✉️base/🧬️schema/🔺️diff/🔣️.json`
- TS mirrors: document snapshot, image snapshot/mutations/diff, video diff (already arrays).
- Probe: `✉️base/🔬️probes/📜️script.ts`.
