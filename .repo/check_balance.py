import re
files = {
    'semio/net/Semio/Semio.cs': r'^(?://\s*)?#(end)?region\s',
    'semio/go/main.go': r'^// #(end)?region\s',
    'semio/js/index.ts': r'^// #(end)?region\s',
    'semio/py/main.py': r'^# #(end)?region\s',
}
for fp, pat in files.items():
    with open(fp, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    s = []
    for i, l in enumerate(lines):
        m = re.match(pat, l.strip())
        if m:
            if m.group(1) is None:
                s.append(i + 1)
            elif s:
                s.pop()
    print(f'{fp}: unclosed={len(s)} total={len(lines)}')
