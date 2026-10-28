"""🧮️ Separate exact MeasureValue carriers from actual coordinate candidate WHERE decisions."""
from pathlib import Path
import contextlib,ctypes,io,json,struct,sys
ticket=Path(__file__).parent
dependency=str(ticket/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency,buffer,len(buffer))
    dependency=buffer.value or dependency
sys.path.insert(0,dependency)

class MeasureOracle:
    """🧭️ The public observation result contains only owned closed dictionaries."""
    def observe(self,row):
        raise NotImplementedError

class IfcOpenShellMeasure(MeasureOracle):
    """🧊️ Independent physical parsing and full released rules preserve their distinct scopes."""
    def __init__(self):
        import ifcopenshell
        import ifcopenshell.validate
        self.library=ifcopenshell
        self.validation=ifcopenshell.validate
        self.version=ifcopenshell.version
        aggregate=ifcopenshell.ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2').declaration_by_name('IfcComplexNumber').as_type_declaration().declared_type().as_aggregation_type()
        self.array={'kind':aggregate.type_of_aggregation_string(),'lower':aggregate.bound1(),'upper':aggregate.bound2()}
    def literal(self,value):
        kind=value['kind']
        if kind=='string':
            return "'"+''.join("''"if char=="'"else'\\\\'if char=='\\'else'\\X2\\'+char.encode('utf-16-be').hex().upper()+'\\X0\\'if ord(char)>126 else char for char in value['value'])+"'"
        if kind=='integer':
            return value['value']
        if kind=='real':
            return repr(struct.unpack('>d',struct.pack('>Q',int(value['value']['bits'])))[0]).replace('e','E')
        if kind=='aggregate':
            return '('+','.join(self.literal(item)for item in value['value'])+')'
        raise AssertionError('closed measure input')
    def encode(self,value):
        if isinstance(value,int):
            return {'kind':'integer','value':str(value)}
        if isinstance(value,float):
            return {'kind':'real','value':{'bits':str(struct.unpack('>Q',struct.pack('>d',value))[0])}}
        if isinstance(value,str):
            return {'kind':'string','value':value}
        if isinstance(value,tuple):
            return {'kind':'aggregate','value':[self.encode(item)for item in value]}
        raise AssertionError('closed independent carrier')
    def validate(self,body,attribute):
        header="ISO-10303-21;HEADER;FILE_DESCRIPTION(('Audit'),'2;1');FILE_NAME('measures.ifc','2026-10-22T00:00:00',('Audit'),('Audit'),'Audit','Audit','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;"
        output=io.StringIO()
        with contextlib.redirect_stdout(output),contextlib.redirect_stderr(output):
            model=self.library.file.from_string(header+body+'ENDSEC;END-ISO-10303-21;')
            selected=getattr(model.by_id(1),attribute)
            logger=self.validation.json_logger()
            self.validation.validate(model,logger,express_rules=True)
        diagnostics=[{'attribute':str(item.get('attribute','')),'message':str(item.get('message',''))}for item in logger.statements]
        return {'member':selected.is_a().upper(),'value':self.encode(selected.wrappedValue),'admitted':not diagnostics,'diagnostics':diagnostics,'libraryOutput':output.getvalue(),'entity':str(model.by_id(1))}
    def observe(self,row):
        if not row['physical']:
            return {'id':row['id'],'physical':False,'reason':'explicit incomplete original draft'}
        selected=row['member']+'('+','.join(self.literal(item)for item in row['items'])+')'
        coordinate=self.validate('#1=IFCRIGIDOPERATION(#7,#8,'+selected+','+selected+',$);#7=IFCPROJECTEDCRS(\'EPSG:2056\',$,$,$,$,$,$);#8=IFCPROJECTEDCRS(\'EPSG:2056\',$,$,$,$,$,$);','FirstCoordinate')
        carrier=self.validate('#1=IFCMEASUREWITHUNIT('+selected+',#21);#21=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);','ValueComponent')
        exact=coordinate['member']==row['member']and carrier['member']==row['member']and coordinate['value']==row['items'][0]and carrier['value']==row['items'][0]
        return {'id':row['id'],'physical':True,'exact':exact,'coordinate':coordinate,'carrier':carrier,'expectedDomain':row['domain'],'expectedReady':row['ready']}

law=json.loads((ticket.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-measure-select-input.json').read_text(encoding='utf-8'))
oracle=IfcOpenShellMeasure()
rows=[oracle.observe(row)for row in law['cases']]
(ticket/'🗑️generated/type-measure-select-oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell','version':oracle.version,'schema':law['schema'],'arrayMetadata':oracle.array,'nativeExecuted':False,'rows':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'physical':sum(row['physical']for row in rows),'drafts':sum(not row['physical']for row in rows),'exact':sum(row.get('exact',False)for row in rows),'matched':sum(row.get('exact',False)and row['coordinate']['admitted']==row['expectedReady']and row['carrier']['admitted']==row['expectedDomain']for row in rows if row['physical'])}))
assert all(row['exact']and row['coordinate']['admitted']==row['expectedReady']and row['carrier']['admitted']==row['expectedDomain']for row in rows if row['physical'])
