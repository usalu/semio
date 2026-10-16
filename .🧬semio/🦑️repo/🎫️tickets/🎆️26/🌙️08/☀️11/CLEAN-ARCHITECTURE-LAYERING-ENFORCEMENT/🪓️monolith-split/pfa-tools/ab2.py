import os, sys, shutil, json, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa, ab
plugin, tag = sys.argv[1], sys.argv[2]
root = os.path.join(pfa.SCR, 'ab', plugin, 'repo')
ab.assert_scratch(root)
ab.sh('git checkout -q -- . && git clean -fdq', cwd=root)
pdir, arts = ab.PLUGINS[plugin]
res = {}
res['head-only'] = ab.gate(root, arts[tag], tag + '-C0')
sp = importlib.import_module('specs.' + tag).SPEC
base_s = os.path.join(root, os.path.dirname(sp['file']))
base_r = os.path.join(pfa.REPO, os.path.dirname(sp['file']))
shutil.copyfile(os.path.join(base_r, '🦀️.rs'), os.path.join(base_s, '🦀️.rs'))
for c in sp['children']:
    os.makedirs(os.path.join(base_s, c['folder']), exist_ok=True)
    shutil.copyfile(os.path.join(base_r, c['folder'], '🦀️.rs'), os.path.join(base_s, c['folder'], '🦀️.rs'))
res['head-plus-split'] = ab.gate(root, arts[tag], tag + '-C1')
for k, v in res.items():
    print(k, v and {m: (d['base'], d['combined'], d['onlyCombined']) for m, d in v['layout'].items()})
json.dump(res, open(os.path.join(os.path.dirname(pfa.T), '🗑️generated', 'pfa-ab2-%s.json' % tag), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
