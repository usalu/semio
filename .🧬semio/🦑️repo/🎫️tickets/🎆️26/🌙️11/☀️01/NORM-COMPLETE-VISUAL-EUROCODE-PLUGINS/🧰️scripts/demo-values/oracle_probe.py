"""Evaluates every demo case of a code with the independent Python oracle: OK (named values), UNSUPPORTED (no formulation), ERROR. Run with python3 -I (needs numpy/scipy of the repo venv): oracle_probe.py <code> [annex]."""
import re, sys, traceback
sys.path.insert(0, "."); from dv_lib import *
code = sys.argv[1]; annex = next((a for a in sys.argv[2:] if not a.startswith("--")), "de")
mod = oracle(code)
doc = json.loads(demo_doc_path(code).read_text(encoding="utf-8"))
v = vocab(code)
def split(case):
    n, vec = {}, {}
    for x in case["numbers"]:
        m = re.fullmatch(r"(.+)\.(\d+)", x["id"])
        if m: vec.setdefault(m.group(1), []).append((int(m.group(2)), x["value"]))
        else: n[x["id"]] = x["value"]
    vec = {k: [y for _, y in sorted(xs)] for k, xs in vec.items()}
    return n, {t["id"]: t["value"] for t in case["texts"]}, vec
def run(entry, n, s, vec):
    if code == "en1991":
        return mod.ev(entry, annex, n, s, vec)
    if code == "en1996":
        return mod.catalogue({"entry": entry, "annex": annex, "num": n, "text": s, "vec": vec})
    if code == "en1995":
        r = mod.catalogue({"cases": [{"entry": entry, "case": {"annex": annex.capitalize(), "num": n, "text": s}}]})
        return r["cases"][0]["values"]
    raise SystemExit("no probe for " + code)
res = {}
for case in doc["cases"]:
    n, s, vec = split(case)
    try:
        out = run(case["entryId"], n, s, vec)
        res[case["entryId"]] = ("OK", out) if out else ("EMPTY", None)
    except KeyError as e:
        res[case["entryId"]] = ("UNSUPPORTED" if str(e).strip("'") == case["entryId"] else "KEYERROR " + str(e), None)
    except Exception as e:
        res[case["entryId"]] = ("ERROR " + type(e).__name__ + " " + str(e)[:80], None)
if __name__ == "__main__":
    ok = [k for k, (st, _) in res.items() if st == "OK"]
    print(code, "OK", len(ok), "of", len(res))
    for k, (st, _) in res.items():
        if st != "OK": print("  ", k, st)
