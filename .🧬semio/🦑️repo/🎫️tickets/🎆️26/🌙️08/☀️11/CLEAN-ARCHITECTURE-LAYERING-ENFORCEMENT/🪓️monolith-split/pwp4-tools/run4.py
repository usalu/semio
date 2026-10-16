# ▶️ Dry run of the S0 cuts: writes every produced/changed file under an out dir (mirror of PLUG), widens visibility by the definition scan, reports sizes. usage: run4.py OUTDIR [--no-widen]
import sys, os, shutil, json
from spec4 import *
import widen4
out = sys.argv[1]; widen = '--no-widen' not in sys.argv
shutil.rmtree(out, ignore_errors=True)
written = {}; report = []
def reader(extra):
    def read(p):
        rel = os.path.relpath(os.path.normpath(p), os.path.normpath(PLUG))
        if rel in extra: return extra[rel]
        return open(p, encoding='utf-8').read() if os.path.exists(p) else None
    return read
stage = []
for src, pieces, rules in CUTS:
    fc = FileCut(src)
    dest = assign(fc, rules, unwrapped={p.unwrap for p in pieces if p.unwrap})
    resid, texts = build(fc, pieces, dest)
    unknown = {d for d in dest if d != 'stay'} - {p.pid for p in pieces}
    assert not unknown, unknown
    cnt = {}
    for i, d in enumerate(dest): cnt[d] = cnt.get(d, 0) + (fc.chunks[i].e - fc.chunks[i].s + 1)
    print(src, 'lines', len(fc.R), '->', {k: v for k, v in cnt.items()})
    stage.append((src, fc, pieces, resid, texts))
if widen:
    for src, fc, pieces, resid, texts in stage:
        extra = {src: resid}
        for p in pieces:
            if p.pid in texts: extra[p.path] = texts[p.pid]
        read = reader(extra)
        for p in pieces:
            if p.pid not in texts or p.mounted_elsewhere or p.existing and False: continue
            own = widen4.subtree(PLUG + '/' + p.path, read)
            outside = {}
            for q in [src] + [x.path for x in pieces if x.pid in texts and x.pid != p.pid]:
                outside.update(widen4.subtree(PLUG + '/' + q, read, {}))
            for k in list(outside):
                if k in own: del outside[k]
            outside = {k: v for k, v in outside.items() if k not in own}
            texts[p.pid] = widen4.widen_piece(texts[p.pid], {k: v for k, v in outside.items() if k != PLUG + '/' + p.path}, report, p.pid, via=p.unwrap)
for src, fc, pieces, resid, texts in stage:
    written[src] = resid
    for p in pieces:
        if p.pid in texts: written[p.path] = texts[p.pid]
for rel, t in written.items():
    p = os.path.join(out, rel); os.makedirs(os.path.dirname(p), exist_ok=True); open(p, 'w', encoding='utf-8').write(t)
json.dump(report, open(GEN + '/pwp4-widen-report.json', 'w'), ensure_ascii=False, indent=0)
import hashlib
man = {}
for rel in written:
    p = PLUG + '/' + rel
    man[rel] = hashlib.sha256(open(p, 'rb').read()).hexdigest() if os.path.exists(p) else None
json.dump(man, open(out + '.manifest.json', 'w'), ensure_ascii=False, indent=0)
print(len(written), 'files written to', out, '| widened members:', len(report))
