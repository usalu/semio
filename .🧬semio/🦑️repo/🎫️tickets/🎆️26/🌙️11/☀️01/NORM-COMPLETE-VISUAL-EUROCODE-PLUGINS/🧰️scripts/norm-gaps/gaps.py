import json, os, re, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ART = ROOT + "/🗿️artifacts"
TICKET = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS"
SPEC = TICKET + "/📐️specs"
CODES = ["en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999"]
EXCL = ("spec-trace", "🧭️spec-trace", "🧪️tests", "tests", "🎪️showcase", "🎬️scene", "👁️viewer", "✏️editor", "📚️examples", "🧫️fixtures", "fixtures", "🖼️assets")

def artdir(code):
    for d in os.listdir(ART):
        if d.endswith(code):
            return os.path.join(ART, d, "🏅️standards/🔖️1/🪆️subsets/✳️any")

def corpus(code):
    texts = []
    roots = [artdir(code), ROOT + "/⚖️compliance"]
    for r in roots:
        for dp, dn, fn in os.walk(r):
            if any(x in dp.split(os.sep) or x in os.path.basename(dp) for x in EXCL) and "⚖️compliance" not in dp or ("spec-trace" in dp or "🧭️spec-trace" in dp or "🧪️tests" in dp):
                continue
            for f in fn:
                if f.endswith(".rs"):
                    p = os.path.join(dp, f)
                    try:
                        texts.append((p, open(p, encoding="utf-8").read()))
                    except Exception:
                        pass
    return texts

def target_lines(code):
    fname = {"en1990": "en1990-en1991-target.md", "en1991": "en1990-en1991-target.md", "en1992": "en1992-target.md",
             "en1993": "en1993-target.md", "en1994": "en1994-en1999-target.md", "en1995": "en1995-en1996-target.md",
             "en1996": "en1995-en1996-target.md", "en1997": "en1997-en1998-target.md", "en1998": "en1997-en1998-target.md",
             "en1999": "en1994-en1999-target.md"}[code]
    return open(os.path.join(SPEC, fname), encoding="utf-8").read().splitlines()

def find_check(frag, corp):
    hits = [p for p, t in corp if frag in t]
    return hits

out = {}
for code in CODES:
    j = json.load(open(os.path.join(artdir(code), "🧭️spec-trace/🔣️.json"), encoding="utf-8"))
    corp = corpus(code)
    tl = target_lines(code)
    spec_ids = j["specIds"]
    items_ids = [i["id"] for i in j["items"]]
    rows = []
    for it in j["items"]:
        r = it["realisation"]
        k = r["kind"]
        en = it["description"]["en"]
        de = it["description"]["de"]
        rowtag = ""
        for l in tl:
            if re.match(r"^\|\s*`?" + re.escape(it["id"]) + r"`?\s*\|", l):
                rowtag = l
                break
        tag_unverified = bool(re.search(r"\[V\]|\[verify\]|NA-DE \?|unverified|ungeprüft", rowtag))
        notes = []
        ev = {}
        if k == "check":
            frags = r["checks"]
            found = [f for f in frags if find_check(f, corp)]
            miss = [f for f in frags if f not in found]
            idhits = [p for p, t in corp if ('"%s"' % it["id"]) in t]
            ev = {"checks": frags, "found": found, "missing": miss, "idHits": [os.path.relpath(p, ART) for p in idhits][:3]}
            if not found and idhits:
                status = "implemented"
                notes.append("trace check fragments stale; catalogue entry %s exists" % it["id"])
            elif not found:
                status = "missing"
                notes.append("check fragments absent from implementation")
            elif miss:
                status = "partial"
                notes.append("some check fragments absent: " + ", ".join(miss))
            else:
                status = "implemented"
        elif k == "design-case":
            e = r.get("entry") or r.get("id") or list(r.values())[0]
            hit = [p for p, t in corp if ('"%s"' % e) in t]
            ev = {"entry": e, "hits": len(hit)}
            status = "implemented" if hit else "missing"
            if not hit:
                notes.append("design-case entry %s not in catalogue" % e)
        elif k == "data":
            path = list(r.values())[0] if isinstance(list(r.values())[0], str) else str(r)
            tok = re.split(r"[ :(]", path.strip())[0]
            tok = tok.split("::")[-1].split("..")[0].strip()
            hit = [p for p, t in corp if tok and tok in t]
            ev = {"path": path, "token": tok, "hits": len(hit)}
            status = "implemented" if hit else "missing"
            if not hit:
                notes.append("data token %s not found" % tok)
        else:
            reason = json.dumps(r, ensure_ascii=False)
            status = "blocked-unverified" if re.search(r"unverified|print|neither in the repository|not in the repository|marks", reason, re.I) else "missing"
            ev = {"reason": r}
            notes.append("not modelled")
        if status == "implemented" and (re.search(r"unverified|ungeprüft|\bverify\b|provisional|print", en + " " + de, re.I) or tag_unverified):
            status = "blocked-unverified"
            notes.append("unverified/print-dependent value")
        if status == "implemented" and re.search(r"not modelled|not carried|does not carry|is not modelled|special study|out of scope|not covered|not used", en, re.I):
            status = "partial"
            notes.append("description limits scope")
        rows.append({"id": it["id"], "clause": it["clause"], "kind": k, "status": status, "notes": notes, "ev": ev, "en": en, "row": rowtag[:200]})
    out[code] = {"specIds": spec_ids, "items": rows, "notInItems": [s for s in spec_ids if s not in items_ids], "itemsNotInSpec": [s for s in items_ids if s not in spec_ids], "specIdsNotInTargetMd": [s for s in spec_ids if not any(s in l for l in tl)]}
json.dump(out, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
for code in CODES:
    o = out[code]
    from collections import Counter
    c = Counter(r["status"] for r in o["items"])
    print(code, len(o["items"]), dict(c), "notInItems", o["notInItems"], "notInMd", o["specIdsNotInTargetMd"][:10])
