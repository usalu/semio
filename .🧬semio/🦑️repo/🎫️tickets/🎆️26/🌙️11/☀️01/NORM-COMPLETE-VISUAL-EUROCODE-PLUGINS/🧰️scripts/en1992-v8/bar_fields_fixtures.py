"""EN 1992 v8: appends `length` + `bend` to every bar row of every committed en1992 JSON fixture (text insertion, formatting preserved). Idempotent."""
import pathlib, re
R = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992')
PAT = re.compile(r'("layerId": [^\n]*,\n(?P<i>[ ]*)"y": [^\n]*,\n[ ]*"z": [^\n]*,\n[ ]*"diameter": [^\n,]*)(?P<end>\n)(?!(?P=i)"length")')
n = 0
for p in R.rglob('*.json'):
    s = p.read_text()
    if '"layerId"' not in s: continue
    t = PAT.sub(lambda m: m.group(1) + ',\n' + m.group('i') + '"length": 0.0,\n' + m.group('i') + '"bend": "auto"' + m.group('end'), s)
    if t != s:
        p.write_text(t); n += 1
print('files', n)
