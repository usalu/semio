"""🌐️ Extract every bilingual Text object (en/de) from the staffage catalog and the cast manifest into a TSV for review."""
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding='utf-8')
ROOT = sys.argv[1]
OUT = sys.argv[2]
FILES = []
for dirpath, _dirs, names in os.walk(ROOT):
    for name in names:
        if name.endswith('.json'):
            FILES.append(os.path.join(dirpath, name))
FILES.sort()
rows = []
problems = []


def walk(node, path, rel):
    if isinstance(node, dict):
        keys = set(node.keys())
        if 'en' in keys or 'de' in keys:
            if keys <= {'en', 'de'}:
                rows.append((rel, path, node.get('en'), node.get('de')))
                return
            problems.append(('mixed-keys', rel, path, sorted(keys)))
        for key, value in node.items():
            walk(value, path + '.' + key, rel)
    elif isinstance(node, list):
        for index, value in enumerate(node):
            walk(value, '%s[%d]' % (path, index), rel)


for file in FILES:
    rel = os.path.relpath(file, ROOT).replace(os.sep, '/')
    with open(file, encoding='utf-8') as handle:
        data = json.load(handle)
    walk(data, '', rel)

with open(OUT, 'w', encoding='utf-8') as handle:
    for rel, path, en, de in rows:
        field = path.split('.')[-1] if path else ''
        handle.write('%s\t%s\t%s\t%s\n' % (rel, path, (en or '').replace('\t', ' ').replace('\n', ' '), (de or '').replace('\t', ' ').replace('\n', ' ')))

print('[DEBUG] files=%d rows=%d mixed=%d' % (len(FILES), len(rows), len(problems)))
for problem in problems[:40]:
    print('[DEBUG] mixed', problem)
missing = [r for r in rows if not isinstance(r[2], str) or not isinstance(r[3], str) or not r[2].strip() or not r[3].strip()]
print('[DEBUG] missing-or-empty=%d' % len(missing))
for r in missing[:60]:
    print('[DEBUG] missing', r[0], r[1], repr(r[2])[:60], repr(r[3])[:60])
same = [r for r in rows if isinstance(r[2], str) and isinstance(r[3], str) and r[2].strip() == r[3].strip()]
print('[DEBUG] identical-en-de=%d' % len(same))
for r in same:
    print('[DEBUG] same', r[0], r[1], r[2][:90])
fields = {}
for rel, path, en, de in rows:
    name = re.sub(r'\[\d+\]', '[]', path)
    fields[name] = fields.get(name, 0) + 1
print('[DEBUG] distinct field paths=%d' % len(fields))
for name in sorted(fields):
    print('[DEBUG] field %5d %s' % (fields[name], name))
