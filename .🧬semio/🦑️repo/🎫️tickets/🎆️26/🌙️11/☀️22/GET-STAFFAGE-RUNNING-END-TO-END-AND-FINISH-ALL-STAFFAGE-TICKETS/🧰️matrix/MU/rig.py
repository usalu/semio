"""🦴️ The rig, poses, clips and anchors of an asset document, and with bones named the shapes of those bones per scale and view: `python3 rig.py <id> [bone,…|all] [scale,…] [view,…]`."""
import json, glob, sys
S='/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog'
a=json.load(open(sys.argv[1] if sys.argv[1].endswith('.json') else glob.glob(f'{S}/*/{sys.argv[1]}/🔣️.json')[0]))['asset']
J=lambda x: json.dumps(x,ensure_ascii=False)
if len(sys.argv)<3:
    print('dims',a['dimensions'],'default',a['presentation']['defaultPose'])
    for b in a['rig']['bones']: print('bone',J({k:v for k,v in b.items() if k not in('title',)})[:260])
    for p in a['poses']: print('pose',p['id'],'root',J(p.get('root')),'bones',J(p.get('bones'))[:900],'contacts',p.get('contacts'))
    for c in a['clips']: print('clip',c['id'],c['durationS'],'loop',c['loop'],J(c['keys'])[:500],'events' , [k for k in c if k not in('id','title','durationS','loop','keys','description')])
    for n in a['anchors']: print('anchor',J({k:v for k,v in n.items() if k not in('title',)})[:220])
    for k,r in a['representations'].items(): print('rep',k,r['status'],r.get('validRange'),{v:len(s) for v,s in (r.get('geometry') or {}).items()},J(r.get('sections'))[:200])
else:
    bones=sys.argv[2].split(','); scales=sys.argv[3].split(',') if len(sys.argv)>3 else None; views=sys.argv[4].split(',') if len(sys.argv)>4 else None
    for k,r in a['representations'].items():
        if scales and k not in scales: continue
        for v,l in (r.get('geometry') or {}).items():
            if views and v not in views: continue
            for s in l:
                if bones==['all'] or s['bone'] in bones: print(k,v,J(s)[:420])
