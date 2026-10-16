"""📤️ Root-visible names of every renderer crate (items, modules and `pub use` re-exports, glob re-exports followed), for the import synthesis of the test bins."""
import collections, json, os, re
from lib import *

MOUNT = re.compile(r"((?:#\[[^\]]*\]\s*)*)(pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
_cache = {}
DEFINED = {}


def child_mounts(path):
    """[(module name, pub?, file, cfg text)] of the file modules declared in `path`."""
    text = read(path)
    clean = blank(text)
    here = os.path.dirname(path)
    out = []
    for m in MOUNT.finditer(clean):
        attrs = text[m.start(1):m.end(1)]
        pm = PATH_ATTR.search(attrs)
        if not pm:
            continue
        cfg = " ".join(re.findall(r"#\[\s*cfg\([^\n]*\)\s*\]", attrs))
        out.append((m.group(3), bool(m.group(2)), os.path.normpath(os.path.join(here, pm.group(1))), cfg))
    return out


def module_exports(path, crates, stack=()):
    """name -> set of cfg strings visible at the top of the module file `path`."""
    key = path
    if key in _cache:
        return _cache[key]
    if path in stack or not os.path.exists(path):
        return {}
    out = collections.defaultdict(set)
    defined = set()
    text = read(path)
    mounts = {n: (p, f, c) for n, p, f, c in child_mounts(path)}
    for it in itm.table(text):
        if it["kind"] in ("impl", "use"):
            continue
        if not it["vis"].startswith("pub") and not it.get("thread_local"):
            continue
        out[it["name"]].add(" ".join(a for a in it["attrs"] if a.startswith("#[cfg(")))
        defined.add(it["name"])
    for name, (is_pub, f, cfg) in mounts.items():
        if is_pub:
            out[name].add(cfg)
    for cfg, vis, p, local, glob in uses(text, top_only=True):
        if not vis.startswith("pub"):
            continue
        if glob:
            if len(p) == 1 and p[0] in mounts:
                sub = module_exports(mounts[p[0]][1], crates, stack + (path,))
                for n, cs in sub.items():
                    out[n] |= {(cfg + " " + c).strip() for c in cs}
                    if n in DEFINED.get(mounts[p[0]][1], ()):
                        defined.add(n)
            elif p and p[0].startswith(PFX):
                suffix = p[0][len(PFX):].replace("_", "-")
                if suffix in crates and len(p) == 1:
                    for n, cs in crate_exports(suffix, crates, stack + (path,)).items():
                        out[n] |= {(cfg + " " + c).strip() for c in cs}
        elif local:
            out[local].add(cfg)
    res = {k: v for k, v in out.items()}
    _cache[key] = res
    DEFINED[path] = defined
    return res


def crate_exports(suffix, crates=None, stack=()):
    crates = crates or crate_table()
    root = T + "/" + crates[suffix] + "/🦀️.rs"
    return module_exports(root, crates, stack)


def build_index():
    crates = crate_table()
    idx = collections.defaultdict(dict)
    defs = collections.defaultdict(set)
    for suffix in crates:
        exp = crate_exports(suffix, crates)
        for name, cfgs in exp.items():
            idx[name][suffix] = sorted(cfgs)
        root = T + "/" + crates[suffix] + "/🦀️.rs"
        for name in DEFINED.get(root, ()):
            defs[name].add(suffix)
    return idx, defs


if __name__ == "__main__":
    idx = build_index()
    print(len(idx), "names")
    for n in ["take_component_surface_close_request", "publish_component_surface_close_terminal", "ComponentSurfaceCloseOwner", "ShellState", "collect_fixture_actions", "engine_surface_token"]:
        print(n, idx.get(n))
