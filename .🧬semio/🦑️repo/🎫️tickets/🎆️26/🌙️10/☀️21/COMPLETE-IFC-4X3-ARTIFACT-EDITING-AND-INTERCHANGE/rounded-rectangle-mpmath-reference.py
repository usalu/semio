"""▢ High precision independent ideal/sampled rounded-profile formulas."""
from pathlib import Path
import sys,json,hashlib
p=Path(__file__).parent;sys.path.insert(0,str(p/'🗑️generated/determinant'));import mpmath as mp
mp.mp.dps=100
contract=p/'rounded-rectangle-profile-cases.json';data=json.loads(contract.read_text(encoding='utf-8'));three=json.loads((p/'🗑️generated/rounded-rectangle-three-reference.json').read_text(encoding='utf8'));external={r['id']:r for r in three['rows']};out=[]
def m(x):return mp.mpf(x)
def encoded(x):return mp.nstr(x,90)
for row in data['cases']:
 r={'id':row['id']};out.append(r)
 if row['formal']!='valid':r['status']='source-contract-refusal-no-ideal-claim';continue
 x,y=map(m,row['dimensions']);radius=m(row['radius']);unit=mp.mpf('0.001') if row['unitPrefix']=='MILLI' else mp.mpf(1);phi=m(row['profile']['rotation']);tilt=m(row['solid']['tilt']);sx,sy,sz=map(m,row['mappingScale']);c,s=mp.cos(phi),mp.sin(phi);ct,st=mp.cos(tilt),mp.sin(tilt)
 u=[sx*c,sy*ct*s,sz*st*s];v=[-sx*s,sy*ct*c,sz*st*c];aa=sum(z*z for z in u);bb=sum(z*z for z in v);ab=sum(a*b for a,b in zip(u,v));sigma=mp.sqrt((aa+bb+mp.sqrt((aa-bb)**2+4*ab**2))/2);rho=radius*unit*sigma;n=external[row['id']]['curvedSegments'];theta=2*mp.pi/n
 area=x*y-(4-mp.pi)*radius**2;sample=x*y-4*radius**2+n*radius**2*mp.sin(theta)/2;area_error=n*radius**2*(theta-mp.sin(theta))/2
 direction=list(map(m,row['direction']));factor=m(row['depth'])*abs(direction[2])/mp.sqrt(sum(z*z for z in direction))*abs(sx*sy*sz)*unit**3
 tolerance=max(m(row['sampling']['absoluteChordErrorMetres']),m(row['sampling']['relativeChordError'])*rho);sagitta=rho*(1-mp.cos(mp.pi/n));idealN=4
 while rho*(1-mp.cos(mp.pi/idealN))>tolerance:idealN+=4
 r.update(status=external[row['id']]['status'],curveSegments=n,idealSagittaMinimumMultiple4=idealN,worldRadius=encoded(rho),worldSagittaUpper=encoded(sagitta),worldTolerance=encoded(tolerance),sagittaAdmitted=bool(sagitta<=tolerance),idealArea=encoded(area),sampledArea=encoded(sample),areaError=encoded(area_error),areaErrorIdentity=bool(abs(area-sample-area_error)<=max(1,abs(area))*mp.mpf('1e-90')),areaErrorIdentityResidual=encoded(abs(area-sample-area_error)),idealVolume=encoded(area*factor),sampledVolume=encoded(sample*factor),volumeError=encoded(area_error*factor))
 if external[row['id']].get('volume') is not None:r['threeVolumeAbsoluteError']=encoded(abs(m(external[row['id']]['volume'])-sample*factor));r['threeAreaAbsoluteError']=encoded(abs(m(external[row['id']]['area'])-sample))
(p/'🗑️generated/rounded-rectangle-mpmath-reference.json').write_text(json.dumps({'mpmath':mp.__version__,'digits':mp.mp.dps,'sourceSha256':hashlib.sha256(contract.read_bytes()).hexdigest(),'producerExecuted':False,'rows':out},indent=2)+'\n',encoding='utf8')
print('[DEBUG]',[(r['id'],r.get('curveSegments'),r.get('sagittaAdmitted'),r.get('areaErrorIdentity')) for r in out])

