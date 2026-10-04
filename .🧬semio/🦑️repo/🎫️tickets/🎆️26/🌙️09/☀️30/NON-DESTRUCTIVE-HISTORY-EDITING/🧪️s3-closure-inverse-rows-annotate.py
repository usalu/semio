"""🧺️ S3-CLOSURE step 4: writes each multi-row leaf's `x-semio-inverse-rows` into its payload schema (design §20.5)."""
import json, re, subprocess, sys
ROOT = '/Users/ueli/Documents/semio'
CEILING = 1025
T = lambda **fields: {'perTarget': fields}
B = lambda n: {'bounded': n}
F = lambda n: {'fixed': n}
ROWS = [
    ('📏️layout', '📏️layout', None, 'drag-frames', T(targets=1)),
    ('📏️layout', '📏️layout', None, 'rotate-frames', T(targets=2)),
    ('📏️layout', '📏️layout', None, 'scale-frames', T(targets=2)),
    ('🖍️draw', '🖍️drawing', None, 'drag-layers', T(targets=1)),
    ('🖍️draw', '🖍️drawing', None, 'rotate-layers', T(targets=1)),
    ('🖍️draw', '🖍️drawing', None, 'scale-layers', T(targets=1)),
    ('🖍️draw', '🖍️drawing', None, 'drag-path-points', T(targets=1)),
    ('🀄️wfc', '🖼️bitmap', None, 'resize-input', F(2)),
    ('🀄️wfc', '🖼️bitmap', None, 'resize-output', B(1025)),
    ('🀄️wfc', '◻️2d', None, 'delete-slot', B(CEILING)),
    ('🀄️wfc', '◻️2d', None, 'delete-tile', B(CEILING)),
    ('🀄️wfc', '🧊️3d', None, 'delete-slot', B(CEILING)),
    ('🀄️wfc', '🧊️3d', None, 'delete-tile', B(CEILING)),
    ('🀄️wfc', '🔲️grid2d', None, 'resize-grid', B(CEILING)),
    ('🀄️wfc', '🔲️grid2d', None, 'mask-cell', B(2)),
    ('🀄️wfc', '🔲️grid2d', None, 'delete-tile', B(CEILING)),
    ('🀄️wfc', '🧱️grid3d', None, 'resize-grid', F(4)),
    ('🀄️wfc', '🧱️grid3d', None, 'delete-tile', B(CEILING)),
    ('🧩️puzzle', '🖐️5d', None, 'drag-selection2d', T(targets=1)),
    ('🧩️puzzle', '🖐️5d', None, 'drag-selection3d', T(targets=2)),
    ('🧩️puzzle', '🖐️5d', None, 'rotate-selection3d', T(targets=1)),
    ('🧩️puzzle', '🖐️5d', None, 'scale-selection3d', T(targets=1)),
    ('🧩️puzzle', '🖐️5d', None, 'delete-part', B(65)),
    ('🧩️puzzle', '🖐️5d', None, 'remove-part-grip', B(65)),
    ('🧩️puzzle', '🧊️3d', None, 'drag-selection', B(4096)),
    ('🧩️puzzle', '🧊️3d', None, 'rotate-selection', B(4096)),
    ('🧩️puzzle', '🧊️3d', None, 'scale-selection', T(targets=1)),
    ('🧩️puzzle', '🧊️3d', None, 'delete-object', B(65)),
    ('🧩️puzzle', '🧊️3d', None, 'remove-object-vortex', B(65)),
    ('🧩️puzzle', '◻️2d', None, 'delete-node', B(65)),
    ('🧩️puzzle', '◻️2d', None, 'drag-selection', T(targets=1)),
    ('🧩️puzzle', '◻️2d', None, 'scale-selection', T(targets=2)),
    ('🧩️puzzle', '◻️2d', None, 'rotate-selection', T(targets=65)),
    ('🧩️puzzle', '◻️2d', None, 'remove-node-handle', B(2)),
    ('🏗️fem', '◻️2d', None, 'move-selection', T(nodeIds=1, regionIds=1)),
    ('🏗️fem', '🧊️3d', None, 'move-selection', T(nodeIds=1, solidIds=1)),
    ('🌊️flow', '🌊️flow', None, 'delete-widget', B(258)),
    ('🎬️sequence', '🎬️sequence', None, 'delete-step', B(768)),
    ('➗️mathematical', '➗️equation', None, 'delete-node', B(513)),
    ('➗️mathematical', '➗️equation', None, 'delete-nodes', B(768)),
    ('🌀️procedural', '🌀️generation2d', None, 'move-nodes', T(ids=1)),
    ('🌀️procedural', '🧊️generation3d', None, 'move-nodes', T(ids=1)),
    ('🌀️procedural', '🧊️generation3d', None, 'drag-transforms', T(targets=1)),
    ('🌀️procedural', '🧊️generation3d', None, 'scale-transforms', T(targets=1)),
    ('🌀️procedural', '🧊️generation3d', None, 'rotate-transforms', T(targets=1)),
    ('🎞️animate', '🎬️presentation', None, 'delete-tiles', T(ids=1)),
    ('🔋️energy', '🔋️model', None, 'delete-fenestration', B(2)),
    ('🔋️energy', '🔋️model', None, 'delete-surface', B(CEILING)),
    ('🔱️trinity', '🔌️jack', None, 'delete-node', B(CEILING)),
    ('🕸️dag', '🕸️dag', None, 'delete-node', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '📊️table', 'delete-column', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🔺️mesh', 'delete-primitive', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🔺️mesh', 'delete-mesh', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🔺️mesh', 'delete-material', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🕸️graph', 'delete-node', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🧊️brep', 'delete-edge', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🧊️brep', 'delete-shell', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🧊️brep', 'delete-solid', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🧊️brep', 'delete-vertex', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🧊️brep', 'delete-face', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🧰️kit', 'unbind-representation', B(CEILING)),
    ('🗄️stdio', '🧿️semio', '🧰️kit', 'remove-design', F(2)),
]
files = subprocess.run(['git', 'ls-files', '✏️s/🔌️plugins'], cwd=ROOT, capture_output=True, text=True).stdout.splitlines()
leaves = [f.rsplit('/🧬️schema/🔣️.json', 1)[0] for f in files if f.endswith('/🧬️schema/🔣️.json') and '/🧬️mutations/' in f and '/🧪️tests/' not in f and '/🧫️fixtures/' not in f]
def strip(name): return re.sub(r'^[^a-z0-9]+', '', name)
failures, written = [], 0
for plugin, artifact, subset, leaf, rows in ROWS:
    hits = [f for f in leaves if f.split('/')[2] == plugin and f.split('/')[4] == artifact and strip(f.split('/')[-1]) == leaf and (subset is None or f'/{subset}/' in f)]
    if len(hits) != 1:
        failures.append(f'{plugin} {artifact} {subset} {leaf}: {len(hits)} leaf directories'); continue
    schema = ROOT + '/' + hits[0] + '/🧬️schema/🔣️.json'
    text = open(schema, encoding='utf-8').read()
    document = json.loads(text)
    for field in rows.get('perTarget', {}):
        if document.get('properties', {}).get(field, {}).get('type') != 'array':
            failures.append(f'{schema}: perTarget {field} is not an array property')
    if 'x-semio-inverse-rows' in document:
        if document['x-semio-inverse-rows'] != rows: failures.append(f'{schema}: already declares {document["x-semio-inverse-rows"]}')
        continue
    line = '  "x-semio-inverse-rows": ' + json.dumps(rows, ensure_ascii=False, separators=(', ', ': ')).replace('{', '{ ').replace('}', ' }') + ',\n'
    match = re.search(r'^  "title": .*,\n', text, re.M) or re.search(r'^  "\$id": .*,\n', text, re.M)
    if not match:
        failures.append(f'{schema}: no root title/$id line'); continue
    updated = text[:match.end()] + line + text[match.end():]
    if json.loads(updated).get('x-semio-inverse-rows') != rows:
        failures.append(f'{schema}: insertion did not parse back'); continue
    if '--write' in sys.argv:
        open(schema, 'w', encoding='utf-8').write(updated)
    written += 1
print(f'written={written} failures={len(failures)}')
print('\n'.join(failures))
