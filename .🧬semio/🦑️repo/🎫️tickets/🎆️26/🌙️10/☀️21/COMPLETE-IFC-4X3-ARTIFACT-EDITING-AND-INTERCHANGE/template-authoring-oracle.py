"""🧪 Independently author selected IFC templates through native IfcOpenShell APIs and constructors."""
import json
import pathlib
import struct
import sys

ticket = pathlib.Path(__file__).parent
sys.path.insert(0, str(ticket / '🗑️generated' / 'ifc083'))
import ifcopenshell
import ifcopenshell.api.pset.add_pset
import ifcopenshell.api.pset.add_qto
import ifcopenshell.api.control.assign_control
import ifcopenshell.validate

directory = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else ticket / '🗑️generated' / 'template-authoring'
cases = json.loads((directory / 'cases.json').read_text('utf8'))


def plain(value, model):
    if value is None:
        return None
    kind = value['kind']
    if kind == 'typedValue':
        return plain(value['value']['items'][0], model)
    if kind == 'real':
        return struct.unpack('>d', int(value['value']['bits']).to_bytes(8, 'big'))[0]
    if kind == 'integer':
        return int(value['value'])
    if kind == 'reference':
        return model.by_id(int(value['value']))
    if kind == 'aggregate':
        return tuple(plain(item, model) for item in value['value'])
    if kind in ('unset', 'derived'):
        return None
    return value['value']


def typed(value, domain, model):
    return model.create_entity(domain, plain(value, model))


def canonical(value):
    if isinstance(value, ifcopenshell.entity_instance):
        if not value.id():
            return [value.is_a(), canonical(value.wrappedValue)]
        return ['ref', value.is_a(), getattr(value, 'Name', None), getattr(value, 'GlobalId', None)]
    if isinstance(value, (tuple, list)):
        return [canonical(item) for item in value]
    return value


def population(model):
    return sorted([[entity.is_a(), [canonical(item) for item in entity]] for entity in model], key=lambda row: json.dumps(row, sort_keys=True))


results = []
for case in cases:
    name, app, template = case['name'], case['application'], case['template']
    model = ifcopenshell.open(str(directory / f'{name}-source.ifc'))
    product = model.by_id(int(app['target']))
    identities = {row['key']: row for row in app['identities']}
    if 'performance' in app:
        performance = app['performance']
        product = model.create_entity('IfcPerformanceHistory', GlobalId=identities['performance']['guid'], LifeCyclePhase=performance['lifeCyclePhase'], PredefinedType=performance['predefinedType'], ObjectType=performance.get('objectType'), Name=performance.get('name'), Identification=performance.get('identification'), Description=performance.get('description'))
        relation = ifcopenshell.api.control.assign_control(model, relating_control=product, related_object=model.by_id(int(app['target'])))
        relation.GlobalId = identities['control']['guid']
    quantity = template['kind'].startswith('QTO_')
    carrier = ifcopenshell.api.pset.add_qto(model, product=product, name=template['name']) if quantity else ifcopenshell.api.pset.add_pset(model, product=product, name=template['name'])
    if carrier.is_a('IfcRoot'):
        carrier.GlobalId = identities['set']['guid']
        for relation in model.by_type('IfcRelDefinesByProperties'):
            if relation.RelatingPropertyDefinition == carrier:
                relation.GlobalId = identities['relation']['guid']
    carrier.Description = app.get('description')
    if quantity:
        carrier.MethodOfMeasurement = app.get('methodOfMeasurement')
    entries = []
    for selected in app['entries']:
        declaration = next(row for row in template['entries'] if row['name'] == selected['name'])
        domain, form = declaration['dataType'], selected['kind']
        fields = {'Name': selected['name']}
        unit = plain(selected.get('unit'), model)
        if form.startswith('Q_'):
            suffix = {'Q_LENGTH': 'Length', 'Q_AREA': 'Area', 'Q_VOLUME': 'Volume', 'Q_WEIGHT': 'Weight', 'Q_COUNT': 'Count', 'Q_TIME': 'Time'}[form]
            fields.update({suffix + 'Value': plain(selected['value'], model), 'Unit': unit, 'Formula': selected.get('formula'), 'Description': selected.get('description')})
            entity_name = 'IfcQuantity' + suffix
        else:
            fields['Specification'] = selected.get('description')
            if form == 'P_SINGLEVALUE':
                entity_name = 'IfcPropertySingleValue'
                fields.update(NominalValue=typed(selected['value'], domain, model), Unit=unit)
            elif form == 'P_REFERENCEVALUE':
                entity_name = 'IfcPropertyReferenceValue'
                fields.update(PropertyReference=plain(selected['value'], model), UsageName=selected.get('usageName'))
            elif form == 'P_LISTVALUE':
                entity_name = 'IfcPropertyListValue'
                fields.update(ListValues=[typed(value, domain, model) for value in selected['values']], Unit=unit)
            elif form == 'P_ENUMERATEDVALUE':
                entity_name = 'IfcPropertyEnumeratedValue'
                definition = model.create_entity('IfcPropertyEnumeration', Name=declaration['enumeration']['name'], EnumerationValues=[model.create_entity(domain, literal) for literal in declaration['enumeration']['literals']], Unit=unit)
                fields.update(EnumerationValues=[typed(value, domain, model) for value in selected['values']], EnumerationReference=definition)
            elif form == 'P_BOUNDEDVALUE':
                entity_name = 'IfcPropertyBoundedValue'
                fields.update(LowerBoundValue=typed(selected['lower'], domain, model) if 'lower' in selected else None, UpperBoundValue=typed(selected['upper'], domain, model) if 'upper' in selected else None, SetPointValue=typed(selected['setPoint'], domain, model) if 'setPoint' in selected else None, Unit=unit)
            elif form == 'P_TABLEVALUE':
                entity_name = 'IfcPropertyTableValue'
                fields.update(DefiningValues=[typed(value, domain, model) for value in selected['defining']], DefinedValues=[typed(value, declaration['secondaryType'], model) for value in selected['defined']], DefiningUnit=plain(selected.get('definingUnit'), model), DefinedUnit=plain(selected.get('definedUnit'), model), Expression=selected.get('expression'), CurveInterpolation=selected.get('interpolation'))
            else:
                raise AssertionError(form)
        entries.append(model.create_entity(entity_name, **fields))
    setattr(carrier, 'Quantities' if quantity else 'Properties' if carrier.is_a('IfcExtendedProperties') else 'HasProperties', entries)
    model.write(str(directory / f'{name}-oracle.ifc'))
    ours = ifcopenshell.open(str(directory / f'{name}-semio.ifc'))
    same = population(model) == population(ours)
    validations = {}
    for label, candidate in [('semio', ours), ('oracle', model)]:
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(candidate, logger, express_rules=True)
        validations[label] = [{'attribute': row.get('attribute'), 'message': row.get('message')} for row in logger.statements]
    results.append({'name': name, 'template': template['name'], 'forms': [row['kind'] for row in app['entries']], 'samePopulation': same, 'validation': validations})
    if not same:
        (directory / f'{name}-difference.json').write_text(json.dumps({'ours': population(ours), 'oracle': population(model)}, indent=2), 'utf8')
report = {'version': ifcopenshell.version, 'cases': len(results), 'equivalent': sum(row['samePopulation'] for row in results), 'valid': sum(not row['validation']['semio'] and not row['validation']['oracle'] for row in results), 'results': results}
(directory / 'oracle.json').write_text(json.dumps(report, indent=2), 'utf8')
print(json.dumps({key: report[key] for key in ['version', 'cases', 'equivalent', 'valid']}))
assert report['cases'] == report['equivalent'] == report['valid']
