import os, sys, re
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import oksql
import carve, rewrite as RW
_ru = RW.Rewriter.rewrite_use
def ru(self, tree_src, ctx, cur_crate, relative_ok, file_label, *a, **k):
    r = _ru(self, tree_src, ctx, cur_crate, relative_ok, file_label, *a, **k)
    if 'actor' in file_label and tree_src.strip().startswith('super'):
        print('REWRITE_USE', file_label[-50:], repr(tree_src.strip()), cur_crate, relative_ok, '->', type(r).__name__, (r if isinstance(r, (str, list)) else '')[:3] if not isinstance(r, str) else r[:200], flush=True)
    return r
RW.Rewriter.rewrite_use = ru
b = carve.solve(['os-store-sync', 'os-store-sync-native', 'os-store-sync-wasm'], auto=True)
pl = b.plan
for f, c in sorted(pl.file_crate.items()):
    print('FILE', c, f[-70:])
