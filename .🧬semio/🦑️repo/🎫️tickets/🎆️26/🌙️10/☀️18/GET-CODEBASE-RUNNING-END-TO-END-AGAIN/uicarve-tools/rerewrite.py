"""🔁 uicarve rerewrite: apply the rule rewrite again to every file of the listed spec(s) (idempotent), per crate ctx."""
import os, sys, collections
sys.path.insert(0, 'lib')
import carve, crategen
from carve import rd, wr
cnt = collections.Counter()
for sp_path in sys.argv[1:]:
    spec = carve.Spec(sp_path)
    seen = {}
    for k, c in spec.crates.items():
        if c.get('mounts') or os.path.exists(spec.root_file(c)):
            for f in crategen.crate_files(spec, c):
                seen.setdefault(f, k)
    for f, k in seen.items():
        c = spec.crates[k]
        if not f.endswith('.rs') or not os.path.exists(f): continue
        if f == spec.root_file(c) and c.get('mounts'): continue
        old = rd(f); new = carve.rewrite_text(old, spec, k)
        if new != old:
            wr(f, new); cnt[k] += 1
print(dict(cnt))
