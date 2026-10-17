import re, subprocess, sys
INNER = re.compile(r'^#!\[.*\]\s*$')
def errors(path):
    r = subprocess.run(['rustc', '+nightly-2026-07-20', '--edition', '2021', '-Zunpretty=normal', '--error-format=short', '--crate-type=lib', path], capture_output=True, text=True)
    out = []
    for l in r.stderr.splitlines():
        m = re.match(r'^.*?:(\d+):(\d+): error: an inner attribute is not permitted', l)
        if m: out.append(int(m.group(1)))
    return out
def hoist(path):
    for _ in range(50):
        errs = errors(path)
        if not errs: return True
        with open(path, encoding='utf-8') as f: lines = f.read().split('\n')
        n = errs[0] - 1
        assert INNER.match(lines[n]), (path, n + 1, lines[n])
        attr = lines.pop(n)
        i = 0
        while i < len(lines) and (lines[i].startswith('//!') or INNER.match(lines[i])): i += 1
        lines.insert(i, attr)
        with open(path, 'w', encoding='utf-8') as f: f.write('\n'.join(lines))
    return False
for p in sys.argv[1:]:
    print(p[-60:], hoist(p))
