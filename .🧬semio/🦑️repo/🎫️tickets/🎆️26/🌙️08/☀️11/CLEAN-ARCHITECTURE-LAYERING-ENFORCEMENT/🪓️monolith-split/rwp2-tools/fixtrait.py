"""🧹️ Removes visibility qualifiers from fns inside trait impls (E0449) in the given files."""
import re, sys
from model import *
def fix(path):
    raw = open(path, encoding='utf8').read(); lines = raw.split('\n'); changed = 0
    for it in items(raw):
        if it['kind'] == 'impl' and re.match(r'impl(?:<[^>]*>)?\s+[\w:<>, &\']+\s+for\s+', it['name']):
            for i in range(it['head'], it['end'] + 1):
                m = re.match(r'^(\s+)pub(?:\([^)]*\))?\s+((?:const |async |unsafe )*fn )', lines[i])
                if m: lines[i] = m.group(1) + m.group(2) + lines[i][m.end():]; changed += 1
    if changed: open(path, 'w', encoding='utf8').write('\n'.join(lines))
    return changed
if __name__ == '__main__':
    import oracle2_closure as oc
    for c in sys.argv[1:]:
        for f in oc.closure(T + '/' + FOLDER[c] + '/🦀️.rs'):
            n = fix(f)
            if n: print(c, f.split('wgpu/')[-1], n)
