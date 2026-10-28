"""🧮️ Observe actual entity SELECT membership through a physical independent implementation."""
from pathlib import Path
import contextlib,ctypes,io,json,struct,sys

ticket=Path(__file__).parent
dependency=str(ticket/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency,buffer,len(buffer))
    dependency=buffer.value or dependency
sys.path.insert(0,dependency)

class ReferenceSelectOracle:
    """🧭️ Library schema and model types stay behind the owned physical observation interface."""
    def observe(self,row):
        raise NotImplementedError

class IfcOpenShellReferences(ReferenceSelectOracle):
    """🧊️ The released external schema supplies inherited positions and entity membership validation."""
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
        if kind=='derived':
            return '*'
        if kind=='string':
            return "'"+value['value'].replace("'","''")+"'"
        if kind=='enum':
            return '.'+value['value']+'.'
        if kind=='reference':
            return '#'+value['value']
        if kind=='real':
            return repr(struct.unpack('>d',struct.pack('>Q',int(value['value']['bits'])))[0]).replace('e','E')
        if kind=='aggregate':
            return '('+','.join(self.literal(item)for item in value['value'])+')'
        if kind=='typedValue':
            return value['value']['name']+'('+','.join(self.literal(item)for item in value['value']['items'])+')'
        raise AssertionError('closed source fixture')
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
        raw=row['reference']
        if not raw or not raw.isascii() or not raw.isdecimal() or int(raw)==0 or int(raw)>18446744073709551615:
            return {'id':row['id'],'physical':False,'reason':'incomplete or out-of-range reference'}
        source=json.loads(json.dumps(self.law['sources'][row['source']]))
        domain=next(domain for domain in self.law['domains']if domain['source']==row['source'])
        source['entities'][0]['args'][int(domain['index'])]={'kind':'reference','value':raw}
        body=''.join('#'+entity['id']+'='+entity['name']+'('+','.join(self.literal(value)for value in self.flat(entity))+');'for entity in source['entities'])
        header="ISO-10303-21;HEADER;FILE_DESCRIPTION(('Audit'),'2;1');FILE_NAME('references.ifc','2026-10-21T00:00:00',('Audit'),('Audit'),'Audit','Audit','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;"
        output=io.StringIO()
        with contextlib.redirect_stdout(output),contextlib.redirect_stderr(output):
            model=self.library.file.from_string(header+body+'ENDSEC;END-ISO-10303-21;')
            selected=getattr(model.by_id(1),domain['attribute'])
            logger=self.validation.json_logger()
            self.validation.validate(model,logger,express_rules=True)
        diagnostics=[{'attribute':str(item.get('attribute','')),'message':str(item.get('message',''))}for item in logger.statements]
        return {'id':row['id'],'physical':True,'reference':str(selected.id())if selected else None,'member':selected.is_a().upper()if selected else None,'expectedReference':raw,'admitted':not diagnostics,'expectedAdmitted':row['admitted'],'diagnostics':diagnostics,'libraryOutput':output.getvalue(),'stepEntity':str(model.by_id(1))}

root=ticket.parents[6]
law=json.loads((root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-reference-select-input.json').read_text(encoding='utf-8'))
oracle=IfcOpenShellReferences(law)
rows=[oracle.observe(row)for row in law['cases']]
(ticket/'🗑️generated/type-reference-select-oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell','version':oracle.version,'schema':law['schema'],'nativeExecuted':False,'rows':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'physical':sum(row['physical']for row in rows),'drafts':sum(not row['physical']for row in rows),'matched':sum(row['admitted']==row['expectedAdmitted']for row in rows if row['physical'])}))
assert all(row['admitted']==row['expectedAdmitted']and(not row['admitted']or row['reference']==row['expectedReference'])for row in rows if row['physical'])
