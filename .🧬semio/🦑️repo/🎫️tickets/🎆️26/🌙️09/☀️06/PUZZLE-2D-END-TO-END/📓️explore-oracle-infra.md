# Third-Party Oracle Infrastructure for `s.puzzle.2d@1` — Exploration Report

Read-only exploration. `repo`/`semio` MCP servers failed to connect for this whole session
(`invalid initialize params` / `CONNECTION_CLOSED`); everything below was gathered via `find`/`grep`/
`sed`/`python3` against the working tree. No cargo build, no git write, no ticket action was performed.

## 0. TL;DR

`s.puzzle.2d@1`'s external-oracle requirement is **already mechanically discharged today** — its
`🔮️oracle/🔣️.json` was promoted from `cross-semio-implementation` to `verified-native-second-implementation`
on 2026-09-02 (D1 shard, alongside puzzle-3d/5d and 50 other artifacts), and that kind is one of the four
members of `QUALIFYING_ORACLE_KINDS`. But that promotion is a **repo-wide native-artifact exemption**, not
a genuine third-party check, and the ticket's own DoD #2 explicitly wants more: real networkx/shapely
validation of graph-cascade and geometry semantics. The mechanism to add that already exists and is
cheap: register networkx/shapely as `kind: "third-party-library"` oracle entries against
`puzzle-2d-1-mutate` (or narrower split capabilities) — discharge is a purely mechanical
`capabilities.includes(x) && isQualifyingOracleKind(kind)` check (`🟦️.ts:4828` `oracleRequirementBreaches`),
exactly the same mechanism that lets `three`/`manifold-3d` discharge `fem2d-1-mutate` today even though
they only read the mesh carrier, not the load-case semantics. Honeybee → OpenStudio → EnergyPlus is
correctly out of scope here (confirmed: it is being wired for `26/09/06/ENERGY-PLUGIN-END-TO-END`, not
puzzle) — jack's own survey (`networkx`/`igraph`/`petgraph`, declined) is the closer, directly relevant
precedent, and its stated reason (no port-addressed-endpoint notion) is real but narrower than "can never
help": it blocks networkx from being the *whole-document* second implementation, not from being a
*narrow, capability-scoped* graph-cascade oracle.

---

## 1. Protocol V2 — what discharges the external-oracle requirement, exactly

**Schema.** `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json:5` — v2 "requires a QUALIFYING
third-party oracle for every mutation" (ticket `26/08/27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING`).

- `QualifyingOracleKind` (schema line 72-75): enum `["third-party-library", "third-party-cli",
  "standards-reference-tool"]`. **This enum is stale relative to the TS implementation** — see the drift
  note below.
- `SupplementalOracleKind` (line 76-79): `["metamorphic", "inverse", "round-trip", "property",
  "cross-semio-implementation"]` — "can never substitute for a qualifying oracle."
- `OracleRequirement` (line 227-236): `{capability, qualifyingKind, distinctEngineFamilies?}` — one per
  `ManifestMutation`.
- `OracleRegistryEntry` (line 113-150): required fields `id, kind, ecosystem, package, capabilities,
  comparisonProfiles, license, testOnly, engine, productionReachable, networkDuringExecution`. Optional:
  `packages` (further linked libs, schema line 89-101, `OracleLinkedPackage`), `source`, `hostPath`
  (real files use this, e.g. stdio's pdf-writer/lopdf/png entries), `rationale`, `productionDebt`.
- `OracleHostPackage` (schema line 705-716): `{implementation, package, path?, version?, module?,
  features?}` — declares what the runner must provision (pip/npm-installed if no `path`; a local crate if
  `path` is set).

**Drift finding (worth flagging to whoever owns the schema next):** the JSON schema's
`QualifyingOracleKind` enum (3 values, schema `🔣️.json:74`) does **not** include
`"verified-native-second-implementation"`, but `🟦️.ts:2841`'s `QUALIFYING_ORACLE_KINDS` const has **4**
values including it, and `isQualifyingOracleKind` (`🟦️.ts:2790`) — the actual predicate every discharge
check calls — accepts it. CLAUDE.md requires schema-first; today the code is ahead of the schema by one
enum member. Confirmed by `grep -c` on both files: zero hits for the literal string in the `.json` schema,
one const array + doc comments in `🟦️.ts:2835-2841`.

**Discharge mechanism** — `oracleRequirementBreaches` (`🟦️.ts:4828-4860`):
```ts
const supplying = registry.oracles.filter((oracle) => oracle.capabilities.includes(requirement.capability));
const qualifying = supplying.filter((oracle) => isQualifyingOracleKind(oracle.kind));
if (qualifying.length === 0) { /* emits missing-external-oracle */ }
```
This is the **entire** discharge test: at least one registered oracle whose `capabilities` array names
this exact capability id AND whose `kind` is qualifying. It does **not** check that the oracle covers
every mutation kind, every argument, or every semantic effect the capability's mutations have — that is
exactly how `three`/`manifold-3d` (mesh-carrier readers) discharge `fem2d-1-mutate`'s entire 25-kind
vocabulary today even though they read geometry, never a load case (§2). `distinctEngineFamilies`
(default 1) additionally requires that many *independent engine families* among the qualifying set
(`families.size < required` → `insufficient-engine-independence`, medium severity) — relevant if we want
networkx AND shapely both counted as independent (they are: different families, `engineFamilyId` keys off
`engine.family`).

**`verified-native-second-implementation`'s earned-ness gate** —
`nativeSecondImplementationBreaches` (`🟦️.ts:4881`, called from `validateAllContracts` right after
`reimplementationOracleBreaches`, `🟦️.ts:1922`), 8 checks in order, each a distinct breach id:
1. `nativeSecondImplementation` evidence object present → else `native-second-implementation-unearned`.
2. `isSemioNativeArtifact(evidence.format)` (`🟦️.ts:2868`: true unless `artifact.startsWith("s.stdio.")`
   and it isn't `s.stdio.semio`) → else `native-second-implementation-not-native`.
3. `evidence.format` matches an `artifact` this SAME contribution's own `mutationManifests` declares.
4. **100% capability coverage** — `oracle.capabilities` ⊇ every capability the owning manifest declares
   → else `native-second-implementation-partial-coverage`.
5. `noThirdPartySurvey.ecosystemsSearched` and `.candidatesConsidered` non-empty, every candidate has a
   real `package` and a `reason` ≥ 10 chars.
6. `subjectImplementationLanguage` ≠ `secondImplementationLanguage` (case-insensitive) → else
   `native-second-implementation-same-language`.
7. `specificationSource` non-empty (unfalsifiable but required-present, by design).
8. `fixtureCoverage.vectors > 0` and `capabilitiesCovered` non-empty.

Full rule narrative and the 53-entry/872-mutation application (puzzle 2d/3d/5d included, 89 mutations
combined) is `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️d1-native-oracle-discharge.md`.
Key quote (§1): *"`verified-native-second-implementation` is the one exception to 'a second implementation
never discharges', and it exists ONLY for a format no third party can, even in principle, implement."*

**Comparison mechanics.** `CORE_COMPARISON_PROFILES` (`🟦️.ts:66-70`): `ordered-json-v1` ("structural
identity, array order significant, key order never is"), `unordered-json-v1` (arrays as multisets); more
profiles (e.g. `semantic-brep-kernel-v1`, `semantic-pdf-v1`, `semantic-raster-v1`) are registered
per-plugin in `comparisonProfiles`. `canonicalize` (`🟦️.ts:1201`) normalizes before compare;
`compareProjections` (`🟦️.ts:1284`) applies the named profile and returns a `ComparisonVerdict`.
Puzzle-2d's own registration uses `ordered-json-v1` throughout — appropriate, since its projections are
whole-board JSON snapshots/diffs.

**Invocation.** `provisionPythonInterpreter` (`🧰️framework/…/🧪️test/📜️script.ts:492-540`) builds a
**cache-local venv** (never the system interpreter) at
`testCacheDir(repoRoot,"hosts")/python-env-<sha of base+sorted package specs>`, created with
`python -m venv --system-site-packages <dir>` (so anything the machine already has is reused, otherwise
`pip install <pkg>==<version>` runs automatically — no manual `.venv`/`uv` setup is required from us; the
runner does it). It is keyed and rebuilt only when the declared package set's signature changes
(`🧾️packages.json` stamp). `runProbe` (used throughout `script.ts` and `🟦️.ts:2367` etc.) is the
`spawnSync`-based subprocess wrapper with a `budgetMs` ceiling; the case adapters themselves run via the
Python host `🧰️framework/…/🧪️test/📦️packages/🐍️python/🐍️.py` (`python3 🐍️.py --plan <plan.json> --out
<results.jsonl> --adapter <case's 🐍️.py>`), whose `Outcome`/`Context`/`Adapter` classes (lines 49/82/143)
are the facade every python case (second-impl or third-party) imports — `from semio_repo_test import
Adapter, Context, Outcome` (verbatim import line used by e.g.
`✏️s/🔌️plugins/🗄️stdio/…/🏗️ifc/…/🧪️tests/🔺️differential-ifc-4/🐍️.py:44`).

**Budgets.** `TEST_LEVEL_BUDGET_MS` (`🧰️framework/…/📚️library/📦️packages/🟦️typescript/🟦️.ts:1135-1141`):
`fundamental` 15s, `quick` 30s, `long` 300s, `exhaustive` 900s (the "runner's own 900s per-case budget"
referenced in puzzle-2d's own oracle rationale text and in the ticket status log). Overridable via
`SEMIO_TEST_BUDGET_MS`.

**Cache.** `testCacheRoot` = `.🧬semio/🦑️repo/⚡️cache/tests` (`🟦️.ts:1148`), with 6 named children
(`work, hosts, oracles, results, reports`, + a `diffs`-shaped slot in the schema comment — only 5 exist on
disk today: `hosts, reports, results, work`, confirmed by `ls`). Result dirs are named
`test-<slugified-owner-path>-<6-hex-hash>-<case-emoji-name>-oracle-<lang>` (e.g.
`test-s-plugins-raster-artifacts-raster-standards-1-subsets-any-800d3c-🖨️mutate-raster-1-oracle-python`,
confirmed present and `passed: 37` in raster's exploration). **No `puzzle` entries exist under
`.🧬semio/🦑️repo/⚡️cache/tests/results` today** — puzzle-2d has not been run through this pipeline yet in
this cache generation.

---

## 2. Real third-party oracles wired end to end today (2-3 worked examples)

**(a) `s.stdio.semio@v1/🔺️mesh` and `🧊️brep`** —
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🔮️oracle/🔣️.json`:
- `three-carrier-reader`: `kind: "third-party-library"`, `ecosystem: "javascript"`, `package: "three"`,
  `version: "0.182.0"`, capabilities include `semio-v1-mesh-mutate` plus format-specific ones
  (`mesh.carrier.stl/obj/ply/gltf`, `mesh.material.pbr`, `mesh.texture`).
- `manifold-mesh-measure` / `manifold3d-three`: `kind: "third-party-library"`, `package: "manifold-3d"`,
  `version: "3.5.1"`, license Apache-2.0, capabilities `mesh.measure.volume/area`, `mesh.topology.genus`,
  `mesh.generate.boolean`, etc.
- `brepjs-occt` (`…/🧊️brep/🔮️oracle/🔣️.json`): `kind: "third-party-library"`, `package: "brepjs"` `18.119.8`,
  `comparisonProfiles: ["semantic-brep-kernel-v1"]`.
- **Generator script**, the clearest worked pattern:
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/📜️script.ts` — a
  *separate, reviewed-before-commit* fixture generator (never run by a normal test) that imports
  `manifold-3d` (WASM solid-boolean kernel, builds every shape + computes volume/area/genus truth) and
  `three` + its example exporters/loaders (STL/OBJ/PLY/GLTF) to round-trip the shape and re-measure it —
  "two second parties in series, deliberately from different engine families... proves the format
  round-trips, not that either library is right about anything this repository does." Both packages are
  declared in root `🔒️dependencies.json` (`kind: "test-oracle"`, `manifold-3d` 3.5.1 with `oracleIds:
  ["manifold-fem2d-mesh-measure","manifold-fem3d-mesh-measure","manifold-mesh-measure","manifold3d-three"]`).

**(b) `s.fem.fem2d@1/🌐️any`** —
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracle/🔣️.json` registers, on the
**same capability id** `fem2d-1-mutate`, both `fem2d-python-independent` (`cross-semio-implementation`,
declined third parties: `code_aster`/`OpenSees`/`anastruct`/`PyNite` — "compute displacements FROM a
model" while every FEM2D kind edits the model document itself, so none reads `.dsl.semio") **and**
`three-fem2d-mesh-reader` + `manifold-fem2d-mesh-measure` (both `third-party-library`). This is the
concrete precedent for puzzle-2d's situation: a genuine, narrow third-party reference (mesh/geometry) can
sit on the exact same capability id as a whole-vocabulary native second implementation, and its mere
presence with a qualifying `kind` is what flips `missing-external-oracle` off for that capability —
confirmed by re-reading `oracleRequirementBreaches` (§1): it unions ALL oracles naming the capability
before filtering by kind, so a narrow-but-real entry and a broad-but-native one coexist and either alone
would discharge it.

**(c) `s.stdio.ifc@4/✳️any`** — a genuine differential SECOND PRODUCER, not just a reader:
`ifcopenshell-ifc-4-any-differential` (`third-party-library`, python, `0.8.4.post1`) plus
`steputils-ifc-4-any-mutate-reader`. The adapter
`…/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🔺️differential-ifc-4/🐍️.py` (662 lines) is the fullest
worked python-oracle example in the repo: docstring states exactly what IfcOpenShell can/cannot produce
(7 of 11 kinds), measured against a real 2.4 MB / 24,792-entity Nakagin Capsule Tower IFC export, applies
each mutation with `ifcopenshell.file`, re-serializes with IfcOpenShell's own C++ Part-21 writer, and
reads the result back with a **from-scratch** ISO-10303-21 Part21Reader the case author wrote from the
standard's clauses (not from this repo's own step/ifc codec) before `semantic-ifc-v1` compares it to the
Rust subject. `ifcopenshell` is declared once in root `pyproject.toml`'s `test` dependency-group
(`ifcopenshell==0.8.4.post1`) and once in `🔒️dependencies.json` (`kind: ["test-oracle"]`).

**Where the plugin-ROOT `🧪️oracle/🔣️.json` files fit** (checked `🌍️gis/🔮️oracle/🔣️.json` and
`🏗️fem/🧪️oracle/🔣️.json`, both 15 lines): both are **empty stubs** (`oracles: [], mutationCatalogs: []`)
whose only content is `oracleHostPackages: [{implementation:"rust", package:
"semio-s-plugin-stdio-test-oracle", path: "…/🗄️stdio/🧪️oracle/📦️packages/🦀️rust"}]` — they exist purely
because `oracleHostPackagesFor` resolves host packages by walking up from the case owner, and this is how
a plugin without its own native-codec crate still links stdio's shared reference codecs for IO leaves. All
real oracle registrations for gis/fem/cad/puzzle/jack/etc. live at the per-subset
`🏅️standards/…/🪆️subsets/…/🔮️oracle/🔣️.json` level, not the plugin root.

---

## 3. Candidate third-party libraries for puzzle-2d — availability and fit

Checked with `python3 -c "import X"` (system), `uv pip list` / `uv pip show` (repo-managed `.venv`, python
3.14.5), and `grep` over `🔒️dependencies.json` + `pyproject.toml`.

| Library | Installed now? | Where | Fit for puzzle-2d |
|---|---|---|---|
| **shapely** 2.1.2 | Yes, in `.venv` | Transitive dep of `ifcopenshell` (`Required-by: ifcopenshell`), **not declared directly** in `pyproject.toml`'s dependency groups | Good for node geometry: `shape`/`radius`/`width`/`height` (`replace-node-geometry`), `x`/`y` (`move-node`), `scale`/`anchor` — a real `Point`/`Polygon`/`box` can independently confirm area, centroid, bounds after a move/scale. Weaker fit for edge geometry: `gap/shift/rise/rotation/turn/tilt/x/y` (`EDGE_GEOMETRY`, python component `🐍️.py:70`) is a **routing-parameter record**, not two real endpoints — shapely can validate numeric ranges/transform algebra but not "this edge visually connects A to B" without first deriving endpoint coordinates from the handle geometry, which this format does not carry directly. |
| **jsonschema** 4.26.0 | Yes, in `.venv` (`fastjsonschema` 2.21.2 too) | Transitive (not a direct pin found in `pyproject.toml`) | Would validate every mutation payload against `🧬️.schema.json` per mutation dir — a real, independent (C-extension-backed, ships from PyPI) schema validator, but it checks *shape*, not *semantics* (cascade correctness, compatibility-relation bipartite consistency) — best as a supplement, not the qualifying oracle itself. |
| **deepdiff** ≥9.1.0 | Yes | Declared directly, `pyproject.toml` `test` group; `🔒️dependencies.json` entry `kind: ["test-runner"]` | Could independently recompute the `🔺️diff/🔣️.json` documents from `(before, after)` and compare to the committed diff — a genuine second implementation of "what changed," useful against the `🔺️diff` schema explicitly named in the task. Already a repo dependency, zero new provisioning cost. |
| **networkx** | **No** — `ModuleNotFoundError` on system python; absent from `uv pip list`, absent from `pyproject.toml`, absent from `🔒️dependencies.json` | n/a | Not installed anywhere in this repo today. Declining it as a *whole-document* oracle is already argued, twice, in-repo: puzzle-2d's own oracle rationale (`"GraphML, DOT and GEXF all join node to node and none of them can express an edge whose endpoints are ports OWNED BY a node"`) and jack's (`s.trinity.jack`) survey (`"networkx, igraph and petgraph model vertices and edges but have no notion of a PORT-addressed endpoint, of a manifest that closes the kind vocabulary, or of the untyped property bag...edit"`, `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`). Both objections are about *reading the whole carrier*, not about verifying a *narrower, well-defined graph property* — see §4. |
| **graphology** (TS/npm) | Not checked for installation (no npm registry probe attempted; not found in any `package.json` grepped) | n/a | Same shape-mismatch as networkx, but as a second ecosystem (JS vs Python) it would satisfy `distinctEngineFamilies` if ever required alongside a python graph oracle. Lower priority than networkx given the repo's existing python-oracle convention (ifcopenshell, jsonschema, shapely, deepdiff all already live in the same venv). |
| **jsonpatch** (RFC 6902) | Not checked; not found in `pyproject.toml`/`dependencies.json` | n/a | Would need adding. RFC 6902 patches are index/pointer-based; puzzle-2d's own `🔺️diff` schema is domain-shaped (per the schema dir at `…/🧬️schema/🔺️diff/🔣️.json`), so a jsonpatch oracle would need a translation layer — more machinery than deepdiff for the same job. |

**Ecosystems already searched, per the D1/no-third-party-survey convention this repo enforces**
(`noThirdPartySurvey.ecosystemsSearched`, `nativeSecondImplementationBreaches` check 5, `🟦️.ts:4944-4954`):
`python/pypi` is the one already recorded for puzzle-2d/jack/gismap; `js/npm` has not been recorded there.

---

## 4. Recommended oracle design

**Split the monolithic capability, mirroring `s.architect.program`'s own
`program-1-mutate` / `program-1-xlsx-export` split** (`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/⚖️oracle/🔣️.json`)
rather than trying to make one library cover all 26 kinds:

1. **`puzzle-2d-1-graph-cascade`** (new capability, or keep `puzzle-2d-1-mutate` and just add the oracle to
   it — §1 confirms the discharge check does not require full coverage): covers `create-node`,
   `delete-node`, `add-node-handle`, `remove-node-handle`, `replace-node-handle`, `connect-handles`,
   `disconnect-handles`, `connect-kind-compatibility`, `disconnect-kind-compatibility`,
   `replace-kind-catalogs` — the 10 kinds whose correctness is fundamentally graph-shaped (cascade-on-delete,
   handle-ownership, and the kind-compatibility relation, which is exactly a **bipartite graph over kind
   labels** with `bidirectional`/`important`/`specificity` edge attributes,
   `🐍️.py:78` `COMPATIBILITY_FIELDS = ("source", "target", "bidirectional", "important", "specificity")`).
   **Model as a `networkx.MultiDiGraph`**: one node per **handle** (not per board-node — this is the fix for
   jack's stated objection), each carrying `owner=<node id>` as an attribute; edges of the puzzle board
   become graph edges between handle-nodes; deleting a puzzle node = removing all handle-nodes with that
   `owner` (networkx cascades this automatically via `remove_nodes_from` + incident-edge removal, which is
   the exact behavior `delete-node`/`remove-node-handle` need to match); the kind-compatibility relation
   becomes a *second*, separate `networkx.DiGraph` keyed by kind label with the four extra edge attributes.
   This sidesteps the declined-candidate reasoning verbatim (no format is being *read*; a graph is being
   *reconstructed* from the already-parsed JSON board and *independently* checked for the topology
   invariant the mutation claims) and is a materially different circumstance from GraphML/DOT/GEXF (which
   the survey correctly declined as *carrier* formats).
2. **`puzzle-2d-1-geometry`** (or fold into the same capability): covers `move-node`,
   `replace-node-geometry`, `scale-node`, `change-node-anchor`, and — with the caveat above — the
   coordinate-like half of `replace-edge-geometry`/`connect-handles`'s `x`/`y` members.
   **Use shapely** (`Point`, `box`, or `Polygon` per `shape`), asserting bounds/area/centroid transform
   correctly under `move`/`scale`/`replace-node-geometry`'s null-dropping rebuild
   (`🐍️.py:220-226`, `NODE_GEOMETRY = (("shape","newShape"),("radius","newRadius"),("width","newWidth"),
   ("height","newHeight"))`). Already present in `.venv` (transitively) — add a direct `pyproject.toml`
   pin so the dependency is honest rather than accidental, and register a `packages` entry `["shapely"]` in
   the new oracle (schema `OracleLinkedPackage`, `🔣️.json:89-101`).
3. Keep `puzzle-2d-python-independent` (`verified-native-second-implementation`) exactly as-is for the
   remaining field-setter kinds (`change-node-kind`, `edit-node-text`, `change-node-icon`,
   `change-node-visible/locked/root`, `change-edge-kind/visible/locked`, `change-manifest-id`) — these are
   pure record edits with no independently-checkable external semantics; forcing a third-party library onto
   them would be exactly the "category error" fem2d's own rationale warns against for anastruct/PyNite.
4. **Comparison profile**: `ordered-json-v1` for the geometry/graph oracle's own projection (a
   canonicalized `{nodes, edges}` adjacency + attribute dump), matching what the rest of this artifact's
   registration already uses — no new profile needed.
5. **Vectors**: at minimum, one real-world-tagged scenario per newly-covered kind beyond today's single
   handcrafted vector each (the ticket's own DoD #2), reusing the two committed examples
   (`🌲️concrete-forest`, `🏗️nakagin-capsule-tower`) as source boards where their node/handle/edge shapes are
   rich enough — cheaper than authoring 26 new synthetic boards from scratch, and keeps "real-world fixture"
   honest rather than another handcrafted vector.
6. **Registration mechanics**: add two `OracleRegistryEntry` objects to
   `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`'s `oracles[]`
   array (`kind: "third-party-library"`, `ecosystem: "python"`, `package: "networkx"` / `"shapely"`, real
   `version`, `engine: {family: "networkx", implementation: "...", version: "..."}` /
   `{family:"shapely", ...}` — two distinct families satisfies `distinctEngineFamilies` if ever raised
   above 1), and declare both in root `pyproject.toml`'s `test` group + `🔒️dependencies.json` (`kind:
   ["test-oracle"]`) the same way `ifcopenshell`/`mercantile`/`anastruct` already are. No `hostPath` is
   needed (python oracles here have never used one — confirmed `hostPath: null` on every python entry
   checked in §2/§3). The runner's `provisionPythonInterpreter` (§1) then auto-pip-installs both into its
   own cache venv the first time a puzzle-2d test runs — no manual `.venv`/`uv` step required, though
   pre-warming `uv pip install networkx shapely` into the repo's own `.venv` first (shapely is already
   there) is a reasonable local speedup since the runner's venv is created `--system-site-packages`.

**Why this satisfies DoD #2's spirit, not just its letter**: today's `verified-native-second-implementation`
promotion mechanically clears `missing-external-oracle` but is, by the D1 report's own words, "useful, and
explicitly NOT independent evidence" for the second-implementation half, and the whole-format third-party
survey is legitimately negative (confirmed twice, independently, by puzzle-2d's own rationale and jack's).
Scoping networkx/shapely to the two capabilities above is genuinely independent: neither library imports
anything from this repo, neither reads `.dsl.semio`, and each checks a property (topology after cascade;
geometry after transform) this repository's own Rust must also get right, on a **different data model**
(handle-as-node graph; shape-as-geometry) than the repo's own JSON tree — which is exactly the
"two second parties, different engine families" pattern `🔺️mesh`'s generator already proves out at scale.

---

## 5. Oracle-runner blockers observed, and how siblings got tests running

- **900s exhaustive-level budget** (`TEST_LEVEL_BUDGET_MS.exhaustive`, `🟦️.ts:1138`) is a real, named
  blocker: puzzle-2d's own oracle rationale text records that a `parity exhaustive --owner 🗒️note --case
  mutate-note-1` probe was **killed at this exact budget while still compiling the generated subject host**,
  then threw `spawnSync cargo ETIMEDOUT` out of `runProbe` with no summary line — root cause traced (one day
  earlier, per `📓️w14-final-audit.md §5.3`, cited verbatim in the oracle rationale) to an unresolved import
  `component::component_persistent_local` in `semio-framework-plugin` sitting in every generated host's
  dependency graph. This session's own status log independently confirms the generic pattern: shared
  cargo-target-dir lock contention from concurrent sessions is the recurring cause, not the oracle mechanism
  itself.
- **DRAW** (`26/09/05/DRAW-PLUGIN-END-TO-END`): no oracle-specific blocker found in its explore reports;
  its gaps were dispatch/classification and compile-drift, not oracle wiring.
- **RASTER** (`26/09/05/RASTER-PLUGIN-END-TO-END/📓️explore-raster-tests-oracles.md`): got its oracle case
  **actually green** — `.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-raster-artifacts-raster-standards-1-subsets-any-800d3c-🖨️mutate-raster-1-oracle-python/📤️results.jsonl`
  shows `{'passed': 37}` (12 mutate + 12 inverse + 12 spec-vector + 1 identity-round-trip), dated Sep 5
  06:40. Its own oracle is the same shape recommended here for puzzle's leftover field-setters: a
  hand-written Python second implementation, explicitly declining Pillow/`image`/`tiff` as category errors
  (they read pixel files; this format is a **layer tree** over an asset pool) — direct structural analogue
  to puzzle declining GraphML/DOT/GEXF as category errors for a handle-owned-edge board. Raster also
  surfaced a stale/orphaned cache entry (`🟠️mutate-raster-1`, pre-rename, no `results.jsonl`) as a hygiene
  note, not a correctness blocker — worth checking puzzle's own cache dir for the same pattern once its
  case is renamed/re-run (none exist yet, per §1).
- **BLOCK** (`26/09/05/BLOCK-PLUGIN-END-TO-END`): its explore reports are compile/dispatch/dev-boot focused
  (`explore-block{2,3,5}d-editor.md`, `explore-build-test-infra.md`); no dedicated oracle report — likely
  the block family's oracle status mirrors puzzle's own D1-promotion (`s.block.*` is in the same 53-entry
  promoted list, 104 mutations across 2d/3d/5d, per `📓️d1-native-oracle-discharge.md` §4's table).
- **ENERGY** (`26/09/06/ENERGY-PLUGIN-END-TO-END/📓️explore-oracle-toolchain.md`, 235 lines, and
  `📓️bestest-contract.md`): confirms the honeybee→OpenStudio→EnergyPlus chain is being scoped **for this
  sibling ticket specifically**, current versions researched (EnergyPlus 26.1.0, OpenStudio 3.11.0,
  honeybee-energy 1.123.32, all AGPL-3.0 for the Ladybug Tools layer, BSD-3-Clause for EnergyPlus/OpenStudio
  themselves), BESTEST/ASHRAE-140 case data sourced from `NREL/BESTEST-GSR` since EnergyPlus itself no
  longer ships test cases. This substantiates the puzzle-2d ticket's own DoD line declining that chain here
  — it is real, in-flight, plugin-specific infrastructure for a thermal-simulation domain with no
  node-and-port board analog.

## Key files referenced (absolute paths)

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/◻️mutate-puzzle-2d-1/🐍️.py` (26-kind vocabulary, lines 53/59/62/65/70/78/93-120/205-270)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json` (lines 5, 72-79, 89-150, 227-236, 705-740)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` (lines 66-70, 1201, 1284, 2767-2841, 2868-2900, 4828-4960)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` (lines 492-540, 668, 1001-1164)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` (lines 1128-1213)
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py` (lines 1-143)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🔮️oracle/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏗️generator/📜️script.ts`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracle/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🔮️oracle/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🔺️differential-ifc-4/🐍️.py`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/⚖️oracle/🔣️.json`
- `/Users/ueli/Documents/semio/pyproject.toml`
- `/Users/ueli/Documents/semio/🔒️dependencies.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️d1-native-oracle-discharge.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/📓️explore-raster-tests-oracles.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/📓️explore-oracle-toolchain.md`
- `.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-raster-artifacts-raster-standards-1-subsets-any-800d3c-🖨️mutate-raster-1-oracle-python/📤️results.jsonl`
