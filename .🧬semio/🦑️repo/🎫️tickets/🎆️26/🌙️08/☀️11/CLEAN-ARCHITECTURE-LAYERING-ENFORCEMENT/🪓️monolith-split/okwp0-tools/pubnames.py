# 📇 Public surface parity of the facade modules: the set of public names of every carved facade module in the ORIGINAL kernel root (pristine snapshot) against the live facade (hub + crates).
# usage: pubnames.py [module ...]
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import extcheck as X
from okrs import items
import okcrates as C

SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
K = C.K
KREL = C.K_REL
MODULES = ['os_dsl', 'os_pack', 'os_spr', 'list_delta', 'os_plugin_module_schema', 'native_catalog', 'os_semio', 'os_extension']

def all_names(idx, deps, seen=None, depth=0):
    """Names publicly nameable in idx, with glob re-exports expanded through local modules and extern crates. Returns (names, unresolved_globs)."""
    seen = seen if seen is not None else set()
    names = set(idx.names); unresolved = []
    for g in idx.globs:
        r = X.resolve_path(idx, g, X.depsof(idx, deps), depth)
        if r is None or r[0] != 'mod' or r[1] is None: unresolved.append('::'.join(g)); continue
        if id(r[1]) in seen: continue
        seen.add(id(r[1]))
        n, u = all_names(r[1], X.depsof(r[1], deps), seen, depth + 1)
        names |= n; unresolved += u
    return names, unresolved

def kernel_index(root_text, kdir):
    deps = X.deps_of(K)
    idx = X.Index(root_text, kdir, K)
    idx.deps = deps
    X.ROOT_OF[K] = idx
    return idx, deps

def module_names(idx, deps, name):
    e = idx.names.get(name)
    if e is None: return None, []
    if e[0] == 'mod': return all_names(e[1], deps) if e[1] is not None else (set(), ['unreadable'])
    if e[0] == 'use':
        out, unres = set(), []
        for ent in idx.multi.get(name) or [e]:
            r = X.resolve_path(idx, ent[1], deps)
            if r and r[0] == 'mod' and r[1] is not None:
                n, u = all_names(r[1], deps); out |= n; unres += u
            elif r and r[0] in ('item', 'macro'): out.add(name)
            else:
                out.add(name) if ent[1] and ent[1][-1] == name else None
                unres.append('unresolved ' + '::'.join(ent[1]))
        return out, unres
    if e[0] in ('item', 'macro'): return {name}, []
    return set(), ['?']

def hub_text():
    root = open(K + '/🦀️.rs', encoding='utf-8').read()
    def inc(m):
        p = os.path.normpath(os.path.join(K, m.group(1)))
        return open(p, encoding='utf-8').read() if os.path.exists(p) else ''
    return re.sub(r'^include!\("([^"]+)"\);[ \t]*$', inc, root, flags=re.M)

def main():
    mods = sys.argv[1:] or MODULES
    old_text = open(SCR + '/orig/' + KREL + '/🦀️.rs', encoding='utf-8').read()
    # the original hub relies on #[path] literals relative to the kernel package folder
    old_idx, deps = kernel_index(old_text, K)
    # old module trees mount real files: they exist unchanged only for files this task did not touch -> read them from the snapshot by pointing the index at the snapshot tree
    old_k = SCR + '/orig/' + KREL
    X._root_cache.clear()
    old_idx = X.Index(old_text, old_k, 'old'); old_idx.deps = deps
    X.ROOT_OF['old'] = old_idx
    new_k = K
    X._root_cache.clear()
    new_idx = X.Index(hub_text(), new_k, K); new_idx.deps = deps
    X.ROOT_OF[K] = new_idx
    fail = 0
    for m in mods:
        on, ou = module_names(old_idx, deps, m)
        nn, nu = module_names(new_idx, deps, m)
        if on is None or nn is None: print(f'{m}: missing (old {on is not None}, new {nn is not None})'); fail += 1; continue
        lost = sorted(on - nn); gained = sorted(nn - on)
        print(f'{m}: old {len(on)} new {len(nn)} lost {len(lost)} gained {len(gained)} unresolvedGlobs old {len(ou)} new {len(nu)}')
        if lost: print('   LOST', lost[:60]); fail += 1
        if gained: print('   gained', gained[:30])
        if nu: print('   new unresolved globs', nu[:8])
    return fail

if __name__ == '__main__':
    sys.exit(1 if main() else 0)
