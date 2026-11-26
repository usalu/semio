import json
import sys

receipt = json.load(open(sys.argv[1]))
metadata = json.load(open(sys.argv[2]))
units = receipt["units"]
packages = {row["name"]: row for row in metadata["packages"]}
actual = {row["message"]["package_id"].rsplit("#", 1)[-1].split("@", 1)[0] for row in units}
edges = {name: {dep["name"] for dep in package["dependencies"] if dep["kind"] != "dev" and dep["name"] in actual} for name, package in packages.items() if name in actual}
root = "semio-plugin-describe"

def closure(replace):
    graph = dict(edges)
    graph[root] = (graph[root] - set(replace)) | set(replace.values())
    visited, todo = set(), [root]
    while todo:
        name = todo.pop()
        if name in visited:
            continue
        visited.add(name)
        todo.extend(graph.get(name, ()))
    return visited

before = closure({})
after = closure({"semio-framework": "semio-framework-manifest-plugin", "semio-os-kernel": "semio-os-pack"})
removed = before - after
print("[DEBUG] actual receipt units", len(units), "distinct packages", len(actual))
print("[DEBUG] metadata modeled current", len(before), "after", len(after), "removed", len(removed))
print("[DEBUG] metadata missing receipt packages", len(actual - set(packages)))
print("[DEBUG] package removals", *sorted(removed), sep="\n")
