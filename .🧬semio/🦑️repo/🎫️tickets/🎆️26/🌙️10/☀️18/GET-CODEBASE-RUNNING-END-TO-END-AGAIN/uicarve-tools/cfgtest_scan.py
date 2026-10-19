"""🔎 uicarve cfgtest_scan: cfg(test)-gated items of lib crates that moved tests (companions) mention."""
import os, re, sys
sys.path.insert(0, 'lib')
import carve, rs, crategen
from carve import R, UI, rd
spec = carve.Spec(sys.argv[1])
lib_keys = [k for k, c in spec.crates.items() if not c.get('tests_only')]
comp = {k: c for k, c in spec.crates.items() if c.get('tests_only')}
# test text per companion
ctext = {}
for k, c in comp.items():
    t = ''
    for f in crategen.crate_files(spec, c):
        if f.endswith('.rs') and os.path.exists(f): t += rs.mask(rd(f))
    ctext[k] = t
for k in lib_keys:
    c = spec.crates[k]
    for f in crategen.crate_files(spec, c):
        if not f.endswith('.rs') or not os.path.exists(f) or '/🧪️tests/' in f: continue
        s = rd(f); lines = s.split('\n')
        for i, l in enumerate(lines):
            if re.match(r'\s*#\[cfg\(test\)\]\s*$', l):
                j = i + 1
                while j < len(lines) and (lines[j].strip().startswith('#[') or lines[j].strip().startswith('///') or not lines[j].strip()): j += 1
                m = re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+|async\s+|const\s+)*(fn|struct|enum|const|static|type|trait|mod|use|impl)\b\s*([A-Za-z_]\w*)?', lines[j])
                if not m or m.group(1) in ('use', 'mod'): continue
                name = m.group(2) or ''
                hits = [ck for ck, t in ctext.items() if name and re.search(r'\b%s\b' % re.escape(name), t)]
                print('%-16s %s:%d %s %s -> %s' % (k, os.path.relpath(f, UI)[-50:], i + 1, m.group(1), name, hits))
