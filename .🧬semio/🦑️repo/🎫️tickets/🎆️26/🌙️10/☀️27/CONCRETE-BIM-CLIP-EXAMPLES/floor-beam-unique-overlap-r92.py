"""🔬 Reproduce the exact portable red-prism overlap and GEOS precision limitation."""
import json
from pathlib import Path
from shapely.geometry import shape
from shapely.ops import unary_union
from shapely import set_precision
p=Path(__file__).parent;case=json.loads((p/'floor-beam-owned-cases-r92.json').read_text(encoding='utf8'));rows=case['red_intersection_prisms']
for r in rows:
 r['_shape']=shape(r['geometry']);assert r['_shape'].is_valid;assert abs(r['_shape'].area*(r['z'][1]-r['z'][0])-r['volume_m3'])<1e-10
pairwise=[]
for i,a in enumerate(rows):
 for b in rows[i+1:]:
  lo,hi=max(a['z'][0],b['z'][0]),min(a['z'][1],b['z'][1])
  if hi>lo:
   v=a['_shape'].intersection(b['_shape']).area*(hi-lo)
   if v>1e-12:pairwise.append(v)
assert not pairwise,'Portablecounterexampleprismsmustremainindependentlydisjoint'
def union_volume(rs,grid=None):
 zs=sorted(set(z for r in rs for z in r['z']));v=0.
 for lo,hi in zip(zs,zs[1:]):
  mid=(lo+hi)/2;ss=[r['_shape'] if grid is None else set_precision(r['_shape'],grid) for r in rs if r['z'][0]<mid<r['z'][1]]
  if ss:v+=unary_union(ss).area*(hi-lo)
 return v
accepted=sum(r['volume_m3'] for r in rows);variants=[{'diagnostic_grid_m':g,'volume_m3':union_volume(rows,g)} for g in [1e-9,1e-10,1e-11,1e-12]];assert all(abs(x['volume_m3']-accepted)<1e-9 for x in variants);groups=sorted(set((r['floor'],r['floor_material']) for r in rows));out={'source':case['source'],'producer':case['producer'],'scope':'Portableactualexclusivefinish/beamredintersectionprisms; productiongeometryunchanged','unique_union_m3':accepted,'pair_sum_m3':accepted,'positive_pairwise_overlaps_above_1e_minus12_m3':pairwise,'raw_unquantized_union_m3':union_volume(rows),'diagnostic_precision_variants':variants,'corrected_material_groups':[{'floor':k[0],'material':k[1],'volume_m3':sum(r['volume_m3'] for r in rows if (r['floor'],r['floor_material'])==k)} for k in groups],'correction':'Rawglobalunionprecisionlossisnotphysicalduplication. Pairwisedisjointnessandfourprecisiondiagnosticsagree. No sourcequantizationorfeaturediscardisproposed.'};(p/'🗑️generated/floor-beam-unique-overlap-r92.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print(json.dumps(out),flush=True)
