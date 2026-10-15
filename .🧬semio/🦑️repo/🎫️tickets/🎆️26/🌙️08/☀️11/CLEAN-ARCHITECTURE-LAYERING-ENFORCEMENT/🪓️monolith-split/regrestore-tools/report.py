import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from touched import *

LABEL = {
 'bim': '4 BIM model rehome', 'cad': '7 CAD renderer', 'p2d': '9 puzzle 2d editor', 'p5d': '10 puzzle 5d editor',
 'md': '1 markdown-html-compiler', 'pres': '23 presentation react (14 modules + parent)', 'pres2': '2 animate react copy (same tree as 23)',
 'world': '25 infinite world', 'normal': '26 infinite directed normal', 'dag': '27 infinite directed dag',
 'wgpurenderer': '30 wgpu renderer', 'world3d': '32 World3dHost', 'scenes': '33 Scenes', 'shellhost': '34 ShellHost',
 'pluginruntime': '36 PluginRuntime', 'shellhelpers': '39 ShellHelpers', 'oshost': '42 os host', 'osts': '43 os.ts',
 'graphql': '45 repo GraphQL',
}
rows = []
for sid, label in LABEL.items():
    spec = SPECS[sid]
    O, files, Ts = load(spec)
    mid = 'pres' if sid == 'pres2' else sid
    mp = manifest_path(mid)
    man = json.load(open(mp)) if os.path.exists(mp) else {}
    before = set()
    for v in man.values():
        before |= set(v['names'])
    on = O.names()
    names = set()
    unbal = []
    for T in Ts:
        nm, errs = check_balance(T.text)
        names |= nm
        if errs:
            unbal.append(short(T.path))
    diff = [p for p, m in man.items() if os.path.exists(os.path.join(ROOT, p)) and sha(strip_markers(read(os.path.join(ROOT, p)))) != m['stripped']]
    t = touched(mid) if man else []
    rows.append((sid, label, len(on), len(before & on), len(names & on), len(Ts), len(t), len(unbal), len(diff), len(man)))
print('| split | audit row | orig names | tree names before | tree names after | tree files | files touched | unbalanced files | files whose code differs from snapshot |')
print('|---|---|---|---|---|---|---|---|---|')
for r in rows:
    print('| %s | %s | %d | %d | %d | %d | %d | %d | %d |' % (r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]))
