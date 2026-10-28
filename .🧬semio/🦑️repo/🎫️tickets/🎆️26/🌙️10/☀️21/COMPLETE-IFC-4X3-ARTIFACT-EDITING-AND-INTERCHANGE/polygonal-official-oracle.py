"""🧩️ Observe unchanged official polygonal source through a test-only owned oracle interface."""
from pathlib import Path
import ctypes
import hashlib
import json
import math
import sys

ticket = Path(__file__).parent
dependency = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency, buffer, len(buffer))
    dependency = buffer.value or dependency
sys.path.insert(0, dependency)

class PolygonalOracle:
    """🔌️ Return owned primitive observations without exposing a third-party model type."""
    def observe(self, path):
        raise NotImplementedError

class IfcOpenShellPolygonalOracle(PolygonalOracle):
    """🛰️ Keep independent geometry and schema validation behind the test-only boundary."""
    def observe(self, path):
        import ifcopenshell
        import ifcopenshell.geom
        import ifcopenshell.validate
        model = ifcopenshell.open(str(path))
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(model, logger, express_rules=True)
        settings = ifcopenshell.geom.settings()
        settings.set(settings.USE_WORLD_COORDS, True)
        meshes = []
        for product in model.by_type('IfcProduct'):
            if not product.Representation:
                continue
            owner = ifcopenshell.geom.create_shape(settings, product)
            geometry = owner.geometry
            vertices = list(geometry.verts)
            indices = list(geometry.faces)
            points = [vertices[at:at+3] for at in range(0, len(vertices), 3)]
            origin = points[0]
            terms = []
            for at in range(0, len(indices), 3):
                a, b, c = ([n-origin[axis] for axis, n in enumerate(points[indices[at+k]])] for k in range(3))
                terms.append((a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6)
            meshes.append({'product': str(product.id()), 'vertices': len(points), 'triangles': len(indices)//3, 'signedVolume': math.fsum(terms), 'bounds': [[min(p[axis] for p in points) for axis in range(3)], [max(p[axis] for p in points) for axis in range(3)]]})
        return {'version': ifcopenshell.version, 'schema': list(model.header.file_schema.schema_identifiers), 'expressDiagnostics': len(logger.statements), 'statements': logger.statements, 'faceSets': [{'id': str(row.id()), 'closed': row.Closed, 'faces': len(row.Faces), 'holes': sum(len(getattr(face, 'InnerCoordIndices', [])) for face in row.Faces)} for row in model.by_type('IfcPolygonalFaceSet')], 'meshes': meshes}

source = Path(sys.argv[1])
observed = IfcOpenShellPolygonalOracle().observe(source)
result = {'file': source.name, 'sha256': hashlib.sha256(source.read_bytes()).hexdigest(), 'python': sys.executable, 'nativeExecution': False, **observed}
(ticket / '🗑️generated/polygonal-official-oracle.json').write_text(json.dumps(result, ensure_ascii=False, default=str, indent=2), encoding='utf-8')
print('[DEBUG] ' + json.dumps({key: result[key] for key in ['file', 'sha256', 'version', 'expressDiagnostics', 'faceSets', 'meshes']}))
