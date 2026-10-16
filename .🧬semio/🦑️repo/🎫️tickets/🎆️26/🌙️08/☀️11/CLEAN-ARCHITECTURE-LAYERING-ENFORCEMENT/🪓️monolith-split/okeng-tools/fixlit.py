# 🧷 Re-resolves the relative include_str!/include_bytes!/#[path] literals of staging files (items lifted out of a source file keep literals that were relative to the source directory). usage: fixlit.py <dump.json> dry|write
import json, os, re, sys
d = json.load(open(sys.argv[1])); write = sys.argv[2] == 'write'
LIT = re.compile(r'((?:include_str|include_bytes)!\(\s*"|#\[\s*path\s*=\s*")([^"]+)"')
n = 0
for st, src in d['fold'].items():
    if not os.path.exists(st): continue
    t = open(st, encoding='utf8').read()
    sd, dd = os.path.dirname(src), os.path.dirname(st)
    def rep(m):
        global n
        rel = m.group(2)
        if os.path.isabs(rel): return m.group(0)
        new = os.path.relpath(os.path.normpath(os.path.join(sd, rel)), dd)
        if new == rel: return m.group(0)
        n += 1
        return m.group(1) + new + '"'
    t2 = LIT.sub(rep, t)
    if t2 != t:
        print(('fix ' if write else 'would fix ') + st.split('modules/')[-1][:90])
        if write: open(st, 'w', encoding='utf8').write(t2)
print(n, 'literals')
