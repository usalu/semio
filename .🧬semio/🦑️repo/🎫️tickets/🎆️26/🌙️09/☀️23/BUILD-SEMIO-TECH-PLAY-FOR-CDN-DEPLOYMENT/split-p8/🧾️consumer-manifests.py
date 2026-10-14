#!/usr/bin/env python3
"""🧾️ Gives every Cargo package whose sources now name a cut layer crate exactly the dependencies it names.

Reads the consumer map `✂️crate-cut.py apply` wrote (source file -> layers it references), finds each file's owning
package (the nearest `📦️packages/🦀️rust/Cargo.toml` whose `[lib]` tree mounts the file), adds one
`<layer crate> = { workspace = true }` row per layer the package's non-test sources name to `[dependencies]` (layers
only its tests name go to `[dev-dependencies]`), drops the monolith dependency when no source of the package names it
any more, and adds each new crate to the package's workspace root in that root's own declaration style.
Packages under a skipped root (generator-owned hub compositions) are left to their generator.

usage: 🧾️consumer-manifests.py <config.json> <consumers.json> plan|apply
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
CENSUS = os.path.join(HERE, "🕸️module-graph.py")
census = {}
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), census)
Crate = census["Crate"]

SKIP = ("🌎️hub/🧩️compositions/🗄️stdio/",)


def toml_get(text, table, key):
    section = re.search(r"(?ms)^\[" + re.escape(table) + r"\]\s*$(.*?)(?=^\[|\Z)", text)
    if not section:
        return None
    row = re.search(r'(?m)^' + re.escape(key) + r'\s*=\s*"([^"]*)"', section.group(1))
    return row.group(1) if row else None


crates = {}


def files_of(manifest):
    if manifest not in crates:
        text = open(manifest, encoding="utf-8").read()
        lib = toml_get(text, "lib", "path") or "src/lib.rs"
        root = os.path.normpath(os.path.join(os.path.dirname(manifest), lib))
        files = set(Crate(root).files) if os.path.exists(root) else set()
        for target in re.finditer(r'(?ms)^\[\[(?:test|bench|example)\]\].*?^path\s*=\s*"([^"]*)"', text):
            path = os.path.normpath(os.path.join(os.path.dirname(manifest), target.group(1)))
            if os.path.exists(path):
                files |= set(Crate(path).files)
        crates[manifest] = files
    return crates[manifest]


def owner(path):
    directory = os.path.dirname(path)
    while directory.startswith(REPO) and directory != REPO:
        manifest = os.path.join(directory, "📦️packages", "🦀️rust", "Cargo.toml")
        if os.path.exists(manifest) and path in files_of(manifest):
            return manifest
        directory = os.path.dirname(directory)
    return None


def is_test(path, files_root):
    return "🧪️tests" in os.path.relpath(path, REPO).split(os.sep) or "/🧪️tests/" in path


def add_row(text, section, name, after, path=None):
    match = re.search(r"(?ms)^\[" + re.escape(section) + r"\]\s*$(.*?)(?=^\[|\Z)", text)
    row = f'{name} = {{ path = "{path}" }}\n' if path else f"{name} = {{ workspace = true }}\n"
    if not match:
        return text.rstrip("\n") + f"\n\n[{section}]\n{row}"
    body = match.group(1)
    if re.search(r"(?m)^" + re.escape(name) + r"\s*=", body):
        return text
    anchor = re.search(r"(?m)^" + re.escape(after) + r"\s*=.*\n", body)
    at = match.start(1) + (anchor.end() if anchor else len(body.rstrip("\n")) + 1)
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
    root = os.path.normpath(os.path.join(os.path.dirname(manifest), pointer, "Cargo.toml"))
    return root


def add_workspace_rows(root, monolith, rows):
    text = open(root, encoding="utf-8").read()
    inline = re.search(r'(?m)^' + re.escape(monolith) + r'\s*=\s*\{\s*path\s*=\s*"([^"]+)"\s*\}\s*\n', text)
    table = re.search(r'(?m)^\[workspace\.dependencies\.' + re.escape(monolith) + r'\]\s*\npath\s*=\s*"([^"]+)"\s*\n', text)
    if not inline and not table:
        print(f"NOTE {os.path.relpath(root, REPO)} declares no {monolith}; rows appended")
    insert, at = [], (inline or table).end() if (inline or table) else len(text)
    for name, absolute in rows:
        relative = os.path.relpath(absolute, os.path.dirname(root))
        if re.search(r'(?m)^(?:' + re.escape(name) + r'\s*=|\[workspace\.dependencies\.' + re.escape(name) + r'\])', text):
            continue
        insert.append(f'{name} = {{ path = "{relative}" }}\n' if inline else f'\n[workspace.dependencies.{name}]\npath = "{relative}"\n')
    return text[:at] + "".join(insert) + text[at:]


def main():
    config_path, consumers_path, mode = sys.argv[1:4]
    config = json.load(open(config_path, encoding="utf-8"))
    consumers = json.load(open(consumers_path, encoding="utf-8"))
    artifact = os.path.join(REPO, config["artifact"])
    monolith_package = config["crate"].replace("_", "-")
    packages = {layer: spec["package"] for layer, spec in config["crates"].items()}
    dirs = {spec["package"]: os.path.join(artifact, spec["dir"]) for spec in config["crates"].values()}
    needs = {}
    for relative, used in consumers.items():
        path = os.path.join(REPO, relative)
        if relative.startswith(SKIP):
            continue
        manifest = owner(path)
        if manifest is None:
            print(f"UNOWNED {relative} {used}")
            continue
        entry = needs.setdefault(manifest, {"lib": set(), "dev": set()})
        for layer in used:
            if layer != config["aggregator"]:
                entry["dev" if is_test(path, manifest) else "lib"].add(layer)
    for manifest, entry in sorted(needs.items()):
        files = files_of(manifest)
        still = any(re.search(r"(?<![\w:])" + config["crate"] + r"(?!\w)\s*::", open(f, encoding="utf-8").read()) for f in files)
        own = re.search(r'(?m)^name\s*=\s*"([^"]+)"', open(manifest, encoding="utf-8").read()).group(1)
        entry["lib"] = {l for l in entry["lib"] if packages[l] != own}
        entry["dev"] = {l for l in entry["dev"] if packages[l] != own}
        lib, dev = sorted(entry["lib"]), sorted(entry["dev"] - entry["lib"])
        print(f"{os.path.relpath(manifest, REPO)} lib={lib} dev={dev} keep-monolith={still}")
        if mode != "apply":
            continue
        text = open(manifest, encoding="utf-8").read()
        path_style = re.search(r"(?m)^" + re.escape(monolith_package) + r"\s*=\s*\{\s*path\s*=", text) is not None
        section = "dependencies" if re.search(r"(?ms)^\[dependencies\]\s*$(?:(?!^\[).)*^" + re.escape(monolith_package) + r"\s*=", text) else "dev-dependencies"
        for layer in reversed(lib):
            text = add_row(text, "dependencies", packages[layer], monolith_package, os.path.relpath(dirs[packages[layer]], os.path.dirname(manifest)) if path_style else None)
        for layer in reversed(dev):
            text = add_row(text, "dev-dependencies", packages[layer], monolith_package, os.path.relpath(dirs[packages[layer]], os.path.dirname(manifest)) if path_style else None)
        if not still:
            text = drop_row(text, section, monolith_package)
        open(manifest, "w", encoding="utf-8").write(text)
        if path_style:
            continue
        root = workspace_root(manifest)
        rows = [(packages[layer], dirs[packages[layer]]) for layer in lib + dev]
        updated = add_workspace_rows(root, monolith_package, rows)
        open(root, "w", encoding="utf-8").write(updated)


main()
