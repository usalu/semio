import os, sys, subprocess, shutil, importlib, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa, ab
mode = sys.argv[1]
root = ab.build('draw')
for tag in ('arena', 'owned'):
    sp = importlib.import_module('specs.' + tag).SPEC
    f = os.path.join(root, sp['file'])
    ab.assert_scratch(root)
    head = open(f, encoding='utf-8').read()
    orig = open(os.path.join(pfa.T, 'orig', sp['orig']), encoding='utf-8').read()
    assert head == orig, ('HEAD differs from orig', tag)
    open(f, 'w', encoding='utf-8').write(sp['pre'](head))
ab.sh('git add -A && git -c user.name=c4p4 -c user.email=c4p4@example.invalid -c commit.gpgsign=false -c core.hooksPath=/dev/null commit -q -m base-repaired', cwd=root)
ab.overlay_wt('draw', root)
art = ab.PLUGINS['draw'][1]['drawing']
if mode == 'B':
    ab.revert_mine('draw', root, sys.argv[2].split(','))
    for tag in ('arena', 'owned'):
        sp = importlib.import_module('specs.' + tag).SPEC
        f = os.path.join(root, sp['file'])
        src_text = open(f, encoding='utf-8').read()
        open(f, 'w', encoding='utf-8').write(sp['pre'](src_text))
sbx = os.path.join(pfa.SCR, 'ab', 'dump-' + mode)
for s in ('', '-base', '-combined'):
    shutil.rmtree(sbx + s, ignore_errors=True)
out = os.path.join(os.path.dirname(pfa.T), '🗑️generated', 'c4p4-abdump-%s.json' % mode)
for attempt in range(80):
    r = subprocess.run(['bun', os.path.join(pfa.T, 'abdump.ts'), root, art, sbx, out], capture_output=True, text=True, cwd=pfa.REPO)
    import re
    m = re.search(r'(?m)^error: Missing Rust source input: (.+)$', r.stderr + r.stdout)
    if m and 'ABDUMP done' not in r.stdout:
        rel = m.group(1); src = os.path.join(pfa.REPO, rel); dst = os.path.join(root, rel)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        if os.path.isfile(src):
            shutil.copyfile(src, dst)
        else:
            open(dst, 'w').write('{}')
        continue
    break
print(r.stdout[-300:], r.stderr[-800:])
