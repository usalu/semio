import json

# Verified crate dirs (from `find <fam> -name Cargo.toml`), family -> list of (oldDir relative suffix after family root)
families = {
    "dsl": ["rs","core/rs","derive/rs","editor/rs","schema/rs","token/rs"],
    "pack": ["rs","async/rs","cli/rs","core/rs","format/rs","http/rs","index/rs","io/rs","testkit/rs","value/rs"],
    "protocol": ["rs","causal/rs","cli/rs","command/rs","core/rs","crdt/rs","format/rs","history/rs","io/rs","materialize/rs","testkit/rs","wire/rs"],
    "store": ["rs","sync/rs","worker/rs"],
    "vcs": ["rs"],
    "db": ["rs","actor/rs","cli/rs","cluster/rs","compact/rs","conflict/rs","core/rs","document/rs","engine/rs","index/rs","observe/rs","preview/rs","projection/rs","query/rs","security/rs","snapshot/rs","state/rs","storage/rs","storage/neo4j/rs","storage/postgres/rs","storage/sqlite/rs","sync/rs","testkit/rs","wal/rs"],
    "neural": ["dag/rs","engine/rs"],
    "playbook": ["rs"],
}

moves = []
for fam, suffixes in families.items():
    for suf in suffixes:
        oldDir = f"{fam}/{suf}" if suf != "rs" else f"{fam}/rs"
        newDir = f"s/kernel/{fam}/{suf}" if suf != "rs" else f"s/kernel/{fam}/rs"
        tail = newDir[len("s/kernel/"):]
        tail = tail[:-len("/rs")] if tail.endswith("/rs") else tail
        pkg_suffix = tail.replace("/", "-")
        newPkg = f"semio-s-kernel-{pkg_suffix}"
        moves.append({"oldDir": oldDir, "newDir": newDir, "newPkg": newPkg})

# kernel -> s/kernel/geometry (rename)
geometry_suffixes = ["2d/rs","2d/engine/rs","3d/scene/rs","3d/mesh/rs","3d/spatial/rs","3d/brep/rs","3d/brep/engine/rs"]
for suf in geometry_suffixes:
    oldDir = f"kernel/{suf}"
    tail = suf[:-len("/rs")] if suf.endswith("/rs") else suf
    newDir = f"s/kernel/geometry/{tail}/rs"
    pkg_suffix = tail.replace("/", "-")
    newPkg = f"semio-s-kernel-geometry-{pkg_suffix}"
    moves.append({"oldDir": oldDir, "newDir": newDir, "newPkg": newPkg})

# mathematical (all except program)
math_suffixes = ["algebra/rs","cas/rs","causal/rs","entropy/rs","fuzzy/rs","geometry/rs","lie/rs","number/rs","optimize/rs",
    "polynomial/rs","probability/rs","random/rs","sampling/rs","signal/rs","spatial/rs","statistics/rs","tabular/rs","wfc/rs",
    "graph/rs","graph/approximation/rs","graph/bipartite/rs","graph/centrality/rs","graph/cliques/rs","graph/clustering/rs",
    "graph/coloring/rs","graph/community/rs","graph/components/rs","graph/connectivity/rs","graph/cycles/rs","graph/dag/rs",
    "graph/distance/rs","graph/drawing/rs","graph/dsl/rs","graph/flow/rs","graph/generate/rs","graph/generate/random/rs",
    "graph/io/rs","graph/isomorphism/rs","graph/manifest/rs","graph/matching/rs","graph/normal/directed/rs",
    "graph/normal/undirected/rs","graph/operators/rs","graph/paths/rs","graph/planarity/rs","graph/port/directed/normal/rs",
    "graph/port/undirected/rs","graph/similarity/rs","graph/spectral/rs","graph/structure/rs","graph/traversal/rs","graph/trees/rs"]
for suf in math_suffixes:
    oldDir = f"mathematical/{suf}"
    newDir = f"s/kernel/mathematical/{suf}"
    tail = suf[:-len("/rs")] if suf.endswith("/rs") else suf
    pkg_suffix = tail.replace("/", "-")
    newPkg = f"semio-s-kernel-mathematical-{pkg_suffix}"
    moves.append({"oldDir": oldDir, "newDir": newDir, "newPkg": newPkg})

# infinite (except dag/program); canvas -> canvas rename
infinite_suffixes = ["board/normal/directed/rs","board/normal/undirected/rs","board/port/directed/dag/rs",
    "board/port/directed/normal/rs","board/port/directed/rs","board/port/rs","board/port/undirected/rs","board/rs",
    "world/rs"]
for suf in infinite_suffixes:
    oldDir = f"infinite/{suf}"
    newDir = f"s/kernel/infinite/{suf}"
    tail = suf[:-len("/rs")] if suf.endswith("/rs") else suf
    pkg_suffix = tail.replace("/", "-")
    newPkg = f"semio-s-kernel-infinite-{pkg_suffix}"
    moves.append({"oldDir": oldDir, "newDir": newDir, "newPkg": newPkg})
# canvas -> canvas rename (separate, path segment differs old vs new)
moves.append({"oldDir": "infinite/canvas/rs", "newDir": "s/kernel/infinite/canvas/rs", "newPkg": "semio-s-kernel-infinite-canvas"})

# flow: core + all modules (NOT program)
flow_suffixes = ["core/rs","module/bim/rs","module/brep/rs","module/core/rs","module/dictionary/rs","module/draw/rs",
    "module/list/rs","module/logic/rs","module/math/rs","module/text/rs","module/wasm/rs"]
for suf in flow_suffixes:
    oldDir = f"flow/{suf}"
    newDir = f"s/kernel/flow/{suf}"
    tail = suf[:-len("/rs")] if suf.endswith("/rs") else suf
    pkg_suffix = tail.replace("/", "-")
    newPkg = f"semio-s-kernel-flow-{pkg_suffix}"
    moves.append({"oldDir": oldDir, "newDir": newDir, "newPkg": newPkg})

print(json.dumps(moves, indent=2))
print(f"TOTAL={len(moves)}", file=__import__("sys").stderr)
