from pathlib import Path
import json
import re

root = Path('✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot')
path = root / '🔣️.json'
original = path.read_text()
schema = json.loads(original)
definitions = schema.get('$defs', schema.get('definitions'))
homes = {}
for source in (root / '🧱️rows').glob('*/🦀️.rs'):
    for name in re.findall(r'pub struct (\w+)', source.read_text()):
        if name in definitions and definitions[name].get('x-semio-rust', {}).get('derive'):
            assert name not in homes
            homes[name] = source.relative_to(root).as_posix()
assert len(homes) == 29
lines = original.splitlines(keepends=True)
for name, home in homes.items():
    indices = [i for i, line in enumerate(lines) if line.lstrip().startswith(json.dumps(name) + ':')]
    assert len(indices) == 1
    i = indices[0]
    marker = '"x-semio-rust":{"doc":'
    position = lines[i].rfind(marker)
    assert position >= 0
    lines[i] = lines[i][:position] + lines[i][position:].replace(marker, '"x-semio-rust":{"file":' + json.dumps(home, ensure_ascii=False) + ',"doc":', 1)
updated = ''.join(lines)
assert all(json.loads(updated)['$defs'][name]['x-semio-rust']['file'] == home for name, home in homes.items())
assert path.read_text() == original
path.write_text(updated)
