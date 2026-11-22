import re, sys
import os; root = os.environ['PRUNE_TARGET']
out = open(sys.argv[1]).read()
L = open(root).read().split('\n')
edits = {}
for m in re.finditer(r'🦀️\.rs:(\d+):(\d+): warning: unused imports?: (.*)', out):
    if '/../../🦀️.rs' not in m.group(0) and 'rust/../../🦀️.rs' not in out[max(0, m.start() - 200):m.start() + 5]:
        pass
    line = int(m.group(1)); names = re.findall(r'`([^`]+)`', m.group(3))
    edits.setdefault(line, set()).update(names)
for line, names in sorted(edits.items(), reverse=True):
    src = L[line - 1]
    if not src.startswith('use '): continue
    mm = re.match(r'^use (.*?)\{(.*)\};$', src)
    if mm and not any(n in src and '::' in n for n in names):
        items = [i.strip() for i in re.split(r',\s*(?![^{]*\})', mm.group(2)) if i.strip()]
        keep = [i for i in items if i not in names and i.split(' as ')[-1].strip() not in names]
        if keep: L[line - 1] = 'use %s{%s};' % (mm.group(1), ', '.join(keep))
        else: L[line - 1] = None
    else:
        L[line - 1] = None
open(root, 'w').write('\n'.join(l for l in L if l is not None))
print(len(edits), 'lines edited')
