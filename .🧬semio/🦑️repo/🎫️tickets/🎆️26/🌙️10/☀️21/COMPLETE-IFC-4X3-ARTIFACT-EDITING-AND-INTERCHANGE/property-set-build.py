"""🏷️ Compile exact released property metadata into the schema-owned catalog."""
import hashlib
import json
import pathlib
ticket = pathlib.Path(__file__).resolve().parent
generated = ticket / '🗑️generated/property-sets'
source = json.loads((generated / 'compiled.json').read_text(encoding='utf-8'))
comparison = json.loads((generated / 'comparison.json').read_text(encoding='utf-8'))
assert not comparison['mismatches'] and comparison['templates'] == 760 and comparison['entries'] == 3988
destination = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates'
templates = []
for template in sorted(source['templates'], key=lambda row: row['name'].upper()):
    entries = []
    for entry in template['entries']:
        result = {key: entry[key] for key in ['name','definition','kind']}
        result['dataType'] = entry['type']
        if entry['kind'] == 'P_ENUMERATEDVALUE':
            result['enumeration'] = {'name': entry['enumName'], 'literals': entry['enum']}
        if entry['kind'] == 'P_TABLEVALUE':
            result['secondaryType'] = entry['secondary']
            result['expression'] = entry['expression']
        entries.append(result)
    templates.append({**{key: template[key] for key in ['name','definition','kind','applicable','classes','sourceFile','sourceSha256']}, 'entries': entries})
catalog = {'schema': source['schema'], 'source': {'url': source['sourceUrl'], 'commit': source['commit'], 'sha256': source['sourceSha256'], 'language': source['sourceLanguage']}, 'templates': templates}
destination.mkdir(parents=True, exist_ok=True)
(destination / '🔣️.json').write_text(json.dumps(catalog, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
archive = (generated / 'annex-a-psd.zip').read_bytes()
assert hashlib.sha256(archive).hexdigest() == catalog['source']['sha256']
(destination / 'annex-a-psd.zip').write_bytes(archive)
def quoted(value):
    return '"' + ''.join('\\\\' if char == '\\' else '\\"' if char == '"' else '\\n' if char == '\n' else '\\r' if char == '\r' else '\\t' if char == '\t' else f'\\u{{{ord(char):x}}}' if ord(char) < 32 else char for char in value) + '"'
def optional(value):
    return 'None' if value is None else f'Some({quoted(value)})'
def strings(values):
    return '&[' + ','.join(quoted(value) for value in values) + ']'
kinds = {'P_SINGLEVALUE':'SingleValue','P_REFERENCEVALUE':'ReferenceValue','P_ENUMERATEDVALUE':'EnumeratedValue','P_LISTVALUE':'ListValue','P_BOUNDEDVALUE':'BoundedValue','P_TABLEVALUE':'TableValue','Q_WEIGHT':'Weight','Q_LENGTH':'Length','Q_AREA':'Area','Q_VOLUME':'Volume','Q_COUNT':'Count','Q_TIME':'Time'}
def entry_source(entry):
    enumeration = entry.get('enumeration')
    enumeration_source = 'None' if enumeration is None else 'Some(IfcTemplateEnumeration{name:' + quoted(enumeration['name']) + ',literals:' + strings(enumeration['literals']) + '})'
    return 'IfcTemplateEntry{' + ','.join([f'name:{quoted(entry["name"])}', f'definition:{quoted(entry["definition"])}', f'kind:IfcTemplateEntryKind::{kinds[entry["kind"]]}', f'data_type:{quoted(entry["dataType"])}', f'enumeration:{enumeration_source}', f'secondary_type:{optional(entry.get("secondaryType"))}', f'expression:{optional(entry.get("expression"))}']) + '}'
def template_source(template):
    fields = {key: template[key] for key in ['name','definition','kind','applicable']}
    fields.update({'source_file': template['sourceFile'], 'source_sha256': template['sourceSha256']})
    return 'IfcTemplate{' + ','.join(f'{key}:{quoted(value)}' for key, value in fields.items()) + ',classes:' + strings(template['classes']) + ',entries:&[' + ','.join(entry_source(entry) for entry in template['entries']) + ']}'
native = '//! 📦️ Exact immutable ADD2 template declarations.\nstatic CATALOG:IfcTemplateCatalog=IfcTemplateCatalog{schema:"IFC4X3_ADD2",source:IfcTemplateSource{' + ','.join(f'{key}:{quoted(value)}' for key, value in catalog['source'].items()) + '},templates:&[' + ','.join(template_source(template) for template in templates) + ']};\n'
(destination / 'data.rs').write_text(native, encoding='utf-8')
print(json.dumps({'templates': len(templates), 'entries': sum(len(row['entries']) for row in templates), 'archiveSha256': catalog['source']['sha256']}))
