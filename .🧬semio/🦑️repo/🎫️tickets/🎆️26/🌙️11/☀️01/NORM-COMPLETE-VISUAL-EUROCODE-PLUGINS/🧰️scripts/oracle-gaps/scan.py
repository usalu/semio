"""🔎 Rust-vs-oracle check-id scanner. Usage: scan.py [--run] [codes...]. Rust families = normalised "enNNNN." string literals of non-test .rs files; oracle ids = check-outcome oracle run over every document (--run caches to 🗑️generated/oracles/ids-<code>.json) plus its static literals."""
import glob, json, os, re, subprocess, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🗑️generated", "oracles")
PY = "/home/deck/semio/.venv/bin/python"
args = [a for a in sys.argv[1:] if not a.startswith("--")]
CODES = args or ["en%d" % n for n in range(1990, 2000) if n != 1990 + 0 or True]
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
LIT = re.compile(r'"(en\d{4}\.[^"\\]*)"')

def art(code):
    return glob.glob(f"{ROOT}/*{code}")[0] + "/🏅️standards/🔖️1/🪆️subsets/✳️any"

def norm(s):
    s = s.split("{")[0].rstrip(".")
    return s

def rust_families(code):
    import assess
    pats, unresolved = assess.patterns(code)
    fams = {}
    for p, where in pats.items():
        n = re.sub(r"\{[^}]*\}", "{}", p).rstrip(".")
        if len(n.replace("{}", "").strip(".").split(".")) < 3 and "{}" not in n:
            continue
        fams.setdefault(n, []).extend(where)
    keep = {f: w for f, w in fams.items() if not any(o != f and o.startswith(f + ".") for o in fams)}
    for t in spec_trace(code):
        n = re.sub(r"\{[^}]*\}", "{}", t).rstrip(".")
        if t.startswith("."):
            if any(f.endswith(n) or (n + ".") in f for f in list(keep) if not f.startswith(".")):
                continue
            keep.setdefault(n, []).append("spec-trace (suffix)")
        elif not any(pattern_regex(f).match(n) or n.startswith(f + ".") or f.startswith(n + ".") for f in keep):
            keep.setdefault(n, []).append("spec-trace")
    return keep, unresolved

def spec_trace(code):
    out = set()
    for p in glob.glob(art(code) + "/🧭️spec-trace/*.rs") + glob.glob(art(code) + "/🧭️spec-trace/**/*.rs", recursive=True):
        for blk in re.findall(r"Realisation::Check\(&\[(.*?)\]", open(p, encoding="utf8").read(), re.S):
            out.update(re.findall(r'"([^"]+)"', blk))
    return out

def pattern_regex(fam):
    if fam.startswith("."):
        return re.compile(r"^en\d{4}.*" + re.escape(fam) + r"(\..+)?$")
    parts = [re.escape(x) for x in fam.split("{}")]
    return re.compile("^" + "[^.]+".join(parts) + r"(\..+)?$")

def static_ids(code):
    out = set()
    for p in glob.glob(art(code) + "/🔮️oracles/**/*.py", recursive=True):
        if "__pycache__" in p:
            continue
        for m in re.findall(r'f?"(en\d{4}(?:-\d)?\.[^"]*)"', open(p, encoding="utf8").read()):
            n = re.sub(r"\{[^}]*\}", "{}", m).rstrip(".")
            if len(n.replace("{}", "").strip(".").split(".")) >= 3:
                out.add(n)
    return out

def static_match(fam, static):
    key = fam.replace("{}", "*")
    for s in static:
        sk = s.replace("{}", "*")
        if sk == key or re.fullmatch(re.escape(sk).replace(r"\*", "[^.]+"), key.replace("*", "x")) or re.fullmatch(re.escape(key).replace(r"\*", "[^.]+"), sk.replace("*", "x")):
            return True
    return False

def documents(code):
    a = art(code) + "/🧪️tests"
    docs = set(glob.glob(a + "/🧫️fixtures/🎪️showcase/🔣️.json") + glob.glob(a + "/🧫️fixtures/🚦️check-outcome/*/🔣️.json") + glob.glob(a + "/🧫️fixtures/📄️oracle-documents/*/🔣️.json"))
    for f in glob.glob(a + "/🚦️check-outcome-*/🥒️.feature"):
        for u in re.findall(r"shared://(\S+?)\s*\|", open(f, encoding="utf8").read()):
            p = a + "/🧫️fixtures/" + u
            if p.endswith(".json") and os.path.exists(p):
                docs.add(p)
    return sorted(docs)

def oracle_ids(code, run):
    cache = os.path.join(OUT, f"ids-{code}.json")
    if not run and os.path.exists(cache):
        return json.load(open(cache))
    script = art(code) + "/🔮️oracles/🚦️check-outcome/🐍️.py"
    ids, errs = {}, []
    for d in documents(code):
        r = subprocess.run([PY, "-I", script, "--json"], input=open(d, "rb").read(), capture_output=True)
        if r.returncode:
            errs.append((d.split("/🧫️fixtures/")[1], r.stderr.decode()[-160:]))
            continue
        for row in json.loads(r.stdout)["checks"]:
            ids.setdefault(row["id"], []).append(d.split("/🧫️fixtures/")[1])
    res = {"ids": {k: v[:2] for k, v in ids.items()}, "errors": errs, "documents": len(documents(code))}
    os.makedirs(OUT, exist_ok=True)
    json.dump(res, open(cache, "w"), ensure_ascii=False, indent=1)
    return res

def covered(fam, ids, static):
    rx = pattern_regex(fam)
    if any(rx.match(i) for i in ids) or any(rx.match(s) or s == fam for s in static):
        return "exact"
    stem = fam.split("{}")[0].rstrip(".")
    if not fam.startswith(".") and stem in ids:
        return "exact"
    if not fam.startswith(".") and any(stem.startswith(i.rsplit(".", 1)[0] + ".") or stem == i.rsplit(".", 1)[0] for i in ids if i.count(".") >= 3):
        return "coarse"
    return None

if __name__ == "__main__":
    run = "--run" in sys.argv
    summary = []
    for code in CODES:
        rf, unresolved = rust_families(code)
        o = oracle_ids(code, run)
        static = static_ids(code)
        ids = set(o["ids"])
        cov = {f: covered(f, ids, set()) for f in rf}
        coarse = sorted(f for f, v in cov.items() if v == "coarse")
        dyn_missing = sorted(f for f, v in cov.items() if v is None)
        both_missing = sorted(f for f in dyn_missing if not static_match(f, static))
        only_static = sorted(set(dyn_missing) - set(both_missing))
        rxs = [pattern_regex(f) for f in rf]
        oracle_extra = sorted(i for i in ids if not any(r.match(i) for r in rxs))
        json.dump({"rust": len(rf), "oracleIds": len(ids), "missing": dyn_missing, "coarse": coarse, "missingNoStatic": both_missing, "staticOnly": only_static, "oracleNotRust": oracle_extra, "errors": o["errors"], "unresolved": len(unresolved), "rustFamilies": rf, "documents": o["documents"]}, open(os.path.join(OUT, f"gap-{code}.json"), "w"), ensure_ascii=False, indent=1)
        print(code, "rust families", len(rf), "oracle ids", len(ids), "docs", o["documents"], "missing", len(dyn_missing), "coarse", len(coarse), "missing(also static)", len(both_missing), "errors", len(o["errors"]))
