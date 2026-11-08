# 🔎 Audit of norm artifacts: spec-trace items vs implemented checks, tests and oracles (read-only, stdlib only)
import json, os, re, sys, bisect, collections

ARTS = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
TICKET = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SPECS = os.path.join(TICKET, [d for d in os.listdir(TICKET) if d.endswith("specs")][0])
OUT = os.path.join(TICKET, "🗑️generated", "audit-norm", "result.json")
SPEC_FILE = {"en1990": "en1990-en1991-target.md", "en1991": "en1990-en1991-target.md", "en1992": "en1992-target.md",
             "en1993": "en1993-target.md", "en1994": "en1994-en1999-target.md", "en1999": "en1994-en1999-target.md",
             "en1995": "en1995-en1996-target.md", "en1996": "en1995-en1996-target.md",
             "en1997": "en1997-en1998-target.md", "en1998": "en1997-en1998-target.md"}
SKIP_DIRS = {"target", "node_modules", ".git", "dist"}
SKIP_FILES = {"Cargo.lock"}
ID_HEAD = re.compile(r"(?<![A-Za-z0-9])(E\d{1,2}\.\d{2}|E\d{1,2}-\d{2}|[A-Z]{1,3}-\d{2,3}[a-z]?|[A-Z]{1,3}\d-\d{2}|E\d{1,2}\.\d{1,2}|X\d{1,2}|V-\d{3}[a-z]?)(?![A-Za-z0-9])")
TESTY = ("🧪", "🧫", "🔬", "🥒", "test", "fixture")

def arts():
    out = {}
    for d in sorted(os.listdir(ARTS)):
        key = re.sub(r"^[^a-z]*", "", d)
        out[d] = key
    return out

def load_text(path):
    try:
        if os.path.getsize(path) > 3_000_000: return None
        with open(path, "rb") as f: raw = f.read()
        if b"\0" in raw[:4096]: return None
        return raw.decode("utf-8")
    except Exception:
        return None

def index(root):
    files = []
    for dp, dns, fns in os.walk(root):
        dns[:] = [x for x in dns if x not in SKIP_DIRS]
        for fn in fns:
            if fn in SKIP_FILES: continue
            p = os.path.join(dp, fn)
            t = load_text(p)
            if t is not None: files.append((os.path.relpath(p, root), t))
    return files

def classify(rel):
    segs = rel.split(os.sep)
    if "spec-trace" in rel: return "spec-trace"
    if any(("oracle" in s) or ("🔮" in s) for s in segs): return "oracle"
    if any(any(t in s for t in TESTY) for s in segs) or rel.endswith(".feature"): return "test"
    if rel.endswith(".md"): return "doc"
    return "impl"

TOK = re.compile(r"[A-Za-z0-9][A-Za-z0-9._\-]*")

def scan(files, needles):
    exact = set(needles)
    prefixes = {n for n in needles if n.endswith(".")}
    subs = {n for n in needles if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._\-]*", n)}
    hits = collections.defaultdict(list)
    if not exact: return hits
    for rel, text in files:
        kind = classify(rel)
        if kind == "spec-trace": continue
        nl = None
        for m in TOK.finditer(text):
            t = m.group(0)
            cands = []
            if t in exact: cands.append(t)
            u = t.rstrip(".")
            if u in exact and u != t: cands.append(u)
            i = t.find(".")
            while i != -1:
                pre = t[:i + 1]
                if pre in prefixes: cands.append(pre)
                if t[:i] in exact: cands.append(t[:i])
                i = t.find(".", i + 1)
            if not cands: continue
            if nl is None: nl = [x.start() for x in re.finditer("\n", text)]
            ln = bisect.bisect_left(nl, m.start()) + 1
            for c in set(cands): hits[c].append((kind, rel, ln))
        for n in subs:
            i = text.find(n)
            if i != -1: hits[n].append((kind, rel, text.count("\n", 0, i) + 1))
    return hits

def first(hits, kinds, limit=1):
    return [(r, l) for k, r, l in hits if k in kinds][:limit]

def stdlib_only(rel, text):
    mods = set()
    for line in text.splitlines():
        m = re.match(r"^\s*(?:import|from)\s+([A-Za-z_][A-Za-z0-9_]*)", line)
        if m: mods.add(m.group(1))
    std = getattr(sys, "stdlib_module_names", None)
    if std is None or not rel.endswith(".py"): return "unknown (non-python or no stdlib list)"
    third = sorted(m for m in mods if m not in std)
    return ("third-party: " + ", ".join(third)) if third else "stdlib-only"

def main():
    report = {}
    for d, key in arts().items():
        if key not in ("en1990", "en1991", "en1992", "en1993", "en1994", "en1995", "en1996", "en1997", "en1998", "en1999"): continue
        root = os.path.join(ARTS, d)
        trace_p = os.path.join(root, "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧭️spec-trace", "🔣️.json")
        trace = json.load(open(trace_p, encoding="utf-8"))
        files = index(root)
        fmap = dict(files)
        subset = os.path.join(root, "🏅️standards", "🔖️1", "🪆️subsets", "✳️any")
        needles = []
        items = []
        for it in trace["items"]:
            r = it["realisation"]; k = r["kind"]
            ids = [it["id"]]
            if k == "check": ids += r.get("checks", [])
            if k == "design-case": ids.append(r["entry"])
            needles += ids
            items.append((it, k, ids))
        hits = scan(files, needles)
        rows = []
        for it, k, ids in items:
            impl = []; tst = []; orc = []
            for n in ids:
                for kind, rel, ln in hits.get(n, []):
                    if kind == "impl" and len(impl) < 2: impl.append((rel, ln, n))
                    if kind == "test" and len(tst) < 2: tst.append((rel, ln, n))
                    if kind == "oracle" and len(orc) < 2: orc.append((rel, ln, n))
            path_ok = None
            if k == "data":
                tok = it["realisation"]["path"].split(" ")[0]
                path_ok = os.path.exists(os.path.join(subset, tok)) if "/" in tok and "::" not in tok else None
            orc_third = []
            for rel, ln, n in orc[:1]:
                t = fmap.get(rel, "")
                orc_third.append(stdlib_only(rel, t))
            rows.append({"id": it["id"], "clause": it["clause"], "kind": k,
                         "reason": (it["realisation"].get("reason", {}) or {}).get("en", ""),
                         "impl": impl, "test": tst, "oracle": orc, "oracle_deps": orc_third,
                         "data_path_exists": path_ok,
                         "agnostic_test": any(rel.endswith((".feature", ".json", ".toml", ".csv")) for rel, _, _ in tst)})
        report[key] = {"dir": d, "spec": trace["spec"], "rows": rows, "n": len(rows)}
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    json.dump(report, open(OUT, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for key, r in report.items():
        c = collections.Counter(x["kind"] for x in r["rows"])
        noimpl = [x["id"] for x in r["rows"] if x["kind"] in ("check", "design-case") and not x["impl"]]
        notest = [x["id"] for x in r["rows"] if x["kind"] in ("check", "design-case") and x["impl"] and not x["test"]]
        noorc = [x["id"] for x in r["rows"] if x["kind"] in ("check", "design-case") and not x["oracle"]]
        print(key, r["n"], dict(c), "noimpl=%d notest=%d nooracle=%d" % (len(noimpl), len(notest), len(noorc)))

if __name__ == "__main__":
    main()
