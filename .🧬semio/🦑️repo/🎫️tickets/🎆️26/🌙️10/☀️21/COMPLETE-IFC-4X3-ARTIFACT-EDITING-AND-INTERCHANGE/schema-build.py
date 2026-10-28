"""📚 Build the IFC4X3_ADD2 catalog from the retained official EXPRESS input."""
import hashlib
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[7]
BASE = ROOT / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog'
SOURCE = BASE / 'IFC4X3_ADD2.exp'


def expression(text):
    text = ' '.join(text.split()).upper()
    match = re.fullmatch(r'(LIST|SET|BAG|ARRAY)\s*\[\s*([^:\]]+)\s*:\s*([^\]]+)\s*\]\s*OF\s+(.*)', text)
    if match:
        item = match[4]
        optional = item.startswith('OPTIONAL ')
        item = re.sub(r'^OPTIONAL\s+', '', item)
        unique = item.startswith('UNIQUE ')
        item = re.sub(r'^UNIQUE\s+', '', item)
        return dict(kind='aggregate', aggregate=match[1], lower=int(match[2]) if re.fullmatch(r'-?\d+', match[2].strip()) else None, upper=int(match[3]) if re.fullmatch(r'-?\d+', match[3].strip()) else None, lowerExpression=match[2].strip(), upperExpression=match[3].strip(), unique=unique or match[1] == 'SET', optional=optional, item=expression(item))
    match = re.fullmatch(r'(\w+)(?:\s*\(\s*(\d+)\s*\))?(?:\s+(FIXED))?', text)
    if not match:
        raise ValueError(f'Unsupported type expression: {text}')
    return dict(kind='named', name=match[1], width=int(match[2]) if match[2] else None, fixed=bool(match[3]))


def sections(text):
    parts = re.split(r'^\s*(DERIVE|INVERSE|UNIQUE|WHERE)\s*$', text, flags=re.M)
    result = {'explicit': parts[0]}
    result.update({parts[i].lower(): parts[i + 1].strip() for i in range(1, len(parts), 2)})
    return result


def declarations(text):
    return [x.strip() for x in text.split(';') if x.strip()]


raw = SOURCE.read_bytes()
source = re.sub(r'\(\*.*?\*\)', '', raw.decode('utf-8-sig'), flags=re.S)
entities, types = {}, {}
for name, body in re.findall(r'\bTYPE\s+(\w+)\s*=\s*(.*?)END_TYPE\s*;', source, flags=re.S | re.I):
    parts = sections(body)
    head = parts['explicit'].split(';')[0].strip()
    enumeration = re.fullmatch(r'ENUMERATION\s+OF\s*\((.*?)\)', head, flags=re.S | re.I)
    select = re.fullmatch(r'SELECT\s*\((.*?)\)', head, flags=re.S | re.I)
    record = dict(name=name, kind='enum' if enumeration else 'select' if select else 'defined', whereRules=declarations(parts.get('where', '')))
    if enumeration or select:
        record['members'] = [x.strip().upper() for x in (enumeration or select)[1].split(',')]
    else:
        record['typeExpr'] = expression(head)
        record['members'] = []
    types[name.upper()] = record
for name, body in re.findall(r'\bENTITY\s+(\w+)(.*?)END_ENTITY\s*;', source, flags=re.S | re.I):
    head, rest = body.split(';', 1)
    supertype = re.search(r'SUBTYPE\s+OF\s*\(\s*(\w+)\s*\)', head, flags=re.I)
    parts = sections(rest)
    attrs = []
    for attr in declarations(parts['explicit']):
        match = re.fullmatch(r'(\w+)\s*:\s*(OPTIONAL\s+)?(.*)', attr, flags=re.S | re.I)
        if not match:
            raise ValueError((name, attr))
        attrs.append(dict(name=match[1], typeExpr=expression(match[3]), optional=bool(match[2]), derived=False, declaredBy=name))
    derived = []
    for attr in declarations(parts.get('derive', '')):
        match = re.fullmatch(r'(?:(SELF\\\w+)\.)?(\w+)\s*:\s*(.*?)\s*:=\s*(.*)', attr, flags=re.S | re.I)
        if not match:
            raise ValueError((name, attr))
        derived.append(dict(name=match[2], typeExpr=expression(match[3]), redeclares=match[1], expression=match[4].strip()))
    entities[name.upper()] = dict(name=name, abstractEntity=bool(re.search(r'\bABSTRACT\b', head)), supertype=supertype[1].upper() if supertype else None, attributes=attrs, derived=derived, inverse=declarations(parts.get('inverse', '')), uniqueRules=declarations(parts.get('unique', '')), whereRules=declarations(parts.get('where', '')), supertypeExpression=head.strip())


def flattened(name):
    entity = entities[name]
    attrs = [dict(x) for x in flattened(entity['supertype'])] if entity['supertype'] else []
    attrs.extend(dict(x) for x in entity['attributes'])
    for derived in entity['derived']:
        if derived['redeclares']:
            index = next(i for i, x in enumerate(attrs) if x['name'].upper() == derived['name'].upper())
            attrs[index]['derived'] = True
    return attrs


for name, entity in entities.items():
    entity['allAttributes'] = flattened(name)
functions = {name.upper(): body.strip() for name, body in re.findall(r'\bFUNCTION\s+(\w+)(.*?)END_FUNCTION\s*;', source, flags=re.S | re.I)}
rules = {name.upper(): body.strip() for name, body in re.findall(r'\bRULE\s+(\w+)(.*?)END_RULE\s*;', source, flags=re.S | re.I)}
catalog = dict(schema='IFC4X3_ADD2', version='4.3.2.0', source=dict(url='https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/IFC4X3_ADD2.exp', mirror='https://raw.githubusercontent.com/buildingSMART/Standards-builds/main/IFC/RELEASE/IFC4_3/HTML/IFC4X3_ADD2.exp', sha256=hashlib.sha256(raw).hexdigest()), validation=dict(structural=True, whereRules=False, uniqueRules=False, inverseRules=False, contextualBounds=False, functions=False, globalRules=False, complexInstances=False, binaryValues=False), entities=entities, types=types, functions=functions, rules=rules)
(BASE / '🔣️.json').write_text(json.dumps(catalog, ensure_ascii=False, separators=(',', ':')) + '\n', encoding='utf-8')


def rust(value):
    if value is None:
        return 'None'
    return 'Some(' + json.dumps(value, ensure_ascii=False) + '.into())' if isinstance(value, str) else 'Some(' + str(value) + ')'


def rexpr(value):
    if value['kind'] == 'named':
        return f'IfcTypeExpression {{ kind:"named".into(),name:{rust(value["name"])},width:{rust(value["width"])},fixed:{str(value["fixed"]).lower()},..Default::default() }}'
    return f'IfcTypeExpression {{ kind:"aggregate".into(),aggregate:{rust(value["aggregate"])},lower:{rust(value["lower"])},upper:{rust(value["upper"])},lower_expression:{rust(value["lowerExpression"])},upper_expression:{rust(value["upperExpression"])},unique:{str(value["unique"]).lower()},optional:{str(value["optional"]).lower()},item:Some(Box::new({rexpr(value["item"])})),..Default::default() }}'


def rstrings(values):
    return 'vec![' + ','.join(json.dumps(v, ensure_ascii=False).replace('\\/', '/') + '.into()' for v in values) + ']'


def rattr(value):
    return f'IfcAttributeDeclaration {{name:{json.dumps(value["name"])}.into(),type_expr:{rexpr(value["typeExpr"])},optional:{str(value["optional"]).lower()},derived:{str(value["derived"]).lower()},declared_by:{json.dumps(value["declaredBy"])}.into()}}'


lines = ['// 📚 Retained catalog data derived from the adjacent official EXPRESS schema.', 'fn build_catalog() -> IfcCatalog {', 'let mut entities = BTreeMap::new();', 'let mut types = BTreeMap::new();']
for name, entity in entities.items():
    derived = ','.join(f'IfcDerivedDeclaration {{name:{json.dumps(x["name"])}.into(),type_expr:{rexpr(x["typeExpr"])},redeclares:{rust(x["redeclares"])},expression:{json.dumps(x["expression"])}.into()}}' for x in entity['derived'])
    lines.append(f'entities.insert("{name}".into(),IfcEntityDeclaration {{name:"{entity["name"]}".into(),abstract_entity:{str(entity["abstractEntity"]).lower()},supertype:{rust(entity["supertype"])},attributes:vec![{",".join(rattr(x) for x in entity["attributes"])}],all_attributes:vec![{",".join(rattr(x) for x in entity["allAttributes"])}],derived:vec![{derived}],inverse:{rstrings(entity["inverse"])},where_rules:{rstrings(entity["whereRules"])},unique_rules:{rstrings(entity["uniqueRules"])}}});')
for name, typ in types.items():
    lines.append(f'types.insert("{name}".into(),IfcTypeDeclaration {{name:"{typ["name"]}".into(),kind:"{typ["kind"]}".into(),members:{rstrings(typ["members"])},type_expr:{"Some("+rexpr(typ["typeExpr"])+")" if "typeExpr" in typ else "None"},where_rules:{rstrings(typ["whereRules"])}}});')
for kind, entries in [('functions', functions), ('rules', rules)]:
    lines.append(f'let mut {kind} = BTreeMap::new();')
    for name, body in entries.items():
        lines.append(f'{kind}.insert("{name}".into(),{json.dumps(body, ensure_ascii=False)}.into());')
lines.extend([f'IfcCatalog {{schema:"IFC4X3_ADD2".into(),source_sha256:"{catalog["source"]["sha256"]}".into(),entities,types,functions,rules}}', '}'])
(BASE / 'data.rs').write_text('\n'.join(lines) + '\n', encoding='utf-8')
print(json.dumps(dict(entities=len(entities), types=len(types), sha256=catalog['source']['sha256'])))
