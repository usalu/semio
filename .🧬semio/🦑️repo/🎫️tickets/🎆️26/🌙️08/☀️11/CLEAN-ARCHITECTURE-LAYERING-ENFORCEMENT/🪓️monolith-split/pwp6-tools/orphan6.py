# orphan / placement scan of the instance family: every impl block must live in the crate that owns its type (inherent) or its type or trait (trait impl).
# The crate of a file is the crate whose root directory (or staging directory) contains it.
import sys; sys.path.insert(0, '.')
from lib6 import *
import defscan6 as D
from s0plan import CRATE_ROOT
from comps6 import COMPONENTS

CROOT = dict(CRATE_ROOT)
def crate_of_file(rel):
    if rel == '🧵️instance/🔌️plugin-app-impl/🦀️.rs': return 'instance-app'
    best = None
    for c, root in CROOT.items():
        if rel == root + '/🦀️.rs' or rel.startswith(root + '/📥️staging/'):
            if best is None or len(root) > len(CROOT[best]): best = c
    if best: return best
    for c, root in CROOT.items():
        if rel == root + '/🦀️.rs': return c
    return None

def type_crate(root=PLUG):
    """type name -> crate that defines it: components, the composer, and every struct/enum/trait defined in a family file"""
    out = {'VcsArtifactApp': 'instance-app'}
    for cf, (n, c, k) in COMPONENTS.items(): out[n] = c
    for f in D.family_files(root[:-len(PLUGP)] if root != PLUG else ROOT):
        c = crate_of_file(f)
        raw = open(root + f, encoding='utf-8').read(); M = mask(raw)
        for m in re.finditer(r'(?m)^(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|trait|union)\s+(\w+)', M):
            out.setdefault(m.group(1), c)
    return out

def scan(verbose=True, root=PLUG):
    tc = type_crate(root)
    rows = []; bad = []
    for f in D.family_files(root[:-len(PLUGP)] if root != PLUG else ROOT):
        c = crate_of_file(f)
        if c is None:
            if verbose: print('no crate for', f)
            continue
        raw = open(root + f, encoding='utf-8').read(); M = mask(raw)
        for m in re.finditer(r'(?m)^(?:#\[[^\n]*\]\s*)*(?:unsafe\s+)?impl\b[^{;]*', M):
            head = m.group(0)
            tgt, tr = pt.impl_target(re.sub(r'\s+', ' ', head.split('\n', 1)[-1] if head.lstrip().startswith('#[') else head) + ' {')
            if tgt is None: continue
            owner = tc.get(tgt)
            trait_owner = tc.get(tr) if tr else None
            line = raw.count('\n', 0, m.start()) + 1
            rows.append((f, line, c, tgt, tr, owner, trait_owner))
            if tr is None:
                if owner is not None and owner != c: bad.append((f, line, c, tgt, tr, owner))
            else:
                if owner is not None and owner != c and (trait_owner is None or trait_owner != c): bad.append((f, line, c, tgt, tr, owner))
    return rows, bad

if __name__ == '__main__':
    rows, bad = scan()
    comp_names = {v[0] for v in COMPONENTS.values()}
    app_inherent = [r for r in rows if r[3] == 'VcsArtifactApp' and r[4] is None]
    print('impl blocks', len(rows))
    byc = collections.Counter(r[2] for r in app_inherent)
    print('inherent impl blocks of VcsArtifactApp by crate (tier-1 must be 0):', dict(byc))
    print('misplaced impls:', len(bad))
    for b in bad[:40]: print('  ', b)
