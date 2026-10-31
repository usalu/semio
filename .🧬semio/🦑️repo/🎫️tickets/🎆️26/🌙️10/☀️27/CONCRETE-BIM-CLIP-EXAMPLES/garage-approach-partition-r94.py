"""📦 Capture actual candidate pairs rejected by the production partition proof."""
import importlib,json,sys
from pathlib import Path
from fractions import Fraction
from shapely.geometry import Polygon
root=Path.cwd();ticket=Path(__file__).parent;sys.path.insert(0,str(root));api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');out=ticket/'🗑️generated/native-r92';candidate=json.loads((out/'garage-approach-candidate-r94.json').read_text(encoding='utf-8'))
records=sorted([(api._material_bounds(p),api.part_footprint(p),i) for i,p in enumerate(candidate['parts'])],key=lambda row:row[0][0][0]);active=[];pairs=[];total=0
for bounds,outline,index in records:
    active=[r for r in active if r[0][0][1]>bounds[0][0]]
    for other,ring,j in active:
        if any(bounds[k][1]<=other[k][0] or other[k][1]<=bounds[k][0] for k in (1,2)):continue
        if not api._surface_separated(outline,ring):
            total+=1
            if len(pairs)<1000:pairs.append({'a':index,'b':j,'a_row':candidate['cells'][index],'b_row':candidate['cells'][j],'a_outline':outline,'b_outline':ring,'intersection_geos_area':Polygon(outline).intersection(Polygon(ring)).area,'a_part':candidate['parts'][index],'b_part':candidate['parts'][j]})
    active.append((bounds,outline,index))
reference=json.loads((ticket/'garage-approach-input-r94.json').read_text(encoding='utf-8'))
original_parts=[{'outline':ring,'z0':-.19,'z1':-.15} for ring in reference['reserved_cells']]
original_disjoint=api._parts_disjoint(original_parts)
decimal_disjoint=api._parts_disjoint([{**p,"outline":[[Fraction(str(v)) for v in point] for point in p["outline"]]} for p in original_parts])
report={'original_parts_disjoint':original_disjoint,'decimal_reinterpreted_input_disjoint':decimal_disjoint,'unproven_pairs':total,'same_plane_pairs':sum(p['a_row']['patch']==p['b_row']['patch'] for p in pairs),'pairs':pairs};(out/'garage-approach-partition-red-r94.json').write_text(json.dumps(report,ensure_ascii=True,indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'original_parts_disjoint':original_disjoint,'same_plane_pairs':report['same_plane_pairs'],'unproven_pairs':total,'examples':[{k:v for k,v in p.items() if k in ('a','b','a_row','b_row','intersection_geos_area')} for p in pairs[:8]]}))
