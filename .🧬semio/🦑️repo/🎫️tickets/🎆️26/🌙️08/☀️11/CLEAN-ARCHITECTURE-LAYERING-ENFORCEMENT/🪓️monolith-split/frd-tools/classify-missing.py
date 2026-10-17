import os, json, sys, collections, re
ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G = F + "/🗑️generated/"
ex = json.load(open(F + "/frd-tools/excluded.json", encoding="utf-8"))["fragments"]
rows = [l.rstrip("\n").split("\t") for l in open(G + "frd-verify-paths.tsv", encoding="utf-8") if l.strip()]
out = []
cnt = collections.Counter()
for f, kind, lit, mans in rows:
    owner = next((x["owner"] for x in ex if x["fragment"] in f), None)
    mans = mans.split(";")
    res = {"file": f, "kind": kind, "lit": lit, "manifests": mans, "owner": owner}
    if owner and owner != "self":
        res["class"] = "skipped-owner"; cnt["skipped-owner"] += 1; out.append(res); continue
    repo = kind.startswith("repo_")
    fixes = set(); found = False
    tail = re.sub(r'^(?:\.{1,2}/)+', '', lit)
    for m in mans:
        md = os.path.dirname(os.path.join(ROOT, m))
        a = md; hit = None
        while True:
            cand = os.path.join(a, tail)
            if tail and os.path.exists(cand): hit = os.path.relpath(cand, md); break
            if a == ROOT or a == "/": break
            a = os.path.dirname(a)
        if hit: found = True; fixes.add(hit)
    if len(fixes) == 1 and found: res["class"] = "reanchor"; res["new"] = fixes.pop(); cnt["reanchor"] += 1
    elif found: res["class"] = "reanchor-ambiguous"; cnt["reanchor-ambiguous"] += 1; res["new"] = sorted(fixes)
    else: res["class"] = "not-found"; cnt["not-found"] += 1
    out.append(res)
json.dump(out, open(G + "frd-missing-class.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(dict(cnt))
by = collections.Counter((r["class"], r["kind"]) for r in out); print(dict(by))
