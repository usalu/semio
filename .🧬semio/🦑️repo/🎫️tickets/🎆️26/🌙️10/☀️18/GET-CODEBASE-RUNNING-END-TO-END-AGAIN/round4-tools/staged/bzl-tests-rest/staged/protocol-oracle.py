#!/usr/bin/env python3
"""Independent Python implementation of the test protocol: reads `{protocol, fixture}` JSON on stdin and prints the answer to every fixture section. The XML side uses the standard library parser, not the scanner of the TypeScript module."""
import hashlib, json, re, sys
import xml.etree.ElementTree as ET

def utf16(value):
    return value.encode("utf-16-be")

def last(value):
    return re.split(r"[\\/]", value)[-1]

def read_shard(protocol, environment):
    value = environment.get(protocol["environment"]["shard"])
    if value is None or value == "":
        return None
    match = re.fullmatch(r"([1-9][0-9]*)/([2-9][0-9]*)", value)
    if not match:
        return "refused"
    index, total = int(match.group(1)), int(match.group(2))
    if index > total or total > protocol["shard"]["maximumTotal"]:
        return "refused"
    return {"index": index, "total": total}

def shard_files(files, shard):
    if shard is None:
        return list(files)
    ordered = sorted(set(files), key=utf16)
    return [file for position, file in enumerate(ordered) if position % shard["total"] == shard["index"] - 1]

def refusing(args, flags):
    return any(a == f or a.startswith(f + "=") for a in args for f in flags)

def match_tool(protocol, command, args):
    for tool in protocol["tools"]:
        m = tool["match"]
        if "commandBasenames" in m and last(command) not in m["commandBasenames"]:
            continue
        if "firstArgument" in m:
            rest = list(args[1:] if m.get("skipToolchain") and args and args[0].startswith("+") else args)
            if not rest or rest[0] != m["firstArgument"]:
                continue
        if "argumentBasenames" in m and not any(last(a) in m["argumentBasenames"] for a in args):
            continue
        if "alsoContains" in m and m["alsoContains"] not in args:
            continue
        return tool
    return None

def plan(protocol, command, args, shard, junit):
    tool = match_tool(protocol, command, args)
    if tool is None:
        return {"tool": None, "args": list(args), "sharded": False, "junit": None}
    flags = []
    sharded = shard is not None and "shardFlag" in tool and not refusing(args, tool.get("shardRefusedBy", []))
    if sharded:
        flags.append(tool["shardFlag"].replace("${index}", str(shard["index"])).replace("${total}", str(shard["total"])))
    used = junit if junit is not None and "junitFlags" in tool and not refusing(args, tool.get("junitRefusedBy", [])) else None
    if used is not None:
        flags += [f.replace("${file}", used) for f in tool["junitFlags"]]
    if flags:
        stop = args.index("--") if "--" in args else len(args)
        rewritten = list(args[:stop]) + flags + list(args[stop:])
    else:
        rewritten = list(args)
    return {"tool": tool["id"], "args": rewritten, "sharded": sharded, "junit": used}

def number(value, default):
    try:
        return float(value) if value is not None else default
    except ValueError:
        return default

def suites(xml):
    root = ET.fromstring(xml)
    top = [root] if root.tag == "testsuite" else [child for child in root if child.tag == "testsuite"]
    rows = []
    for suite in top:
        def count(name, tag):
            value = suite.get(name)
            if value is not None and re.fullmatch(r"\d+", value):
                return int(value)
            return sum(1 for _ in suite.iter(tag))
        time = number(suite.get("time"), 0.0)
        rows.append({"name": suite.get("name", "suite"), "tests": count("tests", "testcase"), "failures": count("failures", "failure"), "errors": count("errors", "error"), "skipped": count("skipped", "skipped"), "time": time if time >= 0 else 0.0})
    return rows

def slug(protocol, value):
    return re.sub(r"[^A-Za-z0-9._-]", "_", value)[: protocol["report"]["maximumSlugCodeUnits"]]

def report_path(protocol, environment):
    names = protocol["environment"]
    explicit = environment.get(names["xmlOutputFile"])
    if explicit:
        if not explicit.startswith("/") and not re.match(r"^[A-Za-z]:[\\/]", explicit):
            return "refused"
        return explicit
    if environment.get(names["report"]) == "0":
        return None
    root = environment.get(protocol["report"]["root"])
    if not root or not root.startswith("/"):
        return None
    for row in protocol["report"]["identity"]:
        project, target = environment.get(row["project"]), environment.get(row["target"])
        if project and target and re.search(protocol["report"]["defaultTargets"], target):
            digest = hashlib.sha256(project.encode()).hexdigest()[:8]
            return "/".join([root.rstrip("/"), protocol["report"]["resultsRoot"], "%s-%s" % (slug(protocol, project), digest), slug(protocol, target), protocol["report"]["fileName"]])
    return None

def totals(rows):
    return {key: sum(row[key] for row in rows) for key in ("tests", "failures", "errors", "skipped", "time")}

def main():
    data = json.load(sys.stdin)
    protocol, fixture = data["protocol"], data["fixture"]
    out = {
        "shards": [shard_files(row["files"], None if row["shard"] is None else read_shard(protocol, {protocol["environment"]["shard"]: row["shard"]})) for row in fixture["shards"]],
        "reads": [read_shard(protocol, row["environment"]) for row in fixture["reads"]],
        "refusedShards": [read_shard(protocol, {protocol["environment"]["shard"]: value}) for value in fixture["refusedShards"]],
        "plans": [plan(protocol, row["command"], row["args"], None if row["shard"] is None else read_shard(protocol, {protocol["environment"]["shard"]: row["shard"]}), row["junit"]) for row in fixture["plans"]],
        "junit": [suites(row["xml"]) for row in fixture["junit"]],
        "paths": [report_path(protocol, row["environment"]) for row in fixture["paths"]],
        "refusedPaths": [report_path(protocol, environment) for environment in fixture["refusedPaths"]],
        "reports": [totals(row["suites"]) for row in fixture["reports"]],
    }
    json.dump(out, sys.stdout, ensure_ascii=False)

if __name__ == "__main__":
    main()
