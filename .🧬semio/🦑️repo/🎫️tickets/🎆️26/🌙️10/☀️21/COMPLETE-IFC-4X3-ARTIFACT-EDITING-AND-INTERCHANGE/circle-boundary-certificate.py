"""⭕ Independent binary64 threshold/minimality certificates for bounded quadrant search."""
import json,math,sys
from pathlib import Path
t=Path(__file__).resolve().parent;sys.path.insert(0,str(t/'🗑️generated/determinant'));import mpmath as mp
mp.mp.dps=100;rows=[]
for nominal in [4,8,72,500,4096]:
 exact=2*mp.sin(mp.pi/(2*nominal))**2;nearest=float(exact)
 for side,value in [('below',math.nextafter(nearest,-math.inf)),('nearest',nearest),('above',math.nextafter(nearest,math.inf))]:
  tol=mp.mpf(value);n=next((n for n in range(4,4097,4)if 2*mp.sin(mp.pi/(2*n))**2<=tol),None);rows.append({'nominal':nominal,'side':side,'radius':1,'absolute':value,'absoluteHex':value.hex(),'relative':0,'maximum':4096,'minimalSegments':n,'nominalPasses':exact<=tol,'binary64DirectPasses':2*math.sin(math.pi/(2*nominal))**2<=value})
(t/'🗑️generated/geometry-query/circle-boundary-certificate.json').write_text(json.dumps({'library':mp.__version__,'precision':mp.mp.dps,'rows':rows,'scope':'Independent exact binary64 input threshold tests, not universal Math.sin proof'},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'cases':len(rows),'directComparisonMismatches':sum(r['nominalPasses']!=r['binary64DirectPasses']for r in rows)}))
