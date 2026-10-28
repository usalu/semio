"""🔬 Corroborate native domain IFC entity shapes with the retained independent schema."""
import json
import pathlib
import sys

root = pathlib.Path(__file__).parent
sys.path.insert(0, str(root / '🗑️generated' / 'ifc083'))
import ifcopenshell

f = ifcopenshell.file(schema='IFC4X3_ADD2')
rows = []
for name in ['IfcDerivedUnit', 'IfcStructuralAnalysisModel', 'IfcStructuralPointConnection', 'IfcStructuralCurveMember', 'IfcRelConnectsStructuralMember', 'IfcBoundaryNodeCondition', 'IfcBuilding']:
    entity = f.create_entity(name)
    rows.append({'entity': name, 'attributes': [entity.attribute_name(i) for i in range(len(entity))]})
out = root / '🗑️generated' / 'native-domain-review.json'
out.write_text(json.dumps({'nativeExecuted': False, 'schema': f.schema, 'rows': rows}, indent=2), encoding='utf-8')
print('[DEBUG] Independent domain schema attributes checked')
