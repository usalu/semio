"""🧱️ Observe declared block source through a test-only independent physical geometry interface."""
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
import ifcopenshell
import ifcopenshell.geom
import ifcopenshell.validate


class BlockOracle:
    """🧩️ Keep third-party classes private to this physical testing adapter."""

    def observe(self, row):
        """🔬️ Return only owned records of physical validation and mesh measurements."""
        raise NotImplementedError


class IfcOpenShellBlockOracle(BlockOracle):
    """🧱️ Construct independent complete authored derivatives of the pinned original block."""

    def observe(self, row):
        """📐️ Measure actual current STEP roles, SI assignment, frames and nonuniform mapping."""
        original = Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/csg-primitive.ifc')
        f = ifcopenshell.open(str(original))
        point = lambda values: f.create_entity('IfcCartesianPoint', tuple(float(x) for x in values))
        direction = lambda values: f.create_entity('IfcDirection', tuple(float(x) for x in values)) if values is not None else None
        axis = lambda origin, z=None, x=None: f.create_entity('IfcAxis2Placement3D', point(origin), direction(z), direction(x))
        product = f.by_id(1000)
        product.ObjectPlacement = f.create_entity('IfcLocalPlacement', None, axis([0, 0, 0]))
        f.by_type('IfcProject')[0].UnitsInContext.Units[0].Prefix = row['unitPrefix']
        block = f.by_id(1022)
        block.Position = axis(row['position']['origin'], row['position']['axis'], row['position']['refDirection'])
        block.XLength, block.YLength, block.ZLength = (float(n) for n in row['dimensions'])
        csg = f.by_id(1021)
        if row['damage'] == 'wrong-root':
            csg.TreeRootExpression = f.create_entity('IfcPlane', axis([0, 0, 0]))
        representation = product.Representation.Representations[0]
        for layer in row['mapping']:
            old = f.create_entity('IfcShapeRepresentation', representation.ContextOfItems, 'Body', 'CSG', representation.Items)
            origin = axis(layer['origin'], [0, 0, 1], [math.cos(layer['originRotation']), math.sin(layer['originRotation']), 0])
            source = f.create_entity('IfcRepresentationMap', origin, old)
            target = f.create_entity('IfcCartesianTransformationOperator3DnonUniform', direction(layer['axes'][0]), direction(layer['axes'][1]), point(layer['target']), layer['scale'], direction(layer['axes'][2]), *layer['secondary'])
            representation.Items = (f.create_entity('IfcMappedItem', source, target),)
            representation.RepresentationType = 'MappedRepresentation'
        generated = ticket / '🗑️generated/csg-block'
        generated.mkdir(exist_ok=True)
        physical = generated / (row['id'] + '.ifc')
        physical.write_text(f.to_string(), encoding='utf-8')
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(f, logger, express_rules=True)
        result = dict(id=row['id'], sha256=hashlib.sha256(physical.read_bytes()).hexdigest(), diagnostics=logger.statements)
        try:
            settings = ifcopenshell.geom.settings()
            settings.set(settings.USE_WORLD_COORDS, True)
            owner = ifcopenshell.geom.create_shape(settings, product)
            mesh = owner.geometry
            vertices = list(mesh.verts)
            indices = list(mesh.faces)
            points = [vertices[i:i + 3] for i in range(0, len(vertices), 3)]
            origin = points[0]
            volume = 0
            for at in range(0, len(indices), 3):
                a, b, c = [[points[indices[at + k]][j] - origin[j] for j in range(3)] for k in range(3)]
                volume += (a[0] * (b[1]*c[2]-b[2]*c[1]) + a[1] * (b[2]*c[0]-b[0]*c[2]) + a[2] * (b[0]*c[1]-b[1]*c[0])) / 6
            result['mesh'] = dict(vertices=len(points), triangles=len(indices)//3, bounds=[[min(p[k] for p in points) for k in range(3)], [max(p[k] for p in points) for k in range(3)]], orientedVolume=volume)
        except Exception as error:
            result['geometryError'] = str(error)
        return result


rows = json.loads((ticket / 'csg-block-neutral-cases.json').read_text(encoding='utf-8'))['cases']
oracle = IfcOpenShellBlockOracle()
result = []
for row in rows:
    try:
        observation = oracle.observe(row)
    except Exception as error:
        observation = dict(id=row['id'], constructionError=str(error))
    result.append(observation)
    print('[DEBUG] ' + json.dumps(observation, default=str), flush=True)
(ticket / '🗑️generated/csg-block-oracle.json').write_text(json.dumps(dict(version=ifcopenshell.version, rows=result), indent=2, default=str), encoding='utf-8')
