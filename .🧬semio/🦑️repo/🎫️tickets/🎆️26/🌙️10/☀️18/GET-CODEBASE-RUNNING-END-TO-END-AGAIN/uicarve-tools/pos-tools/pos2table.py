"""📊 POS2 report table: reads GEN/pos2-<family>-<label>-result.json and prints markdown rows (one per family and configuration).
usage: pos2table.py <family>=<before-label>:<after-label> ...     (a missing before label prints the after row only)"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from poslib import GEN, PREFIX
from pos import is_trait_scope


def counts(e):
    c = {"privacy": 0, "trait-scope": 0, "resolve": 0, "type": 0, "other": 0}
    inh = 0
    for d in e.get("new_list", []):
        if d.get("stub"):
            continue
        if d.get("inherited"):
            inh += 1
            continue
        k = "trait-scope" if is_trait_scope(d) else d["cat"]
        c[k] += 1
    return c, inh


def load(fam, label):
    p = f"{GEN}/{PREFIX}-{fam}-{label}-result.json"
    return json.load(open(p, encoding="utf-8")) if os.path.exists(p) else None


print("| family | cfg | crates | canary | errors cur (noise) | base | privacy before > after | trait-scope b > a | resolve b > a | type b > a | other b > a | quarantine | HIR-cull / ICE |")
print("| --- | --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |")
for arg in sys.argv[1:]:
    fam, labs = arg.split("=")
    bl, al = (labs.split(":") + [None])[:2] if ":" in labs else (None, labs)
    a = load(fam, al)
    b = load(fam, bl) if bl else None
    if a is None:
        print(f"| {fam} | - | - | NO RESULT | | | | | | | | | |")
        continue
    for cfg, e in a["summary"].items():
        ca, ia = counts(e)
        cb = counts(b["summary"][cfg])[0] if b and cfg in b["summary"] else ca
        fmt = lambda k: f"{cb[k]} > {ca[k]}"
        q = ",".join(f"{s}:{len(v) if v else 'blocked'}" for s, v in (a.get("quarantined") or {}).items()) or "0"
        hc = sum(len(v) for k, v in (a.get("hir_cull") or {}).items() if k.endswith(cfg))
        ic = sum(len(v) for k, v in (a.get("ice_bodies") or {}).items() if k.endswith(cfg))
        canary = "yes" if len(e.get("canary_cur") or []) == 3 else "NO"
        print(f"| {fam} | {cfg} | {len(a['sides']['cur']['crates'])} | {canary} | {e['cur_total']} ({e['cur_noise']}) | {e.get('base_total', '-')} | {fmt('privacy')} | {fmt('trait-scope')} | {fmt('resolve')} | {fmt('type')} | {fmt('other')} | {q} | {hc}/{ic} |")
