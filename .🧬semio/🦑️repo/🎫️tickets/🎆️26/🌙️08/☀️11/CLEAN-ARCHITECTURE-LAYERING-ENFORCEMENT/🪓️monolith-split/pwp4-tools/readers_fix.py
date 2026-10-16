# 📖️ Re-points source-text readers of the two cut files from readFileSync to readModuleTreeSource (dry run unless --apply).
import re, os, sys, subprocess
from common import *
APPLY = '--apply' in sys.argv
RT = os.path.normpath(PLUG + '/⚛️reactor/🏃️runtime/🦀️.rs'); RC = os.path.normpath(PLUG + '/⚛️reactor/🦀️.rs')
HELPER = ROOT + '/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts'
files = subprocess.run(['git', '-c', 'core.quotepath=false', '-c', 'core.fsmonitor=false', 'grep', '-l', '-E', '🏃️runtime/🦀️\\.rs|🦀️\\.rs', '--', '*.ts', '*.tsx', ':!.🧬semio'], cwd=ROOT, capture_output=True, text=True).stdout.split('\n')
CALL = re.compile(r'readFileSync\(\s*new URL\(\s*"([^"]*)"\s*,\s*import\.meta\.url\s*\)\s*,\s*"utf8"\s*\)')
changed = []
for rel in files:
    if not rel or rel.startswith('.🧬semio'): continue
    p = ROOT + '/' + rel
    t = open(p, encoding='utf-8').read()
    def sub(m):
        tgt = os.path.normpath(os.path.join(os.path.dirname(p), m.group(1)))
        if tgt in (RT, RC): return f'readModuleTreeSource(new URL("{m.group(1)}", import.meta.url))'
        return m.group(0)
    t2 = CALL.sub(sub, t)
    if t2 == t: continue
    n = len(CALL.findall(t)) 
    if 'readModuleTreeSource' not in t:
        rel_imp = os.path.relpath(HELPER, os.path.dirname(p))
        imp = f'import {{ readModuleTreeSource }} from "{rel_imp}";\n'
        lines = t2.split('\n'); last = max((i for i, l in enumerate(lines) if l.startswith('import ')), default=-1)
        lines.insert(last + 1, imp.rstrip('\n')); t2 = '\n'.join(lines)
    changed.append((rel, sum(1 for a, b in zip(CALL.findall(t), CALL.findall(t)) )))
    if APPLY: open(p, 'w', encoding='utf-8').write(t2)
for c in changed: print(c[0][:150])
print(len(changed), 'files', 'applied' if APPLY else 'dry run')
