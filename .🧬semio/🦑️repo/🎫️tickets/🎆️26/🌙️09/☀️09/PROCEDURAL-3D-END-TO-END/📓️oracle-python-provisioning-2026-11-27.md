# 🐍️ Oracle Python provisioning (2026-11-27)

## Root cause
The test host provisions its Python venv only from `oracleHostPackages`. The generation3d subset
manifest registered `example-geometry-scipy` (scipy, nested numpy) but left `oracleHostPackages: []`,
so nothing was provisioned. The registry had two sources of truth for one fact.

## Fix
- Single source of truth: `oracleHostPackagesOf(oracle)` and `packagesForOwner` in
  `test/🕸️dependencies/🟨️.mjs` (+ `.d.mts`) derive the external host rows from the registered
  oracles (primary + nested `packages`; python -> python, javascript -> typescript; lock markers,
  `node:` builtins, empty packages skipped). `oracleHostPackagesFor` therefore returns local rows plus derived rows.
- Schema: `OracleRegistryEntry` and `OracleLinkedPackage` gain optional `module`; `OracleHostPackage` documented as local `path` source or host-only library no oracle registers.
- `provisionPythonInterpreter` accepts PEP 440 ranges (`>=2.5.0`): `pythonRequirement`, `pythonVersionSatisfies`.
- Redundant external rows removed from 14 manifests (pets, course, repo modules events/model/mcp/workspace/tickets/graphql/contributors/goals/languages/todos/yaml, puzzle 2d) and `pypdf` from hub stdio. `module` added for norm `blue-prints` (blueprints) and `PyNiteFEA` (PyNite).
- README of the test module updated.

## Why the field is not removed entirely
`oracleHostPackages` still carries (a) local `path` sources (Rust crates, norm/energy python modules), which no oracle describes, and (b) host-only external libraries with no registered oracle (plugins stdio, bim, energy jsonschema, hub stdio simplejson/ifcopenshell). Follow-up: register those as oracles and drop the rows.

## Contract drift fixed in the Python lane (not geometry)
`📐️example-geometry-3d-1/🐍️.py` read stale paths: brep descriptor moved to `📐️brep/🧩️component/🔣️.json` (key `manifest.topicContributions`), fixtures moved to `<example>/🧪️tests/🧫️fixtures/🧩️example/`, a 9th example (`mesh-workbench`) exists. Oracle now derives its closed form (1 + (1-2a)^2 d, extent 1x1x(1+d)). No geometry mismatches.

## Tests
- New `test/🧪️tests/🔮️oracle-host-provisioning/{🟦️.ts,🐍️.py}` + JSON vectors: TS 7 pass; Python twin validated against PEP 440 `packaging` (run via pip's vendored copy, pass; not routable through `test_file`).
- Contract `🐍️.py` via `test_file`: PASS (numpy 2.5.0 + scipy, 26 expectations, 9 examples).
- TS twin Hexagonal Mushroom Column: 4 pass.
- `oracle-governance` fails at import ("test reached the live repository store"), `oracle-source-ownership` 4 fail (missing norm en1991 mutation schema dirs, ran under system python 3.9): not caused by this change; norm owners now also derive external rows (expectation updated to local rows only).
- Risk: norm python cases now provision a cache venv for their registered oracles (pins from oracle versions); unverified.

## Follow-up (oracles as the single source of truth)
- `oracleHostPackages` now means LOCAL in-repo source only: schema requires `path`, the type/README say so, and `externalOracleHostPackages` (dependency classification, repository contract, governance tests) is removed because registered oracles already classify through the oracle ratchet. The field stays only for the `path` rows (Rust crates, norm/energy python modules).
- Host-only libraries registered or removed: bim plugin rows (shapely, ifcopenshell, lxml, three, jsonpatch, deepdiff, jsonschema, pypdf, numpy, sympy) were all already registered by the bim model subset oracles; energy `jsonschema` by `energyplus-25-2-0-direct-epjson`; stdio simplejson/ifcopenshell/pypdf by the i-json, ifc and pdf subset oracles. The two now-empty plugin manifests (`✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔣️.json`, `🏙️bim/🔮️oracles/🔣️.json`) are deleted. Hub stdio python rows removed; its `pypdf-pdf-1-4-text` oracle moved to the pdf 1.4 base subset, the only owner of that case (it was unreachable from there, so the case had no pypdf).
- `oracleHostPython` resolves `SEMIO_PYTHON`, then the locked workspace environment (`provisionedPythonV1`, now exported from the toolchains barrel), then `python3`; the `.venv` fallback is gone.
- norm `PyNiteFEA` module was wrong (`PyNite`); real import is `Pynite` (found by provisioning the en1993 owner for real). Fixed in the norm manifest and the vector fixture.
- oracle-governance: `repository-scan` opens a private repository workspace; `withRepositoryTestArtifactsV1` gained `serves` (source checkouts whose test outputs the private store holds), so tests keep reading the checkout and never touch the live store.
- oracle-source-ownership: en1991 adapter read schemas by the shortened fixture directory (3 renamed mutations); it now uses `sourceMutationDirectoryName`, and the en1991 feature uses the shortened fixture names like en1993. Stale data fixed: en1993 inverse table lacked 135 kinds, din4108 and en1998 lacked 7 kinds each in both tables, en1998 adapter now reads its kinds/vectors from its catalog (74 kinds were missing), fifteen-adapter glob scoped to `*mutate-*`, scenario count moved to the fixture (649).

## Results
- Norm provisioning (en1993 owner, real registry, cache venv, network pip): 16 rows, interpreter provisioned, every external module imports (after the `Pynite` fix). Temporary test removed.
- generation3d contract Python lane: PASS. `oracle-host-provisioning` TS: 7 pass. `oracle-source-ownership`: 6 pass (was 4 fail). Typecheck of all touched TS: 0 errors.
- `oracle-governance`: imports and runs (34 pass, 5 fail, was an import failure). Remaining 5 are live-registry findings unrelated to this change: `forms-python-independent` and `layout-1-python-independent` are partial second implementations; layout `🧳️retained/🔮️oracle.ts` imports `layout-structure-json-patch` in production; ui WindowFrame imports `three-damp`; `json` classified production-reachable with no recorded debt.
- Not done: discovery-contract and comparison-profiles share `repository-scan` and now reach the private store, but their own `repoRoot`-relative path assertions (e.g. cache root under the checkout) were not re-verified.
