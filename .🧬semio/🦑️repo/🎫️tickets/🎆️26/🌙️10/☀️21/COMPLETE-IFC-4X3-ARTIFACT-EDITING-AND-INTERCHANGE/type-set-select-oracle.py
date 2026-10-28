"""🧮️ Observe actual typed SET constructors, identity uniqueness and entity membership independently."""
from pathlib import Path
import contextlib,ctypes,io,json,struct,sys
ticket=Path(__file__).parent
dependency=str(ticket/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency,buffer,len(buffer))
    dependency=buffer.value or dependency
sys.path.insert(0,dependency)

class SetSelectOracle:
    """🧭️ External schema/model objects remain behind an owned physical observation interface."""
    def observe(self,row):
        raise NotImplementedError

class IfcOpenShellSet(SetSelectOracle):
    """🧊️ The external released schema validates real SET identity and actual source subtypes."""
    def __init__(self,law):
        import ifcopenshell
        import ifcopenshell.validate
        self.library=ifcopenshell
        self.validation=ifcopenshell.validate
        self.version=ifcopenshell.version
        self.law=law
        schema=ifcopenshell.ifcopenshell_wrapper.schema_by_name(law['schema'])
        self.declarations={declaration.name().upper():declaration for declaration in schema.declarations()}
    def literal(self,value):
        kind=value['kind']
        if kind=='unset':
            return '$'
        if kind=='string':
            return "'"+value['value'].replace("'","''")+"'"
        if kind=='reference':
            return '#'+value['value']
        if kind=='real':
            return repr(struct.unpack('>d',struct.pack('>Q',int(value['value']['bits'])))[0]).replace('e','E')
        if kind=='aggregate':
            return '('+','.join(self.literal(item)for item in value['value'])+')'
        if kind=='typedValue':
            return value['value']['name']+'('+','.join(self.literal(item)for item in value['value']['items'])+')'
        raise AssertionError('closed original source')
    def flat(self,entity):
        if not entity['complex']:
            return entity['args']
        parts={part['name']:part['args']for part in entity['complex']}
        parts[entity['name']]=entity['args']
        declaration=self.declarations[entity['name']]
        chain=[]
        while declaration:
            chain.append(declaration.name().upper())
            declaration=declaration.supertype()
        return [value for name in reversed(chain)for value in parts[name]]
    def observe(self,row):
        if not row['physical']:
            return {'id':row['id'],'physical':False,'reason':'explicit incomplete or unresolved carrier'}
        source=json.loads(json.dumps(self.law['source']))
        source['entities'][0]['args'][int(self.law['index'])]=row['input']
        body=''.join('#'+entity['id']+'='+entity['name']+'('+','.join(self.literal(value)for value in self.flat(entity))+');'for entity in source['entities'])
        header="ISO-10303-21;HEADER;FILE_DESCRIPTION(('Audit'),'2;1');FILE_NAME('sets.ifc','2026-10-21T00:00:00',('Audit'),('Audit'),'Audit','Audit','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;"
        output=io.StringIO()
        with contextlib.redirect_stdout(output),contextlib.redirect_stderr(output):
            model=self.library.file.from_string(header+body+'ENDSEC;END-ISO-10303-21;')
            selected=getattr(model.by_id(1),self.law['attribute'])
            logger=self.validation.json_logger()
            self.validation.validate(model,logger,express_rules=True)
        aggregate=row['input']['kind']=='typedValue'
        actual=[str(value.id())for value in selected.wrappedValue]if aggregate else [str(selected.id())]
        expected=[value['value']for value in row['input']['value']['items'][0]['value']]if aggregate else [row['input']['value']]
        diagnostics=[{'attribute':str(item.get('attribute','')),'message':str(item.get('message',''))}for item in logger.statements]
        return {'id':row['id'],'physical':True,'carrier':row['input']['kind'],'member':selected.is_a().upper(),'references':actual,'expectedReferences':expected,'exact':actual==expected,'admitted':not diagnostics,'expectedAdmitted':row['ready'],'diagnostics':diagnostics,'libraryOutput':output.getvalue(),'stepEntity':str(model.by_id(1))}

law=json.loads((ticket.parents[6]/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-set-select-input.json').read_text(encoding='utf-8'))
oracle=IfcOpenShellSet(law)
rows=[oracle.observe(row)for row in law['cases']]
(ticket/'🗑️generated/type-set-select-oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell','version':oracle.version,'schema':law['schema'],'nativeExecuted':False,'rows':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'physical':sum(row['physical']for row in rows),'drafts':sum(not row['physical']for row in rows),'matched':sum(row.get('exact',False)and row.get('admitted')==row.get('expectedAdmitted')for row in rows)}))
assert all(row['exact']and row['admitted']==row['expectedAdmitted']for row in rows if row['physical'])
