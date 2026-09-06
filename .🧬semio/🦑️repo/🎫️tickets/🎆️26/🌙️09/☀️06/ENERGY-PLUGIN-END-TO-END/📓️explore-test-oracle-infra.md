# Repository Test-Oracle & Feature-Test Infrastructure — Explore for Energy Physics Oracle

Read-only survey answering: how does this repo declare, provision, and run third-party test oracles
and language-agnostic feature tests, so that `✏️s/🔌️plugins/🔋️energy` (artifact `s.energy.model`,
subset root `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any`) can add an EnergyPlus/OpenStudio/
Honeybee physics oracle plus BESTEST comparison tests. All paths repo-relative.

---

## 1. The oracle manifest schema, energy's current manifests, and the DECLINED→reversed pattern

### 1.1 Energy's own manifests today

- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json` — the
  **subset-level** oracle registry (one artifact × standard × subset). Today it registers ONE entry,
  `id: "energy-model-1-python-independent"`, `kind: "verified-native-second-implementation"`,
  `ecosystem: "python"`, `package: ""` (empty — it is a from-scratch second implementation, not a
  distributed package), `capabilities: ["energy-model-1-mutate"]`,
  `comparisonProfiles: ["ordered-json-v1"]`, `license: "AGPL-3.0-only"`, `testOnly: true`,
  `productionReachable: false`, `networkDuringExecution: false`, plus an `engine` block
  `{"family":"none","implementation":"in-repository second implementation","version":"0"}` and a
  `nativeSecondImplementation` block (`format`, `noThirdPartySurvey.candidatesConsidered`,
  `subjectImplementationLanguage`, `secondImplementationLanguage`, `specificationSource`,
  `fixtureCoverage`). `noOracleDecisions` is `[]` — empty, not absent.
- `✏️s/🔌️plugins/🔋️energy/🧪️oracle/🔣️.json` — the **plugin-level** contribution. It is deliberately
  empty (`"oracles": [], "noOracleDecisions": [], "mutationCatalogs": []`) except for one
  `oracleHostPackages` entry pointing at `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust`
  (`package: "semio-s-plugin-stdio-test-oracle"`) — this is the Rust crate the energy subset's
  `identity-round-trip` scenario reaches through the shared stdio oracle-law module. Its own
  `_comment` states the rule: a subset manifest cannot express which crate a plugin's test adapters
  link, because `oracleHostPackagesFor` resolves host packages from the **nearest contributor at or
  above the case owner**, and a case owner is always an artifact-root-and-below subset, never a
  plugin.

### 1.2 The schema that validates both

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json` — `$id`
`https://semio-tech.com/schema/repo/test/v2`, `x-semio.schemaVersion: 2`. Key `$defs`:

- `QualifyingOracleKind` (line 72) — `enum: ["third-party-library", "third-party-cli",
  "standards-reference-tool"]` — the ONLY kinds that can discharge a mutation's external-oracle
  requirement.
- `SupplementalOracleKind` (line 76) — `enum: ["metamorphic", "inverse", "round-trip", "property",
  "cross-semio-implementation"]` — required supplements, never a substitute. `cross-semio-
  implementation` explicitly means "a second implementation written inside this repository — useful,
  and explicitly NOT independent evidence."
- `OracleRegistryEntry` (line 111) — `required: ["id","kind","ecosystem","package","capabilities",
  "comparisonProfiles","license","testOnly","engine","productionReachable",
  "networkDuringExecution"]`. Notable optional fields: `version`, `lockDigest` (`Sha256` pattern
  `^sha256:[0-9a-f]{64}$`), `packages` (array of `OracleLinkedPackage` for further pinned
  dependencies), `source.{repository,commit,license}`, `platforms`, `homepage`, `rationale`,
  `hostPath`, `productionDebt`. `testOnly` is a hard `const: true`.
- `EngineFamily` (line ~55) — `{family, implementation, version}` — independence is accounted at the
  FAMILY level ("two wrappers around one kernel are one oracle, not two").
- `OracleRequirement` (line 224) — `{capability, qualifyingKind, distinctEngineFamilies?}` — what a
  mutation needs before release.
- `ManifestMutation` (line 239) / `MutationManifest` (line 281) — the domain owner's mutation
  inventory; `oracleRequirements` is `minItems: 1` and required on every `ManifestMutation`.
- `FixtureFile` (line 329) — `{role, path, mediaType, sha256, bytes?}` — no size ceiling anywhere in
  this file.
- `FixtureManifest` (line 382) — `required: ["schema","id","class","target","units","files",
  "provenance","comparisonProfile","reproducible"]`; `class` is `FixtureClass = enum["real-world",
  "handcrafted","third-party-generated"]`; `generator` (`FixtureGenerator`, required when a fixture
  was produced by a third-party oracle) needs `{oracle, packageVersion, engineFamily, engineVersion,
  command, platform}` — "a generated fixture with no reproducible generator record is a fixture
  nobody can audit."
- `NoOracleDecision` (line 685) — `required: ["id","capabilities","rationale","substitutes"]`,
  `rationale` has `minLength: 20`, and `coversMutations` is a structural `const: false` — "a runtime
  mutation can never be discharged this way" (schema comment at the top of the file, line 10-ish).
- `OracleHostPackage` (line 705) — `{implementation, package, path?, version?, module?, features?}`.

### 1.3 The DECLINED → reversed pattern (energy is the live worked example)

Energy's own history, kept verbatim inside its current `rationale` string (see 1.1), is the exact
answer: a **DECLINED** survey entry is never deleted — it is carried forward as prose inside the
`rationale` of whatever supersedes it, and the `noOracleDecisions` array is **narrowed to an empty
`capabilities` list rather than deleted** once a real registration exists, "because its own
investigation … remains the honest record of what was checked." Concretely, for `s.energy.model`:

1. EnergyPlus and OpenStudio were surveyed and DECLINED (the survey text is preserved verbatim in the
   `[history, kept verbatim …]` section of the current `rationale`).
2. A no-oracle decision (`energy-model-mutation-semantics`) originally recorded that. The energy
   feature file (`…/🧪️tests/🏛️mutate-energy-model-1/🥒️.feature`) states explicitly: "the no-oracle
   decision this replaces … is narrowed to an empty `capabilities` list rather than deleted."
3. `🐍️.py` beside the feature file (see §2 below) became a real, independently-written, EXECUTED
   second implementation — reclassified `[D3 2026-09-02]` from a no-oracle decision to an oracle of
   `kind: "cross-semio-implementation"`, then `[2026-09-02, D1]` promoted to
   `"verified-native-second-implementation"` once it covered 100% of the manifest's capability with a
   committed fixture vector.
4. The rationale still states the discharge boundary honestly: under Protocol v2 this registration is
   `cross-semio-implementation`/`verified-native-second-implementation` — a required SUPPLEMENTAL
   oracle that does **not** discharge the mutation's external-oracle requirement; "a qualifying
   third-party reference (`third-party-library`/`third-party-cli`/`standards-reference-tool`) is still
   owed and none exists for a semio-native carrier no third party reads." This is exactly the gap an
   EnergyPlus/OpenStudio/Honeybee physics oracle would fill, IF a future physics-facing artifact reads
   or writes a format one of those tools actually understands (e.g. an IDF/OSM export subset, or a
   BESTEST comparison harness that operates on such an export) — `s.energy.model`'s own `.dsl.semio`
   carrier itself is explicitly not such a format.

### 1.4 The ifcopenshell precedent (heavy pip-installed third-party oracle)

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🔮️oracle/🔣️.json` registers
TWO oracles side by side for the same capability, illustrating the READER-vs-PRODUCER distinction a
physics oracle will need to reason about the same way:

- `ruststep-ifc-2x3-base-mutate` — `ecosystem: "rust"`, `package: "ruststep"`, `version: "0.4"`,
  `kind: "cross-semio-implementation"` (reclassified down from `third-party-library` because the
  EXPECTED result is still computed by this repo's own `🦀️oracle.rs`, so ruststep is read-only
  independent evidence about structure, not a second producer).
- `ifcopenshell-ifc-2x3-base-differential` — `ecosystem: "python"`, `package: "ifcopenshell"`,
  `version: "0.8.4.post1"`, `license: "LGPL-3.0-or-later"`, `kind: "third-party-library"` (the
  QUALIFYING kind — IfcOpenShell both reads AND writes ISO 10303-21, making it a genuine second
  PRODUCER). `engine: {family: "ifcopenshell", implementation: "IfcOpenShell C++/Python IFC engine",
  version: "0.8.4.post1"}`.

Its `fixtureManifests[].generator` records exactly how each committed fixture was produced:
`{"oracle": "ifcopenshell-ifc-2x3-base-differential", "packageVersion": "0.8.4.post1", "engineFamily":
"ifcopenshell", "engineVersion": "0.8.4.post1", "command": "uv run --group test python3
.🧬semio/…/🔨️f1-ifc-generate.py", "platform": "darwin-arm64"}` — i.e. the fixture-generation
command is `uv run --group test python3 <script>`, run once, with the output committed as an
ordinary fixture file (`bytes: 188288`/`188310` etc., `sha256:` pinned).

`brepjs` (cad plugin) is a comparable heavy-oracle precedent for a compiled/native third-party
library rather than a pip one; not read in full here, but the same `OracleRegistryEntry` shape
applies (per memory: brepjs already ships in the cad plugin's production dependency graph rather than
being test-only, so it is a different case from ifcopenshell/EnergyPlus, which are strictly
`testOnly: true`).

---

## 2. Provisioning: cache, Python environment, and the `--adapter` invocation

### 2.1 Sanctioned download/cache helper precedent

`📜️script.ts:236-301` (`ensureSccache`) is the canonical shape for a sanctioned third-party binary
download: resolve a platform-specific release asset name (`sccacheReleaseAsset`, line 288), download
under `.🧬semio/🦑️repo/⚡️cache/sccache` (`join(getRepoMetaDir(WORKSPACE_ROOT), "⚡️cache", "sccache")`,
line 258) via `curl -fSL`, extract, `chmod +x`, and install to `~/.local/bin` (or
`%LOCALAPPDATA%\bin` on Windows). No `sha256` pin is present in this particular helper (it trusts the
GitHub release URL directly), but a `Sha256` pattern (`^sha256:[0-9a-f]{64}$`) is a first-class schema
type (`🧰️framework/…/🧬️schema/🔣️.json:22`) used throughout `FixtureFile`, `OracleRegistryEntry.
lockDigest`, `RunManifest`, etc. — an EnergyPlus/OpenStudio release archive download should record its
digest the same way if committed to the cache layout.

`.🧬semio/🦑️repo/⚡️cache` also hosts `vcpkg` (`📜️script.ts:19981`), `cmake` presets
(`📜️script.ts:19993`), and `neo4j-community-<version>` (`📜️script.ts:22715`) — i.e. it is the
general-purpose location for any downloaded toolchain/binary the repo provisions itself, keyed by
subdirectory name and version.

### 2.2 Python oracle environments — root `pyproject.toml`, not a package-local one

`./pyproject.toml` is the single Python workspace manifest (`uv` workspace,
`requires-python = ">=3.14,<3.15"`). Third-party test-only oracles for stdio subsets already live in
its `[dependency-groups] test` array (not `dev`):

```toml
test = [
    "pytest>=9.1.1", "pytest-cov>=7.1.0", "pytest-timeout>=2.4.0", "deepdiff>=9.1.0",
    "mercantile>=1.2.1", "lxml>=6.1.3", "python-pptx>=1.0.2", "python-docx>=1.2.0",
    "openpyxl>=3.1.5", "html5lib>=1.1", "steputils>=0.1",
    "ifcopenshell==0.8.4.post1",
    "anastruct>=1.6.1", "scikit-fem>=10.0.2", "PyNiteFEA>=1.1.6",
]
```

An EnergyPlus-adjacent Python binding (e.g. `eppy`, or an `openstudio` wheel if one exists for the
pinned CPython) would be added as one more pinned entry in this SAME `test` group — there is no
per-package `pyproject.toml` for oracles; `dependencyCollectPython`
(`📜️script.ts:18608-18650`) walks the WHOLE repo for any `pyproject.toml` (skipping
`node_modules/.git/.venv/.nx/target/dist/compose/.🧬semio`) and classifies any dependency under a
`dependencies = [...]` (`project.dependencies`, kind `runtime`) or `dev|test|lint|docs = [...]`
section (kind `test`, i.e. `test-runner` per `dependencyKindOf`) — so it is this root file, not a
subdirectory one, that is scanned for this purpose. The root file's own comment block
(`./pyproject.toml`, just above `[tool.pytest.ini_options]`) states explicitly that
`testpaths = ["compose/py", "compose/engine"]` is COMPOSE-scoped and that non-compose Python tests
(i.e. every `🧪️tests/<case>/🐍️.py` adapter) are "owned by their language-neutral domain owner … and
executed by the repository's own Python host," never discovered through pytest.

The EnergyPlus binary itself (as opposed to a Python binding) is NOT a `uv`/pip package — the
existing `📓️explore-oracle-toolchain.md` in this ticket folder already researched the actual
EnergyPlus/OpenStudio release-asset download shape; provisioning that archive would follow the
`ensureSccache`-style cache-dir + curl + extract pattern (§2.1) into `⚡️cache`, most likely
`⚡️cache/energyplus/<version>` or similar, rather than a `uv` dependency.

### 2.3 The Python adapter (`--adapter`) mechanism

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py` is the native Python host,
invoked as `python3 🐍️.py --plan <plan.json> --out <results.jsonl> --adapter <🐍️component.py>`
(module docstring, top of file). Key shapes:

- `Outcome(projection, raw=None, diagnostics=None, artifacts=None, production_dispatch=None)` — one
  scenario handler's return value; `.artifact(role, path, media_type)` and `.dispatched(operation,
  bridge_version)` are chainable builders. `production_dispatch` is only set by a SUBJECT handler
  that genuinely invoked production dispatch — its absence is how a vector-replay adapter is
  detected.
- `Context(plan, scenario, role, repo_root)` — `.fixture(uri)` resolves a declared `asset://`/
  `shared://`/`local://` fixture to an absolute path (raising `KeyError` if undeclared — "an
  undeclared URI is an error, never a default"), `.fixture_bytes(uri)`, `.copy_fixture(uri, as_name)`,
  `.target()` (the `SubsetTarget` this case is scoped to), `.seed`.
- `Adapter(implementation="python")` — `.oracle(scenario, handler)` and `.subject(scenario, handler)`
  register handlers by role; a module MUST define `def adapter() -> Adapter`
  (`_load_adapter`, loaded via `importlib.util.spec_from_file_location`, module name
  `semio_test_adapter`, with `semio_repo_test` aliased to the host module itself so `from
  semio_repo_test import Adapter, Context, Outcome` works with no installed package).

Energy's own live example, `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/
✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py`, registers exactly ONE role — oracle only
(`Adapter("python").oracle("mutate-replace-model", _mutate).oracle("inverse-replace-model",
_inverse)`) — with the docstring explicitly noting "registering these handlers as subjects too would
make the reference its own subject and manufacture a guaranteed-green self-comparison." Note the
on-disk filename for this repo's adapter is `🐍️.py` (single-file-per-directory taxonomy convention),
NOT `🐍️component.py` as the docstrings/feature-file prose call it by an older informal name — a new
BESTEST adapter should follow the actual on-disk convention, `🐍️.py`, matching every sibling case.

A heavy third-party physics call (spawning the EnergyPlus/OpenStudio binary as a subprocess from
inside a Python adapter's `_mutate`/oracle handler) is architecturally identical to
`ifcopenshell`'s in-process call — the adapter just shells out instead of `import`ing, using
`Context.fixture(uri)` to get the input weather/IDF file path and `Context.artifact(role, filename)`
to place produced output under the plan's own `artifactDir`.

---

## 3. Feature-test discovery, tags, `asset://`, fixture size, and test levels

### 3.1 Discovery: a repo-wide walk for any directory literally named `🧪️tests`

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:539-583`
(`discoverTestCases`) does a full recursive `walkDirectories` over the ENTIRE repo (skipping
`node_modules,.git,target,dist,build,out,storybook-static,.venv,__pycache__,obj,bin` and taxonomy-
declared exclusions), and for every directory whose `basename` equals `taxonomy.testsDirName`
(`🧪️tests`), enumerates its immediate children as CASE directories. A case is only real if
`<caseDir>/<featureFilename>` exists (the canonical `.feature` name resolved from
`taxonomy.testFeatureFileKindId` → the `gherkin-feature` file kind, emoji `🥒️`, extension `.feature`,
role `test` — `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:1657-1663`). There is
NO fixed depth limit from the repo root — `🧪️tests` can appear at any depth — but exactly ONE level
of nesting is used beneath it (case-dir, then feature file directly inside). The `owner` of a case is
literally `dirname(that 🧪️tests directory)` — for energy's case this is the SUBSET root
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

### 3.2 Tag conventions (read off both energy's and epw's `.feature` files)

Feature-level tags (energy's `🥒️.feature`, top of file): `@capability-energy-model-1-mutate`,
`@oracle-energy-model-1-python-independent`, `@comparison-ordered-json-v1`,
`@mutations-energy-model-1-any` — i.e. `@capability-<id>`, `@oracle-<oracle-id>`,
`@comparison-<profile-id>`, `@mutations-<catalog-id>`.

Scenario-level tags: `@id-mutate` / `@id-inverse` / `@id-identity-round-trip` (maps to the `id-*`
scenario grouping), `@level-{fundamental,quick,long,exhaustive}` (one per `TestLevel`, schema
`🧰️framework/…/🧬️schema/🔣️.json:26`), `@mode-{differential,conformance,round-trip,property,error}`
(`TestMode`, schema line 30). A BESTEST comparison scenario would plausibly be tagged
`@mode-differential` (comparing this repo's simulated output against EnergyPlus/OpenStudio's) and
likely `@level-long` or `@level-exhaustive` given real physics-solver runtimes — see §3.4.

### 3.3 `asset://` resolution — against the OWNER ROOT, not a fixtures directory

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:991-1020`
(`resolveFixtures`) resolves `shared://` against `discovered.sharedFixtureDir`
(`<owner>/🧫️fixtures`), `local://` against `discovered.localFixtureDir` (`<caseDir>/🧫️fixtures`), and
`asset://` against `discovered.owner` directly (line 1006: `scheme === "asset" ?
discovered.owner : …`). The doc comment (lines 996-999) states the rationale explicitly: "Real-world
artifacts are already committed where the domain keeps them (examples, assets), and they are large;
copying a multi-megabyte document into a fixtures directory would duplicate history for no gain." Both
energy's own feature (`asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`, resolved against the subset
root) and its mutation-vector references (`asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/…`)
follow this. A BESTEST weather file or reference IDF would be placed as a genuine example/asset under
the subset's `📚️examples/` tree (or a plugin-level shared asset) and referenced the same way, rather
than duplicated per-scenario in `🧫️fixtures`.

Every URI is guarded against path escape (`resolve(abs).startsWith(guard + sep)`, line 1013) and
pinned to a content digest at plan time (`fileDigest(abs)`, line 1017) — this is also how a `sha256`
gets attached to a fixture without a manifest author computing it by hand.

### 3.4 Fixture size — no schema/gate ceiling; 1.6 MB is well inside real precedent

Neither the schema (`FixtureFile.bytes` is `{"type":"integer","minimum":0}`, no maximum — schema
line 329-336) nor `📜️script.ts` enforces a size ceiling on a committed fixture file. No
`.gitattributes` LFS rule exists for domain-model formats (`.gitattributes` only marks generic binary
extensions like `.png/.zip/.wasm/.pack.semio` as `binary`, for line-ending purposes — no LFS filter at
all is configured repo-wide). The nearest thing to a size gate is `CLEAN_TICKET_DIR_MAX_BYTES = 10 *
1024 * 1024` (`📜️script.ts:20050`), which caps a TICKET folder's scratch/generated content, not a
plugin's committed fixtures. Empirically, committed binary fixtures already reach:
- `…/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧫️fixtures/➕️insert-entity-applied/➡️after.ifc` — 2,507,646
  bytes (≈2.4 MB).
- `…/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🧫️fixtures/🏠️mechanical-housing-threaded-boss/
  🎯️expected.step` — 1,623,630 bytes (≈1.55 MB).

A 1.6 MB EPW BESTEST weather file is therefore squarely inside existing committed-fixture practice —
no gate would reject it, and it should carry a `FixtureManifest` with `class: "real-world"` (or
`"third-party-generated"` if it's a canonical BESTEST case file redistributed from ASHRAE 140 rather
than authored in-repo) plus real `provenance.{source,license,url,attribution}`.

### 3.5 `test quick|long|exhaustive` level gating

`TestLevel = enum["fundamental","quick","long","exhaustive"]` (schema line 26). Resolution:
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`:
- `activeTestLevel()` (line 1150) reads `process.env.SEMIO_TEST_LEVEL`, defaulting to `"fundamental"`.
- `resolveTestLevel(segments)` (line 1159) takes `segments[0]` as the level if it names one (else
  falls back to the env var), SETS `process.env.SEMIO_TEST_LEVEL` so every child process (vitest,
  cargo, go, pytest, dotnet) inherits it, and auto-sets `SEMIO_COVERAGE=1` at `"exhaustive"`.
- `testLevelRank(level)` (line 1168) / `testLevelAtLeast(level)` (line 1174) — numeric rank
  0..3.
- `atTestLevel(factory, level)` (line 1184) — `factory.runIf(testLevelAtLeast(level))`, the Vitest
  gate.
- `testLevelBudgetMs`/`testLevelBudgetSeconds` (lines 1193/1198) — wall-clock budget per level,
  `SEMIO_TEST_BUDGET_MS`-overridable.
- `goLevelTestArgs(level)` (line 1203) — per-level `-timeout`, `-short` through `quick`, `-skip
  ^Test(<LevelsAbove>)` for levels not yet reached.

CLI entry: `📜️script.ts:19058` (`resolveTestLevel(segments)` inside the `TestScript` handler
registered at `📜️script.ts:22635`) and `📜️script.ts:18983` maps a level to its Nx target name
(`level === "fundamental" ? "test" : \`test-${level}\``). A physics oracle invoking a real
EnergyPlus/OpenStudio binary (multi-second-to-minute wall time per BESTEST case) should be tagged
`@level-long` or `@level-exhaustive` so it is excluded from the default `fundamental`/`quick` inner
loop and only runs under `bun ./📜️script.ts test long` / `test exhaustive`.

---

## 4. `verify dependencies literal-external` — classification and oracle-conflict listing

All in root `📜️script.ts`.

### 4.1 Command surface

- `📜️script.ts:10535` — `if (segments[0] === "dependencies") { … }` — command router.
- `📜️script.ts:10947-10960` — `args[0] === "summary" || args[0] === "literal-external"` branch:
  builds a `DependencyTruthReport`; for `literal-external`, throws unless `report.meetsTarget`, and on
  failure prints one line per `report.oracleConflicts` (`oracle-conflict <ecosystem>:<name> declared
  by <users>`) and per `report.toolchainConflicts`
  (`toolchain-owner-conflict js:<name>@<version> declared by <user> (lock-owned=<bool>)`), then throws
  `target=0, current=<n>, oracle-conflicts=<n>, toolchain-owner-conflicts=<n>,
  toolchain-failures=<n>`.
- `📜️script.ts:11335-11336` — CI gate registration: `⚖️gate📦️dependencies` runs plain `verify
  dependencies`; `⚖️gate📦️dependencies0️⃣` runs `verify dependencies literal-external`.

### 4.2 How a package becomes `test-oracle` instead of `literal-external`

- `DependencyKind = "production-runtime" | "production-build" | "repository-tooling" |
  "test-runner" | "test-oracle"` (`📜️script.ts:17805`). `test-oracle` is reserved, per the type's own
  comment (line ~17802), "for packages an approved [oracle registration] is about."
- Collection (`dependencyCollectPython`, `📜️script.ts:18608-18650`; analogous Cargo/JS/Go/.NET
  collectors) walks every manifest in the repo and records each third-party dependency with its
  declaring manifest path and section-derived `DependencyKind`.
- `dependencyOracleRegistryPackages(repoRoot)` (`📜️script.ts:18479-18503`) builds a `package name →
  oracle id[]` map by reading the CORE oracle registry location
  (`taxonomy.testOracleRegistryLocation`) plus EVERY plugin/subset contribution manifest discovered
  via `taxonomy.testContributionDirName`/`testContributionDirectoryOverrides`
  (i.e. every `🔮️oracle/🔣️.json` AND every plugin-level `🧪️oracle/🔣️.json`, exactly the files read
  in §1), parsing each one's `oracles[].{id,package}` — so an energy `oracle/🔣️.json` entry with
  `"package": "eppy"` (or whatever the chosen binding is named on PyPI) is exactly what would need to
  be added for the gate to recognize it.
- `dependencyClassifyOracleEntry(entry, oracleIds, directoryOverrides, defaultDirectoryName)`
  (`📜️script.ts:18465-18476`): if the package name has NO matching oracle registration, it is left
  alone (ordinary `literal-external` classification). If it DOES match, every declaration is checked:
  `DEPENDENCY_TEST_DOMAIN_PATH_RE = /(?:^|\/)(?:🧪️test|🔬️probes|🏭️generator|🧫️fixtures)\//u`
  (line 18462) plus an `isContribution(path)` check (declaring manifest sits under a
  `testContributionDirName`-named directory) — a declaration is "test-domain" if it matches either.
  `productDeclarations` = declarations that are NEITHER test-domain NOR contribution-scoped AND have
  kind `production-runtime`/`production-build`. If `productDeclarations.length === 0`, the WHOLE entry
  is reclassified `kinds = ["test-oracle"]` (line 18474) — a package declared only in
  `[dependency-groups] test` (root `pyproject.toml`), which is exactly where the energy physics oracle
  would live, satisfies this trivially since it is not even a production section. If there IS a
  production-side declaration, the entry instead gets `oracleConflictUsers` set (line 18475) — this is
  the list surfaced as `report.oracleConflicts` (§4.1) — i.e. an oracle name can never quietly become
  a production dependency somewhere else in the tree; that always fails
  `verify dependencies literal-external`.

### 4.3 Where oracle conflicts are listed / surfaced

`report.oracleConflicts` (built at `📜️script.ts:18801`, `classifiedThirdParty.filter(entry =>
(entry.oracleConflictUsers?.length ?? 0) > 0)`) and `report.toolchainConflicts` (line 18812,
`unauthorizedRows`) are both read into `meetsTarget` (line 18807: `totals.literalExternal === 0 &&
oracleConflicts.length === 0 && unauthorizedRows.length === 0 && toolchain.failures.length === 0`) and
printed verbatim by the CLI (§4.1). There is no separate persisted "oracle-conflicts list" file — the
list is recomputed live from the union of {every manifest in the repo} × {every oracle registry
package}, every time the gate runs.

---

## Practical implication for the energy BESTEST oracle

1. Add the chosen Python physics binding (or `pywin32`-free `eppy`-style IDF wrapper — whichever
   ecosystem is actually chosen) to root `./pyproject.toml`'s `[dependency-groups] test` array
   (§2.2), pinned by exact version like `ifcopenshell==0.8.4.post1`.
2. Register it in a NEW or existing energy-tree `🔮️oracle/🔣️.json` (subset-level, per §1.1/§1.2) as an
   `OracleRegistryEntry` with `ecosystem: "python"`, `package: "<pypi-name>"`, `kind:
   "third-party-library"` (qualifying, if it independently reads/writes/simulates a real EnergyPlus/
   OpenStudio input — this is the piece energy currently lacks and DECLINED), a real `engine.family`
   (e.g. `"energyplus"`), `testOnly: true`, `productionReachable: false`, and `rationale` documenting
   what BESTEST case(s) it discharges.
3. Provision the actual EnergyPlus/OpenStudio binary via a `ensureSccache`-style cache-dir download
   (§2.1) into `.🧬semio/🦑️repo/⚡️cache/…` if a compiled solver binary is needed beyond the pip
   binding — record a `sha256:` digest.
4. Write `🧪️tests/<bestest-case>/🥒️.feature` with `@capability-…/@oracle-…/@comparison-…` tags
   (§3.2), `asset://` references to a committed BESTEST IDF/EPW under the subset's `📚️examples/` tree
   (§3.3, §3.4 — a 1.6 MB EPW is fine), `@level-long` or `@level-exhaustive` (§3.5, §5), and a sibling
   `🐍️.py` adapter (§2.3) that shells out to (or binds) the physics engine as the ORACLE role only.
5. Confirm `bun ./📜️script.ts verify dependencies literal-external` passes (§4) — the new package
   must resolve to `test-oracle`, meaning it may be declared ONLY in test-domain/contribution paths.
