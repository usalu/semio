"""🧮️ Observe actual released finite LIST cardinality and direction rules independently."""
from pathlib import Path
import ctypes,json,math,struct,sys

ticket=Path(__file__).parent
dependency=str(ticket/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency,buffer,len(buffer))
    dependency=buffer.value or dependency
sys.path.insert(0,dependency)

class ListOracle:
    """🧭️ Physical observations stay behind the repository-owned oracle boundary."""
    def observe(self,row):
        raise NotImplementedError

class IfcOpenShellList(ListOracle):
    """🧊️ The external validator supplies separate cardinality, WHERE and binary64 evidence."""
    def __init__(self):
        import ifcopenshell
        import ifcopenshell.validate
        self.library=ifcopenshell
        self.validation=ifcopenshell.validate
        self.version=ifcopenshell.version
    def observe(self,row):
        values=[]
        for word in row['words']:
            if not word or not word.isascii() or not word.isdecimal() or int(word)>18446744073709551615:
                return {'id':row['id'],'physical':False,'reason':'not a complete binary64 word'}
            value=struct.unpack('>d',struct.pack('>Q',int(word)))[0]
            if not math.isfinite(value):
                return {'id':row['id'],'physical':False,'reason':'nonfinite binary64'}
            values.append(value)
        literal=','.join(repr(value).replace('e','E') for value in values)
        header="ISO-10303-21;HEADER;FILE_DESCRIPTION(('Audit'),'2;1');FILE_NAME('list.ifc','2026-10-21T00:00:00',('Audit'),('Audit'),'Audit','Audit','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;"
        model=self.library.file.from_string(header+'#1='+row['entity']+'(('+literal+'));ENDSEC;END-ISO-10303-21;')
        entity=model.by_id(1)
        logger=self.validation.json_logger()
        self.validation.validate(model,logger,express_rules=True)
        words=[str(struct.unpack('>Q',struct.pack('>d',value))[0])for value in entity[0]]
        diagnostics=[{'attribute':str(item.get('attribute','')),'message':str(item.get('message',''))}for item in logger.statements]
        return {'id':row['id'],'physical':True,'words':words,'expectedWords':row['words'],'exactWords':words==row['words'],'admitted':not diagnostics,'expectedAdmitted':row['admitted'],'diagnostics':diagnostics,'stepEntity':str(entity)}

root=ticket.parents[6]
law=json.loads((root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-list-input.json').read_text(encoding='utf-8'))
oracle=IfcOpenShellList()
rows=[oracle.observe(row)for row in law['cases']]
output={'oracle':'IfcOpenShell','version':oracle.version,'schema':law['schema'],'nativeExecuted':False,'rows':rows}
(ticket/'🗑️generated/type-list-input-oracle.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'physical':sum(row['physical']for row in rows),'draftRefusals':sum(not row['physical']for row in rows),'matched':sum(row.get('admitted')==row.get('expectedAdmitted')and row.get('exactWords',False)for row in rows if row['physical'])}))
assert all(row['admitted']==row['expectedAdmitted']and row['exactWords']for row in rows if row['physical'])
