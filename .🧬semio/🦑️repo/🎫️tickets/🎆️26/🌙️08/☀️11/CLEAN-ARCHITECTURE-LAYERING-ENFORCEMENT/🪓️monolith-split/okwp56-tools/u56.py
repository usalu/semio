# 🔗 Scoped union differential for OS-KERNEL-WP5-6: sandbox A = pristine snapshot + every live touched file (+ an in-memory overlay of pending outputs), carved crates as modules of the facade; optionally sandbox B = pristine. Writes only below the scratchpad.
# usage: u56.py <tag> <overlay.json|-> <configs> [--crates k1,k2] [--with-b]
import json, os, sys, glob, collections, re, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE)
import okoracle as O, okreal, unionlib as union, okcrates as C
R = O.ROOT
SCRATCH = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/okwp56'
O.GEN = HERE + '/../🗑️generated'
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
extra_touched = []
union.touched = lambda: sorted(set(_orig_touched() + [os.path.relpath(p, R) for p in OVERLAY] + extra_touched))

def git_touched():
    import subprocess
    out = subprocess.run(['git', '-c', 'core.quotepath=false', 'status', '--short', '--untracked-files=all', '--', O.K_REL, union.M_REL], capture_output=True, text=True, cwd=R).stdout
    files = []
    for line in out.split('\n'):
        if len(line) < 4: continue
        p = line[3:].strip().strip('"')
        if ' -> ' in p: p = p.split(' -> ')[1]
        if p.endswith(('.rs', '.json', '.sql', '.txt', '.md', '.toml')) and os.path.exists(os.path.join(R, p)): files.append(p)
    return files

def carved_crates():
    model = C.build()[0]
    keys = []
    for f in sorted(glob.glob(HERE + '/../🗑️generated/okwp23-*.crates')):
        keys += open(f).read().split()
    keys += [k for k in open(HERE + '/crates56.txt').read().split()] if os.path.exists(HERE + '/crates56.txt') else []
    seen = []
    for k in keys:
        if k not in seen: seen.append(k)
    return seen, model

def main():
    tag = sys.argv[1]; ov = sys.argv[2]
    configs = sys.argv[3].split(',')
    crates, model = carved_crates()
    if '--crates' in sys.argv:
        crates = [c for c in sys.argv[sys.argv.index('--crates') + 1].split(',') if c]
    if ov != '-':
        d = json.load(open(ov))
        OVERLAY.update(d['out'])
        union.FOLD.update({os.path.relpath(k, R): os.path.relpath(v, R) for k, v in d.get('fold', {}).items()})
    extra_touched.extend(git_touched())
    for k in crates:
        if k in union.NODES or k in okreal.REAL: continue
        union.NODES[k] = os.path.relpath(model[k]['node'], union.M_REL)
    shutil.rmtree(SCRATCH + '/ok-union-' + tag, ignore_errors=True)
    O.SCR = SCRATCH if False else O.SCR
    base = O.SCR + '/ok-union-' + tag
    union.FOLD.update({})
    allc = list(okreal.REAL) + [k for k in crates if k not in okreal.REAL]
    A = base + '/A'
    shutil.rmtree(base, ignore_errors=True); os.makedirs(base)
    present = union.build(A, allc)
    print('union crates:', present, flush=True)
    os.makedirs(A + '/outdir', exist_ok=True)
    for i, a in enumerate(sys.argv):
        if a == '--seed':
            rel, old, new = sys.argv[i + 1].split('::::')
            fp = os.path.join(A, rel); O.inside(fp, A)
            t = open(fp, encoding='utf8').read()
            assert t.count(old) >= 1, ('seed anchor missing', rel, old[:60])
            open(fp, 'w', encoding='utf8').write(t.replace(old, new, 1)); print('seeded', rel[-50:], repr(old[:40]), '->', repr(new[:40]), flush=True)
    O.neutralize(A)
    out = {}
    for name in configs:
        ea, ta = O.run(A, 'A', name, tag)
        a, ae, aex = O.parse_keyed(ea, A, lambda f: f)
        out[name] = dict(aInternal=sum(a.values()), aExternal=sum(ae.values()), seconds=round(ta))
        print(f'== {name}: A internal={sum(a.values())} external={sum(ae.values())} sec={round(ta)}', flush=True)
    json.dump(out, open(f'{O.GEN}/okwp56-union-{tag}-summary.json', 'w'), indent=1)

if __name__ == '__main__': main()
