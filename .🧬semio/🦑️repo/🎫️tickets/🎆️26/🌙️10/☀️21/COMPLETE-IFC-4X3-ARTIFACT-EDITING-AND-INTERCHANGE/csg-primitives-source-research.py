"""🗼️ Observe released finite CSG primitive domains and physical local-axis geometry before implementation."""
from pathlib import Path
from html.parser import HTMLParser
import ctypes
import hashlib
import json
import math
import sys
import urllib.request

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
from ifcopenshell import ifcopenshell_wrapper


class PrimitiveSourceOracle:
    """🧭️ Return owned declarations, formal diagnostics and measured physical coordinates."""

    def observe(self, name, parameters):
        """🧊️ Keep external runtime classes behind a test-only geometry boundary."""
        f = ifcopenshell.file(schema='IFC4X3_ADD2')
        origin = f.create_entity('IfcCartesianPoint', (0., 0., 0.))
        position = f.create_entity('IfcAxis2Placement3D', origin)
        primitive = f.create_entity(name, Position=position, **parameters)
        solid = f.create_entity('IfcCsgSolid', primitive)
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(f, logger, express_rules=True)
        raw = f.to_string().encode('utf-8')
        (folder / (name + '.ifc')).write_bytes(raw)
        declaration = ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2').declaration_by_name(name)
        result = dict(name=name, parameters=parameters, sha256=hashlib.sha256(raw).hexdigest(), declarations=[dict(name=a.name(), type=str(a.type_of_attribute()), optional=a.optional()) for a in declaration.all_attributes()], diagnostics=logger.statements)
        owner = ifcopenshell.geom.create_shape(ifcopenshell.geom.settings(), solid)
        mesh = owner.geometry if hasattr(owner, 'geometry') else owner
        vertices = list(mesh.verts)
        indices = list(mesh.faces)
        points = [vertices[i:i+3] for i in range(0, len(vertices), 3)]
        volume = math.fsum((points[indices[i]][0]*(points[indices[i+1]][1]*points[indices[i+2]][2]-points[indices[i+1]][2]*points[indices[i+2]][1]) + points[indices[i]][1]*(points[indices[i+1]][2]*points[indices[i+2]][0]-points[indices[i+1]][0]*points[indices[i+2]][2]) + points[indices[i]][2]*(points[indices[i+1]][0]*points[indices[i+2]][1]-points[indices[i+1]][1]*points[indices[i+2]][0]))/6 for i in range(0, len(indices), 3))
        result['mesh'] = dict(vertices=len(points), triangles=len(indices)//3, bounds=[[min(p[k] for p in points) for k in range(3)], [max(p[k] for p in points) for k in range(3)]] if points else None, orientedVolume=volume)
        if name == 'IfcRectangularPyramid':
            result['mesh']['points'] = points
        return result


class TextReader(HTMLParser):
    """📚️ Save primary source text without executing page scripts or embedded instructions."""

    def __init__(self):
        """📰️ Collect visible source fragments in document order."""
        super().__init__()
        self.parts = []

    def handle_data(self, value):
        """🔖️ Preserve each nonempty text fragment for bounded local inspection."""
        if value.strip():
            self.parts.append(value.strip())


folder = ticket / '🗑️generated/csg-primitives'
folder.mkdir(exist_ok=True)
oracle = PrimitiveSourceOracle()
rows = []
for name, parameters in [('IfcRectangularPyramid', dict(XLength=2., YLength=1., Height=3.)), ('IfcRightCircularCylinder', dict(Height=3., Radius=1.)), ('IfcRightCircularCone', dict(Height=3., BottomRadius=1.)), ('IfcSphere', dict(Radius=1.))]:
    url = 'https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/' + name + '.htm'
    data = urllib.request.urlopen(url).read()
    (folder / (name + '.htm')).write_bytes(data)
    reader = TextReader()
    reader.feed(data.decode('utf-8'))
    (folder / (name + '.txt')).write_text('\n'.join(reader.parts), encoding='utf-8')
    observation = oracle.observe(name, parameters)
    observation.update(primaryUrl=url, primarySha256=hashlib.sha256(data).hexdigest())
    rows.append(observation)
    (folder / (name + '.json')).write_text(json.dumps(observation, indent=2, default=str), encoding='utf-8')
    print('[DEBUG] ' + json.dumps(observation, default=str), flush=True)
(folder / 'oracle.json').write_text(json.dumps(dict(version=ifcopenshell.version, rows=rows), indent=2, default=str), encoding='utf-8')
