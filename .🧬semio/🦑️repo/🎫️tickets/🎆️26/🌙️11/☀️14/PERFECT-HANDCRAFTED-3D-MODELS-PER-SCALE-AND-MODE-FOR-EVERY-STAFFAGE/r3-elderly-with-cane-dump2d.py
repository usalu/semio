"""📋 Compact listing of the 2D shapes of elderly-with-cane per scale and view."""
import json,glob,re,sys
F=glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/elderly-with-cane/🔣️.json")[0]
a=json.load(open(F))['asset']
for s in sys.argv[1].split(','):
  for v in sys.argv[2].split(','):
    print('==',s,v)
    for sh in a['representations'][s]['geometry'][v]:
        if sh['kind']=='path':
            n=re.findall(r'-?\d+\.?\d*(?:e-?\d+)?',sh['d']); xs=[float(x) for x in n[0::2]]; ys=[float(x) for x in n[1::2]]
            cmds=''.join(sorted(set(re.findall(r'[A-Za-z]',sh['d']))))
            geo=f"path[{cmds}] n={len(xs)} x[{min(xs):.3f},{max(xs):.3f}] y[{min(ys):.3f},{max(ys):.3f}]"
        elif sh['kind']=='poly':
            p=sh['points']; geo=f"poly n={len(p)} c={sh['closed']} x[{min(q[0] for q in p):.3f},{max(q[0] for q in p):.3f}] y[{min(q[1] for q in p):.3f},{max(q[1] for q in p):.3f}]"
        else: geo=sh['kind']+' '+' '.join(f"{k}={sh[k]}" for k in sh if k in('x0','y0','x1','y1','r0','r1','cx','cy','rx','ry','x','y','w','h','r','rotationDeg','followTip'))
        extra={k:sh[k] for k in sh if k not in('id','bone','role','z','filled','tone','kind','d','points','closed','region','material','x0','y0','x1','y1','r0','r1','cx','cy','rx','ry','x','y','w','h','r','rotationDeg')}
        print(f"  {sh['id']:22s} {sh['bone']:10s} {sh['role']:7s} z={sh['z']:<5} f={int(sh['filled'])} t={sh['tone']:<6} {sh.get('region','-'):10s} {geo} {extra if extra else ''}")
