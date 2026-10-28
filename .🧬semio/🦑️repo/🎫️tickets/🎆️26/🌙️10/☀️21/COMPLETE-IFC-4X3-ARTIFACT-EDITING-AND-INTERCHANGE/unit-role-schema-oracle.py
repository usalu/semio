"""⚖️ Independent original derived/named unit schema validation observations."""
import ctypes,json,sys
from pathlib import Path
root=Path(__file__).parent;dep=str(root/'🗑️generated/oracle')
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,short,len(short));dep=short.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.validate
out=root/'🗑️generated/unit-traversal';out.mkdir(exist_ok=True);rows=[]
for name in ['density','modulus','one-element-exponent-one','userdefined-missing','duplicate-length-assignment','wrong-named-dimensions']:
 f=ifcopenshell.file(schema='IFC4X3_ADD2');metre=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Name='METRE');mm=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Prefix='MILLI',Name='METRE');kg=f.create_entity('IfcSIUnit',UnitType='MASSUNIT',Prefix='KILO',Name='GRAM');second=f.create_entity('IfcSIUnit',UnitType='TIMEUNIT',Name='SECOND')
 if name=='duplicate-length-assignment':subject=f.create_entity('IfcUnitAssignment',Units=(metre,mm))
 elif name=='wrong-named-dimensions':subject=f.create_entity('IfcContextDependentUnit',Dimensions=f.create_entity('IfcDimensionalExponents',0,0,0,0,0,0,0),UnitType='LENGTHUNIT',Name='wrong')
 else:
  values=[(kg,1),(mm,-3)]if name=='density'else[(kg,1),(metre,-1),(second,-2)]
  if name=='one-element-exponent-one':values=[(metre,1)]
  elements=[f.create_entity('IfcDerivedUnitElement',Unit=unit,Exponent=exponent)for unit,exponent in values];subject=f.create_entity('IfcDerivedUnit',Elements=tuple(elements),UnitType='MASSDENSITYUNIT'if name=='density'else'USERDEFINED'if name=='userdefined-missing'else'MODULUSOFELASTICITYUNIT')
 logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True);diagnostics=[{'level':r.get('level'),'attribute':r.get('attribute'),'message':str(r.get('message'))}for r in logger.statements];f.write(str(out/('schema-'+name+'.ifc')));rows.append({'id':name,'subject':subject.id(),'diagnostics':diagnostics,'expectedValid':name in ['density','modulus'],'observedValid':not diagnostics});assert(not diagnostics)==(name in ['density','modulus'])
(out/'schema-oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell.validate express_rules=True','version':ifcopenshell.version,'rows':rows,'nativeExecution':False},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'cases':len(rows),'matched':len(rows),'valid':sum(r['observedValid']for r in rows)}))
