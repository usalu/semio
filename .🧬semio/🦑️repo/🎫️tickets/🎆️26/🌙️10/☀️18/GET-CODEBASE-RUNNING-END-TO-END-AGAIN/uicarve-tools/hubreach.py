import json, os, re, sys, subprocess
sys.path.insert(0, 'lib')
import carve
R = carve.R
hubs = [l.strip() for l in subprocess.run(['git', 'ls-files', '--', '🌎️hub/**/Cargo.toml', '🌎️hub/Cargo.toml'], cwd=R, capture_output=True, text=True).stdout.split('\n') if l.strip()]
seen = {}; queue = [R + '/' + h for h in hubs]
def deps_of(mp):
    t = carve.rd(mp); out = []
    for m in re.finditer(r'(?m)^[ \t]*"?[\w\-]+"?\s*=\s*\{[^\n]*?path\s*=\s*"([^"]+)"', t):
        out.append(os.path.normpath(os.path.join(os.path.dirname(mp), m.group(1), 'Cargo.toml')))
    for m in re.finditer(r'(?m)^path\s*=\s*"([^"]+)"', t):
        pass
    return out
names = {}
while queue:
    mp = queue.pop()
    if mp in seen or not os.path.exists(mp): continue
    t = carve.rd(mp); nm = re.search(r'(?m)^name\s*=\s*"([^"]+)"', t)
    seen[mp] = nm.group(1) if nm else None
    queue += deps_of(mp)
pk = sorted(n for n in seen.values() if n and n.startswith('semio-framework-ui'))
print(len(seen), 'manifests;', len(pk), 'ui packages reached')
for n in pk:
    if 'wgpu' in n or 'tui' in n or n == 'semio-framework-ui': print(n)
