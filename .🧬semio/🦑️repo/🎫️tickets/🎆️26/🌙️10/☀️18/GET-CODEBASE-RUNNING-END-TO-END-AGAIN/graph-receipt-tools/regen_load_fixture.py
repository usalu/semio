#!/usr/bin/env python3
"""Regenerates the `implementation` and `shapeImplementation` sections of the load fixture with an independent Python model (no shared code with the module under test).

usage: regen_load_fixture.py [--write]"""
import hashlib, json, os, sys

WS = "/Users/ueli/Documents/semio"
LIB = WS + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🔣️.json"


def canonical(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True)


def identity(row):
    return canonical([row["kind"], row["path"], row["fact"]])


def implementation_model(rows):
    seen, out = set(), {"all": [], "environment": []}
    for row in rows:
        plain = {k: v for k, v in row.items() if k != "scope"}
        if identity(plain) in seen:
            continue
        seen.add(identity(plain))
        if plain["kind"] != "stamp":
            out["all"].append(plain)
        elif row.get("scope") in ("environment", "all"):
            out["environment"].append(plain)
    pairs = sorted([[row["kind"], row["path"], row["fact"] if row["kind"] == "stamp" else row["fact"]["sha256"]] for row in rows], key=lambda p: (p[1], str(p[2])))
    out["digest"] = hashlib.sha256(canonical(pairs).encode("utf8")).hexdigest()
    return out


def digest_row(path, tag):
    return {"kind": "digest", "path": path, "fact": {"sha256": tag * 64, "size": 10, "mtimeMs": 1700000000000.5}}


def stamp_row(id_, stamp, scope):
    return {"kind": "stamp", "path": "inferrer:" + id_, "fact": stamp, "scope": scope}


def shape_model(workspace, part, stubs, implementation):
    processed = {o["name"] for o in part["owners"]} | {o["name"] for o in part["edgeOnly"]}
    edges_of, other = {}, []
    for edge in part["edges"]:
        (edges_of.setdefault(edge["source"], []) if edge["source"] in processed else other).append(edge)
    claimed, owners = set(), []

    def claim(base, roots):
        keys, rows = {identity(r) for r in base}, list(base)
        for root in roots:
            if not root or root == ".":
                continue
            prefix = workspace + "/" + root + "/"
            for row in part["rows"]:
                if row["path"].startswith(prefix) and identity(row) not in keys:
                    keys.add(identity(row))
                    rows.append(row)
        claimed.update(keys)
        return rows

    for o in part["owners"]:
        entry = {"name": o["name"], "root": o["root"], "kind": "unavailable" if o.get("unavailable") else "project", "node": o.get("unavailable") or o["node"], "edges": edges_of.get(o["name"], []), "targets": [] if o.get("unavailable") else list(o["node"].get("targets", {}).keys()), "rows": claim(o.get("nodeRows", []), [o["root"], o.get("nativeRoot")])}
        for field in ("config", "nativeRoot", "tag"):
            if o.get(field):
                entry[field] = o[field]
        owners.append(entry)
    for o in part["edgeOnly"]:
        stub = stubs[o["name"]]
        owners.append({"name": o["name"], "partial": "edges", "edges": edges_of.get(o["name"], []), "rows": claim([], [stub["root"], stub.get("nativeRoot")])})
    for entry in owners:
        if "partial" not in entry:
            entry["rows"] = entry["rows"] + implementation["all"]
    env_rows = part["env"]["rows"] + [r for r in part["rows"] if identity(r) not in claimed] + implementation["all"] + implementation["environment"]
    return {"owners": owners, "env": {"digest": part["env"]["digest"], "rows": env_rows}, "global": {"version": 1, "externalNodes": part["externalNodes"], "edges": other, "unavailable": part["parseUnavailable"]}, "replaceTags": part.get("replaceTags", [])}


vectors = [
    [],
    [digest_row("lib/data/policy.json", "1")],
    [stamp_row("nodes", "semio.graph-inferrer.nodes/1", "project"), stamp_row("native", "semio.graph-inferrer.native/1", "native")],
    [stamp_row("core", "semio.graph-inferrer.core/1", "all"), stamp_row("edges", "semio.graph-inferrer.edges/1", "environment"), digest_row("lib/data/policy.json", "2")],
    [digest_row("lib/data/a.json", "4"), digest_row("lib/data/b.json", "5"), digest_row("lib/data/a.json", "4")],
    [digest_row("lib/data/a.json", "4"), digest_row("lib/data/a.json", "6")],
    [stamp_row("core", "semio.graph-inferrer.core/1", "all"), stamp_row("core", "semio.graph-inferrer.core/1", "all"), stamp_row("core", "semio.graph-inferrer.core/2", "all"), digest_row("lib/data/ü\U0001F600.json", "8")],
]

path = LIB
for name in os.listdir(os.path.dirname(path)):
    pass
fixture = json.load(open(path, encoding="utf8"))
fixture["implementation"] = [{"rows": rows, "expected": implementation_model(rows)} for rows in vectors]
shape = fixture["shapeImplementation"]
implementation = {"all": [digest_row("lib/data/policy.json", "4")], "environment": [stamp_row("edges", "semio.graph-inferrer.edges/1", "environment"), stamp_row("core", "semio.graph-inferrer.core/1", "all")]}
shape["implementation"] = implementation
shape["expected"] = shape_model(shape["workspaceRoot"], shape["part"], shape["stubs"], implementation)
text = json.dumps(fixture, ensure_ascii=False, indent=1, sort_keys=True) + "\n"
if "--write" in sys.argv:
    open(path, "w", encoding="utf8").write(text)
    print("written", len(text))
else:
    print(len(text))
