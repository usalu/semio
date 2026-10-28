"""🔗 Independent reference expectations use IfcOpenShell's own declaration and subtype metadata."""
import ctypes
import json
from pathlib import Path
import re
import sys
from typing import Protocol

ticket=Path(__file__).resolve().parent
module_path=str(ticket/'🗑️generated'/'oracle')
if sys.platform=='win32':
    short=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path,short,len(short))
    module_path=short.value or module_path
sys.path.insert(0,module_path)
import ifcopenshell
import ifcopenshell.validate

root=next(parent for parent in ticket.parents if (parent/'package.json').exists())
fixture=json.loads((root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/reference-reduction.json').read_text('utf8'))
schema=ifcopenshell.ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
class ReferenceOracle(Protocol):
    def admitted(self,row:dict)->bool: ...

def named(domain):
    wrapper=domain.as_named_type()
    return wrapper.declared_type() if wrapper else None
def resolve(domain):
    declaration=named(domain)
    if declaration and declaration.as_type_declaration():
        return resolve(declaration.declared_type())
    return domain
def subtype(actual,expected):
    declaration=schema.declaration_by_name(actual).as_entity()
    while declaration:
        if declaration.name().upper()==expected.upper():return True
        declaration=declaration.supertype()
    return False
def member(declaration,actual=None):
    if declaration.as_entity():return actual is None or subtype(actual,declaration.name())
    select=declaration.as_select_type()
    if select:return any(member(child,actual) for child in select.select_list())
    alias=declaration.as_type_declaration()
    return bool(alias and admits(alias.declared_type(),actual))
def admits(domain,actual=None):
    declaration=named(domain)
    return bool(declaration and member(declaration,actual))
def constructor(domain,name):
    declaration=named(domain)
    if not declaration:return False
    def walk(declaration):
        if declaration.name().upper()==name.upper():return not bool(declaration.as_entity())
        select=declaration.as_select_type()
        return bool(select and any(walk(child) for child in select.select_list()))
    return walk(declaration)

class IfcDeclarationOracle:
    def admitted(self,row):
        try:return self.evaluate(row)
        except (AssertionError,ValueError,KeyError,IndexError,AttributeError,TypeError):return False
    def evaluate(self,row):
        fields=row['path'].split('/')[1:];assert fields.pop(0)=='entities'
        source=fixture['snapshot']['entities'][int(fields.pop(0))]
        part=None
        if fields[0]=='complex':fields.pop(0);part=source['complex'][int(fields.pop(0))]
        assert fields.pop(0)=='args';index=int(fields.pop(0));declaration=schema.declaration_by_name((part or source)['name'])
        attributes=declaration.attributes() if part else declaration.all_attributes();attribute=attributes[index]
        assert not declaration.derived()[next(index for index,other in enumerate(declaration.all_attributes()) if other.name()==attribute.name())]
        domain=attribute.type_of_attribute();current=(part or source)['args'][index];optional=attribute.optional();unique=None;selected=None
        while fields and fields[0]=='value' and len(fields)>1:
            fields.pop(0)
            if current['kind']=='typedValue':
                typed=current['value'];assert fields.pop(0)=='items' and len(typed['items'])==1
                assert constructor(domain,typed['name']);actual=schema.declaration_by_name(typed['name']);assert not actual.as_entity()
                index=int(fields.pop(0));current=typed['items'][index];domain=actual.declared_type();optional=False;unique=None;selected=None
            else:
                assert current['kind']=='aggregate';aggregate=resolve(domain).as_aggregation_type();assert aggregate
                index=int(fields.pop(0));unique=current['value'] if aggregate.type_of_aggregation_string().upper()=='SET' else None;selected=index
                current=current['value'][index];domain=aggregate.type_of_element();optional=False
        kind='whole'
        if fields==['appendReference']:
            assert current['kind']=='aggregate';aggregate=resolve(domain).as_aggregation_type();assert aggregate and aggregate.type_of_aggregation_string().upper()!='ARRAY'
            assert aggregate.bound2()<0 or len(current['value'])<aggregate.bound2()
            unique=current['value'] if aggregate.type_of_aggregation_string().upper()=='SET' else None;selected=None;domain=aggregate.type_of_element();optional=False;kind='append'
        elif fields==['reference']:kind='virtual'
        elif fields==['value']:assert current['kind']=='reference';kind='payload'
        else:assert fields==[]
        assert admits(domain)
        incoming=row['value']
        if kind=='whole':
            assert isinstance(incoming,dict)
            if incoming=={'kind':'unset'}:identity=None
            else:assert set(incoming)=={'kind','value'} and incoming['kind']=='reference' and type(incoming['value']) is int;identity=incoming['value']
        else:
            assert isinstance(incoming,str)
            if incoming=='':assert kind=='virtual';identity=None
            else:assert re.fullmatch(r'[1-9][0-9]{0,19}',incoming);identity=int(incoming)
        if identity is None:return optional
        assert 0<identity<2**64
        if unique is not None:
            for index,other in enumerate(unique):
                if index!=selected and other=={'kind':'reference','value':identity} and (kind=='append' or current!={'kind':'reference','value':identity}):return False
        targets=[target for target in fixture['snapshot']['entities'] if target['id']==identity]
        return len(targets)==1 and any(admits(domain,name) for name in [targets[0]['name']]+[part['name'] for part in targets[0].get('complex',[])])

oracle:ReferenceOracle=IfcDeclarationOracle()
for row in fixture['cases']:
    admitted=oracle.admitted(row)
    if admitted!=row['admitted']:
        oracle.evaluate(row)
        raise AssertionError(row['name'])
model=ifcopenshell.file(schema='IFC4X3_ADD2')
length=model.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Name='METRE')
area=model.create_entity('IfcSIUnit',UnitType='AREAUNIT',Name='SQUARE_METRE')
time=model.create_entity('IfcSIUnit',UnitType='TIMEUNIT',Name='SECOND')
assignment=model.create_entity('IfcUnitAssignment',Units=(length,area))
assignment.Units=(time,area)
logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True);assert not logger.statements,logger.statements
assignment.Units=(area,area)
logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True);assert logger.statements
print(json.dumps({'oracle':'IfcOpenShell '+ifcopenshell.version,'neutralCases':len(fixture['cases']),'officialDeclarationAndSubtypeChecks':True,'actualUniqueUnitReplacement':True,'actualDuplicateSetExpressRefusal':True,'exactU64':'Python decimal/domain only; external entity IDs not fabricated','physicalComplexMapping':'not evaluated','nativeExecution':False}))
