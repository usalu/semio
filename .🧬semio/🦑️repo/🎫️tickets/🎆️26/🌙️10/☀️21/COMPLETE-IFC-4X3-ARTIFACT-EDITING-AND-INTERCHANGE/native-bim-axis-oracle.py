"""🧭 Independent native placement expectations via actual IFC4x3 EXPRESS rules and matrices."""
import ctypes
import json
import math
from pathlib import Path
import sys
from typing import Protocol

ticket=Path(__file__).resolve().parent
module_path=str(ticket/'🗑️generated'/'oracle')
if sys.platform=='win32':
    short=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path,short,len(short))
    module_path=short.value or module_path
sys.path.insert(0,module_path)
import ifcopenshell
import ifcopenshell.validate
from ifcopenshell.util.placement import get_axis2placement

class PlacementOracle(Protocol):
    def evaluate(self,angle:float,present:tuple[bool,bool])->tuple[int,list[list[float]]]: ...

class IfcPlacementOracle:
    def evaluate(self,angle:float,present:tuple[bool,bool])->tuple[int,list[list[float]]]:
        model=ifcopenshell.file(schema='IFC4X3_ADD2')
        point=model.create_entity('IfcCartesianPoint',(1.,2.,4.))
        z=model.create_entity('IfcDirection',(0.,0.,1.)) if present[0] else None
        x=model.create_entity('IfcDirection',(math.cos(angle),math.sin(angle),0.)) if present[1] else None
        axis=model.create_entity('IfcAxis2Placement3D',point,z,x)
        logger=ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(model,logger,express_rules=True)
        return len(logger.statements),get_axis2placement(axis).tolist()

root=next(parent for parent in ticket.parents if (parent/'package.json').exists())
fixture=json.loads((root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/bim-axis-cases.json').read_text('utf8'))
oracle:PlacementOracle=IfcPlacementOracle()
for case in fixture['cases']:
    angle=case['angle'];errors,matrix=oracle.evaluate(angle,(angle!=0.,angle!=0.))
    assert errors==0,(case,errors)
    expected=[[math.cos(angle),-math.sin(angle),0.,1.],[math.sin(angle),math.cos(angle),0.,2.],[0.,0.,1.,4.],[0.,0.,0.,1.]]
    assert all(math.isclose(actual,value,rel_tol=1e-12,abs_tol=1e-12) for row,wanted in zip(matrix,expected) for actual,value in zip(row,wanted)),case
for present in [(True,False),(False,True)]:
    errors,_=oracle.evaluate(0.,present)
    assert errors>0,present
print(json.dumps({'oracle':'IfcOpenShell '+ifcopenshell.version,'schema':fixture['schema'],'pairedRotationExpectations':len(fixture['cases']),'oneSidedIdentityRefusals':2,'nativeExecution':False}))
