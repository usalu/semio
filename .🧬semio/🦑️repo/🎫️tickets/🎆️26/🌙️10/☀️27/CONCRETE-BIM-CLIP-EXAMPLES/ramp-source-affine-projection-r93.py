"""📐 Verify a source-affine T junction through exact input and independent integer-affine GEOS."""
import importlib,json,sys
from fractions import Fraction
from pathlib import Path
from jsonschema import Draft202012Validator
from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely.affinity import scale
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;case=json.loads((ticket/'ramp-source-affine-projection-r93.json').read_text(encoding='utf8'));Draft202012Validator(json.loads((ticket/'ramp-source-affine-projection-r93.schema.json').read_text(encoding='utf8'))).validate(case)
U=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧩️regions.🐍️')
def scalar(v):
    """🔢 Decode a language-independent exact source expression without decimal reseeding."""
    return Fraction(v['numerator'],v['denominator']) if isinstance(v,dict) else Fraction(v)
polygons=[[[scalar(v) for v in point] for point in ring] for ring in case['polygons']];expected=scale(unary_union([Polygon([[int(x*case['scale'][0]),int(y*case['scale'][1])] for x,y in ring]) for ring in polygons]),xfact=1/case['scale'][0],yfact=1/case['scale'][1],origin=(0,0));rows=[]
for label,inputs in [('private',polygons),('published',[[[float(v) for v in p] for p in ring] for ring in polygons])]:
    regions=U.polygon_union(inputs);body=unary_union([Polygon(r['outer'],r['holes']) for r in regions]);length=sum(Polygon(r['outer'],r['holes']).length for r in regions);row={'mode':label,'regions':len(regions),'area_difference':body.symmetric_difference(expected).area,'boundary_length':length,'boundary_difference':body.boundary.symmetric_difference(expected.boundary).length};rows.append(row)
    if label=='private':assert len(regions)==1 and body.symmetric_difference(expected).area==0 and length==4 and body.boundary.symmetric_difference(expected.boundary).length==0
print('[DEBUG] '+json.dumps(rows));(ticket/'🗑️generated/ramp-source-affine-projection-r93.json').write_text(json.dumps({'pass':True,'rows':rows},indent=2),encoding='utf8')
