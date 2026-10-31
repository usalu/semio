"""📐 Independently measure the exact rejected immutable paint polygon before any oracle change."""
import json
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
TICKET=Path(__file__).resolve().parent
rows=json.loads((TICKET/'🗑️generated/paint-polygon-diagnostic.json').read_text(encoding='utf-8'))
report=[]
for row in rows:
    vertices=np.asarray(row['row']['world_vertices'])
    local=vertices-vertices[0]
    crosses=np.cross(local[:,None,:],local[None,:,:]).reshape(-1,3)
    cross=crosses[np.argmax(np.linalg.norm(crosses,axis=1))]
    normal=cross/np.linalg.norm(cross)
    planarity=float(np.abs(local@normal).max())
    drop=np.argmax(np.abs(normal))
    projected=np.delete(local,drop,axis=1)
    normalized=projected/np.ptp(projected,axis=0)
    edges=np.roll(normalized,-1,axis=0)-normalized
    following=np.roll(edges,-1,axis=0)
    turns=edges[:,0]*following[:,1]-edges[:,1]*following[:,0]
    following=np.roll(normalized,-1,axis=0)
    signed=float((normalized[:,0]*following[:,1]-normalized[:,1]*following[:,0]).sum()/2)
    hull=ConvexHull(normalized)
    crossings=[]
    def turn(a,b,c):
        u,v=b-a,c-a
        return float(u[0]*v[1]-u[1]*v[0])
    for i,a in enumerate(normalized):
        b=normalized[(i+1)%len(normalized)]
        for j in range(i+1,len(normalized)):
            if j==i+1 or (i==0 and j==len(normalized)-1):
                continue
            c,d=normalized[j],normalized[(j+1)%len(normalized)]
            if turn(a,b,c)*turn(a,b,d)<0 and turn(c,d,a)*turn(c,d,b)<0:
                crossings.append([i,j])
    area=float(np.linalg.norm(np.cross(local,np.roll(local,-1,axis=0)).sum(axis=0))/2)
    report.append({'row':row['row'],'vertex_count':len(vertices),'world_area':area,'unit_normal':normal.tolist(),'planarity_residual':planarity,'coordinate_64ulp':float(64*np.spacing(np.abs(vertices).max())),'projected_normalized_turns':turns.tolist(),'projected_signed_area':signed,'scipy_hull_area':float(hull.volume),'scipy_hull_vertices':hull.vertices.tolist(),'strict_crossings':crossings,'convex':bool((turns>=0).all() or (turns<=0).all())})
(TICKET/'🗑️generated/rejected-fragment-geometry.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print('[DEBUG] '+json.dumps(report),flush=True)
