"""🗼️ Observe actual complete project/product CSG round primitives through an owned test interface."""
from pathlib import Path
import hashlib
import json
import math
import ifcopenshell
import ifcopenshell.geom
import ifcopenshell.validate

ticket = Path(__file__).parent
folder = ticket / '🗑️generated/csg-round-products'
folder.mkdir(parents=True, exist_ok=True)


class RoundProductOracle:
    """🔎️ Return only owned physical declaration, formal and mesh observations."""

    def observe(self, name, dimensions):
        """🏗️ Retain a real project, units, context, placed product and one CSG representation."""
        source = ifcopenshell.file(schema='IFC4X3_ADD2')
        point = source.create_entity('IfcCartesianPoint', (0.0, 0.0, 0.0))
        axis = source.create_entity('IfcAxis2Placement3D', point)
        context = source.create_entity('IfcGeometricRepresentationContext', ContextType='Model', CoordinateSpaceDimension=3, Precision=1e-6, WorldCoordinateSystem=axis)
        unit = source.create_entity('IfcSIUnit', UnitType='LENGTHUNIT', Name='METRE')
        units = source.create_entity('IfcUnitAssignment', (unit,))
        source.create_entity('IfcProject', GlobalId='0000000000000000000001', Name=name, RepresentationContexts=(context,), UnitsInContext=units)
        primitive = source.create_entity(name, Position=axis, **dimensions)
        solid = source.create_entity('IfcCsgSolid', primitive)
        representation = source.create_entity('IfcShapeRepresentation', context, 'Body', 'CSG', (solid,))
        shape = source.create_entity('IfcProductDefinitionShape', Representations=(representation,))
        placement = source.create_entity('IfcLocalPlacement', RelativePlacement=axis)
        product = source.create_entity('IfcBuildingElementProxy', GlobalId='0000000000000000000002', Name=name, ObjectPlacement=placement, Representation=shape)
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(source, logger, express_rules=True)
        raw = source.to_string().encode('utf-8')
        (folder / (name + '.ifc')).write_bytes(raw)
        result = {'name': name, 'dimensions': dimensions, 'entities': len(list(source)), 'formalDiagnostics': logger.statements, 'sha256': hashlib.sha256(raw).hexdigest(), 'product': product.id(), 'primitive': primitive.id(), 'solid': solid.id()}
        try:
            owner = ifcopenshell.geom.create_shape(ifcopenshell.geom.settings(), product)
            geometry = owner.geometry
            flat = list(geometry.verts)
            indices = list(geometry.faces)
            points = [flat[at:at + 3] for at in range(0, len(flat), 3)]
            volume = math.fsum((points[indices[at]][0] * (points[indices[at + 1]][1] * points[indices[at + 2]][2] - points[indices[at + 1]][2] * points[indices[at + 2]][1]) + points[indices[at]][1] * (points[indices[at + 1]][2] * points[indices[at + 2]][0] - points[indices[at + 1]][0] * points[indices[at + 2]][2]) + points[indices[at]][2] * (points[indices[at + 1]][0] * points[indices[at + 2]][1] - points[indices[at + 1]][1] * points[indices[at + 2]][0])) / 6 for at in range(0, len(indices), 3))
            result['mesh'] = {'status': 'nonempty' if points and indices else 'empty', 'vertices': len(points), 'triangles': len(indices) // 3, 'volume': volume, 'bounds': [[min(point[axis] for point in points) for axis in range(3)], [max(point[axis] for point in points) for axis in range(3)]] if points else None}
        except RuntimeError as error:
            result['mesh'] = {'status': 'unsupported', 'error': str(error)}
        return result


oracle = RoundProductOracle()
rows = [oracle.observe('IfcRightCircularCylinder', {'Height': 3.0, 'Radius': 1.0}), oracle.observe('IfcRightCircularCone', {'Height': 3.0, 'BottomRadius': 1.0})]
result = {'version': ifcopenshell.version, 'nativeExecuted': False, 'rows': rows}
(folder / 'oracle.json').write_text(json.dumps(result, indent=2, default=str), encoding='utf-8')
print('[DEBUG] ' + json.dumps(result, default=str), flush=True)
