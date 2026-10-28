"""🧪 Retain exact external official sources with independently verified baseline metadata."""
import hashlib
import json
import pathlib
ticket = pathlib.Path(__file__).resolve().parent
generated = ticket / '🗑️generated/corpus'
manifest = json.loads((generated / 'downloaded.json').read_text(encoding='utf-8'))
oracle = json.loads((generated / 'ifcopenshell.json').read_text(encoding='utf-8'))
own = json.loads((generated / 'semio.json').read_text(encoding='utf-8'))
assert manifest['commit'] == oracle['commit'] == own['commit']
assert len(manifest['files']) == len(oracle['files']) == len(own['files']) == 46
oracle_by_path = {row['path']: row for row in oracle['files']}
own_by_path = {row['path']: row for row in own['files']}
destination = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official'
cases = []
for source in manifest['files']:
    independent = oracle_by_path[source['path']]
    local = own_by_path[source['path']]
    assert not independent.get('error') and local.get('roundtrip') is True
    assert independent['entities'] == local['entities']
    assert independent['sha256'] == local['sha256'] == source['sha256']
    assert bool(independent['diagnostics']) == bool(local['diagnostics'])
    name = pathlib.PurePosixPath(source['path']).name
    target = destination / name
    assert len(str(target).encode('utf-16-le')) // 2 <= 256
    content = (generated / source['file']).read_bytes()
    assert hashlib.sha256(content).hexdigest() == source['sha256']
    target.write_bytes(content)
    diagnostics = [{'kind': row['type'], 'entityId': str(row['instance']), 'attribute': row['attribute']} for row in independent['diagnostics']]
    cases.append({**{key: source[key] for key in ['path','url','blob','sha256','bytes']}, 'file': name, 'entities': independent['entities'], 'diagnostics': diagnostics})
assert len({row['file'] for row in cases}) == 46
result = {'schema': 'IFC4X3_ADD2', 'repository': manifest['repository'], 'commit': manifest['commit'], 'notice': 'buildingSMART International Ltd.; unchanged informative examples; see source repository README for release and license terms.', 'oracle': {'implementation': 'IfcOpenShell', 'version': '0.9.0.post1', 'expressRules': True}, 'cases': cases}
(destination / 'cases.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print(json.dumps({'retained': len(cases), 'entities': sum(row['entities'] for row in cases), 'independentDiagnostics': sum(len(row['diagnostics']) for row in cases)}))
