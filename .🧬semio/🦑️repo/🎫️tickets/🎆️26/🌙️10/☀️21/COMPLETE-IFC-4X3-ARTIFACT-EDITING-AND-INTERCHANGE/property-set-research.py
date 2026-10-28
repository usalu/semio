"""🏷️ Inspect exact released property-set source locations before selecting an authoring contract."""
import json
import hashlib
import io
import pathlib
import urllib.request
import xml.etree.ElementTree as ET
import zipfile
from collections import Counter
ticket = pathlib.Path(__file__).resolve().parent
output = ticket / '🗑️generated/property-sets'
output.mkdir(parents=True, exist_ok=True)
revision = '610ee3f95db9d0380b1eb8de4dc6e17833531518'
url = f'https://api.github.com/repos/buildingSMART/Standards-builds/contents/IFC/RELEASE/IFC4_3?ref={revision}'
with urllib.request.urlopen(urllib.request.Request(url, headers={'User-Agent': 'semio-ifc-source-research'}), timeout=30) as response:
    content = response.read()
(output / 'release.json').write_bytes(content)
html = next(row for row in json.loads(content) if row['name'] == 'HTML')
tree_url = f'https://api.github.com/repos/buildingSMART/Standards-builds/git/trees/{html["sha"]}'
with urllib.request.urlopen(urllib.request.Request(tree_url, headers={'User-Agent': 'semio-ifc-source-research'}), timeout=30) as response:
    tree_content = response.read()
(output / 'html-tree.json').write_bytes(tree_content)
archive_entry = next(row for row in json.loads(tree_content)['tree'] if row['path'] == 'annex-a-psd.zip')
archive_url = f'https://raw.githubusercontent.com/buildingSMART/Standards-builds/{revision}/IFC/RELEASE/IFC4_3/HTML/annex-a-psd.zip'
target = output / 'annex-a-psd.zip'
if target.exists():
    archive_content = target.read_bytes()
else:
    with urllib.request.urlopen(archive_url, timeout=30) as response:
        archive_content = response.read(16 * 1024 * 1024)
assert len(archive_content) == archive_entry['size']
assert hashlib.sha1(b'blob ' + str(len(archive_content)).encode() + b'\0' + archive_content).hexdigest() == archive_entry['sha']
target.write_bytes(archive_content)
archive = zipfile.ZipFile(io.BytesIO(archive_content))
roots, tags, kinds, datatypes = Counter(), Counter(), Counter(), Counter()
rows = []
templates = []
property_kinds = {'TypePropertySingleValue': 'P_SINGLEVALUE', 'TypePropertyReferenceValue': 'P_REFERENCEVALUE', 'TypePropertyEnumeratedValue': 'P_ENUMERATEDVALUE', 'TypePropertyListValue': 'P_LISTVALUE', 'TypePropertyBoundedValue': 'P_BOUNDEDVALUE', 'TypePropertyTableValue': 'P_TABLEVALUE'}
quantity_types = {'Q_WEIGHT': 'IfcMassMeasure', 'Q_LENGTH': 'IfcLengthMeasure', 'Q_AREA': 'IfcAreaMeasure', 'Q_VOLUME': 'IfcVolumeMeasure', 'Q_COUNT': 'IfcCountMeasure', 'Q_TIME': 'IfcTimeMeasure'}
for name in archive.namelist():
    if not name.lower().endswith('.xml'):
        continue
    content = archive.read(name)
    root = ET.fromstring(content)
    tag = lambda item: item.tag.rsplit('}', 1)[-1]
    roots[tag(root)] += 1
    for item in root.iter():
        tags[tag(item)] += 1
        if tag(item).startswith('TypeProperty'):
            kinds[tag(item)] += 1
        if tag(item) == 'DataType':
            datatypes[item.get('type')] += 1
    rows.append({'file': name, 'bytes': len(content), 'sha256': hashlib.sha256(content).hexdigest(), 'root': tag(root), 'attributes': root.attrib})
    assert root.find('IfcVersion').get('version') == 'IFC4X3_ADD2'
    entries = []
    for entry in root.findall('PropertyDefs/PropertyDef') + root.findall('QtoDefs/QtoDef'):
        item = {'name': entry.findtext('Name'), 'definition': entry.findtext('Definition') or '', 'secondary': None, 'enum': None}
        if tag(entry) == 'QtoDef':
            item['kind'] = entry.findtext('QtoType')
            item['type'] = quantity_types[item['kind']]
        else:
            declared = list(entry.find('PropertyType'))
            assert len(declared) == 1
            declared = declared[0]
            item['kind'] = property_kinds[tag(declared)]
            if item['kind'] == 'P_ENUMERATEDVALUE':
                item['type'] = 'IfcLabel'
                enumeration = declared.find('EnumList')
                item['enumName'] = enumeration.get('name')
                item['enum'] = [value.text for value in enumeration.findall('EnumItem')]
            elif item['kind'] == 'P_REFERENCEVALUE':
                item['type'] = declared.get('reftype')
            elif item['kind'] == 'P_TABLEVALUE':
                item['type'] = declared.find('DefiningValue/DataType').get('type')
                item['secondary'] = declared.find('DefinedValue/DataType').get('type')
                item['expression'] = declared.findtext('Expression') or ''
            elif item['kind'] == 'P_LISTVALUE':
                item['type'] = declared.find('ListValue/DataType').get('type')
            else:
                item['type'] = declared.find('DataType').get('type')
        entries.append(item)
    templates.append({'name': root.findtext('Name'), 'definition': root.findtext('Definition') or '', 'kind': root.get('templatetype'), 'applicable': root.findtext('ApplicableTypeValue') or '', 'classes': [item.text for item in root.findall('ApplicableClasses/ClassName')], 'entries': entries, 'sourceFile': name, 'sourceSha256': hashlib.sha256(content).hexdigest()})
    if name.endswith('Pset_WallCommon.xml'):
        (output / 'Pset_WallCommon.xml').write_bytes(content)
summary = {'commit': revision, 'url': archive_url, 'sha256': hashlib.sha256(archive_content).hexdigest(), 'bytes': len(archive_content), 'roots': dict(roots), 'propertyKinds': dict(kinds), 'dataTypes': dict(datatypes), 'tags': dict(tags), 'files': rows}
(output / 'summary.json').write_text(json.dumps(summary, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
(output / 'compiled.json').write_text(json.dumps({'schema': 'IFC4X3_ADD2', 'commit': revision, 'sourceUrl': archive_url, 'sourceSha256': summary['sha256'], 'sourceLanguage': 'en', 'templates': sorted(templates, key=lambda row: row['name'])}, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
oracle_path = output / 'ifcopenshell.json'
if oracle_path.exists():
    oracle = json.loads(oracle_path.read_text(encoding='utf-8'))
    independent = {row['name']: row for row in oracle['templates']}
    assert set(independent) == {row['name'] for row in templates}
    mismatches = []
    for row in templates:
        other = independent[row['name']]
        for key in ['kind', 'applicable']:
            if row[key] != other[key]:
                mismatches.append({'set': row['name'], 'field': key, 'source': row[key], 'oracle': other[key]})
        source_entries = [{key: value[key] for key in ['name','kind','type','secondary','enum']} for value in row['entries']]
        if sorted(source_entries, key=lambda value: value['name']) != sorted(other['entries'], key=lambda value: value['name']):
            mismatches.append({'set': row['name'], 'field': 'entries', 'source': source_entries, 'oracle': other['entries']})
    (output / 'comparison.json').write_text(json.dumps({'implementation': oracle['implementation'], 'version': oracle['version'], 'templates': len(templates), 'entries': sum(len(row['entries']) for row in templates), 'mismatches': mismatches}, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({'independentTemplates': len(templates), 'independentEntries': sum(len(row['entries']) for row in templates), 'mismatches': len(mismatches)}))
print(json.dumps({key: value for key, value in summary.items() if key not in ['files','tags','dataTypes']}))
