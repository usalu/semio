import json,sys
d=json.load(open(sys.argv[1]))['asset']
print(d['dimensions'],d['presentation'])
print("BONES")
for b in d['rig']['bones']:
    print(' ',b['id'],b['parent'],round(b['x'],3),round(b['y'],3),b['angleDeg'],b['lengthM'],b['joint'].get('type'),b['joint'].get('minDeg'),b['joint'].get('maxDeg'),b.get('mass',{}).get('kg'))
print("ANCH")
for a in d['anchors']: print(' ',a['id'],a['bone'],a['x'],a['y'],a['kind'],a.get('support'))
print("SHAPES")
for s in d['geometry']['elevation']:
    g=s.get('d') or s.get('points') or ''
    print(' ',s['id'],s['bone'],s['kind'],s['role'],'lod',s.get('lod'),'z',s['z'],'f',s['filled'],s.get('region'),str(g)[:60])
print("POSES",[ (p['id'],p.get('key')) for p in d['poses']])
print("CLIPS",[(c['id'],c['durationS'],c['loop'],len(c['keys'])) for c in d['clips']])
print("MAT",[m['id'] for m in d['materials']])
print("REFS",[r['id'] for r in d['provenance']['references']])
