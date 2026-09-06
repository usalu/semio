# 🔮 Third-party oracle candidates for `s.puzzle.2d@1`

Read-only research. All claims below were executed, not assumed: a throwaway `uv venv`
at `oracle-venv` (Python 3.12.13, provisioned by `uv` since system Python is 3.9.6) and
a throwaway `bun` project at `ts-oracle`, both under this session's scratchpad, not in
the repo. Every snippet quoted here was run with `oracle-venv/bin/python <file>.py` or
`bun run <file>.ts` and printed the `PASS:` lines shown. Nothing was installed into the
repo; no repo file besides this one was written; no cargo, no git.

## 1. The data model these oracles must respect

Read from `✏️s/…/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and the 26
per-mutation `🧬️.schema.json` files (all 26 declare
`"$schema": "http://json-schema.org/draft-07/schema#"` — confirmed by grep, **not**
2020-12, see §4):

- `Puzzle2dNode`: `id, x, y, anchor, handles[]` required; optional `nodeKind, shape,
  radius, width, height, text, iconKind, root, scale, visible, locked`. `handles` is an
  array of `{id, handleKind, angle}` **owned by the node** (nested, not a sibling
  collection).
- `Puzzle2dEdge`: `id, source, target, gap, shift, rise, rotation, turn, tilt, x, y`
  required; `source`/`target` are **handle ids** (`"handle-1"`, `"handle-2"`), not node
  ids — confirmed directly from a committed fixture
  (`➖remove-node-handle/🧪️tests/🚫️removes-handle-2-e261f3/📸️snapshot/⬅️before/🔣️.json`).
- `meta`: manifest id, `kindCompatibility` relation (`source, target, bidirectional,
  important, specificity`), optional kind catalogs.
- Cascades confirmed by the same fixture's committed `🔺️diff`: `remove-node-handle` on
  `node-b`'s only handle empties `node-b.handles` **and** removes `edge-1` from `edges`
  (the wire whose `target` was that handle) in the same mutation.

This is the structural fact every candidate below is scored against: **an edge's
endpoints are ports (handles) owned by a node**, not the node itself.

## 2. Existing oracle registration (what's already on record)

`✏️s/…/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json` already carries a Python
second-implementation oracle (`verified-native-second-implementation`), and states
verbatim: *"GraphML, DOT and GEXF all join node to node and none of them can express an
edge whose endpoints are ports OWNED BY a node."* It flags a qualifying **third-party**
reference (`third-party-library` / `third-party-cli` / `standards-reference-tool`) as
**"still owed"** — that gap is this ticket's job, and this file is the survey for it.

**Finding: the blanket claim is right about GraphML/GEXF and wrong about DOT.** See §5.

## 3. Python candidates (all installed via `uv pip install` into `oracle-venv`, 3.12.13)

| package | version | license | install |
|---|---|---|---|
| networkx | 3.6.1 | BSD-3-Clause | `uv pip install networkx` |
| shapely | 2.1.2 | BSD-3-Clause | `uv pip install shapely` |
| jsonschema | 4.26.0 | MIT | `uv pip install jsonschema` |
| jsondiff | 2.2.1 | MIT | `uv pip install jsondiff` |
| deepdiff | 9.1.0 | MIT | `uv pip install deepdiff` |
| jsonpatch | 1.33 | Modified BSD | `uv pip install jsonpatch` |
| pydot | 4.0.1 | MIT | `uv pip install pydot` |
| igraph (python-igraph) | 1.0.0 | **GPL-2.0+** | `uv pip install igraph` |

All resolved and installed in ~7s with no build failures on Python 3.12 (`uv venv
--python 3.12` transparently downloaded a standalone CPython 3.12.13 since the host's
system Python is 3.9.6 — `uv` provisions interpreters on demand, no manual pyenv step
needed). `igraph`'s GPL license is a real constraint if it were ever a *shipped*
runtime dependency (it is not — CLAUDE.md's "no runtime deps on external libraries"
rule doesn't reach test/oracle tooling — but GPL should still weigh against it versus
the equally-capable, more-permissively-licensed `networkx` for a repo that otherwise
keeps everything MIT/BSD).

### 3.1 `networkx` — graph/cascade oracle (recommended)

Bipartite model: two edge *relations* in one `MultiDiGraph` — `"owns"` (node→handle)
and `"wire"` (handle→handle, keyed by the puzzle edge id). This is not a
transliteration of the Rust: cascade-on-delete is networkx's own `remove_node`
semantics (it silently drops incident edges), so "does removing a handle sever its
wire" is answered by the library's own graph invariant, not by code we wrote to mimic
the subject.

```python
import networkx as nx
g = nx.MultiDiGraph()
g.add_edge("node-a", "node-a:handle-1", rel="owns")
g.add_edge("node-b", "node-b:handle-2", rel="owns")
g.add_edge("node-a:handle-1", "node-b:handle-2", key="edge-1", rel="wire")
g.remove_node("node-a:handle-1")           # remove-node-handle
assert not any(g.edges[e]["rel"] == "wire" for e in g.edges(keys=True))
print("PASS")                               # -> PASS (run, not assumed)
```//`oracle-venv/proof_networkx.py`, ran clean: both `remove-node-handle` and
`delete-node` cascades (the latter walking owned handles first, then letting
`remove_node` drop their wires) reproduce the committed fixture behavior.

**What it verifies independently:** `create/delete-node`, `add/remove-node-handle`
(and their edge cascades), `connect/disconnect-handles`, and — via
`nx.node_link_data` — an ordered-JSON projection usable as an independent structural
serialization to diff against the subject.
**What it cannot verify:** geometry (position/scale/anchor numbers), kind-compatibility
*rules* (it can hold the relation as edge data but has no concept of "compatible kind"
beyond what you encode), field-level payload shape (that's jsonschema's job).

### 3.2 `shapely` — geometry oracle

```python
from shapely.geometry import box
from shapely import affinity
node = box(8, 9, 12, 11)                    # x=10,y=10,w=4,h=2
moved = affinity.translate(node, xoff=5, yoff=-3)
assert (moved.centroid.x, moved.centroid.y) == (15, 7)   # move-node
scaled = affinity.scale(node, xfact=2, yfact=2, origin=(8, 9))  # scale-node about anchor
assert scaled.bounds[0:2] == (8.0, 9.0)      # anchor corner stays fixed
```
`oracle-venv/proof_shapely.py` ran clean for `move-node` (translate), `scale-node`
about an explicit anchor origin (`affinity.scale(origin=...)` — this is exactly the
"scale about anchor" primitive the mutation vocabulary needs), and an extent sanity
check for `replace-node-geometry`'s circle→rectangle rebuild.
**Cannot verify:** the anchor *enum* semantics (`fixed`/`derived`/etc — that's a board
concept, not a geometry one), which field gets dropped when an argument is `null`
(schema/fixture concern), handle angles (shapely has no polar/angle-on-owner concept).

### 3.3 `jsonschema` — payload validator

All 26 mutation schemas are draft-07 (verified: `find … -name '🧬️.schema.json' -exec
grep '"\$schema"'` → 26/26 identical `draft-07` line). `jsonschema.validate` autoselects
the right validator from `$schema`, so no manual draft override is needed in practice;
`Draft202012Validator` was also run directly against the same schema body and accepted
it (schema bodies here don't use draft-07-only keywords, so they're forward-compatible)
— `oracle-venv/proof_jsonschema.py`, both the positive fixture-payload case and a
negative `additionalProperties` rejection ran clean.
**Verifies:** payload shape per mutation kind (required fields, extra-field rejection).
**Cannot verify:** anything about *effect* — it doesn't know what `remove-node-handle`
does to a snapshot, only that its arguments are two strings.

### 3.4 Diff oracles — three independent libraries against the same fixture

Using the committed `remove-node-handle` fixture's real `before.json`/`after.json`
(copied verbatim, not re-derived):

```python
from deepdiff import DeepDiff
import jsondiff, jsonpatch
DeepDiff(before, after)                      # -> iterable_item_removed: edge-1, handle
jsondiff.diff(before, after, syntax="symmetric")   # -> touches nodes, edges
patch = jsonpatch.make_patch(before, after)
assert patch.apply(before) == after          # round-trip, exact byte match
```
`oracle-venv/proof_diff.py` ran clean on all three: `DeepDiff` (MIT) found the removed
handle and the removed `edge-1` as `iterable_item_removed`; `jsondiff` (MIT, different
algorithm/library) independently flagged the same two top-level keys; `jsonpatch`
(BSD) computed a 2-op RFC6902 patch (`remove /edges/0`, `remove /nodes/1/handles/0`)
and **round-tripped** `before` through it to reproduce `after` exactly — this is the
strongest of the three because it's a reproduction proof, not just a detection.
**Recommended primary:** `jsonpatch` for the round-trip proof against every committed
`🔺️diff`; `deepdiff`/`jsondiff` as secondary corroboration since they're structurally
independent implementations of "what changed."

### 3.5 `pydot` / DOT — the existing oracle note is wrong about DOT specifically

DOT has first-class port syntax: `record`-shaped (or HTML-like) node labels declare
named ports, and edges address them as `"node":"port"` or via `headport=`/`tailport=`
attributes. This is *exactly* "an edge endpoint that is a port owned by a node" — the
same shape as `edge.source = "handle-1"` where `handle-1` lives inside `node-a`.

```python
import pydot
g = pydot.Dot("puzzle2d", graph_type="digraph")
g.add_node(pydot.Node("node-a", shape="record", label="{ <handle-1> handle-1 }"))
g.add_node(pydot.Node("node-b", shape="record", label="{ <handle-2> handle-2 }"))
g.add_edge(pydot.Edge("node-a:handle-1", "node-b:handle-2"))
print(g.to_string())
# digraph puzzle2d {
# "node-a" [shape=record, label="{ <handle-1> handle-1 }"];
# "node-b" [shape=record, label="{ <handle-2> handle-2 }"];
# "node-a":"handle-1" -> "node-b":"handle-2";
# }
```
`oracle-venv/proof_pydot.py` ran clean: record ports, the `headport`/`tailport`
attribute spelling, and a round-trip parse (`pydot.graph_from_dot_data`) that recovers
the port-qualified edge endpoint — pydot is not write-only here.

**Caveat that keeps the existing decision's conclusion correct, if not its reasoning:**
`dot`/`neato`/`gvpr` are **not installed on this host** (`which dot` → nothing; no
Homebrew keg). Even if installed, DOT/Graphviz is a text format plus a *layout
renderer* — it has no algorithmic API for cascade-delete, kind-compatibility, or
mutation semantics the way `networkx` does. So: DOT/pydot is a genuine, better-than-
claimed way to *express* the shape (useful as a standards-reference interchange check
or a visual QA render target for the playground), but it is not a semantic oracle
substitute for `networkx`. Recommend: correct the oracle note's over-broad claim about
DOT, but do not adopt DOT/Graphviz as the primary graph oracle — keep `networkx`.
GraphML and GEXF remain correctly declined: both are plain XML graph-interchange
formats with no port primitive at all.

### 3.6 `igraph` — redundant with networkx, not recommended

`oracle-venv/proof_igraph.py` ran clean (vertex deletion cascades incident edges,
same result as networkx). Offers nothing networkx lacks for this vocabulary, is
GPL-licensed (vs. networkx's BSD-3), and is a heavier C-extension install. Listed for
completeness, not recommended.

## 4. TypeScript/bun candidates (installed via `bun add` into `ts-oracle`)

| package | version | license |
|---|---|---|
| graphology | 0.26.0 | MIT |
| graphology-operators | 1.6.1 | MIT |
| graphology-utils | 2.5.2 | MIT |
| @dagrejs/graphlib | 4.0.5 | MIT |
| ajv | 8.20.0 | MIT |
| fast-json-patch | 3.1.1 | MIT |
| @flatten-js/core | 1.6.14 | MIT |

All installed cleanly (`bun add …`, ~1.6s, 39 resolved packages, no native builds).

### 4.1 `graphology` — recommended TS graph oracle

```ts
import Graph from "graphology";
const g = new Graph({ type: "directed", multi: true });
g.addDirectedEdge("node-a", "node-a:handle-1", { rel: "owns" });
g.addDirectedEdge("node-b", "node-b:handle-2", { rel: "owns" });
g.addDirectedEdgeWithKey("edge-1", "node-a:handle-1", "node-b:handle-2", { rel: "wire" });
g.dropNode("node-a:handle-1");               // remove-node-handle
if (g.hasEdge("edge-1")) throw new Error();  // graphology's OWN invariant, not our code
```
`ts-oracle/proof_graphology.ts` ran clean for both `remove-node-handle` and
`delete-node` cascades — same bipartite owns/wire model as the networkx proof, same
result, independent library/runtime (JS engine vs CPython), independent implementation
of "no dangling edges."

### 4.2 `@dagrejs/graphlib` — compound graphs, one real finding

Compound graphs (`compound: true`) let a handle be a literal **child** of its node via
`setParent`, which is structurally closer to "handle lives inside node" than an "owns"
edge. `removeNode` on a child does cascade its incident edges (same as graphology).
**But**, verified directly: removing the **parent** node does *not* auto-remove its
children — `ts-oracle/proof_graphlib.ts` printed `FINDING: … leaves child node present
= true`. This is a real, run-verified asymmetry between the two TS libraries: whoever
picks graphlib's compound-node encoding for "handle owned by node" must still hand-walk
`delete-node`'s cascade exactly as the Rust does (children first, then edges), same as
this repo already must; graphology's plain "owns" edge + `dropNode` gets `delete-node`
almost for free by walking `outNeighbors` once. Prefer graphology for that reason.

### 4.3 `ajv` — 2020-12 payload validator

```ts
import Ajv2020 from "ajv/dist/2020";
const ajv = new Ajv2020({ allErrors: true });
const validate = ajv.compile({ $schema: "https://json-schema.org/draft/2020-12/schema", ...schema });
```
`ts-oracle/proof_ajv_patch_flatten.ts` ran clean: accepts the valid fixture payload,
rejects an `additionalProperties` violation — same result as Python `jsonschema`,
independent implementation/runtime. (Repo schemas are draft-07, per §3.3; Ajv2020 was
run against an equivalent 2020-12 schema body per the original ask, to prove the TS
ecosystem's 2020-12 capability specifically — for the actual repo files, `ajv` in its
default draft-07 mode, or `Draft7Validator` in Python, is what to point at
`🧬️.schema.json` unmodified.)

### 4.4 `fast-json-patch` — third independent diff oracle

Computed a 2-op RFC6902 patch from the same `before.json`/`after.json` fixture and
round-tripped it to reproduce `after.json` exactly — same two ops Python's `jsonpatch`
found (`remove /edges/0`, `remove /nodes/1/handles/0`), computed by a wholly separate
library/runtime. Good second-ecosystem corroboration for the diff-oracle requirement.

### 4.5 `@flatten-js/core` — TS geometry oracle

`Polygon.translate(new Vector(dx, dy))` reproduced the same `move-node` bbox
translation as shapely's `affinity.translate`. API note found by running it: `Vector`
must be constructed explicitly (`translate(5, -3)` throws `ILLEGAL_PARAMETERS`;
`translate(new Vector(5, -3))` works) — recorded here so nobody re-discovers it the
hard way. Adequate as a lightweight second-ecosystem geometry check; shapely remains
the stronger primary (richer affine API, `origin=` for anchor-relative scale is a
one-line call there vs. manual matrix composition in flatten-js for the same op).

## 5. CLI / standards-reference tools

- **Graphviz (`dot`/`neato`/`gvpr`):** not installed (`which dot/neato/gvpr` → none;
  no Homebrew keg present). Not installed as part of this read-only pass (would touch
  the system, out of scope). DOT-the-*language*'s port capability was verified via
  `pydot` alone (§3.5) without needing the binary — `pydot` builds/parses DOT text
  in-process. If the actual `dot` binary is wanted later (e.g. to literally render the
  puzzle board for visual QA of the playground), `brew install graphviz` is the
  install path; it adds no runtime dependency to any shipped crate/package, it would be
  a dev-only visualization aid.
- **`jq`:** already present (`/usr/bin/jq`). Useful for canonical key-ordering /
  structural diffing of committed fixture JSON at the shell level, but it's a text/JSON
  tool with no graph or geometry semantics — not a candidate oracle for mutation
  *effects*, only a formatting/CI-lint aid.

## 6. Honeybee → OpenStudio → EnergyPlus — evaluated and declined

Checked directly (web search against `ladybug-tools` docs, not from memory alone):
`dragonfly.room2d` module, `Room2D.solve_adjacency` / `Room2D.intersect_adjacency`.
Confirmed behavior: Dragonfly's `Room2D` represents a building room as a closed 2D
floor-plate polygon (via `ladybug_geometry`); adjacency between two `Room2D`s is
established by **finding wall *segments* that geometrically coincide** between two
room footprints (an intersection/coincidence test on `LineSegment2D`s) and then setting
a `Surface` (or `Adiabatic`) boundary condition on each matching segment. `Building`
aggregates `Story` objects of `Room2D`s and can check for storey collisions; ultimately
everything is translated to Honeybee `Model`/`Face`/`Room` objects and handed to
OpenStudio → EnergyPlus for thermal/energy simulation.

Reading the puzzle2d board onto this chain (rooms-as-nodes, adjacencies-as-edges) is
**forced, not genuine**, for reasons specific and checkable, not just "it's for a
different domain":

1. **No ported handles.** A puzzle2d edge names two pre-existing, independently
   *typed* handle ids (`handleKind`, drawn from a kind catalog, with an explicit
   `kindCompatibility` relation gating which kinds may connect). Dragonfly adjacency
   has no analogous entity: wall segments aren't named or typed sub-objects a room
   "owns" ahead of time — they're geometric edges of the polygon, and "compatibility"
   is purely "do these two segments coincide in space," not a lookup against a
   typed-kind relation.
2. **No handle-owned cascade to model.** Puzzle2d's defining cascade — deleting a
   node's handle severs the edge that named it; deleting a node severs every edge on
   any of its handles — has no counterpart in `solve_adjacency`: removing a `Room2D`
   just means its segments no longer participate in the next coincidence pass; there
   is no "the edge remembers which port it was attached to and must be told to die"
   relation the way this vocabulary requires.
3. **Domain mismatch on node fields.** Half of `Puzzle2dNode`'s vocabulary
   (`root`, `iconKind`, `anchor`, `scale`, arbitrary `text`) is diagram/UI state with
   no Room2D counterpart, and Room2D in turn carries story/floor-height/program data
   the puzzle board has no field for. Reusing the library would mean using it for
   *none* of what it's actually good at (thermal simulation) and forcing its polygon
   type to stand in for a diagramming node it wasn't designed to be.
4. **This matches what's already recorded.** Both `.../◻️2d/🔮️oracle/🔣️.json` (this
   subset) and the analogous 3D/5D registrations independently reached the same
   conclusion about neighboring geometry standards (glTF/USD/IFC/STEP AP214): each
   carries an element placed in *one* space, none carries "a port owned by a node,
   addressed by a typed compatibility relation, with node-then-edge cascade." Honeybee/
   Dragonfly/EnergyPlus is the same shape of mismatch one layer further from geometry
   (thermal simulation, not even a generic scene graph) — if the geometry standards
   were declined for missing ported-handle semantics, the energy-simulation chain
   built on top of an even coarser room-adjacency abstraction cannot supply it either.

**Recommendation: keep it declined**, exactly as `📓️status.md` already records
("Honeybee → OpenStudio → EnergyPlus is not a fitting instrument for a 2D
node-and-port puzzle board … it belongs to the 🔋️energy ticket"). This survey confirms
that by checking the actual `solve_adjacency` mechanism rather than repeating the
domain-mismatch intuition; do not spend implementation effort trying to force this
mapping.

## 7. Recommended minimal qualifying set

| role | pick | why |
|---|---|---|
| graph/cascade oracle (Python) | **networkx** 3.6.1 (BSD-3) | bipartite owns/wire `MultiDiGraph`; `remove_node`'s own no-dangling-edge behavior *is* the cascade check, not code we wrote to mimic the subject |
| geometry oracle (Python) | **shapely** 2.1.2 (BSD-3) | `affinity.translate`/`affinity.scale(origin=…)` map directly onto `move-node`/`scale-node`-about-anchor; richer affine API than the TS alternative |
| schema validator (Python) | **jsonschema** 4.26.0 (MIT) | validates the repo's actual draft-07 `🧬️.schema.json` files with no override needed; `Draft202012Validator` confirmed usable if the schemas are ever migrated |
| diff oracle (Python) | **jsonpatch** 1.33 (BSD), corroborated by **deepdiff**/**jsondiff** | round-trips `before`→`after` exactly against every committed `🔺️diff`, the strongest form of "confirms the diff" (reproduction, not just detection) |
| second ecosystem (TS/bun) | **graphology** 0.26.0 (MIT) + **ajv** 8.20.0 (MIT) + **fast-json-patch** 3.1.1 (MIT) | same three roles (graph, schema, diff) reproduced in a second language/runtime; `graphology`'s edge-drop invariant is a cleaner cascade check than `@dagrejs/graphlib` compound graphs, which do **not** auto-cascade parent→children (verified) |
| declined | igraph (GPL, redundant), Graphviz/DOT as *semantic* oracle (no algorithmic API — fine only as a standards-reference/visual-QA aid), Honeybee/OpenStudio/EnergyPlus (§6) | — |

### Comparison profile by capability

| capability | networkx | shapely | jsonschema | jsonpatch/deepdiff/jsondiff | graphology | ajv | fast-json-patch |
|---|---|---|---|---|---|---|---|
| node/handle-owns structure | yes (owns edges) | no | no | no | yes | no | no |
| edge = handle↔handle wire | yes | no | no | no | yes | no | no |
| remove-node-handle cascade | yes (`remove_node`) | no | no | detects it post-hoc | yes (`dropNode` invariant) | no | detects it post-hoc |
| delete-node cascade | yes (manual walk + `remove_node`) | no | no | detects it post-hoc | yes (manual walk + `dropNode`) | no | detects it post-hoc |
| position/scale/anchor geometry | no | yes | no | no | no | no | no |
| kind-compatibility relation | as edge/attr data only | no | no | no | as edge/attr data only | no | no |
| payload shape validation | no | no | yes | no | no | yes | no |
| before→after reproduction | no | no | no | yes (jsonpatch apply) | no | no | yes (apply) |

### How to represent handle-owned edges without transliterating the Rust

The Rust subject stores handles nested inside `Puzzle2dNode.handles[]` and edges as a
sibling `Puzzle2dSnapshot.edges[]` list addressing handle ids by string. An oracle that
just re-nests the same two arrays and re-implements "look up the node whose handles
contain this id, then filter its edges" **would** be a transliteration. The two-relation
graph encoding used in every proof above avoids that because it forces a different
representation of the *same fact*:

- `"owns"` edges (node → handle) turn "a node's handle list" into **graph incidence**
  (`g.out_edges(nodeId)`) instead of an array-membership scan — a structurally
  different data structure computing the same fact.
- `"wire"` edges (handle → handle) turn "an edge's source/target" into **graph
  adjacency** between handle vertices, so cascade becomes "what happens to a vertex's
  incident edges when the vertex is deleted" — a question the graph library answers
  with its *own*, independently-implemented deletion semantics (`networkx.remove_node`,
  `graphology.dropNode`), not with code copied from `s.puzzle.2d`'s Rust cascade
  handler.
- Kind-compatibility is stored as edge/vertex *attributes* on the same graph (not a
  parallel Rust-shaped `Vec<CompatibilityRecord>`), so `connect-handles` validation
  becomes an attribute lookup on the graph rather than a linear scan over a cloned
  Rust-shaped list.
- Geometry (`shapely`/`flatten-js`) is kept **entirely separate** from the graph
  representation — a `Polygon`/`box` per node, addressed by node id, with no handle or
  edge concept at all — so geometry mutations are checked with an affine-transform
  library's own math, never by re-deriving the subject's stored `x/y/width/height`
  formulas.

## 8. Files produced (scratchpad only, not in repo)

- `oracle-venv/` — Python 3.12.13 venv + `proof_networkx.py`, `proof_shapely.py`,
  `proof_jsonschema.py`, `proof_diff.py`, `proof_pydot.py`, `proof_igraph.py`,
  `before.json`/`after.json`/`mutation.json` (copied verbatim from the committed
  `➖remove-node-handle/🧪️tests/🚫️removes-handle-2-e261f3` fixture).
- `ts-oracle/` — bun project + `proof_graphology.ts`, `proof_graphlib.ts`,
  `proof_ajv_patch_flatten.ts`.

All under this session's scratchpad
(`/private/tmp/claude-501/-Users-ueli-Documents-semio/c1117632-c36f-478e-b3f6-2b268c8a2b95/scratchpad`),
outside the repo; nothing here was written to the repo except this report file.
