import pathlib,json,struct,math
import ifcopenshell,ifcopenshell.ifcopenshell_wrapper as w
p=pathlib.Path(__file__).resolve().parent
schema=w.schema_by_name('IFC4X3_ADD2');names=['IfcRightCircularCylinder','IfcCsgSolid','IfcAxis2Placement3D','IfcCartesianPoint','IfcDirection']
metadata={name:[{'ownerSlot':i,'name':a.name(),'optional':a.optional(),'type':str(a.type_of_attribute())}for i,a in enumerate(schema.declaration_by_name(name).all_attributes())]for name in names}
words=['9218868437227405312','18442240474082181120','9221120237041090560'];numeric=[]
for word in words:
 value=struct.unpack('>d',int(word).to_bytes(8,'big'))[0];f=ifcopenshell.file(schema='IFC4X3_ADD2')
 try:
  item=f.create_entity('IfcRightCircularCylinder',Height=3.,Radius=value);text=f.to_string();numeric.append({'word':word,'finite':math.isfinite(value),'constructor':'accepted','step':text.split('DATA;')[1].split('ENDSEC;')[0].strip(),'physicalClaim':False})
 except Exception as e:numeric.append({'word':word,'finite':math.isfinite(value),'constructor':'refused','error':str(e),'physicalClaim':False})
result={'library':ifcopenshell.version,'metadata':metadata,'nonfinite':numeric,'collapse':{'translation':1e20,'localDistinct':[0,1,3],'translated':[1e20+x for x in [0,1,3]],'allEqual':len(set(1e20+x for x in [0,1,3]))==1,'schemaInvalidity':False}}
out=p/'🗑️generated/csg-round-independent/supplemental-facts.json';out.write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] metadata5/nonfinite3/collapse captured')
