"""🏭 Independent Python oracle of the generator output coupling (standard library only): contract inputs, output ownership, declared output paths, check/generate coupling and the content digest of an output root. Requests arrive as JSON on stdin: {"policy", "workspaceRoot", "requests": [...]}; the digest request reads the tree under a given directory."""
import hashlib, json, os, posixpath, re, sys


class Refused(Exception):
    pass


def contract_inputs(contract, generator_inputs):
    kind = contract.get("inputDiscovery", {}).get("kind") if contract.get("inputDiscovery") else None
    fingerprint = generator_inputs.get(kind) if kind is not None else None
    if contract.get("inputDiscovery") and not fingerprint:
        raise Refused("Generator input discovery has no fingerprint owner: " + kind)
    inputs = ["{workspaceRoot}/" + path for path in contract["inputPatterns"]]
    if fingerprint:
        inputs.append({"dependentTasksOutputFiles": fingerprint["output"]})
    return {"inputs": inputs, "dependsOn": [fingerprint["target"]] if fingerprint else []}


def owners(contracts, root, project, targets):
    out = {}
    for contract in contracts.values():
        if contract["ownership"] != "owned":
            continue
        if contract["ownerPath"] == root:
            out.setdefault(contract["target"], [])
        for output in contract["outputRoots"]:
            owner = output.get("producer") or contract
            if owner["ownerPath"] != root:
                continue
            target = owner.get("target")
            if not (isinstance(target, str) and target.startswith(project + ":")) or target[len(project) + 1:] not in targets:
                raise Refused("Generator output has no Build producer: " + output["path"])
            out.setdefault(target, []).append("{workspaceRoot}/" + output["path"])
    return [[key, value] for key, value in out.items()]


def output_path(output, root, workspace):
    if not (output.startswith("{projectRoot}/") or output.startswith("{workspaceRoot}/")):
        return None
    replaced = output.replace("{projectRoot}", root, 1).replace("{workspaceRoot}", ".", 1)
    return posixpath.normpath(posixpath.join(workspace, replaced))


def couplings(name, target_inputs, targets, root, workspace):
    if not re.match(r"^check(?:-|$)", name):
        return []
    seen = set(target_inputs)
    out = []
    for sibling, definition in targets.items():
        if sibling == name or not re.match(r"^generate(?:-|$)", sibling):
            continue
        for output in definition.get("outputs", []):
            absolute = output_path(output, root, workspace)
            if absolute is None or absolute in seen:
                continue
            seen.add(absolute)
            out.append(absolute)
    return out


def digest(path):
    h = hashlib.sha256()

    def walk(entry):
        if not os.path.lexists(entry) or (os.path.islink(entry) and not os.path.exists(entry)):
            return
        if os.path.islink(entry):
            return
        if os.path.isdir(entry):
            for name in sorted(os.listdir(entry)):
                walk(os.path.join(entry, name))
        elif os.path.isfile(entry):
            h.update((os.path.relpath(entry, path).replace(os.sep, "/") if entry != path else "").encode("utf-8") + b"\0")
            h.update(open(entry, "rb").read())

    walk(path)
    return h.hexdigest()


def run(request, context):
    kind = request["kind"]
    try:
        if kind == "inputs":
            return {"result": contract_inputs(request["contract"], context["policy"])}
        if kind == "owners":
            return {"result": owners(request["contracts"], request["root"], request["project"], set(request["targets"]))}
        if kind == "path":
            return {"result": output_path(request["output"], request["root"], context["workspaceRoot"])}
        if kind == "coupling":
            return {"result": couplings(request["name"], request["authored"], request["targets"], request["root"], context["workspaceRoot"])}
        if kind == "digest":
            return {"result": digest(request["path"])}
    except Refused as error:
        return {"error": str(error)}
    raise SystemExit("unknown request " + kind)


context = json.load(sys.stdin)
print(json.dumps([run(request, context) for request in context["requests"]], ensure_ascii=False))
