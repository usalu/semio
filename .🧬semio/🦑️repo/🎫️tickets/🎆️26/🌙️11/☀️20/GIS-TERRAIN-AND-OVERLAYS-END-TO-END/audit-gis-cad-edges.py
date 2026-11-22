"""🔍️ Audit GIS CAD dependencies and check the resolved workspace lock graphs."""

import json
import re
import subprocess
from collections import defaultdict, deque
from pathlib import Path

GIS = Path("✏️s/🔌️plugins/🌍️gis")
TERRAIN = GIS / "🗿️artifacts/🏔️gisterrain"
MAP = GIS / "🗿️artifacts/🗺️gismap"
S = "semio-s-artifact-stdio-"
GIS_PREFIX = "semio-s-artifact-gis-"
CAD = {S + name for name in ("dwg", "dxf", "pdf")}
SOURCE_PATTERN = r"stdio[_-](dwg|dxf|pdf)|DwgSnapshot|DxfSnapshot"
PACKAGE_DELIMITER = "\n[[package]]\n"
DEPENDENCIES = re.compile(r'(\ndependencies = \[\n)(.*?)(\n\])', re.S)
DEP_ROW = re.compile(r'^ "([^"]+)",$')
NAME = re.compile(r'^name = "([^"]+)"$', re.M)
VERSION = re.compile(r'^version = "([^"]+)"$', re.M)
SOURCE = re.compile(r'^source = "([^"]+)"$', re.M)
ROOTS = [
    TERRAIN, MAP, GIS / "🏭️bridge", Path("✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services"),
    Path("✏️s/🧑‍💻dev/🧹️fixture-sweep"), Path("🌎️hub"),
]
METADATA = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "cargo",
            "metadata", "--offline", "--locked", "--no-deps", "--format-version=1", "--manifest-path"]


def contents(path):
    return path.read_bytes().decode("utf-8")


def lock_graph(text):
    header, *blocks = text.split(PACKAGE_DELIMITER)
    blocks = {package_id(block): block for block in blocks}
    by_name = defaultdict(list)
    for package in blocks:
        by_name[package[0]].append(package)
    graph = {}
    for package, block in blocks.items():
        found = DEPENDENCIES.search(block)
        edges = []
        for row in found.group(2).splitlines() if found else ():
            edge = DEP_ROW.fullmatch(row).group(1)
            name, *qualifiers = edge.split(" ", 2)
            candidates = by_name[name]
            if qualifiers:
                candidates = [item for item in candidates if item[1] == qualifiers[0]]
            if len(qualifiers) == 2:
                candidates = [item for item in candidates if item[2] == qualifiers[1][1:-1]]
            assert len(candidates) == 1, (package, edge, candidates)
            edges.append(candidates[0])
        graph[package] = edges
    return header, blocks, graph


def package_id(block):
    found = SOURCE.search(block)
    return NAME.search(block).group(1), VERSION.search(block).group(1), found.group(1) if found else None


def live_packages(graph, roots):
    visited = set()
    queue = deque(roots)
    while queue:
        node = queue.popleft()
        if node not in visited:
            visited.add(node)
            queue.extend(graph[node])
    return visited


def check_lock(root):
    path = root / "Cargo.lock"
    _, blocks, graph = lock_graph(contents(path))
    resolved = subprocess.run(METADATA + [str(root / "Cargo.toml")],
                              capture_output=True, text=True, check=True)
    metadata = json.loads(resolved.stdout)
    members = set(metadata["workspace_members"])
    roots = {(item["name"], item["version"], None)
             for item in metadata["packages"] if item["id"] in members}
    assert len(roots) == len(members) and roots, path
    active = {pkg[0] for pkg in live_packages(graph, roots)}
    for package, dependencies in graph.items():
        if package[0].startswith(GIS_PREFIX):
            assert not {item[0] for item in dependencies} & CAD, package
    assert S + "pdf" in active and S + "dxf" in active, path
    assert bool(S + "dwg" in active) == (root in ROOTS[-2:]), path
    print(path, len(blocks), "packages; DWG", "reachable" if S + "dwg" in active else "absent")


def main():
    references = subprocess.run(["rg", "-n", SOURCE_PATTERN, "-g", "!Cargo.lock", "-g", "!Cargo.toml", str(GIS)],
                               capture_output=True, text=True)
    assert references.returncode == 1, references.stdout + references.stderr
    for artifact in (TERRAIN, MAP):
        for path in artifact.rglob("Cargo.toml"):
            for dep in CAD:
                assert not re.search(r"^" + dep + r"\s*=", contents(path), re.M), (path, dep)
    for root in ROOTS:
        check_lock(root)


if __name__ == "__main__":
    main()
