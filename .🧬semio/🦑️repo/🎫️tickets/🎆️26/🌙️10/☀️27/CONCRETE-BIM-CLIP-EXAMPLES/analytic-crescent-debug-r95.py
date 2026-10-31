"""🔬 Retain bandwise evidence for the thin-crescent independent oracle regression."""
import importlib.util,json,math,sys
from decimal import localcontext
from fractions import Fraction
from pathlib import Path
sys.dont_write_bytecode=True
T=Path(__file__).parent;R=next(p for p in T.parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());spec=importlib.util.spec_from_file_location('crescent_diagnostic',R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧊️solids/➖️difference/🧪️tests/🔬️scipy.py');O=importlib.util.module_from_spec(spec);spec.loader.exec_module(O);c=next(r for r in json.loads((T/'analytic-difference-intersections-r95.json').read_text())['cases']if r['id']=='same_guid_shifted_two_nanometres_is_real_intersection');rows=[];original=O.quad

def observed(fn,*args,**kwargs):
    """🧾 Record the actual unchanged SciPy integral and its exact normalization interval."""
    closure={k:v.cell_contents for k,v in zip(fn.__code__.co_freevars,fn.__closure__)};value,error=original(fn,*args,**kwargs);rows.append({'left':float(closure['origin']),'right':float(closure['origin']+closure['width']),'width':float(closure['width']),'value':value,'error':error});return value,error
O.quad=observed;v,e=O.intersection_volume(c['left'],c['right']);points=[];d=Fraction.from_float(2e-9)
for x in(.1,.5,.9,.99,1-2e-9,1-1e-9,1.):
    intervals=O._intersect(O.intervals(c['left'],x,Fraction(1,2)),O.intervals(c['right'],x,Fraction(1,2)));actual=math.fsum(hi-lo for lo,hi in intervals);q=Fraction.from_float(x);ra=1-q*q;rb=1-(q-d)**2
    with localcontext()as context:
        context.prec=60;expected=float(2*(O._decimal(rb).sqrt()-O._decimal(ra).sqrt()))if ra>=0 else float(2*O._decimal(rb).sqrt())
    points.append({'x':x,'actual':actual,'expected':expected,'difference':actual-expected})
out={'volume':v,'estimated_error':e,'bands':rows,'points':points};(T/'🗑️generated/analytic-crescent-debug-r95.json').write_text(json.dumps(out,indent=2));print('[DEBUG] '+json.dumps(out),flush=True)
