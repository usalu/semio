#!/usr/bin/env python3
"""🔗️ Synchronises first-party dependency rows of a crate family in consumer manifests after a crate cut (wave E4).

For every manifest under the given roots (or listed explicitly), the package's own source files are the files whose
nearest enclosing package directory is that manifest's package directory, plus every file the manifest's `[lib]` root
mounts (resolved with the canonical closure reader is not needed here: path-mounted roots are approximated by the
manifest's subtree and its `[lib] path` directory). A crate of the family referenced from a non-test file becomes a
`[dependencies]` row, one referenced only from tests a `[dev-dependencies]` row; family rows that are no longer
referenced are removed. Row style follows the manifest's existing family rows (workspace or relative path).

usage: 🔗️sync-crate-deps.py <family-spec.json> [--dry]
family-spec.json: {"crates": {"<package-name>": "<repo-relative package dir>"}, "manifests": [<repo-relative Cargo.toml>...]}
"""
import json, os, re, sys

REPO = "/Users/ueli/Documents/semio"


def lib_dir(manifest):
    s = open(manifest, encoding="utf-8").read()
    m = re.search(r'^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"', s, re.M | re.S)
    return os.path.normpath(os.path.join(os.path.dirname(manifest), os.path.dirname(m.group(1)))) if m else os.path.dirname(manifest)


def own_files(manifest, all_manifests):
    pkg = os.path.dirname(manifest); src = lib_dir(manifest)
    others = {os.path.dirname(m) for m in all_manifests if m != manifest}
    nested = [lib_dir(m) for m in all_manifests if m != manifest]
    out = []
    for base in {pkg, src}:
        for root, ds, fs in os.walk(base):
            if any(root.startswith(o) and o != base and len(o) > len(base) for o in nested + list(others)) and root != base:
                ds[:] = []; continue
            ds[:] = [d for d in ds if d not in ("target", "node_modules") and not any(os.path.join(root, d).startswith(n) and n.startswith(base) and n != base for n in nested)]
            out += [os.path.join(root, f) for f in fs if f.endswith(".rs")]
    return sorted(set(out))


def main():
    spec = json.load(open(sys.argv[1], encoding="utf-8")); dry = "--dry" in sys.argv
    crates = spec["crates"]; idents = {n.replace("-", "_"): n for n in crates}
    manifests = [os.path.join(REPO, m) for m in spec["manifests"]]
    all_manifests = manifests + [os.path.join(REPO, d, "Cargo.toml") for d in crates.values()]
    for man in manifests:
        own_name = re.search(r'^name\s*=\s*"([^"]+)"', open(man, encoding="utf-8").read(), re.M).group(1)
        use = {}
        for f in own_files(man, all_manifests):
            text = re.sub(r"/\*.*?\*/", "", re.sub(r"//[^\n]*", "", open(f, encoding="utf-8").read()), flags=re.S); test = "🧪️tests" in f
            for ident in set(re.findall(r"\b(semio_framework_3d(?:_brep_[a-z]+)?)\b", text)):
                name = idents.get(ident)
                if not name or name == own_name: continue
                use[name] = use.get(name, "test") if test else "prod"
        s = open(man, encoding="utf-8").read()
        rows = list(re.finditer(r'^(semio-framework-3d(?:-brep-[a-z]+)?)(\.workspace = true| = \{ workspace = true(?:, [^}]*)? \}| = \{ path = "[^"]*"(?:, [^}]*)? \})\n', s, re.M))
        style = "workspace" if any("workspace" in r.group(2) for r in rows) or not rows else "path"
        def row(name):
            if style == "workspace": return f"{name}.workspace = true\n"
            rel = os.path.relpath(os.path.join(REPO, crates[name]), os.path.dirname(man))
            return f'{name} = {{ path = "{rel}" }}\n'
        sec_re = r'^\[(dependencies|dev-dependencies)\]\n((?:(?!^\[).*\n?)*)'
        def section(kind):
            m = re.search(r'^\[' + kind + r'\]\n((?:(?!^\[).*\n?)*)', s, re.M)
            return m
        changed = []
        for kind, want in (("dependencies", "prod"), ("dev-dependencies", "test")):
            m = section(kind)
            if not m:
                if any(v == want for v in use.values()):
                    s = s.rstrip("\n") + f"\n\n[{kind}]\n"; m = section(kind)
                else: continue
            body = m.group(1); new = body
            for r in re.finditer(r'^(semio-framework-3d(?:-brep-[a-z]+)?)(\.workspace = true| = \{[^}]*\})\n', body, re.M):
                name = r.group(1)
                if name not in crates: continue
                keep = use.get(name) == want or (kind == "dev-dependencies" and "features" in r.group(2) and name in use)
                if not keep: new = new.replace(r.group(0), "", 1); changed.append(f"-{kind}:{name}")
            for name, how in sorted(use.items()):
                if how == want and not re.search(r'^' + re.escape(name) + r'(\.| =)', new, re.M):
                    new = row(name) + new; changed.append(f"+{kind}:{name}")
            s = s[:m.start(1)] + new + s[m.end(1):]
        if changed:
            print(os.path.relpath(man, REPO), " ".join(changed))
            if not dry: open(man, "w", encoding="utf-8").write(s)


if __name__ == "__main__":
    main()
