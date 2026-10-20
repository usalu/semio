#!/usr/bin/env python3
"""Writes the language-agnostic fixture of the owner claim law (staged/new/claims/fixture.json): 60 seeded worlds of nested owners, native roots, a root owner, lookalike directory names, content duplicates and rows outside every owner; the expected claims come from the independent naive model in oracle.py."""
import importlib.util, json, pathlib, random
HERE = pathlib.Path(__file__).resolve().parent
NEW = HERE / "staged/new/claims"
spec = importlib.util.spec_from_file_location("oracle", NEW / "oracle.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
rng = random.Random(1021)
DIRECTORIES = ["a", "a/b", "a/bc", "a/b/c", "d", "d/e", "f g", "🧰️x", "🧰️x/📦️y", "z"]
KINDS = ["file", "exists", "lstat", "readdir", "analysis", "resolve"]


def make_world(index):
    workspace = "/w" + str(index % 3)
    rows = []
    for number in range(rng.randint(30, 140)):
        base = rng.choice(DIRECTORIES + ["shared", "../other", "a", "a"])
        path = workspace + "/" + base + "/" + rng.choice(["x.ts", "y.json", "z/deep.ts", "m"]) + (str(rng.randint(0, 3)) if rng.random() < 0.5 else "")
        if rng.random() < 0.05:
            path = "/elsewhere/" + str(number)
        fact = {"size": rng.randint(0, 9), "sha256": "%064x" % rng.randint(0, 40)} if rng.random() < 0.6 else rng.choice(["file", "directory", "!ENOENT"])
        rows.append({"kind": rng.choice(KINDS), "path": path, "fact": fact})
    for _ in range(rng.randint(0, 8)):
        rows.append(json.loads(json.dumps(rng.choice(rows))))
    roots = rng.sample(DIRECTORIES, rng.randint(1, 7))
    if rng.random() < 0.3:
        roots.append(".")
    owners, edge_only, stubs = [], [], {}
    for number, root in enumerate(roots):
        owner = {"name": "p%d" % number, "root": root, "nodeRows": rng.sample(range(len(rows)), rng.randint(0, min(12, len(rows))))}
        if rng.random() < 0.3:
            owner["nativeRoot"] = rng.choice(DIRECTORIES)
        owners.append(owner)
    for number in range(rng.randint(0, 3)):
        name = "e%d" % number
        edge_only.append({"name": name})
        stubs[name] = {"root": rng.choice(DIRECTORIES + ["."]), "kind": "project", "targets": []}
        if rng.random() < 0.3:
            stubs[name]["nativeRoot"] = rng.choice(DIRECTORIES)
    part_rows = sorted(rng.sample(range(len(rows)), rng.randint(len(rows) // 2, len(rows))))
    rng.shuffle(part_rows)
    return {"workspaceRoot": workspace, "rows": rows, "owners": owners, "edgeOnly": edge_only, "stubs": stubs, "partRows": part_rows}


worlds = [make_world(index) for index in range(60)]
expected = [oracle.solve(world) for world in worlds]
(NEW / "fixture.json").write_text(json.dumps({"version": 1, "worlds": [dict(world, expect=answer) for world, answer in zip(worlds, expected)]}, ensure_ascii=False, separators=(",", ":")) + "\n", encoding="utf8")
print("claims fixture", len(worlds), "worlds,", sum(len(world["rows"]) for world in worlds), "rows")
