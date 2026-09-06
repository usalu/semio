# 🔮 Wave C1 — third-party oracles for `s.puzzle.2d@1`

Two new test cases, seven new oracle registrations, eight declared oracle host packages, both cases
**actually run** through the repo test module. Everything below was executed, not asserted; the raw
logs are in `🗑️generated/c1/`.

---

## 1. What was built

| Path | What it is |
|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🕸️third-party-puzzle-2d-1/🐍️.py` | Python third-party oracle adapter — networkx 3.6.1, shapely 2.1.2, jsonschema 4.26.0, jsonpatch 1.33 + deepdiff 9.1.0. 5 scenarios. |
| `…/🧪️tests/🕸️third-party-puzzle-2d-1/🥒️.feature` | Its feature contract. `@oracle-puzzle-2d-networkx-graph @comparison-ordered-json-v1 @capability-puzzle-2d-1-mutate`, all scenarios `@level-long`. |
| `…/🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts` | TypeScript second-ecosystem oracle adapter — graphology 0.26.0, jsonschema (npm) 1.5.0, fast-json-patch 3.1.1. 4 scenarios. |
| `…/🧪️tests/🌐️third-party-puzzle-2d-1/🥒️.feature` | Its feature contract. `@oracle-puzzle-2d-graphology-graph`. |
| `…/🔮️oracle/🔣️.json` | **APPEND ONLY**: 7 new `oracles[]` entries + a new `oracleHostPackages[]` array + a rewritten `_comment`. `mutationCatalogs`, `fixtureCoverage`, `mutationManifests`, `noOracleDecisions` untouched — those are A1's. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json` | `QualifyingOracleKind` gains `verified-native-second-implementation` (the TS `QUALIFYING_ORACLE_KINDS` already had it); `OracleRegistryEntry` gains the `nativeSecondImplementation` property and a new `NativeSecondImplementationEvidence` `$def`. |
| `pyproject.toml` | `test` group gains `networkx>=3.6.1`, `shapely>=2.1.2`, `jsonschema>=4.26.0`, `jsonpatch>=1.33` (deepdiff was already there). |
| `package.json` | devDependencies gain `graphology 0.26.0`, `jsonschema 1.5.0`, `fast-json-patch 3.1.1`. `bun install` run; `bun.lock` updated. |

No Rust adapter was added to either case — deliberately, see §6. `✏️s/🔌️plugins/🔒️policy-allowlist.json`
was **not** touched: it carries only a `semantic-vocabulary` key and nothing dependency-related.

### Registrations added to `oracles[]`

| id | ecosystem | package @ version | license | engine family |
|---|---|---|---|---|
| `puzzle-2d-networkx-graph` | python | networkx 3.6.1 | BSD-3-Clause | `networkx` |
| `puzzle-2d-shapely-geometry` | python | shapely 2.1.2 | BSD-3-Clause | `geos` |
| `puzzle-2d-jsonschema-payloads` | python | jsonschema 4.26.0 | MIT | `jsonschema-python` |
| `puzzle-2d-jsonpatch-diff` | python | jsonpatch 1.33 (+ linked deepdiff 9.1.0, MIT) | BSD-3-Clause | `jsonpatch-python` |
| `puzzle-2d-graphology-graph` | javascript | graphology 0.26.0 | MIT | `graphology` |
| `puzzle-2d-jsonschema-js-payloads` | javascript | jsonschema 1.5.0 | MIT | `jsonschema-js` |
| `puzzle-2d-fast-json-patch-diff` | javascript | fast-json-patch 3.1.1 | MIT | `fast-json-patch` |

All seven: `kind: "third-party-library"` (a QUALIFYING kind), `capabilities: ["puzzle-2d-1-mutate"]`,
`comparisonProfiles: ["ordered-json-v1"]`, `testOnly: true`, `productionReachable: false`,
`networkDuringExecution: false`. Seven distinct engine families now supply `puzzle-2d-1-mutate`
(eight counting the second implementation's `none`), so `distinctEngineFamilies` has headroom well
past 1. Every one of the 8 entries validates against `OracleRegistryEntry` (checked with
`jsonschema` `Draft202012Validator`: **0 errors**; before the schema amendment the existing
`puzzle-2d-python-independent` entry did **not** validate).

`ecosystem` is `"javascript"`, not `"typescript"` — the schema enum admits only the former and
`oracleDecision` maps `javascript → typescript` when it picks the adapter. (`s.stdio.semio` mesh's
`semio-mesh-typescript-three-independent` uses `"typescript"` and is therefore schema-invalid; not
mine to fix, listed in §7.)

---

## 2. What the oracles actually check — and why it is not a transliteration

**networkx / graphology (topology).** The board is rebuilt as a multi-directed graph whose vertices
are NODES *and* HANDLES, with `owns` edges (node → handle) and `wire` edges (handle → handle, keyed
by the board's edge id). This is a materially different data structure from the subject's (which
nests handles inside nodes and keeps edges in a sibling list): a node's handle list becomes graph
INCIDENCE, an edge's endpoints become adjacency between PORT vertices. The two cascades this
artifact is defined by are then never written out — they are what `Graph.remove_node` /
`Graph.dropNode` does to a vertex's incident edges — and `networkx.utils.graphs_equal` /
`Graph.export` decides agreement with the committed after-snapshot.

That directly **answers** the survey note this subset already carried ("GraphML, DOT and GEXF all
join node to node and none of them can express an edge whose endpoints are ports OWNED BY a node").
The objection is true of carrier FORMATS and false of graph ALGORITHM libraries: model the port as a
vertex and the two-level connectivity is ordinary incidence.

**shapely (geometry).** Real `Point.buffer` / `box` footprints at the node's own scale;
`move-node` = `affinity.translate`, `scale-node` = `affinity.scale(origin=…)` about the node's own
position, `change-node-anchor` = geometric invariance under `equals_exact`, `replace-node-geometry`
held to the area its own arguments imply (πr², or w·h) and to an unmoved centroid. Every other kind
must leave every footprint exactly where it was — which is also how a rejected/no-op vector is
checked.

**jsonschema ×2 (payload shape).** Every committed `🦠️mutation` (discriminator stripped) validated
against its own leaf `🧬️.schema.json`, plus a negative probe per leaf (an undeclared member must be
rejected), so an accepted payload proves the validator RAN.

**jsonpatch / fast-json-patch (diff), corroborated by deepdiff.** The committed `🔺️diff` is a typed
`Puzzle2dDiff`, not RFC 6902 — so an RFC 6902 patch is *derived* from `(before, after)`,
round-tripped through `apply` to reproduce the after-snapshot exactly, and the typed diff is then
held to it: the top-level members its op paths reach must equal the members the typed diff declares
non-null, and a record belongs in `patched` exactly when RFC 6902 needs an operation to turn its
before-shape into its after-shape. That last test is asked **per record**, not by reading indices off
the whole-document patch — removing a middle entry shifts every later index and a whole-document
patch then expresses a removal as field edits on records that never changed (this bit us on
`🗑️delete-node/🚫️deletes-the-tambour-and-severs-its-ten-edges` before the fix).

**Vectors are discovered, never listed.** The single declared fixture is
`asset://🧬️schema/🧬️mutations/🔣️.json`; every scenario directory under its parent is walked at run
time. A1 is adding vectors continuously and the sibling case's hand-written `Examples` table already
pointed at four directories that no longer exist. A table cannot go stale if there is no table.
Both adapters found **75 vectors** on the run recorded below (26 when this wave started).

**Stated non-coverage** (recorded in the registrations, not implied): the anchor ENUM, a handle's
`angle`, the ORDER of a node's handle list (graph incidence is unordered), the meaning of a kind
label, which of three refusal rules makes `replace-node-handle` a no-op, and an edge's routing record
beyond its `x`/`y` origin (`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt` are routing parameters, not
endpoints — no geometry engine can confirm them).

---

## 3. Runs — both cases really executed

Command (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`), which runs the **oracle phase only**
so no Rust subject host is compiled:

```
SEMIO_PYTHON=<repo>/.venv/bin/python3 bun ./📜️script.ts oracle long --owner 🧩️puzzle --case 🕸️third-party-puzzle-2d-1
bun ./📜️script.ts oracle long --owner 🧩️puzzle --case 🌐️third-party-puzzle-2d-1
```

### Python case — `4 passed / 1 failed`

`[test] level=long cases=1 executed=5 passed=4 failed=1 errored=0 parity=0/0`
Results: `.🧬semio/🦑️repo/⚡️cache/tests/results/test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-🕸️third-party-puzzle-2d-1-oracle-python/📤️results.jsonl`

| scenario | status | library checks | vectors | ms |
|---|---|---|---|---|
| graph-cascade | passed | 115 | 75 | 1 013 |
| kind-compatibility | passed | 150 | 75 | 116 |
| geometry-transforms | passed | 580 | 75 | 3 018 |
| payload-schemas | **failed** | — | 75 | 250 |
| diff-reproduction | passed | 454 | 75 | 9 817 |

**1 299 individual third-party assertions over 75 committed vectors.**

### TypeScript case — `3 passed / 1 errored`

`[test] level=long cases=1 executed=4 passed=3 failed=0 errored=1 parity=0/0`
Results: `…-00b55c-🌐️third-party-puzzle-2d-1-oracle-typescript/📤️results.jsonl`

| scenario | status | library checks | ms |
|---|---|---|---|
| graph-cascade | passed | 115 | 1 948 |
| kind-compatibility | passed | 150 | 22 |
| payload-schemas | **errored** | — | 45 |
| diff-reproduction | passed | 379 | 33 |

**644 assertions.** The TS host reports a thrown `Error` as `errored` where the Python host reports
an `AssertionError` as `failed` — same disagreement, different status word (§7).

The venv the runner builds is at
`.🧬semio/🦑️repo/⚡️cache/tests/hosts/python-env-<sha>`; it installed
`networkx 3.6.1, shapely 2.1.2, jsonschema 4.26.0, jsonpatch 1.33, deepdiff 9.1.0` unattended from
the `oracleHostPackages` declaration. No manual `.venv`/`uv` step was needed beyond pointing
`SEMIO_PYTHON` at a ≥3.11 interpreter — see the blocker in §5.

---

## 4. 🚨 FINDINGS FOR A1 / A2 / main — 24 leaf-schema rejections, found by BOTH ecosystems

`payload-schemas` fails in both cases with **the same 24 rejections**, across 5 mutation leaves.
Python `jsonschema` 4.26.0 and npm `jsonschema` 1.5.0 are independent implementations on independent
runtimes and they agree to the vector, so this is not a validator artefact. **These are not papered
over: the scenario stays red until the schemas are corrected.**

| mutation leaf | member | leaf schema says | committed payloads carry | vectors |
|---|---|---|---|---|
| `⚓change-node-anchor` | `newAnchor` | `enum ["Fixed","Derived"]` | `"fixed"` / `"derived"` | 3 |
| `🌱create-node` | `node/anchor` | `enum ["Fixed","Derived"]` | `"fixed"` | 3 |
| `🌱create-node` | `index` | `type: "integer"` | `null` | 3 |
| `➕add-node-handle` | `index` | `type: "integer"` | `null` | 3 |
| `🧊replace-node-geometry` | `newRadius` | `type: "number"` | `null` | 3 |
| `📚replace-kind-catalogs` | `newCatalogs` | `type: "object"` | `null` | 1 |
| `🪢️connect-handles` | `edgeKind` | `type: "string"` | `null` | 2 |
| `🪢️connect-handles` | `sourceTip` | `type: "string"` | `null` | 3 |
| `🪢️connect-handles` | `targetTip` | `type: "string"` | `null` | 3 |

Two distinct bugs, both in `🧬️schema/🧬️mutations/<leaf>/🧬️.schema.json`:

1. **`Option<T>` is not modelled.** Every optional payload member is typed as the bare inner type
   with no `null` admitted, while the committed payloads (and the Rust that writes them) carry
   explicit `null`. Fix: `"type": ["integer", "null"]` etc., or an `anyOf` with `{"type":"null"}`.
2. **The anchor enum carries Rust variant names.** `Fixed`/`Derived` (PascalCase) where every
   committed payload and every snapshot carries `fixed`/`derived`. The serde rename was not carried
   into the derived schema.

These leaf schemas are derivable by `bun ./📜️script.ts manifest payload-schema --write` in the test
module, so the fix probably belongs in the deriver rather than in 26 hand-edits — worth checking
before editing the JSON. **Routed to whoever owns `🧬️mutations/<leaf>/🧬️.schema.json`** (A1 owns the
fixture trees, A2 owns `🧬️mutations/🔣️.json`; the leaf schemas sit between them — main should
assign). Nothing else in either oracle disagrees with the committed fixtures.

Two things that *did* fail during development and turned out to be oracle bugs, now fixed, recorded
so nobody re-derives them: index-shifted RFC 6902 ops on a middle-of-collection removal (fixed by
per-record patches), and graphology's `mergeEdge` generating random edge keys (fixed by
`mergeEdgeWithKey` on the ordered pair).

---

## 5. 🚨 BLOCKER for main — the runner's default Python cannot provision these pins

Measured, not assumed. Running the python case **without** `SEMIO_PYTHON`
(`🗑️generated/c1/run-python-default-interp.txt`):

```
[test] not-exercised …/🧪️tests/🕸️third-party-puzzle-2d-1 (no implementation served the requested phase(s) oracle)
[test] level=long cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
[test] python oracle host: networkx==3.6.1 is neither importable nor installable into
       .🧬semio/🦑️repo/⚡️cache/tests/hosts/python-env-b9118e83… — ERROR: No matching distribution found
       (same for shapely==2.1.2, jsonschema==4.26.0, deepdiff==9.1.0; only jsonpatch==1.33 installed)
```

Cause: `provisionPythonInterpreter` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts:504`)
builds its cache venv from `process.env.SEMIO_PYTHON ?? "python3"`, and on this machine `python3` is
**3.9.6** while the repository's own `pyproject.toml` declares `requires-python = ">=3.14,<3.15"` and
`.venv` is 3.14.5. Every modern pin is correctly refused by pip for 3.9. The pins are right; the
base interpreter is wrong.

This is a pre-existing framework gap, not a puzzle one — it would bite any case declaring modern
python `oracleHostPackages`. Suggested zero-touch fix (one expression, in the test module, not in my
write region so I did not make it): prefer the repo's own interpreter before the PATH one —
`process.env.SEMIO_PYTHON ?? <repoRoot>/.venv/bin/python3 (…/Scripts/python.exe on win32) if it exists ?? "python3"`.
Until then, **any puzzle-2d oracle run needs `SEMIO_PYTHON=<repo>/.venv/bin/python3`**, and so does
the sibling `◻️mutate-puzzle-2d-1` from now on, because `oracleHostPackages` is declared per OWNER
and therefore now applies to every case under `…/🪆️subsets/✳️any`.

---

## 6. Dependency audit — `verify dependencies literal-external`

Run before and after (`🗑️generated/c1/deps-before.txt`, `deps-after2.txt`). The gate is
**red repo-wide by design** (`zero-target=0`, target 0 literal-external dependencies, current 188) —
that is the standing aspiration, not something this wave introduced. What matters is that my
additions are clean:

| | before | after | delta |
|---|---|---|---|
| js raw / literal-external | 75 / 72 | 78 / 75 | +3 (graphology, jsonschema, fast-json-patch) |
| js `test-oracle` | 5 | 8 | +3 — all three classified as test-oracle |
| python raw / literal-external | 30 / 30 | 34 / 34 | +4 (networkx, shapely, jsonschema, jsonpatch) |
| python `test-oracle` | 8 | 13 | +5 (the four, plus deepdiff promoted from test-runner) |
| python `production-reachable` | 4 | **0** | −4 (declaring shapely/jsonschema directly reclassified them off the production count) |
| **`oracle-conflicts`** | **23** | **23** | **0 — the conflict set is byte-identical** |
| toolchain-owner-conflicts / failures | 0 / 0 | 0 / 0 | 0 |

**One new `oracle-in-production` finding, recorded as `productionDebt` rather than hidden.**
Registering networkx makes the contract phase notice four pre-existing research scripts that import
it — `♻️mit-bestand/🔎️recherche/_neo4j/netz/netplate.py`, `…/netz/netz/mechanisms/connectivity.py`,
`…/review/2026-06-30_zukunftbau_vorstudie_snapshots/_analyze_deep.py`, `…/_report_snapshots.py`.
None is reachable from `s.puzzle.2d`, from any plugin, or from any shipped product, so none can make
this artifact's differential compare an implementation with itself, which is the reachability the
rule exists to stop. They are recorded in `puzzle-2d-networkx-graph.productionDebt` — the schema's
own shrink-only mechanism, which `oracleImportsInProduction` honours and which `dependency` prints
every run, so the debt stays visible. It closes either way round: move `🔎️recherche/_neo4j` behind
the contribution boundary, or add the research subtree to `pathExclusions` in `🔣️taxonomy.json` the
way `compose/` already is. **Neither is this subset's to decide — routed to main.** No other
registered package of mine is imported by any non-test source (grep over `✏️s`, `🧰️framework`,
`🌎️hub`, `♻️mit-bestand`).

**`ajv` was declined and this is why.** `ajv` is the obvious npm draft-07 validator, but it is
declared `production-runtime` by five packages here
(`➗️mathematical`, `💠️lowpoly`, `🌎️hub`, `os/🌉️mcp`, `repo/📚️library`), and
`dependencyClassifyOracleEntry` turns any oracle package with a production declaration into an
`oracle-conflict` — an oracle production can reach is this repository comparing itself with itself.
Registering it would have made conflict #24. The npm `jsonschema` 1.5.0 carries the identical
draft-07 role with no production reachability. The substitution and its reason are recorded in
`puzzle-2d-jsonschema-js-payloads`'s rationale and in the TS feature's description. **If main wants
`ajv` specifically, the prerequisite is removing it from production — not registering it anyway.**

Also verified: no production source imports any of the seven packages (grep over `✏️s`, `🧰️framework`,
`🌎️hub` excluding `🧪️tests/`), so no new `oracle-in-production` breach.

---

## 7. Honeybee → OpenStudio → EnergyPlus — recorded as declined, with the mechanism

Recorded in `puzzle-2d-networkx-graph`'s `rationale` rather than in `noOracleDecisions`, deliberately:
`NoOracleDecision.coversMutations` is structurally `false` and a runtime mutation can never be
discharged that way, so a no-oracle entry for `puzzle-2d-1-mutate` would be malformed — and the
capability now has seven real qualifying oracles anyway. `noOracleDecisions` stays `[]`.

The recorded argument is the mechanism, not domain intuition: Dragonfly's `Room2D.solve_adjacency`
establishes adjacency by testing which wall SEGMENTS of two floor-plate polygons geometrically
coincide. It therefore has (a) no typed, named, pre-existing port that a room OWNS, (b) no
compatibility RELATION gating which port kinds may join, and (c) no cascade in which an edge
remembers the port it was attached to and must be told to die — the three facts this vocabulary is
entirely about. Half of `Puzzle2dNode` (`root`, `iconKind`, `anchor`, `scale`, free `text`) has no
`Room2D` counterpart and `Room2D`'s storey/programme data has none here, so the chain would be used
for none of what it is good at. **The chain belongs to 🔋️energy** and is being wired for ticket
`26/09/06/ENERGY-PLUGIN-END-TO-END`.

---

## 8. Framework observations, routed to main (not fixed here)

1. **`mutationCatalogs` is not in the schema.** `OracleRegistry` in
   `🧰️framework/…/🧪️test/🧬️schema/🔣️.json` is `additionalProperties: false` and has no
   `mutationCatalogs` property, yet every contribution manifest in the repo carries one. Schema-first
   is violated repo-wide. (Distinct from the 158 `contribution-manifest-invalid` breaches, which come
   from a TS-level catalog/owner profile check.)
2. **33 pre-existing `OracleRegistryEntry` schema errors across other owners**, found while checking
   my own (all 266 registered entries validated): missing `comparisonProfiles` on
   `three-carrier-reader` / `manifold-mesh-measure` / `manifold3d-three` / three drawing readers;
   `productionDebt` carrying `reason`/`consequence` instead of `reachableFrom`/`plan`;
   `semio-mesh-typescript-three-independent` using `ecosystem: "typescript"`, which the enum does not
   admit. Puzzle-2d itself is now **0 errors**.
3. **`testCaseSlugPattern` contradicts `pathEmojiPolicy`.** The contract phase requires case
   directories to match `^[a-z0-9]+(?:-[a-z0-9]+)*$` while `pathEmojiPolicy.identity` requires a
   single-emoji-grapheme identity on every path. Every one of the 240 discovered cases breaches
   `case-slug`; my two new cases follow the repo's actual convention (emoji + kebab) and therefore
   add two more instances of a pre-existing, repo-wide taxonomy contradiction. One of the two rules
   has to give.
4. **A case cannot declare that an adapter serves only the oracle role.** `runSubjects` dispatches
   every claimed adapter whose language the owner ships a `📦️packages/<lang>` for, regardless of what
   the adapter registered, so in a full `run`/`parity` a TS oracle adapter under an owner that ships
   TypeScript produces `adapter has no subject registration` errors for every scenario. The
   `@implementation-<lang>` tag is parsed into `FeatureScenario.implementations` and then **never
   read** by anything — it looks like the intended mechanism and is dead. `🔺️mutate-semio-mesh`
   already has exactly this shape today, and my `🌐️third-party-puzzle-2d-1` now does too. The python
   case is unaffected (puzzle ships no `📦️packages/🐍️python`, so no python subject is dispatched).
5. **The five hosts disagree about failure vocabulary.** An assertion failure is `failed` from the
   Python host and `errored` from the TypeScript host, for the same kind of disagreement.

---

## 9. What is left / not done

- `payload-schemas` is red in both cases until §4 is fixed. That is the intended state.
- No Rust adapter was added to either case, on purpose: `materializeRustHost` links the owner's SUT
  crate as an optional dependency and enables it for the subject role, so any Rust adapter here would
  compile `semio-s-plugin-puzzle` — main owns cargo this session and the brief forbade it. The
  consequence is that these two cases carry no subject half; `◻️mutate-puzzle-2d-1` owns that.
- `parity` was not measured (`parity=0/0` in both runs) for the same reason.
- Neither case claims `@mutations-puzzle-2d-1-any`: their scenario ids are per-ROLE, not per-KIND, so
  claiming the catalog would demand 26 `mutate-<kind>` scenario ids that this design does not have.
  `◻️mutate-puzzle-2d-1` remains the case that claims exhaustive per-kind coverage.
- Scripts kept at the ticket root: `🐍️c1-drive-python-oracle.py`, `🟦️c1-drive-typescript-oracle.ts`
  (host-free drivers that run either adapter standalone in ~2 s — how the 24 findings were isolated),
  `🐍️c1-register-oracles.py`, `🐍️c1-register-oracles-second-ecosystem.py` (idempotent appliers of the
  registrations; both skip ids already present). Run logs in `🗑️generated/c1/`.
