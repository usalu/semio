"""📏 Compare actual named unit chains using existing third-party unit API."""
import ctypes,json,math,sys
from pathlib import Path
root=Path(__file__).parent;dep=str(root/'🗑️generated/oracle')
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,short,len(short));dep=short.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.util.unit
vectors=json.loads((root/'unit-traversal-vectors.json').read_text(encoding='utf-8-sig'));rows=[];out=root/'🗑️generated/unit-traversal';out.mkdir(exist_ok=True)
for target in vectors['lengths']:
 f=ifcopenshell.file(schema='IFC4X3_ADD2');units={}
 for row in vectors['lengths']:
  if row['kind']=='si':unit=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Prefix=row['prefix'],Name='METRE')
  else:
   dims=f.create_entity('IfcDimensionalExponents',1,0,0,0,0,0,0);measure=f.create_entity('IfcMeasureWithUnit',ValueComponent=f.create_entity('IfcLengthMeasure',row['factor']),UnitComponent=units[row['base']]);unit=f.create_entity('IfcConversionBasedUnit',Dimensions=dims,UnitType='LENGTHUNIT',Name=row['id'],ConversionFactor=measure)
  units[row['id']]=unit
 assignment=f.create_entity('IfcUnitAssignment',Units=(units[target['id']],));f.create_entity('IfcProject',GlobalId='0000000000000000000001',UnitsInContext=assignment);actual=ifcopenshell.util.unit.calculate_unit_scale(f,'LENGTHUNIT');assert math.isclose(actual,target['scale'],rel_tol=1e-14);f.write(str(out/(target['id']+'.ifc')));rows.append({'id':target['id'],'unit':units[target['id']].id(),'actual':actual,'expected':target['scale'],'match':True})
(out/'oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell.util.unit.calculate_unit_scale','version':ifcopenshell.version,'rows':rows,'derivedDirectOracleAvailable':False,'schemaConformance':'not-evaluated','nativeExecution':False},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'rows':len(rows),'matches':sum(r['match']for r in rows)}))
