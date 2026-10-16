# 🕸️ Item edges between two crate groups. usage: edges4.py FROM_CRATE TO_CRATE [file-substring]
import sys
src = open(HERE_ := __file__).read()
exec(open('dag4.py').read().split("bad = []")[0])
fc, tc = sys.argv[1], sys.argv[2]
flt = sys.argv[3] if len(sys.argv) > 3 else ''
for u, vs in sorted(adj.items(), key=lambda kv: (items[kv[0]]['file'], items[kv[0]]['s'])):
    if crate.get(u) != fc: continue
    for v in sorted(vs, key=lambda x: (items[x]['file'], items[x]['s'])):
        if crate.get(v) != tc: continue
        iu, iv = items[u], items[v]
        if flt and flt not in iu['file'] + iv['file']: continue
        print(f"{iu['file'].split('/')[-2]}:{iu['s']} {iu['defname'] or iu.get('target') or iu['head'][:28]}  ->  {iv['file'].split('/')[-2]}:{iv['s']} {iv['defname'] or iv['head'][:28]}")
