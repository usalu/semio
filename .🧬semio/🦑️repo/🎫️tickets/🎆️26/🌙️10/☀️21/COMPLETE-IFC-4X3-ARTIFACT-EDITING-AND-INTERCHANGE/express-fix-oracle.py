"""👜 Independent multiset multiplicity and IfcOpenShell TYPEOF reprobes; test-only imports."""
import json,pathlib,sys
sys.path.insert(0,sys.argv[1]);from multiset import Multiset
import ifcopenshell
from ifcopenshell.express.rules.IFC4X3_ADD2 import typeof
cases=json.loads(pathlib.Path(sys.argv[2]).read_text(encoding='utf-8-sig'));rows=[]
for row in cases:
 if row.get('error') or '?' in row['left']+row['right']:continue
 left=Multiset(int(value) for value in row['left']);right=Multiset(int(value) for value in row['right']);actual=left<=right if row['operator']=='<=' else left>=right
 assert str(actual).lower()==row['value'];rows.append({'name':row['name'],'actual':str(actual).lower(),'matches':True})
f=ifcopenshell.file(schema='IFC4X3_ADD2')
for entity in [f.create_entity('IfcCartesianPoint',(0.,0.,0.)),f.create_entity('IfcPositiveLengthMeasure',1.)]:rows.append({'constructor':entity.is_a(),'typeof':sorted(typeof(entity))})
output=pathlib.Path(sys.argv[3]);(output/'express-fix-oracle.json').write_text(json.dumps(rows,indent=2),encoding='utf-8');print(json.dumps(rows))
