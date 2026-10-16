# 📚 ident -> lib root of the first-party crates the resolver may enter (kernel dependencies, carved crates).
import os, sys, json, subprocess
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + '/../okwp0-tools')
import ns
import extcheck as X
R = ns.R
CACHE = HERE + '/../🗑️generated/okwp23-idents.json'

def load(refresh=False):
    if not refresh and os.path.exists(CACHE):
        data = json.load(open(CACHE))
    else:
        data = {}
        seen = set(); queue = [ns.K]
        while queue:
            d = queue.pop()
            if d in seen: continue
            seen.add(d)
            try: deps = X.deps_of(d)
            except Exception: continue
            for ident, dd in deps.items():
                if ident not in data:
                    try: data[ident] = X.lib_root(dd)
                    except Exception: continue
                if dd not in seen: queue.append(dd)
        json.dump(data, open(CACHE, 'w'), indent=1, ensure_ascii=False)
    for k, v in data.items(): ns.register_crate(k, v)
    return data

def add_crate(ident, root): ns.register_crate(ident, root)

if __name__ == '__main__':
    d = load(refresh=True)
    print(len(d), 'idents'); 
    for k in sorted(d)[:200]: print(k, d[k].replace(R + '/', '')[-90:])
