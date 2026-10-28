"""🔬 Independent constructor TYPEOF corroboration using IfcOpenShell 0.8.3."""
import sys,json,pathlib
root=pathlib.Path(__file__).parent
sys.path.insert(0,str(root/'🗑️generated'/'ifc083'))
import ifcopenshell
from ifcopenshell.express.rules.IFC4X3_ADD2 import typeof
f=ifcopenshell.file(schema='IFC4X3_ADD2')
rows=[]
for entity in [f.create_entity('IfcCartesianPoint',(0.,0.,0.)),f.create_entity('IfcPositiveLengthMeasure',1.)]:
 rows.append({'constructor':entity.is_a(),'typeof':sorted(typeof(entity))})
(root/'🗑️generated'/'review-oracle.json').write_text(json.dumps(rows,indent=2),encoding='utf8')
print(json.dumps(rows))
