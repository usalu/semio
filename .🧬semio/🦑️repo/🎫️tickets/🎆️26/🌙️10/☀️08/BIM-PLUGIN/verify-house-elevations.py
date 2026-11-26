"""🏛️ Check the simple-ring shortcut on triangulated exchange geometry from the authored house."""
import json
from pathlib import Path
import subprocess
import ifcopenshell
import ifcopenshell.geom
import numpy as np
from shapely.geometry import Polygon
from shapely.ops import unary_union

fixture = next(Path('✏️s/🔌️plugins/🏙️bim').rglob('🏢️ifc4/🏡️example-house/🏡️example-house.ifc'))
model = ifcopenshell.open(str(fixture))
settings = ifcopenshell.geom.settings()
settings.set('use-world-coords', True)
triangles = []
for element in model.by_type('IfcElement'):
    if element.Representation is None:
        continue
    shape = ifcopenshell.geom.create_shape(settings, element)
    mesh = shape.geometry
    points = np.array(mesh.verts).reshape(-1,3)
    faces = points[np.array(mesh.faces, dtype=int).reshape(-1,3)]
    for axis in [1, -1, 2, -2]:
        side = np.sign(axis)
        projected = np.stack((side*faces[:,:,abs(axis)-1],faces[:,:,2]),axis=2)
        delta = projected[:,1:,:]-projected[:,:1,:]
        area = delta[:,0,0]*delta[:,1,1]-delta[:,0,1]*delta[:,1,0]
        front = projected[area>1e-12]
        if len(front):
            triangles.append(front)
rows = [' '.join(map(str,mesh.ravel().tolist())) for mesh in triangles]
checker = Path(__file__).resolve().parent/'🗑️generated/outline-oracle'
result = subprocess.run([str(checker)],input='\n'.join(rows)+'\n',text=True,capture_output=True,check=True)
accepted = 0
for projection, line in zip(triangles,result.stdout.splitlines(),strict=True):
    ring = json.loads(line)
    if ring:
        shape = Polygon(ring)
        union = unary_union([Polygon(tri) for tri in projection])
        assert shape.is_valid and shape.symmetric_difference(union).area <= 1e-8
        accepted += 1
print(f'{accepted}/{len(triangles)} house IFC element elevations form certified simple rings')
