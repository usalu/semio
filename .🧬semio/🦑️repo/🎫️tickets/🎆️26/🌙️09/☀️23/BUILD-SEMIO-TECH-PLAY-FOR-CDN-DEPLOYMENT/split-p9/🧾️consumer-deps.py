#!/usr/bin/env python3
"""🧾️ Gives every consumer package exactly the family crates its sources now name (P9, after `🏗️rehost.py apply`).

Reads `🗑️generated/rehost-<crate>.json` (`consumers`: file -> crate idents) of one family, maps each consumer file to the
package that compiles it (a rehosted family crate by its file list, otherwise the nearest `📦️packages/🦀️rust` package
above the file or a package manifest in the file's own directory chain), adds `<crate> = { workspace = true }` (a
dev-dependency when the file is a test target or a `🧪️tests` file), adds the workspace row in the package's workspace
root, and drops the monolith dependency from packages whose sources no longer name it.

usage: 🧾️consumer-deps.py <config.json> plan|apply
"""
import glob, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))


def toml_load(path):
    out = subprocess.run(["bun", "-e", "console.log(JSON.stringify(Bun.TOML.parse(require('fs').readFileSync(process.argv[1], 'utf8'))))", path], capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


def registry():
    """📇️ Every family crate of every P9 config: package name -> package dir."""
    found = {}
    for path in glob.glob(os.path.join(HERE, "🧾️configs", "*.json")):
        cfg = json.load(open(path, encoding="utf-8"))
        for c in cfg.get("crates", {}).values():
            found[c["package"]] = os.path.join(REPO, c["owner"], "📦️packages", "🦀️rust")
    return found


def family_files():
    """🗂️ Every rehosted file -> the manifest of the family crate that compiles it."""
    owner = {}
    for path in glob.glob(os.path.join(HERE, "🗑️generated", "rehost-*.json")):
        summary = json.load(open(path, encoding="utf-8"))
        cfgs = [json.load(open(p, encoding="utf-8")) for p in glob.glob(os.path.join(HERE, "🧾️configs", "*.json"))]
        cfg = next((c for c in cfgs if "rehost-" + c.get("crate", "") + ".json" == os.path.basename(path)), None)
        if cfg is None:
            continue
        for key, files in summary["files"].items():
            if key not in cfg["crates"]:
                continue
            manifest = os.path.join(REPO, cfg["crates"][key]["owner"], "📦️packages", "🦀️rust", "Cargo.toml")
            for f in files:
                owner[os.path.normpath(os.path.join(REPO, f))] = manifest
    return owner


def package_of(file, owners):
    if file in owners:
        return owners[file]
    d = os.path.dirname(file)
    while d.startswith(REPO) and d != REPO:
        cand = os.path.join(d, "📦️packages", "🦀️rust", "Cargo.toml")
        if os.path.exists(cand) and "[package]" in open(cand, encoding="utf-8").read():
            return cand
        cand = os.path.join(d, "Cargo.toml")
        if os.path.exists(cand) and "[package]" in open(cand, encoding="utf-8").read():
            return cand
        d = os.path.dirname(d)
    return None


def workspace_root(manifest):
    data = toml_load(manifest)
    ws = data.get("package", {}).get("workspace")
    if ws:
        return os.path.normpath(os.path.join(os.path.dirname(manifest), ws, "Cargo.toml"))
    d = os.path.dirname(os.path.dirname(manifest))
    while d.startswith(REPO):
        cand = os.path.join(d, "Cargo.toml")
        if os.path.exists(cand) and "[workspace]" in open(cand, encoding="utf-8").read():
            return cand
        d = os.path.dirname(d)
    return manifest


def add_dep(text, table, name):
    if re.search(r"^" + re.escape(name) + r"\s*=", text, re.M):
        return text
    m = re.search(r"^\[" + re.escape(table) + r"\]\n", text, re.M)
    line = f"{name} = {{ workspace = true }}\n"
    if not m:
        return text.rstrip() + f"\n\n[{table}]\n" + line
    return text[:m.end()] + line + text[m.end():]


def main():
    cfg = json.load(open(sys.argv[1], encoding="utf-8"))
    mode = sys.argv[2]
    summary = json.load(open(os.path.join(HERE, "🗑️generated", "rehost-" + cfg["crate"] + ".json"), encoding="utf-8"))
    reg, owners = registry(), family_files()
    monolith = cfg["crate"].replace("_", "-")
    by_manifest = {}
    for rel_file, idents in summary.get("consumers", {}).items():
        file = os.path.normpath(os.path.join(REPO, rel_file))
        manifest = package_of(file, owners)
        if manifest is None:
            print("NO PACKAGE", rel_file)
            continue
        test = "🧪️tests" in rel_file and file not in owners or re.search(r"\[\[test\]\][^\[]*" + re.escape(os.path.basename(os.path.dirname(file))), open(manifest, encoding="utf-8").read()) is not None
        entry = by_manifest.setdefault(manifest, {"normal": set(), "dev": set()})
        for ident in idents:
            entry["dev" if test else "normal"].add(ident.replace("_", "-"))
    edits = {}
    for manifest, need in sorted(by_manifest.items()):
        text = edits.get(manifest) or open(manifest, encoding="utf-8").read()
        own = toml_load(manifest).get("package", {}).get("name")
        for name in sorted(need["normal"]):
            if name != own:
                text = add_dep(text, "dependencies", name)
        for name in sorted(need["dev"] - need["normal"]):
            if name != own and not re.search(r"^" + re.escape(name) + r"\s*=", text, re.M):
                text = add_dep(text, "dev-dependencies", name)
        edits[manifest] = text
        root = workspace_root(manifest)
        rtext = edits.get(root) or open(root, encoding="utf-8").read()
        for name in sorted(need["normal"] | need["dev"]):
            if name in reg and f"[workspace.dependencies.{name}]" not in rtext and not re.search(r"^" + re.escape(name) + r"\s*=\s*\{\s*path", rtext, re.M):
                row = f"\n[workspace.dependencies.{name}]\npath = \"{os.path.relpath(reg[name], os.path.dirname(root))}\"\n"
                if re.search(r"^\[workspace\.dependencies\]\n", rtext, re.M):
                    rtext = re.sub(r"^\[workspace\.dependencies\]\n", f"[workspace.dependencies]\n{name} = {{ path = \"{os.path.relpath(reg[name], os.path.dirname(root))}\" }}\n", rtext, count=1, flags=re.M)
                elif "\n[workspace.lints" in rtext:
                    rtext = rtext.replace("\n[workspace.lints", row + "\n[workspace.lints", 1)
                else:
                    rtext = rtext.rstrip() + "\n" + row
        edits[root] = rtext
    for manifest, text in edits.items():
        print("==", os.path.relpath(manifest, REPO))
        if mode == "apply":
            open(manifest, "w", encoding="utf-8").write(text)


if __name__ == "__main__":
    main()
