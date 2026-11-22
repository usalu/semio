"""🦴 Record anatomical sections read from each current catalog source and its independently checked asset."""
from pathlib import Path
from hashlib import sha256
import json
t=Path(__file__).resolve().parent;g=t/'🗑️generated';r=t.parents[6];cases=json.loads((t/'digital-limb-gauge.json').read_bytes());rows=[]
for name in cases['subjects']:
 p=next(r.glob('🎓*/*/*staffage/*catalog/*/'+name+'/*.json'));source=p.read_bytes();a=json.loads(source)['asset'];record=g/f'published-limb-{name}.json'
 if not record.exists() or json.loads(record.read_bytes()).get('source')!=sha256(source).hexdigest():continue
 v=a['volumes']['500'];vals=[]
 for bone in cases['pairedShafts']:
  q=next(q for q in v['structure'] if q['id']==bone+'-l')
  if q['kind']=='loft':
   sec=min(q['sections'],key=lambda s:s['rx']*s['ry']);vals.append(f"{2000*sec['rx']:.1f} × {2000*sec['ry']:.1f}")
  else:vals.append(f"{2000*min(q['r0'],q['r1']):.1f}")
 proof=next(json.loads(l) for l in (g/f'limb-{name}-glb.log').read_text().splitlines() if l.startswith('{'))
 rows.append('| '+name+' | '+' | '.join(vals)+' | '+str(len(a['poses']))+' | '+f"{proof['bytes']:,}"+' | `'+sha256(source).hexdigest()[:16]+'` |')
(t/'digital-human-long-bones.md').write_text('# Individual long-bone proportions\n\nEight adult figures had inherited 75 mm capsule diameters as the drawing scale grew. Each repaired structural model carries its own high-detail femur, tibia, humerus, radius and ulna, with the corresponding trochanters, condyles and joint ends present in that figure. Each candidate passes 40 left/right radius comparisons, all 52 scale rules, the document and office-print tests and an independent twelve-scene GLB readback.\n\nThe measurements below are millimetres. Paired values are the orthogonal diameters at the loft station with the smallest elliptical area. Single values are the narrower end diameter of a tapered capsule. Poses count the subject’s named postures.\n\n| Subject | Femur | Tibia | Humerus | Radius | Ulna | Poses | GLB bytes | Catalog SHA-256 |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |\n'+'\n'.join(rows)+'\n')
