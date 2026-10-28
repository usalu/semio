"""⭕️ High precision ideal represented-plane norm and chord certificates."""
import sys,json,math
from pathlib import Path
p=Path(__file__).parent;sys.path.insert(0,str(p/'🗑️generated/determinant'));import mpmath as mp
mp.mp.dps=100
cases=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/mapped-circle-cases.json');data=json.loads(cases.read_text());rows=[]
for row in data['cases']:
 x,y,z=map(mp.mpf,row['scales']);t=mp.mpf(row['tilt']);a=mp.mpf(row['profileRotation']);c,s=mp.cos(t),mp.sin(t);sgn=-1 if row['reflected']else 1
 world=mp.matrix([[x*c,0,z*s],[0,sgn*y,0],[-x*s,0,z*c]]);basis=world*mp.matrix([[mp.cos(a),-mp.sin(a)],[mp.sin(a),mp.cos(a)],[0,0]])
 g=basis.T*basis;aa,bb,cc=g[0,0],g[0,1],g[1,1];sigma=mp.sqrt((aa+cc+mp.sqrt((aa-cc)**2+4*bb**2))/2);tol=max(mp.mpf(row['sampling']['absoluteChordErrorMetres']),mp.mpf(row['sampling']['relativeChordError'])*sigma)
 n=4
 while sigma*(1-mp.cos(mp.pi/n))>tol:n+=4
 count=18*n-12
 rows.append(dict(id=row['id'],sigma=str(sigma),idealMinMultiple4=n,chordError=str(sigma*(1-mp.cos(mp.pi/n))),previousError=str(sigma*(1-mp.cos(mp.pi/(n-4))))if n>4 else None,tolerance=str(tol),outputValues=count,segmentGrantAllows=n<=row['sampling']['maxSegments'],outputGrantAllows=count<=row['maxValues']))
(p/'🗑️generated/mapped-circle-independent-certificate.json').write_text(json.dumps(dict(library='mpmath'+mp.__version__,dps=100,scope='Ideal unit-radius represented plane; excludes coordinate/GLB rounding',rows=rows),indent=2)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps([dict(id=r['id'],n=r['idealMinMultiple4'],values=r['outputValues'])for r in rows]))
