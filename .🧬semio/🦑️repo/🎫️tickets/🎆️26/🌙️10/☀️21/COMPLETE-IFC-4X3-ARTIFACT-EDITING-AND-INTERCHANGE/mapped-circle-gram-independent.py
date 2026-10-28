"""📐️ Check the proposed normalized Gram envelope against high precision binary64 columns."""
import sys,math,json,random
from pathlib import Path
p=Path(__file__).parent;sys.path.insert(0,str(p/'🗑️generated/determinant'));import mpmath as mp
mp.mp.dps=110;eps=2**-52;random.seed(413)
cases=[('identity',[1,0,0,0,1,0]),('near-collinear',[1,1,1,1,1,math.nextafter(1,2)]),('aspect',[1,0,0,0,2**-1022,0]),('subnormal',[5e-324,0,0,0,5e-324,0]),('huge',[1e308,0,0,0,1e308,0])]
for i in range(300):
 power=random.randint(-1000,1000);factor=math.ldexp(1.,power);cases.append(('seed-'+str(i),[random.uniform(-1,1)*factor for _ in range(6)]))
rows=[];worst=mp.inf
for name,a in cases:
 scale=max(map(abs,a));v=[x/scale for x in a];aa=sum(x*x for x in v[:3]);bb=sum(x*x for x in v[3:]);ab=sum(x*y for x,y in zip(v[:3],v[3:]));upper=(.5*(aa+bb+math.hypot(aa-bb,2*ab))+64*eps);bound=math.sqrt(upper)*(1+16*eps)
 exact=[mp.mpf(x)/mp.mpf(scale)for x in a];x=sum(z*z for z in exact[:3]);y=sum(z*z for z in exact[3:]);z=sum(u*v for u,v in zip(exact[:3],exact[3:]));sigma=mp.sqrt((x+y+mp.sqrt((x-y)**2+4*z*z))/2);margin=mp.mpf(bound)/sigma-1;assert margin>=0,(name,margin);worst=min(worst,margin)
 if not name.startswith('seed-'):rows.append(dict(id=name,largest=scale,normalizedBound=bound,exactNormalizedNorm=str(sigma),relativeMargin=str(margin)))
(p/'🗑️generated/mapped-circle-gram-independent.json').write_text(json.dumps(dict(library='mpmath'+mp.__version__,dps=110,samples=len(cases),minimumRelativeMargin=str(worst),rows=rows,scope='Finite deterministic numerical corroboration, not universal JS libm proof'),indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+str(len(cases))+' Gram upperbounds verified; minimum relative envelope '+str(worst))
