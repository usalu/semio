"""tc2: data-driven fragments for the RESULT file."""
import json,glob,os,re,collections,subprocess,sys
G=os.path.abspath(os.path.dirname(__file__)+"/../🗑️generated")
FAM=[("b2","block-2d-editor + tests"),("b5","block-5d-editor + tests"),("model","draw model + model-tests"),("lcmd","layout editor-commands + tests"),("lframes","layout mutations-frames + 2 companions"),("lcontent","layout mutations-content + 2 companions"),("scene","draw scene + scene-tests"),("edcmd","draw editor-commands + tests"),("edint","draw editor-interaction + tests"),("host","draw host + host-tests"),("editor","draw editor + 2 companions"),("transform","draw transform + tests"),("style","draw style + tests"),("structure","draw structure + tests")]
ARTIFACT=[("cannot find attribute `serde` in this scope","testkit feature active in the lib cfg; the stub layer has no serde"),]
def oracle(tags=("v8","v7","v6","v5")):
    out=["| family | crates in the union | test cfg: cur / base errors | NEW | GONE | NEW rows (all) |","|---|---:|---|---:|---:|---|"]
    for fam,label in FAM:
        f=None; used=""
        for tag in tags:
            cand=f"{G}/tc2-pos-{fam}-{tag}-result.json"
            if os.path.exists(cand): f=cand; used=tag; break
        if f is None: out.append(f"| {label} | | not run | | | |"); continue
        r=json.load(open(f)); s=r["summary"].get("test")
        if not s: continue
        rows=collections.Counter()
        for x in s["new_list"]:
            msg=re.sub(r"`[^`]*`","`_`",x["msg"])[:60]
            rows[f"{x['code'] or '-'} {msg}"]+=1
        n=json.load(open(f))
        crates=len(json.load(open(os.path.dirname(__file__)+f"/ttc/families/{fam}.json"))["crates"])
        out.append(f"| {label} (`{used}`) | {crates} | {s['cur_total']} / {s['base_total']} | {s['new']} | {s['gone']} | "+("; ".join(f"{v}x {k}" for k,v in rows.most_common(6)) or "-")+" |")
    return "\n".join(out)
def checkapplied():
    out=["| package | crates (base / combined) | native: base / combined / onlyCombined | `--test`: base / combined / onlyCombined | exit |","|---|---|---|---|---|"]
    for n,label in (("b2","block 2d"),("b5","block 5d"),("layout","layout"),("draw","draw")):
        f=f"{G}/tc2-checkapplied-{n}-{'after6' if n=='draw' else 'after4'}.log"
        if not os.path.exists(f): continue
        t=open(f).read()
        try:
            i=t.index("\n{")+1; j=t.rindex("\n}")+2; d=json.loads(t[i:j])
        except Exception:
            out.append(f"| {label} | unreadable | | | |"); continue
        L=d["layout"]; m_=re.search(r"exit (\d+)",t); ex=m_.group(1) if m_ else ("1 (onlyCombined rows)" if L["--test"]["onlyCombined"] else "0")
        out.append(f"| {label} | {d.get('baseCrates','?')} / {d['crates']} | {L['native']['base']} / {L['native']['combined']} / {L['native']['onlyCombined']} | {L['--test']['base']} / {L['--test']['combined']} / {L['--test']['onlyCombined']} | {ex} |")
    return "\n".join(out)
if __name__=="__main__":
    print(oracle()); print(); print(checkapplied())
