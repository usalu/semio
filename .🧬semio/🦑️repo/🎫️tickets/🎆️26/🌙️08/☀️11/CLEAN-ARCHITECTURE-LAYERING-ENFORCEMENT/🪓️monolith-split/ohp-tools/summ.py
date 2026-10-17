"""📊 One-line-per-config summary of non-stub harvest rows for a pos result json: counts by wanted code.
usage: summ.py <result.json>..."""
import json, sys, collections
WANT = "E0425 E0432 E0433 E0412 E0599 E0061 E0308 E0603 E0616 E0624 E0107 E0282".split()
for p in sys.argv[1:]:
    r = json.load(open(p, encoding="utf-8"))
    for cfg, e in r["summary"].items():
        c = collections.Counter(d["code"] for d in e["new_list"] if not d["stub"] and not d.get("inherited") and d["code"] in WANT)
        print(f"{r['family']:14} {cfg:5} crates={len(r['sides']['cur']['crates']):4} total={e['cur_total']:6} stubnoise={e['cur_noise']:6} canary={','.join(e['canary_cur']) or '-'} nonstub={sum(c.values()):4} " + " ".join(f"{k}={v}" for k, v in sorted(c.items())))
