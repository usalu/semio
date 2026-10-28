"""🐍 Independent oracle of the admission memo key: sha-256 of the canonical JSON of [namespace, toolchain, policy, keyed unit facts] (stdlib only). usage: python -I this.py fixture.json"""
import hashlib, json, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)

def mutate(unit, assignment):
    copy = json.loads(json.dumps(unit))
    for path, value in assignment.items():
        parts = path.split(".")
        at = copy
        for part in parts[:-1]:
            at = at.setdefault(part, {})
        at[parts[-1]] = value
    return copy

def key(unit, toolchain):
    rest = {name: unit.get(name) for name in fixture["keyedFields"]}
    environment = unit.get("compilerEnvironment")
    rest["environment"] = None if environment is None else {k: v for k, v in environment.items() if k not in fixture["volatileEnvironment"]}
    return hashlib.sha256(canonical([fixture["namespace"], toolchain, fixture["policy"], rest]).encode("utf8")).hexdigest()

variants = fixture["same"] + fixture["different"]
print(json.dumps({"base": key(fixture["unit"], fixture["toolchain"]), "keys": [key(mutate(fixture["unit"], row["set"]), row.get("toolchain", fixture["toolchain"])) for row in variants]}))
