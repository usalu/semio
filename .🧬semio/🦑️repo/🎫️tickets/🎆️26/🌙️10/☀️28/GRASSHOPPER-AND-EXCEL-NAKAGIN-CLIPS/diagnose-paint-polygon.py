"""🔎 Retain exact actual paint boundaries rejected by the original independent ear clipper."""
import importlib.util
import json
from pathlib import Path
import sys
import numpy as np
TICKET=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('paint_diagnostic_oracle',TICKET/'pixel-nearest-surface.py')
M=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=M
spec.loader.exec_module(M)
rows=json.loads((TICKET/'🗑️generated/capsule-fragment-faceted-final/coverage-paint-fragments.json').read_text(encoding='utf-8'))
failures=[]
for index,row in enumerate(rows):
    points=np.asarray(row['world_vertices'],float)
    try:
        M.O.face_triangles(tuple(map(tuple,points)),tuple(range(len(points))))
    except ValueError as error:
        cross=np.cross(points[1:]-points[0],np.roll(points[1:]-points[0],1,axis=0))
        failures.append({'index':index,'row':row,'cross_norms':np.linalg.norm(cross,axis=1).tolist(),'error':str(error)})
        if len(failures)==3:
            break
(TICKET/'🗑️generated/paint-polygon-diagnostic.json').write_text(json.dumps(failures,ensure_ascii=False,indent=2),encoding='utf-8')
print('[DEBUG] '+json.dumps(failures),flush=True)
