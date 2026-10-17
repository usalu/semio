"""taxadd.py <kind> <name...>: inserts member names into semanticDirectoryMemberKinds.<kind>.memberNames of the taxonomy by a textual edit (first element greater by code point), re-parses the file, asserts nothing else changed."""
import json, re, sys, os
T = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
kind, names = sys.argv[1], sys.argv[2:]
s = open(T, encoding="utf-8").read()
before = json.loads(s)
m = re.search(r'\n(\s*)"' + re.escape(kind) + r'": \{', s[s.index('"semanticDirectoryMemberKinds"'):])
base = s.index('"semanticDirectoryMemberKinds"')
start = base + m.start()
mn = s.index('"memberNames": [', start)
end = s.index("]", mn)
block = s[mn:end]
lines = block.split("\n")
head, items = lines[0], lines[1:]
indent = re.match(r"\s*", items[0]).group(0)
vals = [json.loads(x.strip().rstrip(",")) for x in items if x.strip()]
for name in names:
    assert name not in vals, name
    idx = next((i for i, v in enumerate(vals) if v > name), len(vals))
    vals.insert(idx, name)
tail_ws = items[-1] if not items[-1].strip() else ""
new_block = head + "\n" + ",\n".join(indent + json.dumps(v, ensure_ascii=False) for v in vals) + "\n" + (tail_ws if tail_ws else indent[:-2])
s2 = s[:mn] + new_block + s[end:]
after = json.loads(s2)
assert after["semanticDirectoryMemberKinds"][kind]["memberNames"] == vals
b2 = json.loads(json.dumps(before)); b2["semanticDirectoryMemberKinds"][kind]["memberNames"] = vals
assert b2 == after
tmp = T + ".tc3-tmp"
open(tmp, "w", encoding="utf-8").write(s2)
os.replace(tmp, T)
print("added", names, "to", kind, len(vals))
