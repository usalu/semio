import hashlib, json, struct, sys
ALPHABET = ["a", "\\", "\n", "ö", '"', "\u0001"]
def source(n):
    return "".join(ALPHABET[i % len(ALPHABET)] for i in range(n))
def digest(text, domain, prefix):
    canonical = json.dumps(text, ensure_ascii=False, separators=(",", ":")).encode()
    h = hashlib.sha256()
    for part in (b"semio.artifact.cursor.v2", struct.pack(">Q", len(domain)), domain, struct.pack(">Q", len(prefix)), prefix, struct.pack(">Q", len(canonical)), canonical):
        h.update(part)
    return len(canonical), list(h.digest())
lengths = sorted(set(list(range(0, 140)) + list(range(180, 270)) + [500, 1000, 4095, 4096, 4097]))
prefix = bytes(range(32))
rows = []
for n in lengths:
    size, d = digest(source(n), b"edit-forward", prefix)
    rows.append({"length": n, "canonicalBytes": size, "digest": d})
json.dump({"domain": "edit-forward", "prefix": list(prefix), "alphabet": ALPHABET, "rows": rows}, sys.stdout, ensure_ascii=False)
