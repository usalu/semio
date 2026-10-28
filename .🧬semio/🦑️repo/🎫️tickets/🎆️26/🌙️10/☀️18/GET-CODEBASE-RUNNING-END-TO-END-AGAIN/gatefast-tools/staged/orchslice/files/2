#!/usr/bin/env python3
"""🐍️ Independent oracle of the selected-scope slice: reads the JSON scenario corpus and recomputes every closure with plain Python (no shared code with the Bun test).

usage: 🐍️.py CORPUS.json      prints {"failures": [...]} and exits 0 when every scenario agrees."""
import json, sys


class Unknown(Exception):
    pass


def closure_model(case):
    owners, external = case["owners"], set(case.get("external", []))
    admitted, active, queue = set(), {}, []

    def admit(name):
        todo = [name]
        while todo:
            at = todo.pop()
            if at in admitted:
                continue
            if at not in owners:
                if at == name or at not in external:
                    raise Unknown(at)
                continue
            admitted.add(at)
            todo.extend(owners[at]["edges"])

    def task(name, target, optional):
        if name not in owners:
            raise Unknown(name)
        admit(name)
        if optional and target not in owners[name]["targets"]:
            return
        if target not in active.setdefault(name, set()):
            active[name].add(target)
            queue.append((name, target))

    groups = set()

    def imported(name):
        return sorted({e for e in owners[name]["edges"] if e in owners})

    def reach(name, inputs):
        for entry in inputs:
            if isinstance(entry, str):
                group = entry[1:] if entry.startswith("^") else entry
                for owner in imported(name) if entry.startswith("^") else [name]:
                    values = owners[owner].get("namedInputs", {}).get(group)
                    if not isinstance(values, list) or (owner, group) in groups:
                        continue
                    groups.add((owner, group))
                    reach(owner, values)
            elif isinstance(entry, dict) and (isinstance(entry.get("input"), str) or entry.get("projects") is not None):
                if entry.get("dependencies") is True:
                    named = imported(name)
                elif entry.get("projects") is None:
                    named = [name]
                else:
                    named = [entry["projects"]] if isinstance(entry["projects"], str) else entry["projects"]
                for owner in named:
                    if owner not in owners:
                        continue
                    admit(owner)
                    if isinstance(entry.get("input"), str):
                        reach(owner, [entry["input"]])

    selection = case["selection"]
    if selection["kind"] == "tasks":
        for row in selection["tasks"]:
            task(row["project"], row["target"], False)
    else:
        for name in case["roots"]:
            admit(name)
        for name in case["roots"]:
            for target in selection.get("targets", []):
                task(name, target, True)
    while queue:
        name, target = queue.pop(0)
        operation = owners[name]["targets"].get(target)
        if operation is None:
            continue
        deps = sorted({e for e in owners[name]["edges"] if e in owners})
        reach(name, operation.get("inputs", []))
        for dep in operation.get("dependsOn", []):
            if isinstance(dep, str):
                if dep.startswith("^"):
                    for value in deps:
                        task(value, dep[1:], True)
                elif ":" in dep:
                    at = dep.rindex(":")
                    task(dep[:at], dep[at + 1:], False)
                else:
                    task(name, dep, False)
            elif isinstance(dep, dict) and isinstance(dep.get("target"), str):
                if dep.get("dependencies"):
                    projects = deps
                elif dep.get("projects") is None:
                    projects = [name]
                elif dep["projects"] == "*":
                    projects = list(owners)
                elif isinstance(dep["projects"], str):
                    projects = [dep["projects"]]
                else:
                    projects = dep["projects"]
                for value in projects:
                    task(value, dep["target"], dep.get("dependencies") is True)
        execution = ((operation.get("metadata") or {}).get("semio") or {}).get("execution") or {}
        if execution.get("kind") == "orchestration":
            for child in execution.get("children", []):
                task(child["project"], child["target"], False)

    def inside(root, path):
        return root == "." or path == root or path.startswith(root + "/")

    for provider in case.get("providers", []):
        if not any(inside(root, owners[name]["root"]) for name in active for root in provider["consumerRoots"]):
            continue
        best = None
        for name, row in owners.items():
            if inside(row["root"], provider["sourcePath"]) and (best is None or len(row["root"]) > len(owners[best]["root"])):
                best = name
        if best is not None:
            admit(best)
    return sorted(admitted)


def expected(case):
    try:
        return {"admitted": closure_model(case)}
    except Unknown:
        return {"error": "unknown"}


if __name__ == "__main__":
    corpus = json.load(open(sys.argv[1], encoding="utf8"))
    failures = [index for index, case in enumerate(corpus["closures"]) if expected(case) != case["expected"]]
    print(json.dumps({"failures": failures}))
    sys.exit(1 if failures else 0)
