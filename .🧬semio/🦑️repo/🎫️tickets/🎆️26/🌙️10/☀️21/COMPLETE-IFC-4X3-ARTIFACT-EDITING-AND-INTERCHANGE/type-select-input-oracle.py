"""🧮️ Observe released SELECT constructors through a separate physical implementation."""
from pathlib import Path
import ctypes,json,math,struct,sys

ticket=Path(__file__).parent
dependency=str(ticket/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency,buffer,len(buffer))
    dependency=buffer.value or dependency
sys.path.insert(0,dependency)

class SelectOracle:
    """🧭️ External library types remain inside this owned observation interface."""
    def observe(self,row):
        raise NotImplementedError

class IfcOpenShellSelect(SelectOracle):
    """🧊️ Actual typed names, primitive payloads and EXPRESS decisions are observed separately."""
    def __init__(self):
        import ifcopenshell
        import ifcopenshell.validate
        self.library=ifcopenshell
        self.validation=ifcopenshell.validate
        self.version=ifcopenshell.version
    def observe(self,row):
        raw=row['raw']
        if row['kind']=='real':
            if not raw or not raw.isascii() or not raw.isdecimal() or int(raw)>18446744073709551615:
                return {'id':row['id'],'physical':False,'reason':'incomplete word'}
            scalar=struct.unpack('>d',struct.pack('>Q',int(raw)))[0]
            if not math.isfinite(scalar):
                return {'id':row['id'],'physical':False,'reason':'nonfinite word'}
            literal=repr(scalar).replace('e','E')
        else:
            literal="'"+raw.replace("'","''")+"'"
        value=row['member']+'('+literal+')'
        body='#1=IFCLAGTIME(\'original\',.PREDICTED.,$,'+value+',.WORKTIME.);' if row['entity']=='IFCLAGTIME' else '#1=IFCPOINTBYDISTANCEEXPRESSION('+value+',$,$,$,#7);#7=IFCPOLYLINE((#9,#10));#9=IFCCARTESIANPOINT((0.,0.));#10=IFCCARTESIANPOINT((1.,0.));'
        header="ISO-10303-21;HEADER;FILE_DESCRIPTION(('Audit'),'2;1');FILE_NAME('select.ifc','2026-10-21T00:00:00',('Audit'),('Audit'),'Audit','Audit','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;"
        model=self.library.file.from_string(header+body+'ENDSEC;END-ISO-10303-21;')
        entity=model.by_id(1)
        payload=entity[3 if row['entity']=='IFCLAGTIME' else 0]
        actual=str(struct.unpack('>Q',struct.pack('>d',payload.wrappedValue))[0]) if row['kind']=='real' else payload.wrappedValue
        logger=self.validation.json_logger()
        self.validation.validate(model,logger,express_rules=True)
        diagnostics=[{'attribute':str(item.get('attribute','')),'message':str(item.get('message',''))}for item in logger.statements]
        return {'id':row['id'],'physical':True,'member':payload.is_a().upper(),'expectedMember':row['member'],'raw':actual,'expectedRaw':raw,'exact':actual==raw and payload.is_a().upper()==row['member'],'admitted':not diagnostics,'expectedAdmitted':row['admitted'],'diagnostics':diagnostics,'stepEntity':str(entity)}

root=ticket.parents[6]
law=json.loads((root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/type-select-input.json').read_text(encoding='utf-8'))
oracle=IfcOpenShellSelect()
rows=[oracle.observe(row)for row in law['cases']]
(ticket/'🗑️generated/type-select-input-oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell','version':oracle.version,'schema':law['schema'],'nativeExecuted':False,'rows':rows},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps({'physical':sum(row['physical']for row in rows),'drafts':sum(not row['physical']for row in rows),'matched':sum(row.get('exact',False)and row.get('admitted')==row.get('expectedAdmitted')for row in rows)}))
assert all(row['exact']and row['admitted']==row['expectedAdmitted']for row in rows if row['physical'])
