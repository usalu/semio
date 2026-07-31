#!/usr/bin/env python3
"""Build the emoji+name vocabulary and rename map for the second migration pass
(every directory becomes <emoji><name>, entry files become 📦<name>.<ext>).
Excludes: node_modules/target/.git/.repo/.nx/pkg/generated, ♻️/recherche content,
präsentation slide-name folders, Next.js's mandated "app" dir, and build/vendor
artifacts (partial_movie_files, _vendor, */interfaces)."""
import os
import json

ROOTS = ["🧰", "✏️", "🌎", "♻️"]
SKIP_DIRNAMES = {"node_modules", "target", ".git", ".repo", ".nx", "pkg", ".DS_Store"}

# Path-prefix exclusions: anything starting with one of these repo-relative prefixes
# keeps its exact current name (not renamed), and is not descended into for renaming
# purposes beyond this point (its OWN name may still be renamed if it doesn't match
# one of these prefixes itself -- these are prefixes of *contents*, checked per-dir).
EXCLUDE_PATH_PREFIXES = [
    "♻️/recherche/",
    "♻️/präsentation/33.projektetage/⚡️/🟦/slide/",
    "🧰/🛍️/🦑/🔨/server/coordinator/⚡️/🟦/js/app",  # Next.js mandated dir name itself
    "🧰/🔨/ui/⚡️/🔷/styling/Elements.Styling",  # .NET project/assembly-name-matched folder tree
]
# Names anywhere in the tree whose entire subtree is excluded (generated/vendor/cache)
EXCLUDE_SUBTREE_NAMES = {"partial_movie_files", "_vendor", "interfaces", "osm-tiles"}

# Exact dirnames that are Cargo/tooling-mandated convention names (like Next.js's "app") and
# must NEVER be renamed regardless of the vocab entry below -- Cargo auto-discovers
# `benches/*.rs` as [[bench]] targets purely by this literal directory name; renaming it
# breaks the bench target with no code-level "fix the lookup" available (found the hard way:
# 3 crates' benches/ got renamed then had to be reverted after cargo check failed).
NEVER_RENAME_EXACT = {"benches", "tests", "examples", "src"}

# Bare-emoji directories left over from the FIRST migration pass (emoji-only, no word) --
# these get the word appended: bare emoji char -> new full name (with the SAME emoji).
BARE_EMOJI_WORDS = {
    "⚡️": "⚡️implementation", "🔨": "🔨module", "🛍️": "🛍️product",
    "🛂": "🛂manifest", "🗿": "🗿artifact", "🎛️": "🎛️app",
    "🦑": "🦑repo", "📓": "📓print", "💻": "💻os",
    "🦀": "🦀rust", "🟦": "🟦typescript", "🐍": "🐍python", "🐹": "🐹go",
    "🔷": "🔷dotnet", "🖋️": "🖋️latex", "🧩": "🧩extension",
    "🧑‍🎨": "🧑‍🎨engine", "🔌": "🔌plugin",
    "⚛️": "⚛️react", "🧊": "🧊wgpu",
    "🧰": "🧰framework", "✏️": "✏️s", "🌎": "🌎hub", "♻️": "♻️mit-bestand",
}

# Word-part overrides for stray PLAIN-named language dirs not yet flattened into a bare
# emoji dir (e.g. a leftover "js" folder) -- rename to the SAME full word as BARE_EMOJI_WORDS
# would use, for consistency, rather than keeping the abbreviation.
WORD_OVERRIDE = {
    "rs": "rust", "js": "typescript", "py": "python", "net": "dotnet",
    "rb": "ruby", "tex": "latex",
}

# #region 🔤Vocabulary
VOCAB = {
    # structural
    "framework": "🧰", "implementation": "⚡️", "module": "🔨", "product": "🛍️",
    "os": "💻", "print": "📓", "repo": "🦑", "s": "✏️", "plugin": "🔌",
    "manifest": "🛂", "artifact": "🗿", "extension": "🧩", "app": "🎛️",
    "renderer": "📺", "engine": "⚙️", "hub": "🌎", "mit-bestand": "♻️",
    # languages
    "rs": "🦀", "js": "🟦", "py": "🐍", "go": "🐹", "net": "🔷", "rb": "💎", "tex": "🖋️",
    "react": "⚛️", "wgpu": "🧊",
    # framework modules
    "math": "🧮", "ui": "🖱️", "asset": "🖼️", "hash": "#️⃣", "editor": "✍️",
    "schema": "🧬", "surface": "🗺️",
    # os modules
    "db": "🛢️", "dsl": "🗣️", "flow": "🌊", "infinite": "♾️", "neural": "🧠",
    "pack": "🎒", "protocol": "📡", "run": "🏃", "store": "🏪", "vcs": "🌿",
    "workflow": "🔁", "dev": "🧑‍💻",
    # s modules
    "2d": "◻️", "3d": "🧊", "imperative": "📜", "mindmap": "💭",
    # slots
    "op": "🔧",
    # plugins
    "animate": "🎞️", "architect": "🏛️", "block": "🧱", "cad": "📐", "dag": "🕸️",
    "draw": "🖍️", "energy": "🔋", "fem": "🏗️", "forms": "📋", "gis": "🌍",
    "layout": "📏", "lowpoly": "💠", "mathematical": "➗", "norm": "📕",
    "note": "🗒️", "playbook": "📖", "procedural": "🌀", "process": "🏭",
    "puzzle": "🧩", "raster": "🖨️", "reasoning": "💡", "remodel": "📸",
    "sequence": "🎬", "shooting": "🎥", "sourcing": "🪵", "space": "🪐",
    "trinity": "🔱", "writer": "✒️",
    # common data dirs
    "example": "📚", "fixture": "🧫", "fixtures": "🧫", "template": "📄",
    "brand": "🏷️", "generated": "🤖", "icon": "🔣", "image": "🖼️",
    "logo": "🪧", "font": "🔤", "mesh": "🥽", "list": "📃", "cursor": "👆",
    "badge": "📛", "metabolism": "🌱", "manifest_dir": "🛂",
    # cad model-definition domain (ids -- rename+fix per user decision)
    "modelDefinition": "🏗️", "typology": "🗂️", "interaction": "🎬",
    "attributeDefinition": "🏷️", "propertyDefinition": "🔧", "propertyKind": "🏷️",
    "statDefinition": "📊", "transformation": "🔀", "action": "🎬",
    "aec-building": "🏢", "aec-building-energy": "🔥", "aec-building-structure": "🏛️",
    "spatial-shape": "📐", "aec.building": "🏢", "aec.building.concrete": "🧱",
    "aec.building.energy": "🔥", "aec.building.structure": "🏛️",
    "aec.building.structure.classic": "🏛️", "aec.building.structure.fem.line": "📏",
    "aec.building.structure.fem.solid": "🧊", "aec.building.structure.fem.surface": "🗺️",
    "spatial.shape": "📐", "from_aec.building.structure": "🔀", "from_building": "🔀",
    "from_geometry": "🔀", "classic": "🏛️", "linefem": "📏", "solidfem": "🧊",
    "surfacefem": "🗺️", "json": "🔣",
    # cad typology instance names (literal Rust/schema type identifiers)
    "Arc": "🌙", "BasePlate": "🟫", "Beam": "🪵",
    "Box": "📦", "Ceiling": "⬆️", "Circle": "⭕", "Column": "🏛️",
    "ControlPointCurve": "〰️", "Cylinder": "🥫", "Door": "🚪",
    "ExternalWall": "🧱", "ExtrudeCurve": "➡️", "Foundation": "🪨",
    "Hull": "🐚", "InterpolateCurve": "🌊", "Line": "📏", "LineElement": "📏",
    "Loft": "🎢", "NetworkSurface": "🕸️", "OneWayReinforcedConcreteSlab": "🧱",
    "Plane": "🗺️", "Polyline": "📐", "Railing": "🚧",
    "ReinforcedConcreteColumn": "🏛️", "ReinforcedConcreteExternalWall": "🧱",
    "ReinforcedConcreteInternalWall": "🧱", "Roof": "🏠", "Slab": "🧱",
    "SolidElement": "🧊", "Sphere": "🌐", "Stair": "🪜", "SurfaceElement": "🗺️",
    "Sweep1": "🌀", "Sweep2": "🌀", "Wall": "🧱", "Window": "🪟", "Windows": "🪟",
    # cad other
    "brepjs": "📐", "net": "🔷", "runtime": "🏃", "stately": "🎰", "query": "🔍",
    "play": "🎮", "camera": "📷",
    # per-language crate subtree names (long tail)
    "algebra": "➕", "approximation": "🎯", "bipartite": "🔀", "centrality": "🎯",
    "cliques": "🔺", "clustering": "🧩", "coloring": "🎨", "community": "🏘️",
    "components": "🧩", "connectivity": "🔗", "cycles": "🔄", "drawing": "🖊️",
    "generate": "🎲", "isomorphism": "🪞", "matching": "🤝", "operators": "🔧",
    "paths": "🛤️", "planarity": "🗺️", "similarity": "🎯", "spectral": "📊",
    "structure": "🏗️", "traversal": "🚶", "trees": "🌳", "graph": "🕸️",
    "cas": "🧮", "entropy": "🎲", "fuzzy": "🌫️", "geometry": "📐", "lie": "🔷",
    "number": "🔢", "optimize": "🎯", "polynomial": "📈", "probability": "🎲",
    "random": "🎲", "sampling": "🎯", "signal": "📶", "spatial": "🗺️",
    "statistics": "📊", "tabular": "📋", "wfc": "🧩", "src": "📂",
    "node-graph": "🕸️", "paint": "🎨", "terrain": "🏔️", "tiled-map": "🗺️",
    "actor": "🎭", "cluster": "🌐", "compact": "🗜️", "conflict": "⚔️",
    "document": "📄", "observe": "👁️", "projection": "📽️", "security": "🔒",
    "snapshot": "📸", "state": "🔘", "storage": "🗄️", "wal": "📝",
    "neo4j": "🌐", "postgres": "🐘", "sqlite": "🪶", "async": "⏳", "cli": "⌨️",
    "core": "🫀", "format": "📐", "http": "🌐", "index": "🔢", "io": "🔌",
    "testkit": "🧪", "value": "🔢", "benches": "⏱️", "derive": "✨",
    "schema": "🧬", "token": "🎟️", "causal": "🔗", "command": "🎮",
    "crdt": "🔀", "history": "📜", "materialize": "💎", "wire": "📡",
    "board": "🎲", "canvas": "🖼️", "port": "🔌", "normal": "➕",
    "directed": "➡️", "undirected": "↔️", "world": "🌍", "r3f": "🎨",
    "react-renderer": "🎨", "compute": "🧮", "bim": "🏗️", "brep": "📐",
    "dictionary": "📖", "draw": "🖍️", "list": "📃", "logic": "🧠",
    "math": "🧮", "module_wasm": "🕸️", "control": "🎮", "beams": "🪵",
    "slabs": "🧱", "windows": "🪟", "fsm": "🔄", "macros": "✨",
    "geo": "🗺️", "camera": "📷", "dense": "🌫️", "feature": "🌟",
    "motion": "🏃", "sfm": "📸", "jack": "🔌", "lsp": "🧠", "shell": "🐚",
    "ram": "🐏", "spine": "🦴", "curate": "🗂️", "wires": "🔌", "home": "🏠",
    "din4108": "📕", "din16798": "📗", "din18599": "📙", "en1990": "📘",
    "en1991": "📘", "en1992": "📘", "en1993": "📘", "en1994": "📘",
    "en1995": "📘", "en1996": "📘", "en1997": "📘", "en1998": "📘",
    "en1999": "📘", "iso16757": "📓", "vdi3805": "📔", "rewrite": "✏️",
    "present": "🎬", "video": "🎥",
    # infra / server
    "client": "💻", "server": "🖥️", "native": "🔩", "bootstrap": "🥾",
    "hook": "🪝", "hooks": "🪝", "lib": "📚", "mcp": "🔌", "vscode": "🧩",
    "coordinator": "🎛️", "graphql": "🔗", "api": "🛰️", "webhooks": "🪝",
    "github": "🐙", "v1": "1️⃣", "auth": "🔑", "health": "💚", "diff": "🔀",
    "scope": "🔭", "warning": "⚠️", "ticket": "🎫", "event": "📅",
    "repo_route": "📦", "breach": "🚨", "[id]": "[🆔id]",
    # print / asset
    "asset": "🖼️", "flyer": "📰", "paper": "📄", "report": "📋",
    "zukunftbau": "🏗️", "anhang": "📎", "demonstrator": "🎪", "katalog": "📖",
    "kelly-slab": "🔤", "share-tech-mono": "🔤", "anta": "🔤", "noto-emoji": "😀",
    "zwischenbericht": "📋",
    # mit-bestand / aggregator / präsentation
    "aggregator": "🧺", "abbau-aufbau": "🏚️", "bericht": "📋", "präsentation": "🎤",
    "33.projektetage": "📅", "public": "🌐", "slide": "🎞️",
    "zukunft-bau-entwerfen-mit-bestand_files": "📁",
    # ui asset
    "cursor": "👆", "introduction": "👋", "source": "🔣", "representation": "🎨",
    # fixtures / sync
    "basic-remote-operations": "🔄", "remote-operations-backlog": "📥",
    "snapshot-replaced": "📸", "worker": "👷", "sync": "🔄",
    # misc single-use build/route dirs
    "some": "📁", "folder": "📁", "shared": "🤝", "text": "📝", "wasm": "🕸️",
    # second batch — found via first vocab-coverage run
    "5d": "🖐️", "directory": "📇", "host": "🖥️", "icon_svgs": "🔣",
    "metabolism_svgs": "🌱", "preview": "👁️", "projekt": "📁", "registry": "📇",
    "scene": "🎬", "styling": "🎨", "tailwind": "🎨", "theme": "🎨", "tui": "⌨️",
    "wit": "📜", "plugin-modules": "🔌",
}
# #endregion 🔤Vocabulary

def has_emoji(s: str) -> bool:
    return any(ord(c) > 0x2000 for c in s)


def is_excluded(relpath: str) -> bool:
    posix = relpath.replace(os.sep, "/")
    for prefix in EXCLUDE_PATH_PREFIXES:
        if posix == prefix.rstrip("/") or posix.startswith(prefix):
            return True
    parts = posix.split("/")
    if any(p in EXCLUDE_SUBTREE_NAMES for p in parts):
        return True
    return False


def collect():
    all_dirs = list(ROOTS)
    for root in ROOTS:
        for dirpath, dirs, files in os.walk(root):
            if is_excluded(dirpath):
                dirs[:] = []
                continue
            dirs[:] = [d for d in dirs if d not in SKIP_DIRNAMES and not d.startswith(".")]
            for d in list(dirs):
                full = os.path.join(dirpath, d)
                if is_excluded(full):
                    continue
                all_dirs.append(full)
    return all_dirs


def main():
    all_dirs = collect()
    missing = set()
    for d in all_dirs:
        name = os.path.basename(d)
        if name in BARE_EMOJI_WORDS:
            continue
        if has_emoji(name):
            continue
        if name not in VOCAB:
            missing.add(name)
    print(f"total dirs in scope: {len(all_dirs)}")
    print(f"missing vocab entries: {len(missing)}")
    for m in sorted(missing):
        print("  MISSING:", repr(m))
    if missing:
        return
    # Build rename map: deepest first
    renames = []
    for d in sorted(all_dirs, key=lambda p: -p.count(os.sep)):
        name = os.path.basename(d)
        parent = os.path.dirname(d)
        if name in BARE_EMOJI_WORDS:
            new_name = BARE_EMOJI_WORDS[name]
        elif has_emoji(name):
            continue
        else:
            word = WORD_OVERRIDE.get(name, name)
            new_name = VOCAB[name] + word
        new_path = os.path.join(parent, new_name)
        if new_path == d:
            continue
        renames.append({"old": d, "new": new_path})
    with open(".repo/🎫/26/07/29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/rename-map-v2.json", "w", encoding="utf-8") as f:
        json.dump(renames, f, ensure_ascii=False, indent=2)
    print(f"wrote {len(renames)} renames")


if __name__ == "__main__":
    main()
