#!/usr/bin/env python3
# 🐍 Independent implementation of the native toolchain contract: reads {policy, environments, identities, tools, links, files} on stdin and prints the expected results as JSON. Standard library only; shares no code with the TypeScript engine.
import hashlib, json, posixpath, sys, xml.etree.ElementTree as xml

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)

def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()

def series_of(policy, project_id):
    project = next(row for row in policy["python"]["projects"] if row["id"] == project_id)
    return policy["python"]["interpreters"][project["interpreter"]]["series"]

def environment_row(policy, row):
    project = next(item for item in policy["python"]["projects"] if item["id"] == row["project"])
    series = series_of(policy, row["project"])
    key = project["id"] + "-" + digest(canonical({"project": project["id"], "uv": policy["python"]["uv"], "series": series, "host": row["host"], "groups": project["groups"], "manifest": row["manifest"], "lock": row["lock"]}))[:16]
    layout = policy["python"]["layout"]
    directory = posixpath.join(row["roots"]["dependencies"], layout["environments"], key)
    interpreters = posixpath.join(row["roots"]["tools"], layout["interpreters"])
    cache = posixpath.join(row["roots"]["dependencies"], layout["cache"])
    variables = dict(policy["python"]["variables"])
    variables.update({"UV_PROJECT_ENVIRONMENT": directory, "UV_PYTHON_INSTALL_DIR": interpreters, "UV_PYTHON": series, "UV_CACHE_DIR": cache})
    groups = [argument for group in project["groups"] for argument in ("--group", group)]
    return {"key": key, "directory": directory, "interpreters": interpreters, "cache": cache, "series": series, "variables": variables,
            "installArguments": ["python", "install", series],
            "syncArguments": ["sync", "--locked", "--project", project["root"]] + groups,
            "runArguments": ["run", "--locked", "--no-sync", "--project", project["root"]]}

def tool_row(policy, tool, host, root):
    spec = policy["go"]["tool"] if tool == "go" else policy["cmake"][tool]
    dist = next(row for row in spec["distributions"] if row["host"] == host)
    directory = posixpath.join(root, "native", tool, spec["version"], host + "-" + dist["sha256"][:16])
    binary = directory if dist["binaryDirectory"] == "." else posixpath.join(directory, dist["binaryDirectory"])
    return {"directory": directory, "binaryDirectory": binary, "url": spec["release"] + "/" + dist["archive"], "identity": "%s:%s:%s:%s" % (tool, spec["version"], host, dist["sha256"])}

def identity_row(policy, row):
    kind, host = row["kind"], row["host"]
    if kind == "python":
        series = sorted({series_of(policy, p["id"]) for p in policy["python"]["projects"]}) if row.get("project") is None else [series_of(policy, row["project"])]
        pins = {"uv": policy["python"]["uv"], "series": series}
    elif kind == "dotnet":
        pins = {"sdk": policy["dotnet"]["sdk"], "rollForward": policy["dotnet"]["rollForward"]}
    elif kind == "cmake":
        pins = {}
        for tool in ("cmake", "ninja"):
            dist = next(item for item in policy["cmake"][tool]["distributions"] if item["host"] == host)
            pins[tool] = {"version": policy["cmake"][tool]["version"], "sha256": dist["sha256"]}
    else:
        dist = next(item for item in policy["go"]["tool"]["distributions"] if item["host"] == host)
        pins = {"version": policy["go"]["version"], "sha256": dist["sha256"]}
    names = policy[kind]["environment"]
    environment = {name: row["environment"][name] for name in names if name in row.get("environment", {})}
    return {"digest": digest(canonical({"kind": kind, "host": host, "pins": pins, "environment": environment})), "pins": pins, "environment": environment}

def escapes(path, target):
    if target[:1] in ("/", "\\") or (len(target) > 1 and target[1] == ":" and target[0].isalpha()):
        return True
    stack = [part for part in path.replace("\\", "/").split("/")[:-1] if part not in ("", ".")]
    for part in target.replace("\\", "/").split("/"):
        if part in ("", "."):
            continue
        if part == "..":
            if not stack:
                return True
            stack.pop()
        else:
            stack.append(part)
    return False

def files(policy):
    sdk = {"sdk": {"version": policy["dotnet"]["sdk"], "rollForward": policy["dotnet"]["rollForward"]}}
    return {"globalJson": json.dumps(sdk, indent=2, ensure_ascii=False) + "\n", "artifacts": policy["dotnet"]["artifacts"]}

def props_facts(text):
    tree = xml.fromstring(text)
    group = tree.find("PropertyGroup")
    return {"root": tree.tag, "use": group.find("UseArtifactsOutput").text, "condition": group.find("ArtifactsPath").attrib["Condition"], "path": group.find("ArtifactsPath").text}

def main():
    data = json.load(sys.stdin)
    policy = data["policy"]
    out = {
        "environments": [environment_row(policy, row) for row in data["environments"]],
        "identities": [identity_row(policy, row) for row in data["identities"]],
        "tools": [dict(row, **tool_row(policy, row["tool"], row["host"], row["root"])) for row in data["tools"]],
        "links": [escapes(row["path"], row["target"]) for row in data["links"]],
        "files": files(policy),
        "props": props_facts(data["props"]) if "props" in data else None,
    }
    sys.stdout.write(json.dumps(out, ensure_ascii=False))

main()
