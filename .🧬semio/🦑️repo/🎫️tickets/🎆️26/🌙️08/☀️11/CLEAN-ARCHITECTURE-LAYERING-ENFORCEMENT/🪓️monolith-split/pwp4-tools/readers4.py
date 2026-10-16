# 📖️ Source-text readers of the two cut files: which string literals a reader asserts that no longer live in the file it reads.
import re, subprocess, os, sys
from common import *
OLD = {'runtime': open(SCR + '/precut/⚛️reactor/🏃️runtime/🦀️.rs', encoding='utf-8').read(), 'root': open(SCR + '/precut/⚛️reactor/🦀️.rs', encoding='utf-8').read()}
def tree_text(rel):
    out = subprocess.run(['bun', SCR + '/treeprint.ts', rel], capture_output=True, text=True)
    return out.stdout
NEW_RESID = {'runtime': open(PLUG + '/⚛️reactor/🏃️runtime/🦀️.rs', encoding='utf-8').read(), 'root': open(PLUG + '/⚛️reactor/🦀️.rs', encoding='utf-8').read()}
NEW_TREE = {'runtime': tree_text('⚛️reactor/🏃️runtime/🦀️.rs'), 'root': tree_text('⚛️reactor/🦀️.rs')}
LIFTMAC = ''.join(open(PLUG + '/' + f, encoding='utf-8').read() for f in ['🧩️extension/🛬️lifted/🏃️runtime/🦀️.rs', '🏗️builder/🔌️program/🛬️lifted/🏃️runtime/🦀️.rs', '📣️macros/⚛️wp4-reactor/🦀️.rs'])
files = subprocess.run(['git', '-c', 'core.quotepath=false', '-c', 'core.fsmonitor=false', 'grep', '-l', '-E', '🏃️runtime/🦀️\\.rs|⚛️reactor/🦀️\\.rs', '--', '*.ts', '*.tsx', '*.mjs', ':!.🧬semio'], cwd=ROOT, capture_output=True, text=True).stdout.split('\n')
STR = re.compile(r'"((?:[^"\\\n]|\\.){14,})"|`((?:[^`\\]|\\.){14,})`|\'((?:[^\'\\\n]|\\.){14,})\'')
for f in files:
    if not f: continue
    t = open(ROOT + '/' + f, encoding='utf-8').read()
    kinds = []
    if re.search(r'🏃️runtime/🦀️\.rs', t): kinds.append('runtime')
    if re.search(r'⚛️reactor/🦀️\.rs', t): kinds.append('root')
    tree = 'readModuleTreeSource' in t or 'listModuleTree' in t
    res = {}
    for k in kinds:
        bad_resid = 0; bad_tree = 0; bad_all = 0; tot = 0
        for m in STR.finditer(t):
            s = m.group(1) or m.group(2) or m.group(3)
            s = s.replace('\\n', '\n').replace('\\"', '"').replace("\\'", "'").replace('\\\\', '\\')
            if s not in OLD[k]: continue
            tot += 1
            if s not in NEW_RESID[k]: bad_resid += 1
            if s not in NEW_TREE[k]:
                bad_tree += 1
                if '-v' in sys.argv: print('    missing in tree:', repr(s[:100]), '| in lifts/macros:', s in LIFTMAC)
            if s not in NEW_TREE[k] and s not in LIFTMAC: bad_all += 1
        res[k] = (tot, bad_resid, bad_tree, bad_all)
    print(f.replace('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/', 'PLUG/')[:110], 'tree-helper' if tree else 'direct', res)
