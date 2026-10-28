"""📚 Generate immutable native declaration slices from the retained released catalog."""
import argparse
import hashlib
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(parent for parent in TICKET.parents if (parent / 'AGENTS.md').exists())
SCHEMA = ROOT / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema'
SOURCE = SCHEMA / '📚️catalog/🔣️.json'
OUTPUT = SCHEMA / '⚖️validation/metadata-data.rs'

def quoted(value):
    return json.dumps(value, ensure_ascii=False).replace('\\/', '/')

def option(value, convert=quoted):
    return 'None' if value is None else f'Some({convert(value)})'

def boolean(value):
    return 'true' if value else 'false'

def fields(value, allowed):
    unknown = set(value) - set(allowed)
    if unknown:
        raise ValueError(f'Unrepresented declaration fields: {unknown}')

def strings(values):
    return '&[' + ','.join(quoted(value) for value in values) + ']'

def expression(value):
    fields(value, ['kind', 'name', 'width', 'fixed', 'aggregate', 'lower', 'upper', 'lowerExpression', 'upperExpression', 'unique', 'optional', 'item'])
    return 'StaticTypeExpression{' + ','.join([
        'kind:' + quoted(value['kind']),
        'name:' + option(value.get('name')),
        'width:' + option(value.get('width'), str),
        'fixed:' + boolean(value.get('fixed', False)),
        'aggregate:' + option(value.get('aggregate')),
        'lower:' + option(value.get('lower'), str),
        'upper:' + option(value.get('upper'), str),
        'lower_expression:' + option(value.get('lowerExpression')),
        'upper_expression:' + option(value.get('upperExpression')),
        'unique:' + boolean(value.get('unique', False)),
        'optional:' + boolean(value.get('optional', False)),
        'item:' + option(value.get('item'), lambda item: '&' + expression(item))
    ]) + '}'

def attribute(value):
    fields(value, ['name', 'typeExpr', 'optional', 'derived', 'declaredBy'])
    return 'StaticAttribute{' + ','.join([
        'name:' + quoted(value['name']), 'type_expr:' + expression(value['typeExpr']),
        'optional:' + boolean(value['optional']), 'derived:' + boolean(value['derived']),
        'declared_by:' + quoted(value['declaredBy'])
    ]) + '}'

def derived(value):
    fields(value, ['name', 'typeExpr', 'redeclares', 'expression'])
    return 'StaticDerived{' + ','.join([
        'name:' + quoted(value['name']), 'type_expr:' + expression(value['typeExpr']),
        'redeclares:' + option(value.get('redeclares')), 'expression:' + quoted(value['expression'])
    ]) + '}'

def array(values, convert):
    return '&[' + ','.join(convert(value) for value in values) + ']'

def entity(key, value):
    fields(value, ['name', 'abstractEntity', 'supertype', 'supertypeExpression', 'attributes', 'allAttributes', 'derived', 'inverse', 'whereRules', 'uniqueRules'])
    return 'StaticEntity{' + ','.join([
        'key:' + quoted(key), 'name:' + quoted(value['name']),
        'abstract_entity:' + boolean(value['abstractEntity']),
        'supertype:' + option(value.get('supertype')),
        'supertype_expression:' + quoted(value.get('supertypeExpression', '')),
        'attributes:' + array(value['attributes'], attribute),
        'all_attributes:' + array(value['allAttributes'], attribute),
        'derived:' + array(value['derived'], derived),
        'inverse:' + strings(value['inverse']),
        'where_rules:' + strings(value['whereRules']),
        'unique_rules:' + strings(value['uniqueRules'])
    ]) + '}'

def declaration(key, value):
    fields(value, ['name', 'kind', 'members', 'typeExpr', 'whereRules'])
    return 'StaticType{' + ','.join([
        'key:' + quoted(key), 'name:' + quoted(value['name']),
        'kind:' + quoted(value['kind']), 'members:' + strings(value['members']),
        'type_expr:' + option(value.get('typeExpr'), lambda item: '&' + expression(item)),
        'where_rules:' + strings(value['whereRules'])
    ]) + '}'

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    raw = SOURCE.read_bytes()
    catalog = json.loads(raw)
    output = '/// 🧊 Immutable released declarations; input generator: retained-metadata.py in the IFC ticket.\n'
    output += 'pub const SOURCE_SHA256:&str=' + quoted(hashlib.sha256(raw).hexdigest()) + ';\n'
    for name, key, convert, kind in [('ENTITIES', 'entities', entity, 'StaticEntity'), ('TYPES', 'types', declaration, 'StaticType')]:
        output += f'pub static {name}:&[{kind}]=&[\n'
        output += ',\n'.join(convert(key, value) for key, value in sorted(catalog[key].items()))
        output += '\n];\n'
    if len(str(OUTPUT)) > 256:
        raise ValueError('Generated path exceeds the repository limit')
    if args.check:
        assert OUTPUT.read_text(encoding='utf-8') == output, 'Static declaration asset is stale'
    else:
        OUTPUT.write_text(output, encoding='utf-8', newline='\n')
    print(json.dumps({'entities': len(catalog['entities']), 'types': len(catalog['types']), 'sourceSha256': hashlib.sha256(raw).hexdigest(), 'outputBytes': len(output.encode('utf-8')), 'checked': args.check}))

if __name__ == '__main__':
    main()
