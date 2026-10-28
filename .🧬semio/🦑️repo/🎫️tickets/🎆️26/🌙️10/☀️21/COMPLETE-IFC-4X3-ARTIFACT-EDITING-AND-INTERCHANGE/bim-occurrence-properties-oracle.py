"""🏛️ Validate actual authored occurrence STEP inputs and unchanged official property evidence behind an owned interface."""
import hashlib
import json
import platform
import struct
from pathlib import Path
from typing import Protocol
import ifcopenshell
import ifcopenshell.validate
import ifcopenshell.util.element

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
class OccurrenceOracle(Protocol):
    def physical(self,row:dict)->dict: ...
def observed(value,kind):
    if kind=='Real':return dict(kind=kind,bits=struct.pack('>d',float(value)).hex())
    return dict(kind=kind,value=str(value) if kind=='Integer' else value)
class IfcOpenShellOccurrenceOracle:
    def physical(self,row):
        file=ifcopenshell.open(row['path']);holder=file.by_id(int(row['entity']))
        relations=[relation for relation in holder.IsDefinedBy if relation.is_a('IfcRelDefinesByProperties') and relation.RelatingPropertyDefinition.is_a('IfcPropertySet') and relation.RelatingPropertyDefinition.Name=='Authored']
        if len(relations)!=1:raise ValueError('Exactly one actual Authored relation required')
        definition=relations[0].RelatingPropertyDefinition
        if len(definition.HasProperties)!=1:raise ValueError('One actual nominal property required')
        property=definition.HasProperties[0];nominal=property.NominalValue
        value=observed(nominal.wrappedValue,row['expected']['kind'])
        expected={key:row['expected'][key] for key in ('kind','bits','value') if key in row['expected']}
        logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(file,logger,express_rules=True)
        values=ifcopenshell.util.element.get_psets(holder,psets_only=True,should_inherit=False)
        convenience=observed(values['Authored']['Value'],row['expected']['kind'])
        return dict(holder=row['holder'],id=row['id'],entity=holder.id(),actualHolder=holder.is_a(),schema=file.schema_identifier,sha256=hashlib.sha256(Path(row['path']).read_bytes()).hexdigest(),nominalType=nominal.is_a(),unit=property.Unit,value=value,expected=expected,agree=value==expected,occurrenceOnlyValue=convenience,occurrenceOnlyAgree=convenience==expected,propertyId=property.id(),setId=definition.id(),relationId=relations[0].id(),relatedIds=[entity.id() for entity in relations[0].RelatedObjects],fullExpressDiagnostics=logger.statements)
def main():
    rows=json.loads((ticket/'🗑️generated/bim-properties-runtime.json').read_text(encoding='utf-8'));oracle:OccurrenceOracle=IfcOpenShellOccurrenceOracle();physical=[oracle.physical(row) for row in rows]
    official=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/linear-placement-of-signal.ifc'
    file=ifcopenshell.open(official);properties=[]
    for relation in file.by_type('IfcRelDefinesByProperties'):
        definition=relation.RelatingPropertyDefinition
        if not definition.is_a('IfcPropertySet'):continue
        for property in definition.HasProperties:
            if not property.is_a('IfcPropertySingleValue'):continue
            nominal=property.NominalValue
            properties.append(dict(relation=relation.id(),holders=[dict(id=entity.id(),type=entity.is_a()) for entity in relation.RelatedObjects],set=dict(id=definition.id(),name=definition.Name),property=dict(id=property.id(),name=property.Name),nominalType=nominal.is_a() if nominal else None,rawNominal=nominal.wrappedValue if nominal else None,unit=property.Unit))
    logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(file,logger,express_rules=True)
    result=dict(python=platform.python_version(),ifcopenshell=ifcopenshell.version,physical=physical,official=dict(path=official.as_posix(),sha256=hashlib.sha256(official.read_bytes()).hexdigest(),schema=file.schema_identifier,scope='Unchanged genuine official input; property evidence is not new spatial-holder applicability',properties=properties,fullExpressDiagnostics=logger.statements))
    (ticket/'🗑️generated/bim-properties-oracle.json').write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8')
    print('[DEBUG] '+json.dumps(dict(physical=len(physical),agree=sum(row['agree'] and row['occurrenceOnlyAgree'] for row in physical),diagnostics=sum(len(row['fullExpressDiagnostics']) for row in physical),officialProperties=len(properties),officialDiagnostics=len(logger.statements))))
if __name__=='__main__':main()
