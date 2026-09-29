#!/usr/bin/env python3
"""📚️ EX1: first cut of a plugin's `semio.example-catalog.v1` fixture from the sources — every editor of the plugin's
editor-catalog list, its app id (`<artifact_kind>@<standard>/<subset>#editor` from its `DIALECT`) and the example ids its
`examples()` publishes (module paths resolved through the crate's `#[path]` module tree; `examples()` absent → the leaves of its
subset's `📚️examples/`). `document` starts as `distinct`; the resolver law prints the exact row wherever the sources disagree.

usage: python3 ex1-fixture.py stdio > fixtures/stdio.json
"""
import json, os, re, sys

REPO = "/Users/ueli/Documents/semio"
PLUGIN = sys.argv[1]
PLUGIN_DIR = {"stdio": "🗄️stdio"}[PLUGIN]
ROOT = f"{REPO}/✏️s/🔌️plugins/{PLUGIN_DIR}"


def crate_roots():
    roots = {}
    for dp, dns, fns in os.walk(ROOT):
        dns[:] = [d for d in dns if d not in ("target", "node_modules", "🧪️tests")]
        if "Cargo.toml" in fns and dp.endswith("📦️packages/🦀️rust"):
            toml = open(os.path.join(dp, "Cargo.toml"), encoding="utf-8").read()
            name = re.search(r'^name = "([^"]+)"', toml, re.M)
            lib = re.search(r'\[lib\][^\[]*?path = "([^"]+)"', toml, re.S)
            if name and lib:
                roots[name.group(1).replace("-", "_")] = os.path.normpath(os.path.join(dp, lib.group(1)))
    return roots


def module_tree(root_file):
    """🌳️ Module path (tuple) → file, for inline `mod x {` blocks and `#[path = "…"] mod x;` leaves reachable from the root."""
    tree = {(): root_file}
    todo = [((), root_file)]
    while todo:
        prefix, file = todo.pop()
        text = open(file, encoding="utf-8").read()
        stack, depth, pending = [], 0, None
        for line in text.split("\n"):
            code = re.sub(r'"(?:\\.|[^"\\])*"', '""', line.split("//")[0]) if not line.lstrip().startswith("#[path") else ""
            path = re.match(r'\s*#\[path = "([^"]+)"\]', line)
            if path:
                pending = path.group(1)
                continue
            leaf = re.match(r"\s*(?:pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*;", line)
            if leaf and pending:
                module = prefix + tuple(name for name, _ in stack) + (leaf.group(1),)
                target = os.path.normpath(os.path.join(os.path.dirname(file), pending))
                if os.path.isfile(target) and module not in tree:
                    tree[module] = target
                    todo.append((module, target))
                pending = None
                continue
            pending = None if not line.strip().startswith("#[") else pending
            block = re.match(r"\s*(?:pub(?:\([a-z]+\))?\s+)?mod\s+(\w+)\s*\{", code)
            if block:
                stack.append((block.group(1), depth))
                tree.setdefault(prefix + tuple(name for name, _ in stack), file)
            for char in code:
                if char == "{":
                    depth += 1
                elif char == "}":
                    depth -= 1
                    while stack and depth <= stack[-1][1]:
                        stack.pop()
    return tree


def main():
    law = open(f"{ROOT}/🧪️tests/✏️editor-catalog/🦀️.rs", encoding="utf-8").read()
    editors = re.findall(r"\(\w+, ([\w:]+), [\w:]+\)", law.split("editor_catalog_laws! {")[1])
    roots = crate_roots()
    trees = {}
    apps = []
    for editor in editors:
        crate, *path, name = editor.split("::")
        tree = trees.setdefault(crate, module_tree(roots[crate]))
        files = {file for file in tree.values() if f"impl ArtifactEditor for {name} " in open(file, encoding="utf-8").read()}
        if len(files) != 1:
            print(f"{editor}: {len(files)} impl files", file=sys.stderr)
            continue
        file = files.pop()
        text = open(file, encoding="utf-8").read()
        impl = text[text.index(f"impl ArtifactEditor for {name} "):]
        constant = re.search(r"const DIALECT: Dialect = ([\w:]+);", impl).group(1).split("::")[-1]
        dialect = re.search(rf"{constant}: Dialect = Dialect \{{ artifact_kind: \"?([\w.-]+)\"?, standard: StandardId\(\"([^\"]+)\"\), subset: (SubsetId::ANY|SubsetId\(\"([^\"]+)\"\)) \}}", open_all(tree))
        if not dialect:
            print(f"{editor}: dialect {constant} not found", file=sys.stderr)
            continue
        kind = dialect.group(1)
        if "." not in kind:
            kind = re.search(rf'const {kind}: &str = "([^"]+)"', open_all(tree)).group(1)
        app = f"{kind}@{dialect.group(2)}/{dialect.group(4) or '*'}#editor"
        body = re.search(r"fn examples\(\) -> Vec<[\w:]*ExampleSource> \{\s*(.*?)\s*\}\n", impl[:20000], re.S)
        ids = []
        if body:
            for module in re.findall(r"([\w:]+)::source\(\)", body.group(1)):
                segments = tuple(module.split("::"))
                segments = segments[1:] if segments[0] == "crate" else ("RELATIVE",) + segments
                leaf = tree.get(segments)
                ident = leaf and re.search(r'pub const ID: &str = "([^"]+)"', open(leaf, encoding="utf-8").read())
                ids.append(ident.group(1) if ident else "demo" if module.endswith("::demo") else f"UNRESOLVED:{module}")
        else:
            subset = os.path.dirname(os.path.dirname(file))
            for leaf in sorted(os.listdir(os.path.join(subset, "📚️examples"))) if os.path.isdir(os.path.join(subset, "📚️examples")) else []:
                source = os.path.join(subset, "📚️examples", leaf, "🦀️.rs")
                ident = os.path.isfile(source) and re.search(r'pub const ID: &str = "([^"]+)"', open(source, encoding="utf-8").read())
                if ident:
                    ids.append(ident.group(1))
        apps.append({"app": app, "examples": [{"id": ident, "document": "distinct"} for ident in ids]})
    json.dump({"schema": "semio.example-catalog.v1", "plugin": PLUGIN, "apps": apps}, sys.stdout, ensure_ascii=False, indent=2)
    print()


def open_all(tree):
    return "\n".join(open(file, encoding="utf-8").read() for file in sorted(set(tree.values())))


if __name__ == "__main__":
    main()
