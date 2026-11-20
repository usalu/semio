from collections import deque
import json
from pathlib import Path
import sys

units = json.loads(Path(sys.argv[1]).read_text())["units"]
by_artifact = {unit["artifact"]: unit for unit in units}


def dependency_paths(root):
    queue = deque([(root, [])])
    visited = set()
    while queue:
        unit, parents = queue.popleft()
        artifact = unit["artifact"]
        if artifact in visited:
            continue
        visited.add(artifact)
        package = unit["package"]
        identity = Path(artifact).parent.parent.name
        path = [*parents, f"{package} {identity}"]
        if package == "semio-framework-pack":
            yield path
        for edge in unit["dependencies"]:
            for dependency in edge["artifacts"]:
                if dependency in by_artifact:
                    queue.append((by_artifact[dependency], path))


for unit in units:
    if unit["exit"] and unit["target"] == "wasm32-wasip2":
        for path in dependency_paths(unit):
            print(" → ".join(path))
