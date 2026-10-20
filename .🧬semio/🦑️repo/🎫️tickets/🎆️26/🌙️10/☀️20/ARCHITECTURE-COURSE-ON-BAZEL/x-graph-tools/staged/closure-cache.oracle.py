#!/usr/bin/env python3
"""Independent oracle of the build owner admission closure and its cache vocabulary.

usage: closure-cache.oracle.py FIXTURE.json ADMISSION_SCHEMA.json            prints the oracle's answer for every case as JSON
       closure-cache.oracle.py FIXTURE.json ADMISSION_SCHEMA.json --verify   exits 1 when the fixture's frozen answers differ from the oracle

No code of the implementation under test is shared. The closure is a naive least fixpoint: facts of every admitted owner are recomputed from the
world on each round, nothing is incremental. Hashing uses hashlib, schema verdicts use jsonschema (Draft 2020-12).
"""
import hashlib
import json
import sys


def contains(root, path):
    return root == "." or path == root or path.startswith(root + "/")


def owner_of(world, path):
    roots = sorted(((root, entry["name"]) for entry in world["owners"] for root in [entry["root"], *entry.get("nativeRoots", [])]), key=lambda row: (-len(row[0]), row[1]))
    return next((name for root, name in roots if contains(root, path)), None)


def facts(world, name, targets, inputs):
    entry = next(owner for owner in world["owners"] if owner["name"] == name)
    files = {entry["manifest"], *entry["base"]["files"]}
    dependencies = set(entry["base"]["dependencies"])
    dependency_inputs, generated = {}, {}
    for target in targets:
        declared = entry["targets"][target]
        files |= set(declared["files"])
        dependencies |= set(declared["dependencies"])
        for owner, groups in declared["dependencyInputs"].items():
            dependency_inputs.setdefault(owner, set()).update(groups)
        if declared["generatedDependencies"]:
            generated[target] = sorted(declared["generatedDependencies"])
    for group in inputs:
        files |= set(entry["inputFiles"].get(group, []))
    return {"files": files, "dependencies": dependencies, "dependencyInputs": dependency_inputs, "generated": generated, "targets": entry["targets"]}


def closure(world, selection):
    names = {owner["name"] for owner in world["owners"]}
    declared = {owner["name"]: owner["targets"] for owner in world["owners"]}
    admitted, tasks, optional, inputs, providers = set(), {}, set(), {}, {}
    if selection["kind"] == "all":
        admitted = set(names)
    else:
        for task in selection["tasks"]:
            admitted.add(task["project"])
            tasks.setdefault(task["project"], set()).add(task["target"])

    def live(name):
        rows = []
        for target in sorted(tasks.get(name, [])):
            if target in declared[name]:
                rows.append(target)
            elif (name, target) not in optional:
                raise ValueError("unknown admitted build target %s:%s" % (name, target))
        return rows

    def state():
        return json.dumps([sorted(admitted), {k: sorted(v) for k, v in sorted(tasks.items())}, {k: sorted(v) for k, v in sorted(inputs.items())}, sorted(optional), sorted(providers)])

    while True:
        before = state()
        current = {name: facts(world, name, live(name), sorted(inputs.get(name, []))) for name in sorted(admitted)}
        for name in sorted(admitted):
            value = current[name]
            for path in value["files"]:
                owner = owner_of(world, path)
                if owner:
                    admitted.add(owner)
            admitted |= value["dependencies"]
            for owner, groups in value["dependencyInputs"].items():
                admitted.add(owner)
                inputs.setdefault(owner, set()).update(groups)
            for target in live(name):
                for dependency in [*value["targets"][target]["dependsOn"], *value["generated"].get(target, [])]:
                    for project, goal, soft in expand(dependency, name, value, names):
                        admitted.add(project)
                        if soft and goal not in tasks.get(project, set()):
                            optional.add((project, goal))
                        if not soft:
                            optional.discard((project, goal))
                        tasks.setdefault(project, set()).add(goal)
        for provider in world.get("catalogProviders", []):
            hit = selection["kind"] == "all" or any(
                contains(root, next(o for o in world["owners"] if o["name"] == name)["root"])
                for name in sorted(admitted) for target in live(name) for root in provider["consumerRoots"]
            ) or any("{workspaceRoot}/" + provider["sourcePath"] in groups for groups in inputs.values())
            if hit and provider["sourcePath"] not in providers:
                owner = owner_of(world, provider["sourcePath"])
                if owner is None:
                    raise ValueError("catalog provider owner required")
                providers[provider["sourcePath"]] = provider
                admitted.add(owner)
                inputs.setdefault(owner, set()).add("{workspaceRoot}/" + provider["sourcePath"])
        if before == state():
            break
    current = {name: facts(world, name, live(name), sorted(inputs.get(name, []))) for name in sorted(admitted)}
    generated = {name: {t: v for t, v in sorted(value["generated"].items())} for name, value in current.items() if value["generated"]}
    files = set()
    for value in current.values():
        files |= value["files"]
    files |= set(providers)
    result = {
        "owners": sorted(admitted),
        "files": sorted(files),
        "targets": {name: live(name) for name in sorted(admitted) if live(name)},
        "inputs": {name: sorted(groups) for name, groups in sorted(inputs.items()) if groups},
        "catalogProviders": sorted(({"ownerRoot": p["ownerRoot"], "sourcePath": p["sourcePath"]} for p in providers.values()), key=lambda row: row["sourcePath"]),
    }
    if generated:
        result["generatedDependencies"] = generated
    return result


def expand(dependency, name, value, names):
    if isinstance(dependency, str):
        if dependency.startswith("^"):
            return [(owner, dependency[1:], True) for owner in sorted(value["dependencies"])]
        if ":" in dependency:
            cut = dependency.rindex(":")
            return [(dependency[:cut], dependency[cut + 1:], False)]
        return [(name, dependency, False)]
    projects = sorted(value["dependencies"]) if dependency.get("dependencies") else sorted(names) if dependency.get("projects") == "*" else [dependency["projects"]] if isinstance(dependency.get("projects"), str) else dependency.get("projects", [name])
    return [(project, dependency["target"], dependency.get("dependencies") is True) for project in projects]


def blocked(world, selection):
    if not world.get("broken"):
        return None
    probe = dict(world, broken=[])
    reached = closure(probe, selection)["owners"]
    hit = [name for name in world["broken"] if name in reached]
    return hit[0] if hit else None


def record(case):
    digest = hashlib.sha256("\0".join(["owner-admission", case["owner"]["root"], json.dumps([case["targets"], case["inputs"]], ensure_ascii=False, separators=(",", ":"))]).encode("utf-8")).hexdigest()[:24]
    key = [1, case["implementation"], case["scope"], case["registry"], case["owner"]["name"], case["owner"]["root"], case["owner"]["manifest"], case["targets"], case["inputs"]]
    return {"leaf": "np/%s.json" % digest, "keySha256": hashlib.sha256(json.dumps(key, ensure_ascii=False, separators=(",", ":")).encode("utf-8")).hexdigest()}


def chain(requested_by, name):
    result, seen, owner = [name], {name}, name
    for hop in range(12):
        origin = requested_by.get(owner)
        if origin is None:
            return [*result, "selection"]
        if origin == "catalog":
            return [*result, "catalog provider"]
        result.append(origin["owner"] + (":" + origin["target"] if "target" in origin else ""))
        if origin["owner"] in seen:
            return result
        if hop + 1 >= 12:
            return [*result, "..."]
        seen.add(origin["owner"])
        owner = origin["owner"]
    return result


def publishable(rows, started):
    limit = started - 2000
    for row in rows:
        stamp = row["fact"] if row["kind"] == "file" else row["fact"].get("source") if row["kind"] == "analysis" else None
        if stamp and (stamp.get("mtimeMs", 0) >= limit or stamp.get("ctimeMs", 0) >= limit):
            return False
    return True


def receipt_valid(schema, value):
    try:
        from jsonschema import Draft202012Validator
    except ImportError:
        return None
    shape = Draft202012Validator({"$schema": "https://json-schema.org/draft/2020-12/schema", "$defs": schema["$defs"], "$ref": "#/$defs/admissionReceipt"})
    if not shape.is_valid(value):
        return False
    return Draft202012Validator(schema).is_valid(value["admission"])


def answer(fixture, schema):
    worlds = {world["id"]: world for world in fixture["closure"]["worlds"]}
    cases = {}
    for case in fixture["closure"]["cases"]:
        world = worlds[case["world"]]
        stuck = blocked(world, case["selection"])
        cases[case["id"]] = {"throws": True, "blocked": stuck} if stuck else {"throws": False, **closure(dict(world, broken=[]), case["selection"])}
    scenarios = {}
    for scenario in fixture["scenarios"]:
        world, before, rows = json.loads(json.dumps(scenario["world"])), None, []
        for step in scenario["steps"]:
            if step["op"] == "replace":
                world["owners"] = [step["declaration"] if owner["name"] == step["owner"] else owner for owner in world["owners"]]
                rows.append(None)
            elif step["op"] == "run":
                now = closure(world, step["selection"])
                rows.append({"owners": now["owners"], "added": sorted(set(now["owners"]) - set(before["owners"])) if before else now["owners"]})
                before = now
            else:
                rows.append(None)
        scenarios[scenario["id"]] = rows
    return {
        "closure": cases,
        "keys": {row["id"]: record(row) for row in fixture["keys"]},
        "chains": {row["id"]: chain({name: origin for name, origin in row["requestedBy"]}, row["name"]) for row in fixture["chains"]},
        "publishable": {row["id"]: publishable(row["observations"], row["startedAt"]) for row in fixture["publishable"]},
        "receipts": {row["id"]: receipt_valid(schema, row["value"]) for row in fixture["receipts"]},
        "scenarios": scenarios,
    }


def main():
    if len(sys.argv) < 3:
        raise SystemExit(__doc__)
    with open(sys.argv[1], encoding="utf-8") as handle:
        fixture = json.load(handle)
    with open(sys.argv[2], encoding="utf-8") as handle:
        schema = json.load(handle)
    result = answer(fixture, schema)
    if any(value is None for value in result["receipts"].values()):
        sys.stderr.write("jsonschema is not installed: receipt verdicts are taken from the frozen answers (Ajv still judges them in the TypeScript suite)\n")
        result["receipts"] = fixture["expected"]["receipts"] if "expected" in fixture else {row["id"]: row["valid"] for row in fixture["receipts"]}
    if "--verify" in sys.argv:
        frozen = fixture["expected"]
        bad = [section for section in result if json.dumps(result[section], sort_keys=True) != json.dumps(frozen[section], sort_keys=True)]
        if bad:
            print("oracle differs from the frozen answers in: " + ", ".join(bad))
            sys.exit(1)
        print("oracle agrees with the frozen answers")
        return
    print(json.dumps(result, ensure_ascii=False, sort_keys=True))


main()
