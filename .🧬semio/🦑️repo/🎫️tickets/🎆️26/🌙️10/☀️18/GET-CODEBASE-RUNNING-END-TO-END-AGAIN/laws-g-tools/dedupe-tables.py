"""Gives each double-routed test file to the table whose root contains it (the cross-table law of the process suites test). usage: python3 dedupe-tables.py [--check]"""
import json, sys
R = '/Users/ueli/Documents/semio/'
L = R + '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
P = R + '🧰️framework/🔨️modules/🏃️process/'
check = '--check' in sys.argv
def canonical(path):
    raw = open(path, encoding='utf-8').read(); doc = json.loads(raw)
    assert json.dumps(doc, ensure_ascii=False, indent=2) + '\n' == raw, path
    return doc
def save(path, doc):
    if not check: open(path, 'w', encoding='utf-8').write(json.dumps(doc, ensure_ascii=False, indent=2) + '\n')
pt = P + '🧪️testing/🗂️suites/🔣️.json'; pdoc = canonical(pt)
pp = P + '📋️project.json'; pproj = canonical(pp)
marker = '/⚡️caching/'
removed = []
keep = []
for s in pdoc['suites']:
    files = s.get('files')
    if files:
        mine = [f for f in files if not (f.startswith('../../🛍️products/🦑️repo/🔨️modules/📚️library' + marker))]
        if len(mine) != len(files):
            removed += [s['id'] + ': ' + f[-70:] for f in files if f not in mine]
            if not mine and not s.get('trees') and not s.get('function'):
                pproj['targets'].pop('test-' + s['id'], None)
                continue
            s['files'] = mine
    keep.append(s)
pdoc['suites'] = keep
save(pt, pdoc); save(pp, pproj)
ct = L + '⚡️caching/🗂️suites/🔣️.json'; cdoc = canonical(ct)
cp = L + '⚡️caching/📋️project.json'; text = open(cp, encoding='utf-8').read()
before = len(cdoc['suites'])
cdoc['suites'] = [s for s in cdoc['suites'] if s['id'] != 'lease-bootstrap']
if len(cdoc['suites']) != before:
    removed.append('caching: suite lease-bootstrap (its file lies below the process root)')
    import re
    new = re.sub(r'    "test-lease-bootstrap": \{.*\},\n', '', text, count=1)
    assert new != text
    json.loads(new)
    if not check: open(cp, 'w', encoding='utf-8').write(new)
save(ct, cdoc)
print('\n'.join(removed) or 'nothing to do')
