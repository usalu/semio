"""🐍 Independent oracle of plan stability: hashes every unit with its dependency indices resolved to unit identities (stdlib only), for two numberings of the same plan. usage: python -I this.py plans.json"""
import hashlib, json, sys

plans = json.load(open(sys.argv[1], encoding="utf8"))

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)

def identity(unit):
    return [unit["pkg_id"], unit["target"]["name"], unit["mode"], unit["platform"], unit["profile"]["name"], sorted(unit["features"])]

def resolved(plan):
    units = plan["units"]
    out = {}
    for unit in units:
        edges = sorted([identity(units[edge["index"]]), edge["extern_crate_name"], edge["public"], edge["noprelude"]] for edge in unit["dependencies"])
        body = {**unit, "dependencies": edges}
        out[canonical(identity(unit))] = hashlib.sha256(canonical(body).encode("utf8")).hexdigest()
    return out

def raw(plan):
    return {canonical(identity(unit)): hashlib.sha256(canonical(unit).encode("utf8")).hexdigest() for unit in plan["units"]}

print(json.dumps({"resolvedEqual": resolved(plans["a"]) == resolved(plans["b"]), "rawEqual": raw(plans["a"]) == raw(plans["b"])}))
