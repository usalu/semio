"""🧷 Workspace-aware re-resolution of the audit graph: `workspace = true` dependencies are resolved through the `[workspace.dependencies]` table of the workspace root that owns the crate (inline and table form), which the audit parse read for the repository root only. Writes crates2.json next to crates.json. Run with `cwd` = the generated folder."""
import json, os, re, collections
from tree import ROOT
GEN = os.environ.get("GRAN_GEN") or os.getcwd()
g = json.load(open(GEN + "/crates.json"))
HDR = re.compile(r'^\s*\[\[?(.+?)\]\]?\s*(#.*)?$')


def sections(text):
    out = collections.OrderedDict()
    cur = ""
    for raw in text.splitlines():
        s = raw.strip()
        m = HDR.match(raw) if s.startswith("[") else None
        if m:
            cur = m.group(1).strip()
            out.setdefault(cur, [])
        else:
            out.setdefault(cur, []).append(raw)
    return out


def path_of(val):
    m = re.search(r'path\s*=\s*"([^"]+)"', val)
    return m.group(1) if m else None


def ws_table(root_dir):
    p = os.path.join(ROOT, root_dir, "Cargo.toml")
    out = {}
    if not os.path.isfile(p):
        return out
    s = sections(open(p, encoding="utf-8", errors="ignore").read())
    for h, lines in s.items():
        if h == "workspace.dependencies":
            for l in lines:
                m = re.match(r'^\s*"?([A-Za-z0-9_\-]+)"?\s*=\s*(.*)$', l)
                if m and path_of(m.group(2)):
                    out[m.group(1)] = os.path.normpath(os.path.join(root_dir, path_of(m.group(2))))
        elif h.startswith("workspace.dependencies."):
            nm = h.split(".", 2)[2].strip('"')
            pv = path_of(" ".join(lines))
            if pv:
                out[nm] = os.path.normpath(os.path.join(root_dir, pv))
    return out


def owning_root(d, sec):
    for l in sec.get("package", []):
        m = re.match(r'^\s*workspace\s*=\s*"([^"]+)"', l)
        if m:
            return os.path.normpath(os.path.join(d, m.group(1)))
    p = d
    while p and p != ".":
        p = os.path.dirname(p) or "."
        t = os.path.join(ROOT, p, "Cargo.toml")
        if os.path.isfile(t) and re.search(r'^\[workspace\]\s*$', open(t, encoding="utf-8", errors="ignore").read(), re.M):
            return p
        if p == ".":
            break
    return "."


WS = {}
KIND = {"dependencies": "normal", "build-dependencies": "build", "dev-dependencies": "dev"}
new_edges = 0
unresolved = collections.Counter()
for d, c in g["crates"].items():
    s = sections(open(os.path.join(ROOT, d, "Cargo.toml"), encoding="utf-8", errors="ignore").read())
    root = owning_root(d, s)
    if root not in WS:
        WS[root] = ws_table(root)
    deps = {"normal": [], "build": [], "dev": []}
    for h, lines in s.items():
        m1 = re.match(r'^(?:target\..+\.)?(dependencies|build-dependencies|dev-dependencies)$', h)
        m2 = re.match(r'^(?:target\..+\.)?(dependencies|build-dependencies|dev-dependencies)\.(.+)$', h)
        if m1:
            k = KIND[m1.group(1)]
            for l in lines:
                m = re.match(r'^\s*"?([A-Za-z0-9_\-]+)"?\s*(\.\s*(path|workspace)\s*)?=\s*(.*)$', l)
                if not m:
                    continue
                dn, sub, val = m.group(1), m.group(3), m.group(4)
                if sub == "path":
                    pv = re.match(r'^"([^"]+)"', val.strip())
                    if pv:
                        deps[k].append((dn, os.path.normpath(os.path.join(d, pv.group(1)))))
                elif path_of(val) and sub != "workspace":
                    deps[k].append((dn, os.path.normpath(os.path.join(d, path_of(val)))))
                elif sub == "workspace" or re.search(r'workspace\s*=\s*true', val):
                    t = WS[root].get(dn)
                    if t:
                        deps[k].append((dn, t))
                    else:
                        unresolved[dn] += 1
        elif m2:
            k = KIND[m2.group(1)]
            dn = m2.group(2).strip('"')
            joined = " ".join(lines)
            if path_of(joined):
                deps[k].append((dn, os.path.normpath(os.path.join(d, path_of(joined)))))
            elif re.search(r'workspace\s*=\s*true', joined):
                t = WS[root].get(dn)
                if t:
                    deps[k].append((dn, t))
                else:
                    unresolved[dn] += 1
    old = {(a, b) for k in ("normal", "build") for a, b in c["deps"][k] if not isinstance(b, list)}
    new = {(a, b) for k in ("normal", "build") for a, b in deps[k]}
    new_edges += len(new - old)
    c["deps"] = deps
g["unresolved_ws_deps"] = dict(unresolved)
json.dump(g, open(GEN + "/crates2.json", "w"), ensure_ascii=False)
tot = sum(1 for c in g["crates"].values() for k in ("normal", "build") for _, t in c["deps"][k] if t in g["crates"])
print("edges (normal+build, resolved to crates)", tot, "added by the workspace-aware pass", new_edges, "still unresolved", sum(unresolved.values()), "distinct", len(unresolved), "external examples", list(unresolved)[:8])
