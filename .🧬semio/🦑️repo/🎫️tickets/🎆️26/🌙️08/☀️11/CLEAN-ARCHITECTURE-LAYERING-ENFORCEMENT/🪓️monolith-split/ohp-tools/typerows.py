"""🔬 Non-stub type rows (E0599/E0308/E0061/E0282/E0107/E0277..) of a pos result, grouped by message with the source line.
usage: typerows.py <result.json> [--cfg lib|test] [--codes E0599,..] [--max N]"""
import json, re, sys, collections, os
a = sys.argv[1:]
r = json.load(open(a[0], encoding="utf-8"))
DEFS = json.load(open("/Users/ueli/.cache/ohp/defs.json", encoding="utf-8"))
only = a[a.index("--cfg") + 1] if "--cfg" in a else None
codes = set(a[a.index("--codes") + 1].split(",")) if "--codes" in a else {"E0599", "E0308", "E0061", "E0282", "E0107"}
mx = int(a[a.index("--max") + 1]) if "--max" in a else 40
LINES = {}
def ctx(f, ln):
    try:
        if f not in LINES:
            LINES[f] = open(os.path.join("/Users/ueli/Documents/semio", f), encoding="utf-8").read().split("\n")
        lines = LINES[f]
        ind = len(lines[ln - 1]) - len(lines[ln - 1].lstrip())
        if ind == 0:
            return "free-fn"
        for k in range(ln - 2, max(-1, ln - 400), -1):
            s = lines[k]
            if s.strip() and (len(s) - len(s.lstrip())) < ind:
                if re.match(r"\s*(?:unsafe\s+)?impl\b", s):
                    return "trait-impl" if re.search(r"\bfor\b", s) else "inherent"
                if re.match(r"\s*(?:pub(?:\([a-z: ]+\))?\s+)?(?:trait|mod)\b", s):
                    return "trait-def" if "trait" in s else "mod-fn"
                ind = len(s) - len(s.lstrip())
                if ind == 0:
                    break
        return "other"
    except Exception:
        return "other"
for cfg, e in r["summary"].items():
    if only and cfg != only:
        continue
    g = collections.OrderedDict()
    for d in e["new_list"]:
        if d["stub"] or d.get("inherited") or d["code"] not in codes:
            continue
        msg = re.sub(r"__m_[A-Za-z0-9_]+", "M", d["msg"])
        lt, hs = d.get("line_text") or "", d.get("hs") or 0
        snip = lt[max(0, hs - 30):hs + 60].strip()
        f = "/".join(re.sub(r"^__m_[A-Za-z0-9_]+/", "", d["file"]).split("/")[-3:])
        relp = os.path.normpath(re.sub(r"^__m_[A-Za-z0-9_]+/", "", d["file"]))
        fam = "/".join(relp.split("/")[:3])
        tag = ""
        m = re.search(r"named `([^`]+)`", msg)
        if d["code"] == "E0599" and m:
            nm = m.group(1)
            ds = [(f_, l_) for f_, l_ in DEFS.get(nm, []) if f_.startswith(fam + "/")]
            kinds = []
            for f_, l_ in ds[:6]:
                kinds.append(ctx(f_, l_))
            if not ds:
                tag = "[no local def] "
            elif any(f_.startswith("🧰️framework") for f_, _ in DEFS.get(nm, [])):
                tag = "[ext-trait/fn name] "
            elif all(k == "trait-impl" for k in kinds):
                tag = "[trait-impl def] "
            else:
                tag = "[LOCAL-%s %s:%d] " % ("/".join(sorted(set(kinds))).upper(), "/".join(ds[0][0].split("/")[-3:]), ds[0][1])
        elif d["code"] in ("E0308", "E0061"):
            tag = "{" + (d.get("label") or "")[:70] + "} "
        g.setdefault((d["code"], tag + msg[:100]), []).append((f, d["line"], snip))
    print(f"== cfg {cfg}: {sum(len(v) for v in g.values())} rows in {len(g)} groups")
    for (code, msg), locs in list(g.items())[:mx]:
        f, ln, snip = locs[0]
        print(f"{code} x{len(locs)} {msg} | {f[-40:]}:{ln} | {snip[:90]}")
