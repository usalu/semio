"""🧮 Probe exact source-operand arithmetic without changing production clipping or tolerances."""
import importlib,json,sys,time
from fractions import Fraction
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');an=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️');ticket=Path(__file__).parent;case=json.loads((ticket/'roof-residual-operands-r92.json').read_text(encoding='utf8'));original=an._subtract;oldloops=an._loops
parts=[{**p,'outline':[[Fraction(x),Fraction(y)]for x,y in p['outline']]}for p in case['parts']];cutters=[{**p,'outline':[[Fraction(x),Fraction(y)]for x,y in p['outline']]}for p in case['cutters']]
def exact(piece,void,digits=9,tolerance=an.EPS):
    """📐 Keep rational vertices through the entire sequence, not a rounded intersection after each plane."""
    return original(piece,void,digits=None,tolerance=tolerance)
an._subtract=exact;an._loops=lambda pieces,tolerance=an.EPS:[];start=time.monotonic()
try:
    result=a._subtract_prisms(parts,cutters);published=[[[float(x),float(y)]for x,y in p['outline']]for p in result];bad=[p for p in published if not Polygon(p).is_valid];area=sum(Polygon(p).area for p in published);expected=Polygon(case['parts'][0]['outline']).difference(unary_union([Polygon(c['outline'])for c in case['cutters']])).area
    out={'parts':len(result),'invalid':bad,'area':area,'expected_geos':expected,'difference':area-expected,'seconds':time.monotonic()-start};(ticket/'🗑️generated'/'roof-rational-probe-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(out),flush=True)
finally:an._subtract=original;an._loops=oldloops
