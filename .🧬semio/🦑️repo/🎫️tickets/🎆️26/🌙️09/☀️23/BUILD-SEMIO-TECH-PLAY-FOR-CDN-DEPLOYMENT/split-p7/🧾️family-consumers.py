#!/usr/bin/env python3
"""🧾️ Gives every external package whose sources name family crates of a cut exactly those crates (split lane P7).

Reads the consumer map of `🗺️plan.json` (source file -> family crates it names), finds each file's owning package (the
nearest `📦️packages/🦀️rust/Cargo.toml`, or a plain `Cargo.toml`, whose lib or `[[test]]` module tree mounts the file),
adds `<crate> = { workspace = true }` rows (`[dev-dependencies]` when only test code names it), drops the monolith row
when no source of the package names the monolith any more, and gives the package's workspace root a row for every new
crate in that root's own declaration style.

usage: 🧾️family-consumers.py <config.json> plan|apply
"""
import collections, importlib.util, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
spec = importlib.util.spec_from_file_location("family_cut", os.path.join(HERE, "✂️family-cut.py"))
fc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fc)
trees = {}


def toml_get(text, table, key):
    section = re.search(r"(?ms)^\[" + re.escape(table) + r"\]\s*$(.*?)(?=^\[|\Z)", text)
    if not section:
        return None
    row = re.search(r'(?m)^' + re.escape(key) + r'\s*=\s*"([^"]*)"', section.group(1))
    return row.group(1) if row else None


def files_of(manifest):
    if manifest not in trees:
        text = open(manifest, encoding="utf-8").read()
        roots = [os.path.normpath(os.path.join(os.path.dirname(manifest), toml_get(text, "lib", "path") or "src/lib.rs"))]
        roots += [os.path.normpath(os.path.join(os.path.dirname(manifest), m.group(1))) for m in re.finditer(r'(?ms)^\[\[(?:test|bench|example|bin)\]\].*?^path\s*=\s*"([^"]*)"', text)]
        files = set()
        for root in roots:
            if os.path.exists(root):
                try:
                    files |= set(fc.Tree(root).files)
                except SystemExit:
                    files.add(root)
        trees[manifest] = files
    return trees[manifest]


def owner(path):
    directory = os.path.dirname(path)
    while directory.startswith(REPO) and directory != REPO:
        for manifest in (os.path.join(directory, "📦️packages", "🦀️rust", "Cargo.toml"), os.path.join(directory, "Cargo.toml")):
            if os.path.exists(manifest) and "[package]" in open(manifest, encoding="utf-8").read() and path in files_of(manifest):
                return manifest
        directory = os.path.dirname(directory)
    return None


def add_row(text, section, name, path=None):
    match = re.search(r"(?ms)^\[" + re.escape(section) + r"\]\s*$(.*?)(?=^\[|\Z)", text)
    row = f'{name} = {{ path = "{path}" }}\n' if path else f"{name} = {{ workspace = true }}\n"
    if not match:
        return text.rstrip("\n") + f"\n\n[{section}]\n{row}"
    if re.search(r"(?m)^" + re.escape(name) + r"\s*=", match.group(1)):
        return text
    at = match.start(1) + len(match.group(1).rstrip("\n")) + 1
    return text[:at] + row + text[at:]


def drop_row(text, section, name):
    match = re.search(r"(?ms)^\[" + re.escape(section) + r"\]\s*$(.*?)(?=^\[|\Z)", text)
    if not match:
        return text
    row = re.search(r"(?m)^" + re.escape(name) + r"\s*=.*\n", match.group(1))
    if not row:
        return text
    return text[:match.start(1) + row.start()] + text[match.start(1) + row.end():]


def workspace_root(manifest):
    text = open(manifest, encoding="utf-8").read()
    pointer = toml_get(text, "package", "workspace")
    if pointer is None:
        return manifest if re.search(r"(?m)^\[workspace\]", text) else None
    return os.path.normpath(os.path.join(os.path.dirname(manifest), pointer, "Cargo.toml"))


def add_workspace_rows(root, monolith, rows):
    text = open(root, encoding="utf-8").read()
    inline = re.search(r'(?m)^' + re.escape(monolith) + r'\s*=\s*\{\s*path\s*=\s*"[^"]+"\s*\}\s*\n', text)
    table = re.search(r'(?m)^\[workspace\.dependencies\.' + re.escape(monolith) + r'\]\s*\npath\s*=\s*"[^"]+"\s*\n', text)
    if not inline and not table:
        print(f"SKIP {os.path.relpath(root, REPO)} declares no {monolith}")
        return text
    insert, at = [], (inline or table).end()
    for name, absolute in rows:
        relative = os.path.relpath(absolute, os.path.dirname(root))
        if re.search(r'(?m)^(?:' + re.escape(name) + r'\s*=|\[workspace\.dependencies\.' + re.escape(name) + r'\])', text):
            continue
        insert.append(f'{name} = {{ path = "{relative}" }}\n' if inline else f'\n[workspace.dependencies.{name}]\npath = "{relative}"\n')
    return text[:at] + "".join(insert) + text[at:]


def main():
    config_path, mode = sys.argv[1], sys.argv[2]
    config = json.load(open(config_path, encoding="utf-8"))
    plan = json.load(open(os.path.join(os.path.dirname(os.path.abspath(config_path)), "🗺️plan.json"), encoding="utf-8"))
    artifact = os.path.join(REPO, config["artifact"])
    monolith = config["crate"].replace("_", "-")
    dirs = {name: os.path.join(artifact, info["owner"], "📦️packages/🦀️rust") if name != monolith else os.path.join(artifact, "📦️packages/🦀️rust") for name, info in plan["family"].items()}
    needs = {}
    for relative, used in plan["consumers"].items():
        path = os.path.join(REPO, relative)
        manifest = owner(path)
        if manifest is None:
            print(f"UNOWNED {relative} {used}")
            continue
        if manifest.startswith(artifact + os.sep) and os.path.dirname(manifest) in [os.path.join(d) for d in dirs.values()]:
            continue
        entry = needs.setdefault(manifest, {"lib": set(), "dev": set()})
        is_test = "🧪️tests" in relative.split("/")
        for crate in used:
            entry["dev" if is_test else "lib"].add(crate)
    for manifest, entry in sorted(needs.items()):
        files = files_of(manifest)
        still = any(re.search(r"(?<![\w:])" + config["crate"] + r"\b", open(f, encoding="utf-8").read()) for f in files if os.path.exists(f))
        lib, dev = sorted(entry["lib"]), sorted(entry["dev"] - entry["lib"])
        print(f"{os.path.relpath(manifest, REPO)} lib={lib} dev={dev} keep-monolith={still}")
        if mode != "apply":
            continue
        text = open(manifest, encoding="utf-8").read()
        inline_monolith = re.search(r'(?m)^' + re.escape(monolith) + r'\s*=\s*\{\s*path\s*=', text)
        section = "dependencies" if re.search(r"(?ms)^\[dependencies\]\s*$(?:(?!^\[).)*^" + re.escape(monolith) + r"\s*=", text) else "dev-dependencies"
        for crate in lib:
            if crate != monolith:
                text = add_row(text, "dependencies", crate, os.path.relpath(dirs[crate], os.path.dirname(manifest)) if inline_monolith else None)
        for crate in dev:
            if crate != monolith:
                text = add_row(text, "dev-dependencies", crate, os.path.relpath(dirs[crate], os.path.dirname(manifest)) if inline_monolith else None)
        if not still:
            text = drop_row(text, section, monolith)
        open(manifest, "w", encoding="utf-8").write(text)
        root = workspace_root(manifest)
        if root and not inline_monolith:
            updated = add_workspace_rows(root, monolith, [(crate, dirs[crate]) for crate in lib + dev if crate != monolith])
            open(root, "w", encoding="utf-8").write(updated)


main()
