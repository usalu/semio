"""🧫 Measures the remaining bone-to-cartilage and cartilage-to-sternum gap for each corrective bridge."""
import json
from pathlib import Path
from math import dist
from runpy import run_path
from contextlib import redirect_stdout
import io
t=Path(__file__).resolve().parent
with redirect_stdout(io.StringIO()):
    f=run_path(str(t/'hu-spine-check.py'))
    g=run_path(str(t/'hu-cartilage.py'))
count=0
for slug,(name,ns,r) in g['sets'].items():
    path=next((g['hum']/slug).glob('*'))
    solids={q['id']:q for q in json.loads(path.read_text())['asset']['volumes']['50']['structure']}
    sternum=solids['sternum'];results=[]
    for n in ns:
        for side in ['l','r']:
            rib=solids[name.format(n=n,s=side)];bridge=solids['cartilage-'+str(n)+'-'+side]
            a=bridge['sections'][0];b=bridge['sections'][-1];tip=rib['sections'][-1]
            axial=dist(a['at'],tip['at'])-min(a['rx'],a['ry'])-min(tip['rx'],tip['ry'])
            sternal=f['clearance'](b['at'],sternum)-min(b['rx'],b['ry'])
            assert max(axial,sternal)<0,(slug,bridge['id'],axial,sternal)
            results.append(max(axial,sternal));count+=1
    print(slug,len(results),'joins, maximum signed gap mm',round(max(results)*1000,1))
print(count,'cartilage joins')
