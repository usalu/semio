# 🔍 FEM plugin — test, fixture and oracle surface exploration

Scope: `✏️s/🔌️plugins/🏗️fem`, artifacts `🗿️artifacts/◻️2d` and `🗿️artifacts/🧊️3d`, standard `🏅️standards/🔖️1`, subsets `🪆️subsets/{🏋️load,🧱️material,🛡️boundary,🕸️mesh,🌐️any,📈️analysis}`. Read-only exploration, no cargo/bun/git-mutating commands run.

## 1. Test-layer inventory and mutation × oracle coverage matrix

### Layer counts (identical shape in both `◻️2d` and `🧊️3d` — full parity)

| Subset | Mutations (kinds) | Rust `#[cfg(test)]`-style fixture cases (`🧬️mutations/<m>/🧪️tests/<case>/{🦠️mutation,🎯️outcome,🔺️diff,📸️snapshot,🦀️.rs}`) | Subset-level `🧪️tests/<name>` (`.feature`+`.rs`+`.py`) | Subset-level `🧫️fixtures/<m>/{⏮️before,⏭️after}.json` |
|---|---|---|---|---|
| 🏋️load | 7 (add-load, change-load-case-self-weight, create/delete-combination, create/delete-load-case, remove-load) | 7/7 complete | 1 (`🏋️mutate-fem{2,3}d-1-load`) | 7/7 |
| 🧱️material | 3 (create/delete/replace-material) | 3/3 complete | 1 | 3/3 |
| 🛡️boundary | 3 (create/delete/replace-support) | 3/3 complete | 1 | 3/3 |
| 🕸️mesh | 11 in 2d (node/element/section ×3 + region ×3), 11 in 3d (same but **solid** instead of **region**) | 11/11 complete | 1 | 11/11 |
| 📈️analysis | 1 (update-analysis-settings) | 1/1 complete | 1 | 1/1 |
| 🌐️any | 0 typed model mutations — `💾️binary` and `📝️text` here are wire-encoding grammars (`.ksy`/`.spicy`/`.abnf` and `.ebnf`/`.g4`/`.grammar.semio`/`.proto`) for the mutation-payload carrier, not editable document verbs, so their `cases=0` is expected, not a gap | 6 (`round-trips-the-committed-document` + one `mutate-fem{2,3}d-1-any-{material,load,mesh,boundary,analysis}` per subset) | n/a (mesh geometry fixtures with `🗿️expected.obj`/`🧊️expected.stl`/`📊️expected.metrics.json` live here instead, see below) |

**Totals per artifact: 25 typed mutation kinds** (7+3+3+11+1), each with exactly **one** Rust fixture-case scenario and exactly **one** subset-level Gherkin/Rust/Python triple. Verified file-by-file (all `mut/out/diff/before/after/rs = Y`) for all 25×2 = 50 mutation-kind fixture cases — e.g. `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/➕️add-load/🧪️tests/📏️appends-a-member-udl-to-the-dead-case/` carries `🦠️mutation/🔣️.json`, `🎯️outcome/🔣️.json`, `🔺️diff/🔣️.json`, `📸️snapshot/⬅️before/🔣️.json`, `📸️snapshot/➡️after/🔣️.json`, `🦀️.rs`. No mutation is missing a fixture. **Exhaustive at the fixture layer: yes.**

The `🕸️mesh` subset additionally carries 12 pure-geometry fixtures per artifact under `🕸️mesh/🧫️fixtures/<name>/{🗿️expected.obj,🧊️expected.stl,📊️expected.metrics.json}` (2d: `rect-floor-slab`, `scale-one-hole-1e{3,6,-3}`, `region-two-holes`, `region-one-hole`, `rect-unit-square`, `rect-thin-plate`, `polygon-{triangle,l-shape}`, `degenerate-sliver-outline`, `degenerate-hairline-thickness`; 3d equivalents plus elevation variants `solid-one-hole-elevated-{high,low,negative}` and `rect-roof-slab`) — these are the generator-built expected-geometry oracles for the 3 region/solid mutations, not per-mutation before/after pairs.

Plugin root `✏️s/🔌️plugins/🏗️fem/🧪️oracle/🔣️.json` carries **no oracles of its own** — by design (`oracles: []`, `mutationCatalogs: []`) — its only content is `oracleHostPackages` pointing at `semio-s-plugin-stdio-test-oracle`; the comment explains every subset registers its own catalog/oracle because a mutation vocabulary belongs to exactly one subset. `📚️examples/🎬️demo` (both artifacts) is a hand-authored `.dsl.semio` + `.rs`/`.ts` example with its own `🧪️tests`, separate from the mutation-fixture machinery. `🔬️probes/📜️script.ts` (in `🌐️any`) hosts `mesh-compare`, the manifold-3d-based measurement harness cited by the oracle rationale. `🏭️generator/🦀️json-engine` (Rust crate: `📖️reader.rs`, `🏭️generate.rs`, `📚️lib.rs`) independently builds the expected geometry fixtures via manifold-3d (through the TS `📜️script.ts` wrapper), which is the "json-engine" referred to in the task.

### Oracle coverage matrix — **this is the real finding**

Read in full: `◻️2d/🏅️standards/🔖️1/🪆️subsets/{load,material,boundary,mesh,analysis}/🔮️oracle/🔣️.json` (each: `oracles: []`, `noOracleDecisions: []`, exactly one `mutationCatalogs` entry registering that subset's verb list — **these files register the vocabulary, not an oracle**) and `🌐️any/🔮️oracle/🔣️.json` (the real oracle registry, `mutationManifests[0].mutations[*].oracleRequirements`), for both `◻️2d` and `🧊️3d`.

Per-mutation oracle requirement, read directly from `oracleRequirements` (identical shape both artifacts, only ids differ 2d/3d, `region`↔`solid`):

| Mutation kind | `<art>-1-mutate-carrier` oracle | `<art>-1-mutate` oracle (the one that actually judges FEM semantics) |
|---|---|---|
| create/delete-node, create/delete/replace-element, create/delete/replace-material, create/delete/replace-section, create/delete/replace-support, create/delete-load-case, add-load, remove-load, change-load-case-self-weight, create/delete-combination, update-analysis-settings **(22 of 25 kinds)** | ✅ `serde-json-fem{2,3}d-carrier-reader` (serde_json, real 3rd-party JSON reader) | ❌ **`None` — undischarged.** Recorded honestly in `noOracleDecisions` as `fem{2,3}d-non-geometry-mutation-semantics`, capability `fem{2,3}d-1-mutate-uncarried`: "a qualifying third-party reference … is still owed and is recorded here as a debt, not a verdict." |
| create/replace/delete-**region** (2d) / create/replace/delete-**solid** (3d) **(3 of 25 kinds)** | — (not carrier-scoped; geometry oracle applies at `fem-1-mutate` directly) | ✅✅ **two** third-party oracles: `three-fem{2,3}d-mesh-reader` (npm `three@0.182.0`, MIT, OBJ/STL parser) **and** `manifold-fem{2,3}d-mesh-measure` (npm `manifold-3d@3.5.1`, solid-boolean kernel measuring volume/area/genus/Hausdorff) |

So: **22/25 mutation kinds per artifact (44/50 total) have zero third-party FEM-semantics oracle.** Only the JSON structural round-trip is discharged (serde_json — proves bytes deserialize to the right JSON tree, not that the tree is FEM-correct beyond what the committed fixture already asserts). Only the 3 geometry-producing kinds per artifact (region/solid create/replace/delete) get genuine independent third-party verification. This gap is **self-reported** by the plugin's own `noOracleDecisions` entry, not something I inferred — the authors explicitly flagged it as an owed debt under "Protocol V2".

The `fem{2,3}d-python-independent` entry (see §2) is explicitly classified `cross-semio-implementation` and the rationale states in caps: "a required SUPPLEMENTAL oracle that does **not** discharge `fem2d-1-mutate`'s external-oracle requirement." So the ticket's stated goal — "every mutation has an exhaustive real-world fixture test, validated against third-party FEM solver oracles" — is met on the **fixture** half (25/25 kinds, both artifacts) but **not** on the **third-party-oracle** half (3/25 kinds, both artifacts; the other 22 are an acknowledged, undischarged debt).

## 2. How the `.py` oracles work

Read in full: `◻️2d/🌐️any/🧪️tests/🧱️mutate-fem2d-1-any-material/🐍️.py` (389 lines) and diffed against the `mesh` sibling (`🐍️.py` in `🕸️mutate-fem2d-1-any-mesh`, also 389 lines) — the six `.py` files under `🌐️any/🧪️tests/*` (`material`, `load`, `mesh`, `boundary`, `analysis`, `round-trips-the-committed-document`) are one shared harness template, differing only in their `KINDS` tuple and docstring.

**Mechanism**: each file is a from-scratch, independent Python re-implementation of the `s.fem.{fem2d,fem3d}` model (all nine snapshot members) and that subset's typed mutation verbs (`apply_mutation`, `inverse_mutation`), written from the JSON schema and the committed `(before, mutation, after)` vectors — "no Rust was read to write this" (line 24). It registers three handler kinds per verb via `Adapter("python").oracle(...)`: `mutate-<kind>` (applies to the real timber-portal-frame/steel-frame fixture and checks `touches_one` — exactly one of the 9 members moved), `inverse-<kind>` (apply then apply-the-computed-inverse, check it `restores` the original), and `spec-vector-<kind>` (replays the committed before/mutation/after triple and checks `equals_committed`). Imports `from semio_repo_test import Adapter, Context, Outcome` — the framework's Python test-host package at `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py`.

**Third-party library used**: **none, for the model/mutation semantics.** The docstring is explicit and reasoned: `code_aster`, `OpenSees`, `anastruct`, and `PyNite` were surveyed and declined because "every one of the twenty-five kinds edits the model document itself … and none of them reads `.dsl.semio`" — a FE solver computes displacements *from* a model, it doesn't validate document-mutation algebra. This second-Python-implementation is a **cross-implementation** oracle only (own-authored, not third-party) — CLAUDE.md's "at least one third-party library" rule is **not met by this file**; it's explicitly logged as not meeting the protocol's external-oracle bar (see §1).

**Real third-party libraries in the plugin are all JS, not Python**, and live one layer up in `🌐️any/🔮️oracle/🔣️.json` + `🌐️any/🔬️probes/📜️script.ts` + `🌐️any/🏭️generator/📜️script.ts`: `three@0.182.0` (OBJ/STL carrier reader) and `manifold-3d@3.5.1` (solid-boolean geometry measurement/generation), both real npm packages, installed (confirmed present in root `node_modules/three` and `node_modules/manifold-3d`), and both **only** wired to the 3 region/solid mutations. `serde_json` (Rust, in-repo, `1.x`) is the third "third-party" reader, used for full-document JSON structural coverage of all 25 kinds via the carrier requirement.

**No real FEM solver of any kind runs anywhere in this plugin** — not scikit-fem, not PyNite, not anastruct, not sfepy, not gmsh/triangle/meshio, not CalculiX. Availability check (`uv run python -c "import <pkg>"` against the repo's own uv workspace venv, Python 3.14, which does carry `numpy 2.5.0` and `scipy 1.18.0` per root `pyproject.toml`'s `dev`/`test` dependency groups):

| Package | Available in `uv run` env? |
|---|---|
| `skfem` (scikit-fem) | ❌ not installed |
| `Pynite`/`pynite` | ❌ not installed |
| `anastruct` | ❌ not installed |
| `sfepy` | ❌ not installed |
| `gmsh` | ❌ not installed |
| `triangle` | ❌ not installed |
| `meshio` | ❌ not installed |
| `numpy` | ✅ 2.5.0 |
| `scipy` | ✅ 1.18.0 |

`ccx`/`calculix` binary: **not found on PATH** (`command -v ccx` exits 1). Root `pyproject.toml` (`[dependency-groups]`) lists only generic data/notebook/doc tooling (`jupyter`, `pandas`, `numpy`, `scipy`, `scikit-learn`, `matplotlib`, `seaborn`, `ruff`, `black`) plus IFC/Office-format libs (`ifcopenshell`, `python-docx`, `python-pptx`, `openpyxl`, `steputils`) for other domains — **zero** FEM-solver or meshing libraries are declared anywhere in `pyproject.toml`/`uv.lock`. So even if the plugin wanted to wire scikit-fem/PyNite/anastruct in today, none of them are currently installable-and-present without adding a new dependency.

## 3. Test discovery and the runner

`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📜️script.ts` (`bun ./📜️script.ts test`) routes to `TestScript.run` → `runCargoTestBudgeted(["semio-s-plugin-fem"], this.repoRoot)`, defined in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1596`. This runs the crate's native `cargo test -p semio-s-plugin-fem` — i.e. **only** the `#[cfg(test)]`-style Rust unit tests physically inside the crate (the `artifacts::fem2d::*`/`artifacts::fem3d::*` module tests seen in §5), **not** the language-agnostic `.feature`/`.py` fixture-case machinery, which is a separate coordinator (`semio_repo_test`, feature/scenario discovery + Python/Rust adapter dispatch) invoked through the repo's own test platform, not through this plugin's own `script.ts test`.

Discovery config lives in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` (`testTaxonomy`, required keys include `fileKinds`, `testsDirName`, `testFixturesDirName`, `testFeatureFileKindId`, `testContributionDirName` (="`🔮️oracle`"), `testMutationVocabularyDirName`, `testGeneratorDirName`, `testProbeDirName`) and in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. The memory-flagged risk ("Taxonomy Filename Drift Blinds Discovery") does **not** apply here: fem's non-standard `🌐️any` subset-folder name (vs. the framework default `✳️any`) is an explicit, matching pair of entries in taxonomy.json — `subsetDirectoryOverrides["✏️s/…/◻️2d/…/🪆️subsets"]["*"] = "🌐️any"` (line ~17492) and `testContributionDirectoryOverrides["✏️s/…/🌐️any"] = "🔮️oracle"` (line ~17915) — both present for `◻️2d` and `🧊️3d`, and on disk every subset folder matches exactly (`🌐️any`, `🏋️load`, `🧱️material`, `🛡️boundary`, `🕸️mesh`, `📈️analysis`). No drift found.

## 4. Cached test results — inconclusive, likely stale/incomplete

`.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-fem-*` contains 22 directories (11 per artifact: 5 subset-level `mutate-fem{2,3}d-1-<subset>` + 6 `🌐️any`-level scenarios, all suffixed `-oracle-python`), all with mtime **2026-09-05 06:30:31/32** (one batch, the day before this ticket). Each directory holds only `🔣️.json` (a bare `{kind:"semio-test-output", testId, cacheKey}` marker — no status/pass/fail field) and an **empty** `📦️artifacts/` dir. Cross-checked against `planExecution`/`markRunComplete` in the test-platform's `🟦️.ts` (lines ~2503–2536): a completed run is supposed to leave a `🏁️done` marker in its work/output dir and a `📤️results.jsonl`; **neither exists** in any of the fem work-dir mirrors either (`.🧬semio/🦑️repo/⚡️cache/tests/work/test-s-plugins-fem-*`, only `📋️plan.json` + the same marker `🔣️.json`). **Conclusion: these are plan-time markers only — there is no artifact proving any of the 22 python-oracle scenarios ever finished, let alone passed, on 2026-09-05.** Per CLAUDE.md ("must validate assumptions … must not say a test is passing when you didn't run it"), this must be re-run to get real pass/fail signal; the cache alone does not show it.

## 5. Last fem ticket's test outcome (26/08/05 migration ticket)

`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🧪️test-final2.txt:433`: `test result: FAILED. 318 passed; 10 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s`. All 10 failures (lines 139-237) are DSL round-trip tests, **every one failing with the identical parser error** `dsl parse failed: expected List, found Absent at 1:1` (or the `spr`/`pack` variants of the same message):

- `artifacts::fem2d::dsl::tests::fem2d_dsl_round_trips_fixture_documents`
- `artifacts::fem2d::dsl::tests::fem2d_dsl_round_trips_bundled_default_example`
- `artifacts::fem2d::spr::tests::fem2d_document_text_round_trips_through_the_store`
- `artifacts::fem2d::pack::tests::fem2d_pack_agrees_with_dsl_for_fixture_documents`
- `artifacts::fem2d::pack::tests::fem2d_pack_agrees_with_dsl_for_bundled_default_example`
- `artifacts::fem3d::dsl::tests::fem3d_dsl_round_trips_fixture_documents`
- `artifacts::fem3d::dsl::tests::fem3d_dsl_round_trips_bundled_default_example`
- `artifacts::fem3d::pack::tests::fem3d_pack_agrees_with_dsl_for_fixture_documents`
- `artifacts::fem3d::spr::tests::fem3d_document_text_round_trips_through_the_store`
- `artifacts::fem3d::pack::tests::fem3d_pack_agrees_with_dsl_for_bundled_default_example`

All 10 are DSL-grammar/text-encoding round-trip failures (`expected List, found Absent at 1:1` — the parser hit end-of-input where it expected a list, i.e. it's being handed an empty/absent document), not solver or mutation-fixture failures; the 318 passing tests include all the engine/solver tests (`simply_supported_beam_matches_analytical_udl_solution`, `truss_is_in_equilibrium_with_finite_bar_forces`, buckling/modal, etc. — all `ok`). This is a snapshot from **26/08/05**, a month before this ticket; it should not be assumed still true today — the DSL grammar may since have been fixed (or moved) in the 26/09/02 ticket referenced throughout the oracle rationale text (`SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`), but this was not verified here (no cargo run performed, per instructions) and no newer full-suite run result was found in cache (§4).

## 6. Fixture realism, and comparison to sibling plugins

**fem's own fixtures are genuinely real-world**: `◻️2d/…/🏋️load/🧪️tests/🏋️mutate-fem2d-1-load/🧫️fixtures/🏗️timber-portal-frame.snapshot.json` is an 8 m-span timber portal frame with a raised floor and a ridge at 7.6 m, mixing `Steel S235` (E=210 GPa), `C24 Timber` (E=11 GPa), two concrete grades (`C30/37` at both 33 GPa/2400 kg/m³ and 30 GPa/2500 kg/m³), and named commercial sections (`IPE 300`, `Timber Post 140×140`, `Timber Floor Beam 80×220`, `Timber Rafter 80×180`, `CHS 76 Foundation Column`) — all values in correct SI units (Pa, m, m², m⁴, kg/m³). The 3d sibling (`🧊️steel-frame.snapshot.json`) follows the same pattern. This satisfies "real-world" in spirit (plausible structural engineering data, not toy numbers) — but note none of it is actually *solved* by an oracle; it's the input document the fixture/oracle tests mutate and re-serialize.

**Comparison to `✏️s/🔌️plugins/🧱️block` and `✏️s/🔌️plugns/📸️remodel`** (both artifact-any oracle registries read in full): fem is **ahead**, not behind, on third-party-oracle discipline —
- `🧱️block/🗿️artifacts/◻️2d/…/✳️any/🔮️oracle/🔣️.json`: only one oracle, `block-2d-python-independent` (python, cross-implementation, same non-third-party category as fem's), **zero** `noOracleDecisions` entries (the gap isn't even self-documented) and **zero** of its 26 mutations get any oracle beyond the cross-implementation one.
- `📸️remodel/🗿️artifacts/📸️remodeling/…/✳️any/🔮️oracle/🔣️.json`: same shape — one `verified-native-second-implementation` python oracle, one `noOracleDecisions` entry, no third-party library anywhere.
- **fem is the only one of the three with any genuine third-party library wired in** (`three`+`manifold-3d`, real npm packages, actually installed), even though it only covers 3/25 mutation kinds. fem is also the only one that gives a full accounting in `noOracleDecisions` naming exactly which capability (`fem{2,3}d-1-mutate-uncarried`) is still owed and why, rather than silently omitting the debt as block does.

`testkit` conformance helpers (`🧰️framework/…/🎒️pack/🧨️testkit`, `…/💻️os/…/🎒️pack/🧪️testkit`, `…/🛢️db/🧪️testkit`, `…/📡️spr/🧪️testkit`) **are** referenced from fem's Rust sources (`grep -rl testkit` hit multiple `.rs` files including the plugin root and both artifact command modules), so fem is not among the apps skipping this per-app assertion layer noted in memory.

## Gaps to fix, prioritized

1. **No third-party FEM-semantics oracle for 22/25 mutation kinds per artifact (44/50 total).** This is the ticket's core goal and is currently unmet for everything except region/solid geometry edits. The plugin's own `noOracleDecisions` entry already scopes the debt precisely (`fem{2,3}d-1-mutate-uncarried`) — closing it means picking a real third-party reference for **document-algebra** mutations (see oracle-strategy proposal below), since a FE solver genuinely cannot judge them (that survey in the .py docstrings is sound and shouldn't be re-litigated).
2. **Cached test results give no real pass/fail evidence** (§4): all 22 fem cache-result dirs are plan-only markers with no `🏁️done`/`📤️results.jsonl`. Before claiming any of the python-oracle scenarios pass, run `bun run script.ts test` (rust) and the repo test coordinator's feature/python path for real, and capture fresh results.
3. **Last known full-suite state (26/08/05) had 10/328 Rust tests failing**, all DSL/pack/spr round-trip parser errors (`expected List, found Absent at 1:1`) in both `fem2d` and `fem3d` — status today is unverified; re-run to confirm fixed vs. still broken before trusting the fixture layer's DSL leg.
4. **No FEM-solver or meshing library is installed anywhere in the repo's Python or Node toolchains** (skfem/PyNite/anastruct/sfepy/gmsh/triangle/meshio all absent; no `ccx` on PATH) — any oracle strategy that assumes one is available needs to add it first (see below).
5. Minor: the `three-fem{2,3}d-mesh-reader`/`manifold-fem{2,3}d-mesh-measure` oracle rationale references a stale path (`../../../../../🧪️tests/mutate-fem2d-1/🐍️component.py`) that no longer exists post the 26/09/02 relocation ticket — harmless (rationale prose, not executable), but worth fixing next time that file is touched, since it will mislead a future reader trying to find the described component.

## Proposed oracle strategy per analysis type (given what's actually installed / feasible)

- **Model-document algebra (22/25 mutation kinds — nodes, elements, materials, sections, supports, load cases/loads/combinations, analysis settings)**: no FE solver can judge these (the plugin's own survey is correct: they never read a solved result). The realistic third-party discharge is a **format/serialization-standard reference**, in the same spirit as the accepted `serde_json`/`quick-xml`/`burntsushi-csv` precedent already cited in the oracle rationale — e.g. validate the exported JSON/CSV/text carriers against a third-party **JSON Schema validator** (already schema-first per CLAUDE.md) run through a third-party implementation (e.g. Python `jsonschema`, itself third-party and easy to add) as a `standards-reference-tool` oracle, which is a legitimate qualifying kind distinct from `cross-semio-implementation`. This would close the debt honestly without pretending a solver applies where it doesn't.
- **Linear static 2D frame/plate** and **3D solid**: none of scikit-fem/PyNite/anastruct/sfepy/CalculiX are installed today. Of these, **`anastruct`** (pure-Python, MIT, 2D frame analysis with distributed/point loads and self-weight) is the lightest add and maps directly onto fem2d's actual feature set (bars/beams, nodal+UDL+area loads, self-weight, supports) — recommend adding it as a `dev`/`test` dependency group entry in the root `pyproject.toml` and wiring a genuine solved-displacement/reaction comparison against fem2d's own solver output for the 2d `📈️analysis` subset (currently the *analysis settings* mutation has no solver check at all, only a document-field oracle gap like every other non-geometry kind).
- **3D solid/frame**: `PyNite` (pure-Python 3D frame/truss FE) is the natural 3D analogue of anastruct and would cover fem3d's beam/truss + support/load path the same way; a full 3D continuum solid solver (scikit-fem or sfepy) is heavier and only worth adding if fem3d's engine grows real solid (not just region/solid-as-slab) elements.
- **Meshing** (region/solid create/replace/delete — the 3 kinds already discharged): current `three`+`manifold-3d` pairing is genuinely strong (independent parser + independent solid-boolean measurement engine, both real third-party, both installed) — no change needed there; it's a model for what the other 22 kinds should eventually have.
- **Modal / buckling**: no oracle exists for these at all today (engine-level Rust tests only, `fem2d_modal_returns_requested_mode_count` etc., all `ok` per §5, but not cross-checked against a third party). If a solver library is added for static analysis (anastruct/PyNite), check whether it also exposes eigen-buckling/modal solving — anastruct does not; a dedicated `scipy.linalg.eig`-based independent modal/buckling check (scipy is already installed) against the plugin's own stiffness/mass matrices would be a pragmatic, already-available interim oracle, though it would be a second in-repo implementation rather than a true third-party FEM solver oracle.

## Files referenced

- `✏️s/🔌️plugins/🏗️fem/🧪️oracle/🔣️.json` — plugin-root (no oracle; host-package pointer only)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/{🏋️load,🧱️material,🛡️boundary,🕸️mesh,📈️analysis}/🔮️oracle/🔣️.json` — vocabulary-only catalogs, `oracles: []`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracle/🔣️.json` — the real oracle registry (`oracles`, `noOracleDecisions`, `mutationManifests[0].mutations[*].oracleRequirements`)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/{🧱️mutate-fem2d-1-any-material,🕸️mutate-fem2d-1-any-mesh}/🐍️.py` — read in full
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📜️script.ts` — `runCargoTestBudgeted` entry point
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1596` (`runCargoTestBudgeted`), `:2503-2536` (`planExecution`/`markRunComplete`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:17491-17510,17915-17926` — fem subset/contribution directory overrides
- `.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-fem-*` — 22 stale plan-only cache markers, mtime 2026-09-05
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION/🧪️test-final2.txt:139-433` — 318 passed / 10 failed, DSL parse errors
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧪️tests/🏋️mutate-fem2d-1-load/🧫️fixtures/🏗️timber-portal-frame.snapshot.json` — realism reference
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`, `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json` — sibling comparison
- `./pyproject.toml` — root Python dependency groups (no FEM-solver libs)
