"""🎨 Independent IfcOpenShell values and EXPRESS enumeration membership evidence."""
import ctypes
import json
from pathlib import Path
import struct
import sys
from typing import Protocol

ticket=Path(__file__).resolve().parent
modules=str(ticket/'🗑️generated'/'oracle')
if sys.platform=='win32':
    short=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(modules,short,len(short))
    modules=short.value or modules
sys.path.insert(0,modules)
import ifcopenshell
import ifcopenshell.validate
import numpy as np
root=next(parent for parent in ticket.parents if (parent/'package.json').exists())
cases=json.loads((root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/enumeration.json').read_text('utf8'))
class EnumerationOracle(Protocol):
    def value(self,file,row): ...
class IfcOpenShellOracle:
    def value(self,file,row):
        assert row['kind']=='typedValue'
        item=row['value']['items'][0]
        if item['kind']=='integer':value=int(item['value'])
        elif item['kind']=='real':value=struct.unpack('>d',int(item['value']['bits']).to_bytes(8,'big'))[0]
        else:value=item['value']
        return file.create_entity(row['value']['name'],value)
    def native(self,value):
        name=value.is_a().upper()
        raw=value.wrappedValue
        if isinstance(raw,str):item={'kind':'string','value':raw}
        elif isinstance(raw,int):item={'kind':'integer','value':str(raw)}
        else:item={'kind':'real','value':{'bits':str(int.from_bytes(struct.pack('>d',raw),'big'))}}
        return {'kind':'typedValue','value':{'name':name,'items':[item]}}
oracle=IfcOpenShellOracle()
roundtrips=0
for row in cases['cases']:
    if not row['available']:continue
    file=ifcopenshell.file(schema='IFC4X3_ADD2')
    allowed=tuple(oracle.value(file,value) for value in row['allowed'])
    definition=file.create_entity('IfcPropertyEnumeration',Name='Palette',EnumerationValues=allowed)
    selected=None if row['selected'] is None else tuple(allowed[index] for index in row['selected'])
    property=file.create_entity('IfcPropertyEnumeratedValue',Name='Colour',EnumerationValues=selected,EnumerationReference=definition)
    assert [oracle.native(value)for value in property.EnumerationReference.EnumerationValues]==row['allowed'],row['id']
    if selected:assert all(oracle.native(value)in row['allowed']for value in property.EnumerationValues)
    roundtrips+=1
file=ifcopenshell.file(schema='IFC4X3_ADD2')
red=file.create_entity('IfcLabel','Red')
blue=file.create_entity('IfcLabel','Blue')
definition=file.create_entity('IfcPropertyEnumeration',Name='Palette',EnumerationValues=(red,blue))
property=file.create_entity('IfcPropertyEnumeratedValue',Name='Colour',EnumerationValues=(red,),EnumerationReference=definition)
valid=ifcopenshell.validate.json_logger()
ifcopenshell.validate.validate(file,valid,express_rules=True)
assert not valid.statements,valid.statements
property.EnumerationValues=(file.create_entity('IfcLabel','Green'),)
invalid=ifcopenshell.validate.json_logger()
ifcopenshell.validate.validate(file,invalid,express_rules=True)
assert any('WR21'in str(row)for row in invalid.statements),invalid.statements
index=json.loads((root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📥️source/🗂️index/cases.json').read_text('utf8'))
ids=np.array([int(value)for value in index['ids']],dtype=np.uint64)
assert np.argsort(ids,kind='stable').tolist()==index['ordinals']
for row in index['lookups']:
    positions=np.flatnonzero(ids==np.uint64(int(row['id']))).tolist()
    assert ('missing'if not positions else'unique'if len(positions)==1 else'ambiguous')==row['status']
    if len(positions)==1:assert positions[0]==row['ordinal']
print(json.dumps({'oracle':'IfcOpenShell '+ifcopenshell.version,'typedAllowedRoundtrips':roundtrips,'exactI64AndBinary64Words':True,'actualMemberExpressValid':True,'actualNonMemberWR21Refusal':True,'sourceIndexOracle':'NumPy '+np.__version__,'sourceIndexRows':len(ids),'sourceIndexExactU64':True,'mixedDefinitionFullValidity':'not asserted','emptySelectedListFullValidity':'not asserted','nativeExecution':False,'uiInteraction':False}))
