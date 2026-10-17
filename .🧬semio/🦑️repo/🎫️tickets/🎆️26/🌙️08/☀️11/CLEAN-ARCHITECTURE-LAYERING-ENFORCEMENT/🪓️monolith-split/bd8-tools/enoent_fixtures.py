import os, sys, collections, json
root = "/Users/ueli/Documents/semio"
F = sys.argv[1]
rows = [l.split("\t") for l in open(F + "/🗑️generated/bd8-ts-enoent.txt", encoding="utf-8").read().split("\n") if l]
res = {"rule": [], "rest": []}
for src, line, rel, tgt in rows:
    parts = src.split("/")
    # owner = prefix before the LAST '🧪️tests' segment of the test source
    idx = max(i for i, p in enumerate(parts) if p == "🧪️tests") if "🧪️tests" in parts else -1
    hit = None
    if idx >= 0:
        owner = "/".join(parts[:idx])
        t = tgt
        if t.startswith(owner + "/"):
            r = t[len(owner) + 1:]
            for head in ("🧬️schema/", "🧫️fixtures/", "🖼️assets/"):
                if r.startswith(head):
                    cand = owner + "/🧪️tests/🧫️fixtures/" + r[len(head):]
                    if os.path.isfile(os.path.join(root, cand)): hit = cand
    (res["rule"] if hit else res["rest"]).append((src, line, rel, tgt, hit))
print(len(res["rule"]), len(res["rest"]))
json.dump(res, open(F + "/🗑️generated/bd8-ts-enoent-rule.json", "w"), ensure_ascii=False, indent=1)
for src, line, rel, tgt, hit in res["rule"]: print("RULE", src[-70:], line, rel[-60:])
