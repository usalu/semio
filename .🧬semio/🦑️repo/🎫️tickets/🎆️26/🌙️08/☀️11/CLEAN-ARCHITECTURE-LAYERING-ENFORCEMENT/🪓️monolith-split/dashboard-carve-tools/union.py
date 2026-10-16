"""🔬 Union differential for the dashboard carve: the producer crates simulated as modules of one crate, compared with the original monolith (analysis only, output to /dev/null)."""
import collections, os, pickle, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
import oracle as o
import spec

def mounts(path):
    """Files mounted by `#[path = ".."]` attributes of one file, in order."""
    text = open(path, encoding="utf-8").read()
    found = []
    for m in re.finditer(r'#\[path = "([^"]+)"\]\s*\n\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', text):
        found.append(os.path.normpath(os.path.join(os.path.dirname(path), m.group(1))))
    return found

def owned(root_file):
    seen, order, stack = set(), [], [root_file]
    while stack:
        path = stack.pop()
        if path in seen:
            continue
        assert os.path.exists(path), path
        seen.add(path)
        order.append(path)
        stack.extend(mounts(path))
    return order

def rewrite_file(path, key):
    text = open(path, encoding="utf-8").read()
    text = re.sub(r"(?<![\w:])(dashboard_\w+)(?=::)", r"crate::\1", text)
    text = re.sub(r"(?<![\w:])crate::(?!dashboard_)", f"crate::{key}::", text)
    text = text.replace("pub(crate)", f"pub(in crate::{key})")
    text = text.replace("pub(super)", f"pub(in crate::{key})")
    open(path, "w", encoding="utf-8").write(text)

def build_union(dashboard_dir, dest, extra_roots=None):
    o.make_sandbox(dest, dashboard_dir)
    base = os.path.join(dest, o.DASH)
    roots = {}
    for key, (node, ident, desc) in spec.CRATES.items():
        roots[key] = os.path.join(base, *node, n.RS)
    roots["facade"] = os.path.join(base, n.PKG, n.RUST, n.RS)
    files_of = {key: owned(root) for key, root in roots.items()}
    seen = {}
    for key, files in files_of.items():
        for f in files:
            assert f not in seen, (f, key, seen[f])
            seen[f] = key
    for key, files in files_of.items():
        for f in files:
            rewrite_file(f, key)
    union = os.path.join(dest, "union.rs")
    with open(union, "w", encoding="utf-8") as handle:
        for key, root in roots.items():
            handle.write(f'#[path = "{os.path.relpath(root, dest)}"]\npub mod {key};\n')
    o.patch_tree(dest)
    return union, files_of

def normalize(counter):
    out = collections.Counter()
    for (code, message, loc), count in counter.items():
        message = re.sub(r"crate::dashboard_\w+::", "crate::", message)
        message = re.sub(r"crate::(?:facade|dashboard_\w+)::", "crate::", message)
        out[(code, message, loc)] += count
    return out

if __name__ == "__main__":
    dashboard_dir, tag = sys.argv[1], sys.argv[2]
    dest = f"{o.SCR}/U-{tag}"
    union, files_of = build_union(dashboard_dir, dest)
    base = pickle.load(open(o.SCR + "/base.pkl", "rb"))
    for cfg, extra in {"native": [], "test": ["--test"]}.items():
        got, err = o.run(dest, union, "semio_dashboard_union", extra, cwd=dest)
        open(f"{o.SCR}/err-U-{tag}-{cfg}.txt", "w").write(err)
        got = normalize(got)
        want = normalize(base[cfg])
        only_new = got - want
        only_old = want - got
        print(f"[{cfg}] split {sum(got.values())} baseline {sum(want.values())} only-split {sum(only_new.values())} only-baseline {sum(only_old.values())}")
        for (code, message, loc), count in sorted(only_new.items())[:60]:
            print("  +", count, code, message[:150], loc.replace(o.DASH + "/", ""))
        for (code, message, loc), count in sorted(only_old.items())[:60]:
            print("  -", count, code, message[:150], loc.replace(o.DASH + "/", ""))
