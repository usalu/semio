# 🔗 Union differential for the WP2/WP3 carve: A = pristine snapshot + the WP0 files + the pending outputs of carve.py (not yet written to the repo), carved crates as modules of the facade; B = pristine snapshot.
# usage: union2.py <tag> <dump.json> [configs] [--seed name]
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE)
import okoracle as O, okreal
import unionlib as union
R = O.ROOT
OVERLAY = {}
_orig_copy = O.copy_files
def copy_files(dest, files, src_root=R):
    _orig_copy(dest, files, src_root)
    if src_root == R:
        for p, t in OVERLAY.items():
            d = os.path.join(dest, os.path.relpath(p, R)); O.inside(d, dest)
            os.makedirs(os.path.dirname(d), exist_ok=True); open(d, 'w', encoding='utf8').write(t)
O.copy_files = copy_files
_orig_touched = union.touched
union.touched = lambda: sorted(set(_orig_touched() + [os.path.relpath(p, R) for p in OVERLAY]))

def main():
    tag = sys.argv[1]; dump = json.load(open(sys.argv[2]))
    OVERLAY.update(dump['out'])
    for k, rf in dump['roots'].items():
        if k in dump['batch']: union.NODES[k] = os.path.relpath(os.path.dirname(rf), R + '/' + union.M_REL)
    configs = sys.argv[3].split(',') if len(sys.argv) > 3 and not sys.argv[3].startswith('--') else ['lib', 'test', 'wasm', 'wasm-test', 'wasip2']
    seed = sys.argv[sys.argv.index('--seed') + 1] if '--seed' in sys.argv else None
    union.FOLD.update({os.path.relpath(k, R): os.path.relpath(v, R) for k, v in dump.get('fold', {}).items()})
    union.run(tag, configs, list(okreal.REAL) + dump['batch'], seed)

if __name__ == '__main__': main()
