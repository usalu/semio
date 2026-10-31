"""🔬 Compare the actual warped handle quad with the existing Rhino mesh triangulation oracle."""
import json
from pathlib import Path
import numpy as np
import rhino3dm as rh
folder=Path(__file__).parent/'🗑️generated'
row=json.loads((folder/'coverage-witness-512/coverage-items.json').read_text(encoding='utf-8'))[1768]
points=np.asarray(row['world_vertices'][:len(row['pixel_polygons'][0])],float)
mesh=rh.Mesh()
for point in points:
    mesh.Vertices.Add(*map(float,point))
mesh.Faces.AddFace(0,1,2,3)
before=[tuple(mesh.Faces[i]) for i in range(len(mesh.Faces))]
mesh.Faces.ConvertQuadsToTriangles()
after=[tuple(mesh.Faces[i]) for i in range(len(mesh.Faces))]
report={'part':row['key'],'vertices':points.tolist(),'before':before,'rhino_triangles':after,'diagonal_02':float(np.linalg.norm(points[2]-points[0])),'diagonal_13':float(np.linalg.norm(points[3]-points[1]))}
(folder/'warped-quad-oracle.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
print('[DEBUG] Actual Rhino quad triangulation:',after,flush=True)
