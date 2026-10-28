"""🧮️ Observe released recursive IfcValue physical carriers behind an owned test-only interface."""
from pathlib import Path
import contextlib,ctypes,io,json,struct,sys
T=Path(__file__).parent
P=str(T/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768)
 ctypes.windll.kernel32.GetShortPathNameW(P,b,len(b))
 P=b.value or P
sys.path.insert(0,P)
class ValueOracle:
 """🧭️ Return owned closed observations without leaking dependency objects."""
 def observe(self,row):raise NotImplementedError
class IndependentIfcValue(ValueOracle):
 """🧊️ Existing external physical parser and full released rules retain separate evidence scopes."""
 def __init__(self):
  import ifcopenshell
  import ifcopenshell.validate
  self.library=ifcopenshell
  self.validation=ifcopenshell.validate
  self.version=ifcopenshell.version
  schema=ifcopenshell.ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
  declaration=schema.declaration_by_name('IfcCompoundPlaneAngleMeasure').as_type_declaration().declared_type().as_aggregation_type()
  self.list={'kind':declaration.type_of_aggregation_string(),'lower':declaration.bound1(),'upper':declaration.bound2()}
 def literal(self,value):
  kind=value['kind']
  if kind=='integer':return value['value']
  if kind=='real':return repr(struct.unpack('>d',struct.pack('>Q',int(value['value']['bits'])))[0]).replace('e','E')
  if kind=='enum':return '.'+value['value']+'.'
  if kind=='binary':return '"'+value['value']+'"'
  if kind=='string':return "'"+''.join("''"if char=="'"else'\\\\'if char=='\\'else'\\X2\\'+char.encode('utf-16-be').hex().upper()+'\\X0\\'if ord(char)>126 else char for char in value['value'])+"'"
  if kind=='aggregate':return '('+','.join(self.literal(item)for item in value['value'])+')'
  raise AssertionError('closed oracle input')
 def encode(self,value,name):
  if isinstance(value,bool):return {'kind':'enum','value':'T'if value else'F'}
  if value is None:return {'kind':'unknown'}
  if isinstance(value,int):return {'kind':'integer','value':str(value)}
  if isinstance(value,float):return {'kind':'real','value':{'bits':str(struct.unpack('>Q',struct.pack('>d',value))[0])}}
  if isinstance(value,tuple):return {'kind':'aggregate','value':[self.encode(item,name)for item in value]}
  if isinstance(value,str):
   if name in {'IFCBOOLEAN','IFCLOGICAL'}and value=='UNKNOWN':return {'kind':'enum','value':'U'}
   if name=='IFCBINARY':
    if value=='':return {'kind':'binary','value':'0'}
    if set(value)<=set('01'):
     unused=(-len(value))%4
     return {'kind':'binary','value':str(unused)+format(int(value,2),f'0{(len(value)+3)//4}X')}
   return {'kind':'string','value':value}
  raise AssertionError('unrecognized external value '+repr(value))
 def observe(self,row):
  if not row['physical']:return {'id':row['id'],'physical':False,'reason':'original incomplete draft; not serialized'}
  token=row['member']+'('+','.join(self.literal(item)for item in row['items'])+')'
  header="ISO-10303-21;HEADER;FILE_DESCRIPTION(('Audit'),'2;1');FILE_NAME('value.ifc','2026-10-22T00:00:00',('Audit'),('Audit'),'Audit','Audit','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;"
  output=io.StringIO()
  with contextlib.redirect_stdout(output),contextlib.redirect_stderr(output):
   model=self.library.file.from_string(header+'#1=IFCMEASUREWITHUNIT('+token+',#7);#7=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);ENDSEC;END-ISO-10303-21;')
   selected=model.by_id(1).ValueComponent
   logger=self.validation.json_logger()
   self.validation.validate(model,logger,express_rules=True)
  name=selected.is_a().upper()
  encoded=self.encode(selected.wrappedValue,name)
  diagnostics=[{'attribute':str(item.get('attribute','')),'message':str(item.get('message',''))}for item in logger.statements]
  return {'id':row['id'],'physical':True,'token':token,'member':name,'value':encoded,'pythonType':type(selected.wrappedValue).__name__,'pythonValue':repr(selected.wrappedValue),'exact':name==row['member']and encoded==row['items'][0],'admitted':not diagnostics,'expected':row['domain'],'diagnostics':diagnostics,'libraryOutput':output.getvalue()}
law=json.loads((T.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-value-select-input.json').read_text(encoding='utf-8'))
oracle=IndependentIfcValue()
rows=[]
for row in law['cases']:
 try:rows.append(oracle.observe(row))
 except Exception as error:rows.append({'id':row['id'],'physical':row['physical'],'failure':type(error).__name__,'message':str(error),'expected':row['domain']})
(T/'🗑️generated/type-value-select-oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell','version':oracle.version,'listMetadata':oracle.list,'nativeExecuted':False,'part21NumberIntegerCertified':False,'rows':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'cases':len(rows),'physical':sum(row['physical']for row in rows),'exact':sum(row.get('exact',False)for row in rows),'matched':sum(row.get('exact',False)and row['admitted']==row['expected']for row in rows if row.get('exact')is not None),'gaps':[row['id']for row in rows if row['physical']and(not row.get('exact')or row.get('admitted')!=row.get('expected'))],'listMetadata':oracle.list}))
