"""🔮 Appends the puzzle-2d third-party oracle registrations, leaving every other member untouched."""
import json

PATH = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json"

NETWORKX_RATIONALE = (
    "NetworkX answers the TOPOLOGY of `s.puzzle.2d`, at "
    "`../🧪️tests/🕸️third-party-puzzle-2d-1/🐍️.py`. It is a genuine third party: it imports nothing from this "
    "repository, reads no `.dsl.semio`, and is handed a data structure this repository does not use. The board is "
    "rebuilt as a `MultiDiGraph` whose vertices are NODES and HANDLES, with `owns` edges (node → handle) and `wire` "
    "edges (handle → handle, keyed by the board's edge id), so a node's handle list becomes graph INCIDENCE rather "
    "than array membership and an edge's endpoints become adjacency between PORT vertices. The two cascades this "
    "subset is defined by — removing a handle severs the wire attached to it, deleting a node severs every wire on "
    "any of its handles — are then not restated by the adapter at all: they are what `Graph.remove_node` does to a "
    "vertex's incident edges, and `networkx.utils.graphs_equal` is what decides agreement with the committed "
    "after-snapshot. This ANSWERS rather than repeats the survey note this subset already carries — \"GraphML, DOT "
    "and GEXF all join node to node and none of them can express an edge whose endpoints are ports OWNED BY a "
    "node\" — which is true of carrier FORMATS and not of a graph ALGORITHM library, because a port modelled as a "
    "vertex is ordinary incidence. It also carries the kind-compatibility relation as a separate `DiGraph` over kind "
    "labels, where the library's one-edge-per-ordered-pair rule enforces the relation's uniqueness and "
    "`networkx.isolates` derives which labels survive a disconnection. WHAT IT DOES NOT ADJUDICATE, stated: the "
    "anchor enum, a handle's `angle`, the ORDER of a node's handle list (graph incidence is unordered), the meaning "
    "of a kind label, and which of three refusal rules makes a `replace-node-handle` vector a no-op. "
    "HONEYBEE → OPENSTUDIO → ENERGYPLUS was evaluated for this artifact and DECLINED, on the mechanism rather than "
    "on domain intuition: Dragonfly's `Room2D.solve_adjacency` establishes adjacency by testing which wall SEGMENTS "
    "of two floor-plate polygons geometrically coincide, so it has no typed, named, pre-existing port a room OWNS, "
    "no compatibility RELATION gating which port kinds may join, and no cascade in which an edge remembers the port "
    "it was attached to and must be told to die — the three facts this vocabulary is about. Half of `Puzzle2dNode` "
    "(`root`, `iconKind`, `anchor`, `scale`, free `text`) has no Room2D counterpart, and Room2D's storey and "
    "programme data has no counterpart here, so the chain would be used for none of what it is good at. That chain "
    "belongs to 🔋️energy and is being wired for ticket 26/09/06/ENERGY-PLUGIN-END-TO-END. Recorded here rather than "
    "as a `noOracleDecision` because a no-oracle decision cannot cover a runtime mutation at all, and this "
    "capability now has real qualifying oracles."
)

SHAPELY_RATIONALE = (
    "Shapely 2 (on GEOS) answers the GEOMETRY of `s.puzzle.2d`, in the same adapter. A node's footprint becomes a "
    "real `Point.buffer` or `box` at the node's own scale, and the geometry verbs become the library's own affine "
    "algebra: `move-node` is `affinity.translate`, `scale-node` is `affinity.scale(origin=…)` about the node's own "
    "position, `change-node-anchor` is a claim of geometric invariance under `equals_exact`, and "
    "`replace-node-geometry`'s null-dropping circle→rectangle rebuild is held to the area its own arguments imply "
    "(πr², or width × height) and to a centroid that did not move. Every other kind must leave every footprint on "
    "the board exactly where it was, which is how a rejected or no-op vector is checked. The extent MEMBERS decide "
    "which figure a node is (a radius is a circle, a width and a height are a rectangle) because `shape` is omitted "
    "when it holds its default. WHAT IT DOES NOT ADJUDICATE: the anchor ENUM (a board concept, not a geometric "
    "one), a handle's polar `angle` on its owner, and an edge's routing record beyond its `x`/`y` origin — "
    "`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt` are routing parameters, not two real endpoints, and this "
    "registration does not pretend a geometry engine can confirm them."
)

JSONSCHEMA_RATIONALE = (
    "jsonschema answers the PAYLOAD SHAPE. Every committed `🦠️mutation`, with its internally tagged discriminator "
    "removed, is validated against its own leaf `🧬️.schema.json` by the draft the schema itself names — all 26 "
    "leaves declare draft-07 — and every leaf schema is additionally handed a member it does not declare, so an "
    "accepted payload proves the validator RAN rather than that the schema was permissive. WHAT IT DOES NOT "
    "ADJUDICATE: effect. It knows that `remove-node-handle` takes two strings; it knows nothing about what removing "
    "a handle does to a board. FIRST RUN FINDING, recorded because it is the point of registering it: this "
    "validator, and the npm `jsonschema` in the second-ecosystem case independently, reject 24 committed payloads "
    "across 5 kinds — `Option<T>` payload members are typed as plain `integer`/`number`/`string`/`object` with no "
    "null admitted while the committed payloads carry explicit `null` (`index`, `newRadius`, `sourceTip`, "
    "`targetTip`, `edgeKind`, `newCatalogs`), and the `anchor` enum is spelled with the Rust variant names "
    "`Fixed`/`Derived` while every committed payload and snapshot carries `fixed`/`derived`. The leaf schemas are "
    "wrong about the payloads this repository actually writes."
)

JSONPATCH_RATIONALE = (
    "jsonpatch answers the DIFF, corroborated by deepdiff. The committed `🔺️diff` is a TYPED `Puzzle2dDiff` with "
    "per-collection `added`/`removed`/`patched`, not RFC 6902, so the oracle DERIVES an RFC 6902 patch from "
    "`(before, after)` with `jsonpatch.make_patch`, round-trips it through `apply` to reproduce the after-snapshot "
    "exactly — a reproduction proof, not a detection — and then holds the typed diff to it: the top-level members "
    "the op paths reach must be exactly the members the typed diff declares non-null, and a record belongs in "
    "`patched` exactly when RFC 6902 needs at least one operation to turn its before-shape into its after-shape. "
    "That last test is asked per RECORD rather than by reading indices off the whole-document patch, because "
    "removing an entry from the middle of a collection shifts every later index and a whole-document patch then "
    "expresses a removal as field edits on records that never changed. deepdiff, a structurally unrelated "
    "algorithm, must agree with jsonpatch about whether the document moved at all, and both must agree with the "
    "committed outcome's own `status`. WHAT IT DOES NOT ADJUDICATE: whether the change was the RIGHT one — only "
    "that the committed diff describes the committed snapshots."
)

GRAPHOLOGY_RATIONALE = (
    "The SECOND ECOSYSTEM. graphology answers the topology again, in bun, in the same bipartite node/handle "
    "encoding, with `Graph.dropNode` supplying the cascade and `Graph.export` — the library's own serializer — "
    "supplying the comparison; the npm `jsonschema` answers the payload shape and `fast-json-patch` answers the "
    "diff, both on the same terms as their Python counterparts. Two engine families on two runtimes agreeing is "
    "what makes a shared bug in one library visible: on the first run the two ecosystems agreed to the vector on "
    "the 24 leaf-schema disagreements recorded under `puzzle-2d-jsonschema-payloads`, which is why those findings "
    "are credible rather than an artefact of one validator. Adapter at "
    "`../🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts`. TWO SUBSTITUTIONS, both argued rather than convenient. `ajv` "
    "was the obvious npm schema validator and is DECLINED: it is declared `production-runtime` by five packages in "
    "this tree, and `verify dependencies literal-external` counts an oracle package production can reach as an "
    "`oracle-conflict` — an oracle production can reach is this repository comparing itself with itself. The npm "
    "`jsonschema` carries the same draft-07 role with no production reachability. GEOMETRY is deliberately not "
    "re-answered here: `@flatten-js/core` needs manual matrix composition for the anchor-relative scale shapely "
    "does in one call, so a second, weaker geometry check would add an engine to the count without adding "
    "evidence. ONE HONEST LIMITATION, recorded rather than hidden: graphology's core has no vertex RELABEL, so a "
    "`replace-node-handle` that re-identifies a port is expressed as drop-and-reattach; the Python half, whose "
    "library does have `relabel_nodes`, is what adjudicates that carry-over independently."
)

ENTRIES = [
    {
        "id": "puzzle-2d-networkx-graph",
        "kind": "third-party-library",
        "ecosystem": "python",
        "package": "networkx",
        "version": "3.6.1",
        "capabilities": ["puzzle-2d-1-mutate"],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "BSD-3-Clause",
        "testOnly": True,
        "engine": {"family": "networkx", "implementation": "NetworkX pure-Python graph algorithms", "version": "3.6.1"},
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://networkx.org/",
        "rationale": NETWORKX_RATIONALE,
    },
    {
        "id": "puzzle-2d-shapely-geometry",
        "kind": "third-party-library",
        "ecosystem": "python",
        "package": "shapely",
        "version": "2.1.2",
        "capabilities": ["puzzle-2d-1-mutate"],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "BSD-3-Clause",
        "testOnly": True,
        "engine": {"family": "geos", "implementation": "Shapely 2 over the GEOS geometry engine", "version": "2.1.2"},
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://shapely.readthedocs.io/",
        "rationale": SHAPELY_RATIONALE,
    },
    {
        "id": "puzzle-2d-jsonschema-payloads",
        "kind": "third-party-library",
        "ecosystem": "python",
        "package": "jsonschema",
        "version": "4.26.0",
        "capabilities": ["puzzle-2d-1-mutate"],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "MIT",
        "testOnly": True,
        "engine": {"family": "jsonschema-python", "implementation": "python-jsonschema draft-07 validator", "version": "4.26.0"},
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://python-jsonschema.readthedocs.io/",
        "rationale": JSONSCHEMA_RATIONALE,
    },
    {
        "id": "puzzle-2d-jsonpatch-diff",
        "kind": "third-party-library",
        "ecosystem": "python",
        "package": "jsonpatch",
        "version": "1.33",
        "packages": [{"package": "deepdiff", "version": "9.1.0", "license": "MIT", "role": "structurally unrelated second differ, corroborating whether the document moved at all", "homepage": "https://zepworks.com/deepdiff/current/"}],
        "capabilities": ["puzzle-2d-1-mutate"],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "BSD-3-Clause",
        "testOnly": True,
        "engine": {"family": "jsonpatch-python", "implementation": "python-json-patch RFC 6902 implementation", "version": "1.33"},
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://python-json-patch.readthedocs.io/",
        "rationale": JSONPATCH_RATIONALE,
    },
    {
        "id": "puzzle-2d-graphology-graph",
        "kind": "third-party-library",
        "ecosystem": "javascript",
        "package": "graphology",
        "version": "0.26.0",
        "packages": [
            {"package": "jsonschema", "version": "1.5.0", "license": "MIT", "role": "draft-07 payload validator for the second ecosystem, standing in for the production-reachable ajv", "homepage": "https://github.com/tdegrunt/jsonschema"},
            {"package": "fast-json-patch", "version": "3.1.1", "license": "MIT", "role": "RFC 6902 differ and patcher for the second ecosystem", "homepage": "https://github.com/Starcounter-Jack/JSON-Patch"},
        ],
        "capabilities": ["puzzle-2d-1-mutate"],
        "comparisonProfiles": ["ordered-json-v1"],
        "license": "MIT",
        "testOnly": True,
        "engine": {"family": "graphology", "implementation": "graphology multi-directed graph over the bun runtime", "version": "0.26.0"},
        "productionReachable": False,
        "networkDuringExecution": False,
        "homepage": "https://graphology.github.io/",
        "rationale": GRAPHOLOGY_RATIONALE,
    },
]

HOST_PACKAGES = [
    {"implementation": "python", "package": "networkx", "version": "3.6.1"},
    {"implementation": "python", "package": "shapely", "version": "2.1.2"},
    {"implementation": "python", "package": "jsonschema", "version": "4.26.0"},
    {"implementation": "python", "package": "jsonpatch", "version": "1.33"},
    {"implementation": "python", "package": "deepdiff", "version": "9.1.0"},
    {"implementation": "typescript", "package": "graphology", "version": "0.26.0"},
    {"implementation": "typescript", "package": "jsonschema", "version": "1.5.0"},
    {"implementation": "typescript", "package": "fast-json-patch", "version": "3.1.1"},
]

COMMENT = (
    "🧩️ This subset's own contribution. `Puzzle2dMutation` is a semio-NATIVE vocabulary carried in "
    "`.dsl.semio`/`.pack.semio`, so no third party reads the CARRIER — but five do read the FACTS the carrier "
    "states, and they are registered below: networkx and graphology for the handle-owned topology and its two "
    "cascades, shapely for node geometry, jsonschema and jsonpatch (corroborated by deepdiff, and mirrored in a "
    "second ecosystem by the npm jsonschema and fast-json-patch) for payload shape and diff reproduction. The "
    "in-repository Python second implementation stays registered alongside them and remains what it always was: "
    "useful evidence, not independent evidence."
)


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        document = json.load(handle)
    known = {entry["id"] for entry in document["oracles"]}
    document["_comment"] = COMMENT
    document["oracles"] = document["oracles"] + [entry for entry in ENTRIES if entry["id"] not in known]
    existing_hosts = {(entry["implementation"], entry["package"]) for entry in document.get("oracleHostPackages", [])}
    document["oracleHostPackages"] = document.get("oracleHostPackages", []) + [entry for entry in HOST_PACKAGES if (entry["implementation"], entry["package"]) not in existing_hosts]
    ordered = {}
    for key in ["$schema", "schemaVersion", "_comment", "oracles", "oracleHostPackages", "noOracleDecisions", "mutationCatalogs", "mutationManifests"]:
        if key in document:
            ordered[key] = document[key]
    for key, value in document.items():
        if key not in ordered:
            ordered[key] = value
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(ordered, ensure_ascii=False, indent=2) + "\n")
    print("oracles=%d hostPackages=%d" % (len(ordered["oracles"]), len(ordered["oracleHostPackages"])))


if __name__ == "__main__":
    main()
