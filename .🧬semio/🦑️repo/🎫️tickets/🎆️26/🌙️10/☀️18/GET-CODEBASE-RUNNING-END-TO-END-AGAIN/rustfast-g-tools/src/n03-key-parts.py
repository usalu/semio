"""🐍 Independent oracle of the unit key parts: the first 8 hex digits of the sha-256 of each part's JSON text (stdlib only). usage: python -I this.py fixture.json"""
import hashlib, json, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))

def part(value):
    return hashlib.sha256(json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest()[:8]

def parts(input):
    return " ".join([
        "toolchain:" + part(input["toolchain"]),
        "args:" + part(input["normalized"]),
        "source:" + part(input["sourceSha256"]),
        "externs:" + part([[row["name"], row["sha256"]] for row in input["externs"]]),
        "environment:" + part(input["environment"]),
    ])

print(json.dumps({"base": parts(fixture["input"]), "variants": [parts({**fixture["input"], **row["set"]}) for row in fixture["variants"]]}, ensure_ascii=False))
