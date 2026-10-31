"""🧪 Validate exact negative certificates against independent native-shape and analytic distance oracles."""
import hashlib,importlib.util,json,sys,time,traceback
from pathlib import Path
import numpy as np
from jsonschema import Draft202012Validator
from scipy.optimize import linprog
from scipy.spatial import ConvexHull
from shapely.geometry import Point,Polygon
sys.dont_write_bytecode=True
T=Path(__file__).parent;paths=[T/n for n in('whole-exact-separation-r95.py','whole-exact-separation-r95.json','whole-exact-separation-r95.schema.json','whole-expanded-interference-r92.py')]
def hashes():
    """🔐 Bind the exact independent validator, neutral cases and actual audit kernel."""
    return{p.name:hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();spec=importlib.util.spec_from_file_location('whole_exact_support_r95',paths[-1]);A=importlib.util.module_from_spec(spec);spec.loader.exec_module(A);case=json.loads(paths[1].read_text(encoding='utf8'));Draft202012Validator(json.loads(paths[2].read_text(encoding='utf8'))).validate(case);assert len({r['id']for r in case['cases']})==len(case['cases']);rows=[];start=time.monotonic()
for c in case['cases']:
    row={'id':c['id'],'expected':c['separated'],'pass':False}
    try:
        a,b=A.rawsolid(c['a']),A.rawsolid(c['b']);kind=c['oracle']
        if kind=='polyhedron':
            points=np.concatenate([a.v,b.v]);origin=(points.min(axis=0)+points.max(axis=0))/2;scale=np.ptp(points,axis=0).max();hulls=[ConvexHull((s.v-origin)/scale)for s in(a,b)];planes=np.concatenate([h.equations for h in hulls]);fit=linprog([0,0,0,-1],A_ub=np.column_stack([planes[:,:3],np.linalg.norm(planes[:,:3],axis=1)]),b_ub=-planes[:,3],bounds=[(None,None)]*3+[(None,None)],method='highs',options={'primal_feasibility_tolerance':1e-10,'dual_feasibility_tolerance':1e-10});radius=float(fit.x[3])*scale if fit.success else None;oracle=not fit.success or radius<=1e-10;row['independent_interior_radius']=radius;assert oracle==c['separated'],('oracle',radius,c['separated'])
        elif kind=='circle_polygon':
            distance=Polygon(c['b']['outline']).distance(Point(c['a']['axis']));r=c['a']['profile']['diameter']['value']/2;row['independent_clearance']=distance-r;assert(distance>=r)==c['separated']
        elif kind=='circle_circle':
            distance=Point(c['a']['axis']).distance(Point(c['b']['axis']));r=sum(c[k]['profile']['diameter']['value']/2 for k in('a','b'));row['independent_clearance']=distance-r;assert(distance>=r)==c['separated']
        elif kind=='void':
            assert Polygon(c['a']['outline'],c['a'].get('holes',[])).intersection(Polygon(c['b']['outline'])).area==0;assert not c['separated'];row['independent_material_area']=0
        elif kind=='circle_hole':
            radius=c['a']['profile']['voids'][0]['diameter']['value']/2;assert all(Point(v).distance(Point(c['a']['axis']))<radius for v in c['b']['outline']);assert not c['separated'];row['independent_material_area']=0
        if c.get('require_conservative_bounds'):
            from fractions import Fraction
            q=lambda v:Fraction.from_float(float(v));cx,cy=c['a']['axis'];radius=q(c['a']['profile']['diameter']['value'])/2;assert q(a.bounds[0])<=q(cx)-radius and q(a.bounds[1])<=q(cy)-radius and q(a.bounds[2])>=q(cx)+radius and q(a.bounds[3])>=q(cy)+radius,'Analytic bounds rounded inward'
        if c.get('numeric_zero_must_remain_unresolved'):
            measured=A.intersection(a,b);assert measured['volume']>0 or measured['error']>0,'An unproved analytic cap silently passed as zero'
        proof=A.exact_support_separation(a,b);assert bool(proof)==c['separated'],('certificate',proof);row['proof']=proof
        if c['separated']and(a.poly is None or b.poly is None):
            result=A.intersection(a,b);assert result['volume']==0 and result['error']==0 and result['method']=='exact_binary64_support_negative';row['integrated_negative']=True
        row['pass']=True
    except Exception:row['error']=traceback.format_exc()
    rows.append(row)
after=hashes();report={'pass':all(r['pass']for r in rows)and before==after,'stable':before==after,'cases':len(rows),'rows':rows,'before':before,'after':after,'seconds':time.monotonic()-start,'scope':'Sufficient exact support negatives only; concave and hollow unproven cases remain on the original intersection path. GEOS analytic distance uses the true circle, with no buffer/facet substitution.'};(T/'🗑️generated/whole-exact-separation-r95.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items()if k not in('rows','before','after')}),flush=True);print('[DEBUG] failing_cases='+str([r['id']for r in rows if not r['pass']]),flush=True);raise SystemExit(not report['pass'])
                