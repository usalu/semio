#!/usr/bin/env python3
"""🧾️ Gives every Cargo package under the consumer roots exactly the family crates its sources name.

For each `.rs` file that names one of the family crate idents, the owning package is the nearest ancestor directory
with a `📦️packages/🦀️rust/Cargo.toml` (or a `Cargo.toml` with a `[package]` table). Files under a `🧪️tests` segment
count as test sources. The package gets `<crate> = { workspace = true }` in `[dependencies]` (or `[dev-dependencies]`
when only tests name it) and its workspace root gets the dependency row in the root's own declaration style.

usage: 🧾️consumers.py <family.json> plan|apply
family: {"crates": {"package-name": "repo-relative package dir"}, "roots": [repo-relative roots]}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ns = {}
exec(compile(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], "module-graph", "exec"), ns)
Crate = ns["Crate"]


_trees = {}


def package_files(manifest):
    """🌳️ Every source file the package compiles (lib tree plus `[[test]]`/`[[bin]]` trees), from its `#[path]` module graph."""
    if manifest not in _trees:
        text = open(manifest, encoding="utf-8").read()
        base = os.path.dirname(manifest)
        roots = re.findall(r'(?ms)^\[(?:lib|\[test\]|\[bin\]|\[bench\]|\[example\])\][^\[]*?^path\s*=\s*"([^"]+)"', text)
        files = set()
        for root in roots:
            path = os.path.normpath(os.path.join(base, root))
            if os.path.exists(path):
                try:
                    files |= set(Crate(path).files)
                except Exception as error:
                    print(f"WARN tree {os.path.relpath(path, REPO)}: {error}")
        _trees[manifest] = files
    return _trees[manifest]


def owner(path):
    d = os.path.dirname(path)
    first = None
    while d.startswith(REPO) and d != REPO:
        for cand in (os.path.join(d, "📦️packages", "🦀️rust", "Cargo.toml"), os.path.join(d, "Cargo.toml")):
            if os.path.exists(cand) and re.search(r"(?m)^\[package\]", open(cand, encoding="utf-8").read()):
                first = first or cand
                if os.path.normpath(path) in package_files(cand):
                    return cand
        d = os.path.dirname(d)
    return None if first else None


def workspace_root(manifest):
    text = open(manifest, encoding="utf-8").read()
    m = re.search(r'(?m)^workspace\s*=\s*"([^"]+)"', text)
    if m:
        return os.path.normpath(os.path.join(os.path.dirname(manifest), m.group(1), "Cargo.toml"))
    if re.search(r"(?m)^\[workspace\]", text):
        return manifest
    d = os.path.dirname(os.path.dirname(manifest))
    while d.startswith(REPO):
        cand = os.path.join(d, "Cargo.toml")
        if os.path.exists(cand) and re.search(r"(?m)^\[workspace\]", open(cand, encoding="utf-8").read()):
            return cand
        d = os.path.dirname(d)
    return None


def has_dep(text, table, name):
    sec = re.search(r"(?ms)^\[" + re.escape(table) + r"\]\s*$(.*?)(?=^\[|\Z)", text)
    if sec and re.search(r"(?m)^" + re.escape(name) + r"\s*=", sec.group(1)):
        return True
    return bool(re.search(r"(?m)^\[" + re.escape(table) + r"\." + re.escape(name) + r"\]", text))


def add_dep(text, table, row):
    if re.search(r"(?m)^\[" + re.escape(table) + r"\]\s*$", text):
        return re.sub(r"(?m)^(\[" + re.escape(table) + r"\]\s*\n)", lambda m: m.group(1) + row + "\n", text, count=1)
    return text.rstrip("\n") + f"\n\n[{table}]\n{row}\n"


def main():
    family = json.load(open(sys.argv[1], encoding="utf-8"))
    mode = sys.argv[2]
    idents = {name.replace("-", "_"): name for name in family["crates"]}
    need = {}
    for root in family["roots"]:
        for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, root)):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated") and not d.startswith(".")]
            for name in filenames:
                if not name.endswith(".rs"):
                    continue
                path = os.path.join(dirpath, name)
                text = open(path, encoding="utf-8").read()
                found = {idents[i] for i in idents if re.search(r"(?<![\w])" + i + r"\b", text)}
                if not found:
                    continue
                manifest = owner(path)
                if manifest is None:
                    print(f"NO-OWNER {os.path.relpath(path, REPO)}")
                    continue
                own = open(manifest, encoding="utf-8").read()
                own_name = re.search(r'(?m)^name\s*=\s*"([^"]+)"', own).group(1)
                is_test = "🧪️tests" in os.path.relpath(path, os.path.dirname(os.path.dirname(os.path.dirname(manifest))))
                for crate in found:
                    if crate == own_name:
                        continue
                    entry = need.setdefault(manifest, {})
                    entry[crate] = entry.get(crate, True) and is_test
    edits = {}
    for manifest, crates in sorted(need.items()):
        text = edits.get(manifest) or open(manifest, encoding="utf-8").read()
        root = workspace_root(manifest)
        for crate, test_only in sorted(crates.items()):
            table = "dev-dependencies" if test_only else "dependencies"
            if has_dep(text, "dependencies", crate) or (test_only and has_dep(text, "dev-dependencies", crate)):
                continue
            uses_ws = root is not None and root != manifest
            row = f"{crate} = {{ workspace = true }}" if uses_ws else f'{crate} = {{ path = "{os.path.relpath(os.path.join(REPO, family["crates"][crate]), os.path.dirname(manifest))}" }}'
            text = add_dep(text, table, row)
            print(f"{os.path.relpath(manifest, REPO)}: [{table}] {crate}")
            if uses_ws:
                rtext = edits.get(root) or open(root, encoding="utf-8").read()
                if not (has_dep(rtext, "workspace.dependencies", crate)):
                    rel = os.path.relpath(os.path.join(REPO, family["crates"][crate]), os.path.dirname(root))
                    if re.search(r"(?m)^\[workspace\.dependencies\]\s*$", rtext):
                        rtext = add_dep(rtext, "workspace.dependencies", f'{crate} = {{ path = "{rel}" }}')
                    else:
                        block = f'[workspace.dependencies.{crate}]\npath = "{rel}"\n\n'
                        m = re.search(r"(?m)^\[workspace\.lints", rtext) or re.search(r"(?m)^\[profile", rtext)
                        rtext = rtext[:m.start()] + block + rtext[m.start():] if m else rtext.rstrip("\n") + "\n\n" + block
                    edits[root] = rtext
                    print(f"  workspace {os.path.relpath(root, REPO)} += {crate}")
        edits[manifest] = text
    if mode == "apply":
        for path, text in edits.items():
            open(path, "w", encoding="utf-8").write(text)
        print(f"applied {len(edits)}")


main()
