#!/usr/bin/env python3
"""🧮 Reference decision of the service invocation admission, written independently of the TypeScript implementation.

usage: oracle.py CORPUS SCHEMA CATALOGUE   -> prints one JSON document {"requests": [...codes], "arguments": [...codes]}
The corpus is symbolic; this program applies the authored policy (SCHEMA) and the operation catalogue (CATALOGUE) to it and decides every row.
"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf-8"))
policy = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]["Policy"]["const"]
catalogue = json.load(open(sys.argv[3], encoding="utf-8"))
NOW = corpus["clock"]
DEFAULT_OPERATIONS = policy["operations"]
BASE = [("scope", ["run"]), ("worktree", "@worktree"), ("agent", "codex-1"), ("notAfter", {"@now": 60000})]


def value(v):
    if isinstance(v, dict) and "@now" in v:
        return NOW + v["@now"]
    return v


def arguments(args, operations):
    if not 1 <= len(args) <= policy["maximumArguments"]:
        return "arguments", []
    op = args[0]
    entry = next((o for o in catalogue["operations"] if o["name"] == op), None)
    if entry is None:
        return "arguments", []
    if op not in operations:
        return "scope", []
    allowed = set(entry.get("flags", []))
    flags = {f["name"]: f for f in catalogue["flags"] if "operations" not in f or op in f["operations"]}
    positionals, projects, index = [], [], 1
    while index < len(args):
        arg = args[index]
        if not arg.startswith("-"):
            positionals.append(arg)
        elif not arg.startswith("--"):
            return "arguments", []
        else:
            name, equals, inline = arg.partition("=")
            inline = inline if equals else None
            spec = flags.get(name)
            if name != "--tool" and (name in policy["forbiddenFlags"] or name not in allowed or spec is None):
                return "arguments", []
            if spec is None:
                return "arguments", []
            val = None
            if spec["value"] == "none":
                if inline is not None:
                    return "arguments", []
            elif spec["value"] == "required":
                if inline is not None:
                    val = inline
                else:
                    index += 1
                    val = args[index] if index < len(args) else None
                if val is None or val.startswith("--"):
                    return "arguments", []
            elif spec["value"] == "optional":
                val = inline
            else:
                return "arguments", []
            if name == "--tool" and val != "nx":
                return "tool", []
            if policy["projectFlags"].get(op) == name and val is not None:
                projects = [p for p in val.split(",") if p]
        index += 1
    low, high = policy["positionals"][op]
    if not low <= len(positionals) <= high:
        return "arguments", []
    for task in positionals:
        if not re.search(policy["taskPattern"], task):
            return "arguments", []
    return "ok", ([positionals[0].split(":")[0]] if op == "run" else projects)


def caveats_of(token):
    caveats = [{"k": k, "v": value(v)} for k, v in BASE if k not in token.get("drop", [])]
    caveats += [{"k": c["k"], "v": value(c["v"])} for c in token.get("extra", [])]
    caveats += [{"k": c["k"], "v": value(c["v"])} for c in token.get("attenuate", [])]
    return caveats


def authorize(caveats, cwd, named, agent):
    for c in caveats:
        k, v = c["k"], c["v"]
        if k == "scope":
            if "run" not in v:
                return "scope"
        elif k == "notAfter":
            if NOW > v:
                return "expired"
        elif k == "cwd":
            v = v.replace("@root", "/ROOT")
            if not (cwd == v or cwd.startswith(v if v.endswith("/") else v + "/")):
                return "scope"
        elif k == "projects":
            if named and not all(any((p.startswith(x[:-1]) if x.endswith("*") else p == x or p.startswith(x + ":")) for x in v) for p in named):
                return "scope"
        elif k == "worktree":
            if v != "@worktree":
                return "worktree"
        elif k == "agent":
            if v != agent:
                return "agent"
        else:
            return "token"
    return None


def decide_request(case):
    token = case["token"]
    if token["kind"] == "empty":
        return "token"
    if token["kind"] != "genuine":
        return "token"
    caveats = caveats_of(token)
    present = {c["k"] for c in caveats}
    for required in policy["requiredCaveats"]:
        if required not in present:
            return "caveats"
    not_after = min(c["v"] for c in caveats if c["k"] == "notAfter")
    if NOW > not_after:
        return "expired"
    if not_after - NOW > policy["maximumLifetimeMilliseconds"]:
        return "unbounded"
    where = case.get("cwd", "inside")
    if where != "inside":
        return "worktree"
    code, named = arguments(case.get("args", ["run", "app:build"]), case.get("grantOperations", DEFAULT_OPERATIONS))
    if code != "ok":
        return code
    agent = case.get("agent", "codex-1")
    denied = authorize(caveats, "/ROOT/pkgs/a", named, agent)
    if denied:
        return denied
    if not named and any(c["k"] == "projects" for c in caveats):
        return "scope"
    if case.get("revoked"):
        return "revoked"
    return "ok"


def decide_arguments(case):
    token = case.get("token", {})
    code, named = arguments(case["args"], case.get("grantOperations", DEFAULT_OPERATIONS))
    if code != "ok":
        return code, named
    caveats = caveats_of(token)
    denied = authorize(caveats, "/ROOT/pkgs/a", named, "codex-1")
    if not denied and not named and any(c["k"] == "projects" for c in caveats):
        denied = "scope"
    return (denied or "ok"), named


result = {"requests": [decide_request(case) for case in corpus["requests"]], "arguments": []}
for case in corpus["arguments"]:
    code, named = decide_arguments(case)
    result["arguments"].append({"code": code, "named": named if code == "ok" else None})
json.dump(result, sys.stdout)
