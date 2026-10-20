#!/usr/bin/env python3
"""Generates staged/closure-cache.fixture.json: hand written worlds, seeded random worlds, vectors and the oracle's frozen answers.

usage: gen_fixture.py > staged/closure-cache.fixture.json
Deterministic (fixed seeds, no clock). The `expected` section is produced by staged/closure-cache.oracle.py, never by the implementation.
Receipts are judged against the live admission schema with the staged cache definitions inserted (the same insertion apply.py performs).
"""
import json
import os
import random
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ORACLE = os.path.join(HERE, "staged", "closure-cache.oracle.py")
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
SCHEMA = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🚪️admission.json")
ANCHOR = '"$defs": {\n    "sourceInputObservation": {'
FOLDER = "📋️project.json"


def target(depends=(), files=(), dependencies=(), inputs=None, generated=()):
    return {"dependsOn": list(depends), "files": list(files), "dependencies": list(dependencies), "dependencyInputs": inputs or {}, "generatedDependencies": list(generated)}


def owner(name, root, targets=None, files=(), dependencies=(), input_files=None, native=()):
    row = {"name": name, "root": root, "manifest": root + "/" + FOLDER, "base": {"files": list(files), "dependencies": list(dependencies)}, "targets": targets or {}, "inputFiles": input_files or {}}
    if native:
        row["nativeRoots"] = list(native)
    return row


def tasks(*pairs):
    return {"kind": "tasks", "tasks": [{"project": project, "target": goal} for project, goal in pairs]}


def worlds():
    chain = {"id": "chain", "broken": ["bad"], "catalogProviders": [], "owners": [
        owner("app", "app", {"test": target(["^build", "lib:gen"], ["lib/gen.ts"], ["lib"], {"lib": ["default"]}), "build": target(["^build"], [], ["lib"])}, ["app/main.ts"], ["lib"]),
        owner("lib", "lib", {"build": target(["^build"], ["util/x.ts"], ["util"]), "gen": target([], [], [])}, [], ["util"], {"default": ["lib/in.ts"]}),
        owner("util", "util", {"build": target([], [], [])}),
        owner("other1", "other1", {"test": target(["^test"], [], ["other2", "bad"])}, [], ["other2"]),
        owner("other2", "other2", {"test": target([], [], [])}),
        owner("bad", "bad", {"test": target([], [], [])}),
    ]}
    optional = {"id": "optional", "broken": [], "catalogProviders": [], "owners": [
        owner("app", "app", {"build": target(["^lint"], [], ["lib", "lib2"])}, [], ["lib", "lib2"]),
        owner("lib", "lib", {"build": target([], [], [])}),
        owner("lib2", "lib2", {"lint": target([], [], []), "build": target([], [], [])}),
    ]}
    forms = {"id": "forms", "broken": [], "catalogProviders": [], "owners": [
        owner("app", "app", {"build": target([{"target": "build", "dependencies": True}, {"target": "x", "projects": ["p"]}, {"target": "y", "projects": "*"}, "self", "q:build"], [], ["lib"]), "self": target([], [], []), "y": target([], [], [])}, [], ["lib"]),
        owner("lib", "lib", {"build": target([], [], []), "y": target([], [], [])}),
        owner("p", "p", {"x": target([], [], []), "y": target([], [], [])}),
        owner("q", "q", {"build": target([], [], []), "y": target([], [], [])}),
        owner("zed", "zed", {"y": target([], [], [])}),
    ]}
    generated = {"id": "generated", "broken": ["shell-gen"], "catalogProviders": [], "owners": [
        owner("consumer", "consumer", {"build": target([], ["fw/shell.ts"], [], None, ["shell-gen:typegen"])}),
        owner("fw", "fw", {"build": target([], [], [])}),
        owner("shell-gen", "shell-gen", {"typegen": target([], [], [])}),
        owner("plain", "plain", {"build": target([], ["fw/index.ts"], [])}),
    ]}
    catalog = {"id": "catalog", "broken": [], "owners": [
        owner("registry-app", "registry/app", {"build": target([], [], [])}),
        owner("registry", "registry", {"build": target([], [], [])}),
        owner("prov", "prov", {"build": target([], [], [])}),
        owner("solo", "solo", {"build": target([], [], [])}),
    ], "catalogProviders": [{"ownerRoot": "prov", "sourcePath": "prov/publication.ts", "consumerRoots": ["registry"]}]}
    nested = {"id": "nested", "broken": [], "catalogProviders": [], "owners": [
        owner("outer", "o1", {"build": target([], ["o1/sub/deep.ts", "o1/own.ts"], [])}),
        owner("inner", "o1/sub", {"build": target([], [], [])}, [], [], None, ["native/inner"]),
        owner("viaNative", "n2", {"build": target([], ["native/inner/lib.rs"], [])}),
    ]}
    cycle = {"id": "cycle", "broken": [], "catalogProviders": [], "owners": [
        owner("a", "a", {"build": target(["^build", "b:build"], [], ["b"])}, [], ["b"]),
        owner("b", "b", {"build": target(["^build", "a:build"], [], ["a"])}, [], ["a"]),
        owner("c", "c", {"build": target([], [], [])}),
    ]}
    return [chain, optional, forms, generated, catalog, nested, cycle]


def random_world(seed):
    rng = random.Random(seed)
    names = ["w%d_%d" % (seed, index) for index in range(rng.randint(7, 14))]
    goals = ["build", "test", "gen", "lint"]
    entries = []
    for name in names:
        picked = rng.sample(goals, rng.randint(1, 3))
        deps = [other for other in rng.sample(names, rng.randint(0, 3)) if other != name]
        targets = {}
        for goal in picked:
            files = ["%s/f%d.ts" % (rng.choice(names) if rng.random() < 0.4 else name, rng.randint(0, 3)) for _ in range(rng.randint(0, 3))]
            tdeps = [other for other in rng.sample(names, rng.randint(0, 2)) if other != name]
            targets[goal] = target([], files, tdeps, {other: ["default"] for other in tdeps if rng.random() < 0.3})
        entries.append(owner(name, name, targets, ["%s/m.ts" % name], deps, {"default": ["%s/in.ts" % rng.choice(names)]}))
    by_name = {entry["name"]: entry for entry in entries}
    for entry in entries:
        for goal, row in entry["targets"].items():
            for _ in range(rng.randint(0, 2)):
                kind = rng.choice(["caret", "self", "cross"])
                if kind == "caret":
                    row["dependsOn"].append("^" + rng.choice(goals))
                elif kind == "self":
                    row["dependsOn"].append(rng.choice(sorted(entry["targets"])))
                else:
                    other = by_name[rng.choice(names)]
                    if other["targets"]:
                        row["dependsOn"].append(other["name"] + ":" + rng.choice(sorted(other["targets"])))
            row["dependsOn"] = sorted(set(row["dependsOn"]))
    return {"id": "random%d" % seed, "broken": [], "catalogProviders": [], "owners": entries}


def random_cases(world, rng):
    cases = []
    candidates = [(entry["name"], goal) for entry in world["owners"] for goal in entry["targets"]]
    for index in range(3):
        picked = rng.sample(candidates, rng.randint(1, 2))
        cases.append({"id": "%s-%d" % (world["id"], index), "world": world["id"], "selection": tasks(*picked)})
    return cases


def keys():
    base = {"implementation": "i" * 64, "scope": "a" * 64, "registry": "b" * 64, "owner": {"name": "@semio-tech/x", "root": "🧰️framework/x", "manifest": "🧰️framework/x/" + FOLDER}}
    return [
        dict(base, id="base", targets=[], inputs=[]),
        dict(base, id="targets", targets=["build", "test"], inputs=[]),
        dict(base, id="inputs-emoji", targets=["build"], inputs=["{workspaceRoot}/✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts"]),
        dict(base, id="other-scope", targets=["build"], inputs=[], scope="c" * 64),
        dict(base, id="other-implementation", targets=["build"], inputs=[], implementation="d" * 64),
        dict(base, id="order-matters", targets=["test", "build"], inputs=[]),
    ]


def chains():
    return [
        {"id": "selection", "name": "a", "requestedBy": [["a", None]]},
        {"id": "three hops", "name": "c", "requestedBy": [["c", {"owner": "b", "target": "test"}], ["b", {"owner": "a"}], ["a", None]]},
        {"id": "catalog", "name": "p", "requestedBy": [["p", "catalog"]]},
        {"id": "cycle", "name": "x", "requestedBy": [["x", {"owner": "y", "target": "t"}], ["y", {"owner": "x"}]]},
        {"id": "unknown owner", "name": "z", "requestedBy": []},
        {"id": "limit", "name": "n0", "requestedBy": [["n%d" % i, {"owner": "n%d" % (i + 1)}] for i in range(20)]},
    ]


def receipts():
    admission = {"owners": ["app"], "files": ["app/📋️project.json"], "targets": {"app": ["build"]}, "catalogProviders": [], "inputs": {}}
    row = {"kind": "file", "path": "/w/app/📋️project.json", "fact": {"size": 3, "mtimeMs": 1.5, "ctimeMs": 2.5, "ino": 7, "sha256": "e" * 64}}
    good = {"version": 1, "scope": "a" * 64, "module": {"implementation": "1" * 64, "publication": "2" * 64}, "startedAt": 1760000000000, "admission": admission, "observations": [row, {"kind": "exists", "path": "/w/x", "fact": False}]}

    def mutate(**changes):
        value = json.loads(json.dumps(good))
        for key, replacement in changes.items():
            if replacement is None:
                del value[key]
            else:
                value[key] = replacement
        return value

    return [
        {"id": "closed", "value": good, "valid": True},
        {"id": "empty observations", "value": mutate(observations=[]), "valid": True},
        {"id": "version two", "value": mutate(version=2), "valid": False},
        {"id": "short scope", "value": mutate(scope="abc"), "valid": False},
        {"id": "missing module", "value": mutate(module=None), "valid": False},
        {"id": "extra property", "value": dict(good, surprise=1), "valid": False},
        {"id": "unknown kind", "value": mutate(observations=[{"kind": "shell", "path": "/x", "fact": 1}]), "valid": False},
        {"id": "open admission", "value": mutate(admission={"owners": "app"}), "valid": False},
        {"id": "negative start", "value": mutate(startedAt=-1), "valid": False},
    ]


def publishable():
    def file(mtime, ctime):
        return {"kind": "file", "path": "/w/x", "fact": {"size": 1, "mtimeMs": mtime, "ctimeMs": ctime, "ino": 1, "sha256": "f" * 64}}

    def analysis(mtime, ctime):
        return {"kind": "analysis", "path": "/w/y", "fact": {"provider": "p", "revision": "r", "source": {"size": 1, "mtimeMs": mtime, "ctimeMs": ctime, "ino": 1, "sha256": "f" * 64}, "resultSha256": "f" * 64, "nestedObservations": []}}

    start = 1760000100000
    return [
        {"id": "old files", "startedAt": start, "observations": [file(start - 5000, start - 5000)], "expected": True},
        {"id": "no rows", "startedAt": start, "observations": [], "expected": True},
        {"id": "young mtime", "startedAt": start, "observations": [file(start - 1999, start - 9000)], "expected": False},
        {"id": "young ctime", "startedAt": start, "observations": [file(start - 9000, start)], "expected": False},
        {"id": "boundary", "startedAt": start, "observations": [file(start - 2000, start - 2000)], "expected": False},
        {"id": "just old enough", "startedAt": start, "observations": [file(start - 2001, start - 2001)], "expected": True},
        {"id": "young analysis source", "startedAt": start, "observations": [analysis(start - 100, start - 9000)], "expected": False},
        {"id": "directory rows never racy", "startedAt": start, "observations": [{"kind": "readdir", "path": "/w", "fact": "0" * 64}], "expected": True},
    ]


def scenarios():
    world = {"id": "cache", "broken": [], "catalogProviders": [], "owners": [
        owner("app", "app", {"test": target(["lib:build"], ["lib/a.ts"], ["lib"])}, ["app/main.ts"], ["lib"]),
        owner("lib", "lib", {"build": target([], ["util/u.ts"], [])}, [], []),
        owner("util", "util", {"build": target([], [], [])}),
        owner("extra", "extra", {"build": target([], [], [])}),
        owner("unrelated", "unrelated", {"build": target([], [], [])}),
    ]}
    selection = tasks(("app", "test"))
    lib = [entry for entry in world["owners"] if entry["name"] == "lib"][0]
    edited = json.loads(json.dumps(lib))
    edited["targets"]["build"]["files"] = ["util/u.ts", "lib/more.ts"]
    grown = json.loads(json.dumps(lib))
    grown["base"]["dependencies"] = ["extra"]
    return [
        {"id": "warm reuse and exact invalidation", "world": world, "steps": [
            {"op": "run", "selection": selection, "expect": "cold"},
            {"op": "run", "selection": selection, "expect": "warm"},
            {"op": "replace", "owner": "lib", "declaration": edited},
            {"op": "run", "selection": selection, "expect": "changed", "changed": ["lib"]},
            {"op": "replace", "owner": "lib", "declaration": grown},
            {"op": "run", "selection": selection, "expect": "changed", "changed": ["lib"]},
            {"op": "registry", "digest": "9" * 64},
            {"op": "run", "selection": selection, "expect": "all"},
            {"op": "implementation", "revision": "rev-two"},
            {"op": "run", "selection": selection, "expect": "all"},
            {"op": "run", "selection": selection, "expect": "warm"},
        ]},
        {"id": "cancelled closure resumes with its finished owners", "world": world, "steps": [
            {"op": "run", "selection": selection, "expect": "cancel", "cancelAfter": 2},
            {"op": "run", "selection": selection, "expect": "resume"},
            {"op": "run", "selection": selection, "expect": "warm"},
        ]},
    ]


def patched_schema():
    with open(SCHEMA, encoding="utf-8") as handle:
        text = handle.read()
    if '"admissionReceipt"' not in text:
        with open(os.path.join(HERE, "staged", "admission.defs.txt"), encoding="utf-8") as handle:
            defs = handle.read()
        if text.count(ANCHOR) != 1:
            raise SystemExit("schema anchor not found")
        text = text.replace(ANCHOR, '"$defs": {\n' + defs + '    "sourceInputObservation": {')
    handle = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, encoding="utf-8")
    handle.write(text)
    handle.close()
    return handle.name


def main():
    schema_path = patched_schema()
    rng = random.Random(20260920)
    hand = worlds()
    randoms = [random_world(seed) for seed in range(1, 31)]
    cases = [
        {"id": "chain course-like", "world": "chain", "selection": tasks(("app", "test"))},
        {"id": "chain build only", "world": "chain", "selection": tasks(("app", "build"))},
        {"id": "chain unrelated broken stays unreached", "world": "chain", "selection": tasks(("app", "test"), ("util", "build"))},
        {"id": "chain broken owner reached", "world": "chain", "selection": tasks(("other1", "test"))},
        {"id": "optional caret skips owners without the target", "world": "optional", "selection": tasks(("app", "build"))},
        {"id": "dependency object forms", "world": "forms", "selection": tasks(("app", "build"))},
        {"id": "generated producer failing is in the closure", "world": "generated", "selection": tasks(("consumer", "build"))},
        {"id": "generated producer unrelated", "world": "generated", "selection": tasks(("plain", "build"))},
        {"id": "catalog consumer activates the provider", "world": "catalog", "selection": tasks(("registry-app", "build"))},
        {"id": "catalog provider stays off for others", "world": "catalog", "selection": tasks(("solo", "build"))},
        {"id": "all admits every owner and provider", "world": "catalog", "selection": {"kind": "all"}},
        {"id": "nested roots resolve to the deepest owner", "world": "nested", "selection": tasks(("outer", "build"))},
        {"id": "native roots own their files", "world": "nested", "selection": tasks(("viaNative", "build"))},
        {"id": "owner cycles terminate", "world": "cycle", "selection": tasks(("a", "build"))},
    ]
    for world in randoms:
        cases.extend(random_cases(world, rng))
    fixture = {
        "version": 1,
        "closure": {"worlds": hand + randoms, "cases": cases},
        "keys": keys(),
        "chains": chains(),
        "receipts": receipts(),
        "publishable": publishable(),
        "scenarios": scenarios(),
    }
    with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, encoding="utf-8") as handle:
        json.dump(fixture, handle, ensure_ascii=False)
        path = handle.name
    try:
        run = subprocess.run([sys.executable, ORACLE, path, schema_path], capture_output=True, text=True)
        if run.returncode:
            raise SystemExit(run.stderr)
        answer = run.stdout
    finally:
        os.unlink(path)
        os.unlink(schema_path)
    fixture["expected"] = json.loads(answer)
    print(json.dumps(fixture, ensure_ascii=False, indent=1, sort_keys=False))


main()
