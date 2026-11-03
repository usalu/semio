"""EN 1992 v8: a bar patch row (any of layerId/y/z/diameter null) carries `length`/`bend` null instead of the row defaults. Idempotent."""
import pathlib, re
R = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992')
PAT = re.compile(r'("layerId": (?P<a>[^\n]*),\n(?P<i>[ ]*)"y": (?P<b>[^\n]*),\n[ ]*"z": (?P<c>[^\n]*),\n[ ]*"diameter": (?P<d>[^\n,]*),\n[ ]*)"length": 0\.0,\n[ ]*"bend": "auto"')
def fix(m):
    if 'null' in (m.group('a'), m.group('b'), m.group('c'), m.group('d')):
        return m.group(1) + '"length": null,\n' + m.group('i') + '"bend": null'
    return m.group(0)
n = 0
for p in R.rglob('*.json'):
    s = p.read_text()
    if '"length": 0.0,' not in s: continue
    t = PAT.sub(fix, s)
    if t != s: p.write_text(t); n += 1; print(p.relative_to(R).parts[-4:-1])
print('fixed', n)
