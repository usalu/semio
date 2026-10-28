"""🐍️ Independent decoder of the staged custody resolution inputs: every input is valid UTF-8 JSON array inside the byte bound, and their concatenation hashes like the original edge list. usage: python3 -I 🐍️.py <inputs.json>"""
import base64, hashlib, json, sys
spec = json.load(open(sys.argv[1], encoding="utf8"))
edges = []
for encoded in spec["inputs"]:
    raw = base64.b64decode(encoded, validate=True)
    assert len(raw) <= spec["limit"], "input beyond the byte bound"
    part = json.loads(raw.decode("utf8"))
    assert isinstance(part, list) and part, "closed non-empty array required"
    edges.extend(part)
digest = hashlib.sha256(json.dumps(edges, separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest()
assert digest == spec["digest"], "concatenated edges differ"
print("ok", len(spec["inputs"]), len(edges))
