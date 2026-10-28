"""🧮️ Observe mixed SELECT wire identity and released positive alias rules independently."""
from pathlib import Path
import contextlib,ctypes,io,json,struct,sys
ticket=Path(__file__).parent
dependency=str(ticket/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency,buffer,len(buffer))
    dependency=buffer.value or dependency
sys.path.insert(0,dependency)

class MixedSelectOracle:
    """🧭️ External entity and validator types stay behind this owned observation contract."""
    def observe(self,row):
        raise NotImplementedError

class IfcOpenShellMixed(MixedSelectOracle):
    """🧊️ The physical library distinguishes ordinary references from defined scalar constructors."""
    def __init__(self,law):
        import ifcopenshell
        import ifcopenshell.validate
        self.library=ifcopenshell
        self.validation=ifcopenshell.validate
        self.version=ifcopenshell.version
        self.law=law
    def literal(self,value):
        kind=value['kind']
        if kind=='unset':
            return '$'
        if kind=='reference':
            return '#'+value['value']
        if kind=='real':
            return repr(struct.unpack('>d',struct.pack('>Q',int(value['value']['bits'])))[0]).replace('e','E')
        if kind=='aggregate':
            return '('+','.join(self.literal(item)for item in value['value'])+')'
        if kind=='typedValue':
            return value['value']['name']+'('+','.join(self.literal(item)for item in value['value']['items'])+')'
        raise AssertionError('closed neutral source')
    def observe(self,row):
        if not row['physical']:
            return {'id':row['id'],'physical':False,'reason':'explicit incomplete or unresolved carrier'}
        source=json.loads(json.dumps(self.law['source']))
        source['entities'][0]['args'][int(self.law['index'])]=row['input']
        body=''.join('#'+entity['id']+'='+entity['name']+'('+','.join(self.literal(value)for value in entity['args'])+');'for entity in source['entities'])
        header="ISO-10303-21;HEADER;FILE_DESCRIPTION(('Audit'),'2;1');FILE_NAME('mixed.ifc','2026-10-21T00:00:00',('Audit'),('Audit'),'Audit','Audit','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;"
        output=io.StringIO()
        with contextlib.redirect_stdout(output),contextlib.redirect_stderr(output):
            model=self.library.file.from_string(header+body+'ENDSEC;END-ISO-10303-21;')
            selected=getattr(model.by_id(1),self.law['attribute'])
            logger=self.validation.json_logger()
            self.validation.validate(model,logger,express_rules=True)
        diagnostics=[{'attribute':str(item.get('attribute','')),'message':str(item.get('message',''))}for item in logger.statements]
        reference=row['input']['kind']=='reference'
        raw=str(selected.id())if reference else str(struct.unpack('>Q',struct.pack('>d',selected.wrappedValue))[0])
        expected=row['input']['value']if reference else row['input']['value']['items'][0]['value']['bits']
        return {'id':row['id'],'physical':True,'carrier':row['input']['kind'],'member':selected.is_a().upper(),'raw':raw,'expectedRaw':expected,'exact':raw==expected,'admitted':not diagnostics,'expectedAdmitted':row['ready'],'diagnostics':diagnostics,'libraryOutput':output.getvalue(),'stepEntity':str(model.by_id(1))}

law=json.loads((ticket.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-mixed-select-input.json').read_text(encoding='utf-8'))
oracle=IfcOpenShellMixed(law)
rows=[oracle.observe(row)for row in law['cases']]
(ticket/'🗑️generated/type-mixed-select-oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell','version':oracle.version,'schema':law['schema'],'nativeExecuted':False,'rows':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'physical':sum(row['physical']for row in rows),'drafts':sum(not row['physical']for row in rows),'matched':sum(row.get('exact',False)and row.get('admitted')==row.get('expectedAdmitted')for row in rows)}))
assert all(row['exact']and row['admitted']==row['expectedAdmitted']for row in rows if row['physical'])
