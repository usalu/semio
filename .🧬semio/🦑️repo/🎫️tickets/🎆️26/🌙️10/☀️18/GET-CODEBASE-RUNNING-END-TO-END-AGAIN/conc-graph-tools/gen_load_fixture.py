#!/usr/bin/env python3
"""Generates the fixture of the load laws: policy flag vectors, language vectors and shaping vectors, answered by an independent Python model.

usage: gen_load_fixture.py"""
import json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "staged", "new", "load")
POLICIES = ["fresh", "current"]


def policy_model(args, env):
    policy, rest, index, explicit = env.get("SEMIO_BUILD_GRAPH_POLICY", "fresh"), [], 0, "SEMIO_BUILD_GRAPH_POLICY" in env
    while index < len(args):
        argument = args[index]
        if argument == "--":
            rest.extend(args[index:])
            break
        if argument == "--graph-policy":
            index += 1
            policy = args[index] if index < len(args) else ""
            explicit = True
        elif argument.startswith("--graph-policy="):
            policy = argument[len("--graph-policy="):]
            explicit = True
        else:
            rest.append(argument)
        index += 1
    return {"error": True} if policy not in POLICIES else {"policy": policy, "explicit": explicit, "args": rest}


def language_model(env):
    for name in ["SEMIO_DASHBOARD_LOCALE", "LC_ALL", "LC_MESSAGES", "LANG"]:
        value = env.get(name)
        if value is not None:
            head = re.split(r"[_.@-]", value)[0].lower()
            if head in ("en", "de"):
                return head
    return "en"


policy_vectors = [
    (["show", "projects"], {}), (["show", "projects", "--graph-policy=current"], {}), (["--graph-policy", "current", "show", "projects"], {}), (["show", "--graph-policy"], {}),
    (["exec", "--graph", "out.json", "--", "--graph-policy=current"], {}), (["show", "projects"], {"SEMIO_BUILD_GRAPH_POLICY": "current"}), (["show", "projects", "--graph-policy=fresh"], {"SEMIO_BUILD_GRAPH_POLICY": "current"}),
    (["show", "projects", "--graph-policy=stale"], {}), (["show", "projects"], {"SEMIO_BUILD_GRAPH_POLICY": "x"}), (["run", "a:b", "--graph=current"], {}),
]
language_vectors = [{}, {"LANG": "de_DE.UTF-8"}, {"LANG": "en_US.UTF-8"}, {"LC_ALL": "de"}, {"LC_ALL": "fr_FR", "LANG": "de_DE"}, {"LC_ALL": "C", "LANG": "de_DE.UTF-8"}, {"SEMIO_DASHBOARD_LOCALE": "de", "LANG": "en_US"}, {"LC_MESSAGES": "DE_CH@euro"}]

W = "/w"
part = {
    "owners": [
        {"name": "a", "root": "pkg/a", "config": "pkg/a/project.json", "node": {"name": "a", "root": "pkg/a", "targets": {"build": {}, "test": {}}}, "nodeRows": [{"kind": "file", "path": W + "/pkg/a/project.json", "fact": {"s": 1}}, {"kind": "file", "path": W + "/shared/x.json", "fact": {"s": 2}}]},
        {"name": "root", "root": ".", "node": {"name": "root", "root": ".", "targets": {}}, "nodeRows": [{"kind": "file", "path": W + "/own.json", "fact": {"s": 3}}]},
        {"name": "u", "root": "pkg/u", "unavailable": {"root": "pkg/u", "project": "u", "source": "pkg/u/project.json", "sha256": "0" * 64, "reason": "bad"}, "nodeRows": []},
        {"name": "t", "root": "pkg/a/tests", "tag": "tests", "node": {"name": "t", "root": "pkg/a/tests", "targets": {"test": {}}}, "nodeRows": []},
    ],
    "edgeOnly": [{"name": "b"}],
    "edges": [{"source": "a", "target": "b"}, {"source": "b", "target": "a"}, {"source": "npm:x", "target": "npm:y"}, {"source": "t", "target": "a"}],
    "externalNodes": {"npm:x": {}},
    "parseUnavailable": [{"root": "pkg/z", "source": "pkg/z/project.json", "sha256": "1" * 64, "reason": "json"}],
    "rows": [
        {"kind": "file", "path": W + "/pkg/a/project.json", "fact": {"s": 1}}, {"kind": "file", "path": W + "/pkg/a/src/x.ts", "fact": {"s": 4}}, {"kind": "file", "path": W + "/pkg/b/src/y.ts", "fact": {"s": 5}},
        {"kind": "file", "path": W + "/Cargo.toml", "fact": {"s": 6}}, {"kind": "file", "path": W + "/pkg/a/tests/a.feature", "fact": {"s": 7}},
    ],
    "env": {"digest": "e" * 64, "rows": [{"kind": "field", "path": "pkg/a/project.json", "fact": {"extract": "project-json", "value": {"name": "a", "nativeRoot": None}}}]},
    "replaceTags": ["tests", "policy"],
}
stubs = {"b": {"root": "pkg/b", "kind": "project", "targets": ["build"]}}


def key(row):
    return json.dumps([row["kind"], row["path"], row["fact"]], sort_keys=True, separators=(",", ":"))


def shape_model(workspace, part, stubs):
    processed = {o["name"] for o in part["owners"]} | {o["name"] for o in part["edgeOnly"]}
    edges_of, other = {}, []
    for edge in part["edges"]:
        (edges_of.setdefault(edge["source"], []) if edge["source"] in processed else other).append(edge)
    claimed, owners = set(), []

    def claim(base, roots):
        keys, rows = {key(r) for r in base}, list(base)
        for root in roots:
            if not root or root == ".":
                continue
            prefix = workspace + "/" + root + "/"
            for row in part["rows"]:
                if row["path"].startswith(prefix) and key(row) not in keys:
                    keys.add(key(row))
                    rows.append(row)
        claimed.update(keys)
        return rows

    for o in part["owners"]:
        entry = {"name": o["name"], "root": o["root"], "kind": "unavailable" if o.get("unavailable") else "project", "node": o.get("unavailable") or o["node"], "edges": edges_of.get(o["name"], []), "targets": [] if o.get("unavailable") else sorted(o["node"].get("targets", {})), "rows": claim(o.get("nodeRows", []), [o["root"], o.get("nativeRoot")])}
        for field in ("config", "nativeRoot", "tag"):
            if o.get(field):
                entry[field] = o[field]
        owners.append(entry)
    for o in part["edgeOnly"]:
        stub = stubs[o["name"]]
        owners.append({"name": o["name"], "partial": "edges", "edges": edges_of.get(o["name"], []), "rows": claim([], [stub["root"], stub.get("nativeRoot")])})
    env_rows = part["env"]["rows"] + [r for r in part["rows"] if key(r) not in claimed]
    return {"owners": owners, "env": {"digest": part["env"]["digest"], "rows": env_rows}, "global": {"version": 1, "externalNodes": part["externalNodes"], "edges": other, "unavailable": part["parseUnavailable"]}, "replaceTags": part["replaceTags"]}


fixture = {
    "version": 1,
    "policy": [{"args": a, "env": e, "expected": policy_model(a, e)} for a, e in policy_vectors],
    "language": [{"env": e, "expected": language_model(e)} for e in language_vectors],
    "shape": {"workspaceRoot": W, "part": part, "stubs": stubs, "expected": shape_model(W, part, stubs)},
}
with open(os.path.join(OUT, "fixture.json"), "w", encoding="utf8") as handle:
    json.dump(fixture, handle, ensure_ascii=False, indent=1, sort_keys=True)
    handle.write("\n")
row_schema = {"type": "object", "required": ["kind", "path", "fact"], "properties": {"kind": {"type": "string"}, "path": {"type": "string"}, "fact": {}}}
schema = {"type": "object", "additionalProperties": False, "required": ["version", "policy", "language", "shape"], "properties": {
    "version": {"const": 1},
    "policy": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["args", "env", "expected"], "properties": {"args": {"type": "array", "items": {"type": "string"}}, "env": {"type": "object", "additionalProperties": {"type": "string"}}, "expected": {"type": "object"}}}},
    "language": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["env", "expected"], "properties": {"env": {"type": "object", "additionalProperties": {"type": "string"}}, "expected": {"enum": ["en", "de"]}}}},
    "shape": {"type": "object", "additionalProperties": False, "required": ["workspaceRoot", "part", "stubs", "expected"], "properties": {"workspaceRoot": {"type": "string"}, "part": {"type": "object"}, "stubs": {"type": "object"}, "expected": {"type": "object"}}}}}
with open(os.path.join(OUT, "fixture-schema.json"), "w", encoding="utf8") as handle:
    json.dump(schema, handle, ensure_ascii=False, indent=1)
    handle.write("\n")
print("policy", len(policy_vectors), "language", len(language_vectors))
