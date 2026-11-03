import json, re, sys
root = sys.argv[1]
profile = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile"
schema = profile + "/🧬️schema/🔣️.json"
fixture = profile + "/🧪️tests/📥️consumption/🔣️.json"
text = open(schema).read()
old = '"prototype-wasm": {\n            "optLevel": "1",\n            "debug": 1,'
assert text.count(old) == 1
open(schema, "w").write(text.replace(old, old.replace('"debug": 1', '"debug": 0')))
body = open(fixture).read()
pattern = re.compile(r'("opt-level": 1,\n(\s+)"debug": )1(,)')
assert len(pattern.findall(body)) == 2
open(fixture, "w").write(pattern.sub(r"\g<1>0\g<3>", body))
print("prototype-wasm debug 1 -> 0 in schema and 2 consumption fixture rows")
