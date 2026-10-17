"""📊 Summarises the oracle result files of the F-04 families: per family and configuration the error totals, NEW/GONE against the HEAD revision, non-stub new rows by category and whether the canary was reached on both sides. usage: famsum.py"""
import glob, json, os, re
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
for p in sorted(glob.glob(F+"/🗑️generated/rsl-rsl[0-9f]*-result.json")):
    name=os.path.basename(p)[4:-12]
    r=json.load(open(p))
    for cfg,e in r["summary"].items():
        news=[d for d in e["new_list"] if not d.get("stub") and not d.get("inherited")]
        cats={}
        for d in news: cats[d["cat"]]=cats.get(d["cat"],0)+1
        can=len(e.get("canary_cur") or [])==3 and len(e.get("canary_base") or [])==3
        print(f"{name:10} {cfg:5} cur {e['cur_total']:6} base {e.get('base_total','-'):>6} NEW {e.get('new','-'):>4} GONE {e.get('gone','-'):>4} non-stub-new {len(news)} {cats} canary {'yes' if can else 'NO'} quarantined {r.get('quarantined')}")
