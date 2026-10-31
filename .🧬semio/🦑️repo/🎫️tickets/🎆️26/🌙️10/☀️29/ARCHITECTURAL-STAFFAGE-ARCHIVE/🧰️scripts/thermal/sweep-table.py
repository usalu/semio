import json, sys, collections
rows = json.load(open(sys.argv[1]))
by = collections.OrderedDict()
for r in rows:
    by.setdefault(r["asset"], []).append(r)
def rng(rs, k, nd=0):
    v = [x for r in rs if r.get(k) for x in r[k]]
    if not v: return "–"
    f = lambda x: f"{x:.{nd}f}"
    return f"{f(min(v))} … {f(max(v))}"
print("| asset | cat. | solves | surface °C | skin °C | emitted W/m² | net rad. W/m² | sensible W/m² | latent W/m² | moisture g/(m²h) | core °C | flags |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|")
for a, rs in by.items():
    ok = sum(1 for r in rs if r["ok"])
    cores = [r["core"] for r in rs if r.get("core")]
    flags = sorted({f for r in rs for f in r["flags"]})
    shown = "; ".join(flags[:3]) + (f" (+{len(flags)-3})" if len(flags) > 3 else "")
    print(f"| {a} | {rs[0]['category']} | {'✓' if ok == len(rs) else '✗'} {ok}/{len(rs)} | {rng(rs,'surface',1)} | {rng(rs,'skin',1)} | {rng(rs,'emitted')} | {rng(rs,'net')} | {rng(rs,'sensible')} | {rng(rs,'latent')} | {rng(rs,'moisture')} | {(f'{min(cores):.2f} … {max(cores):.2f}' if cores else '–')} | {shown or '–'} |")
