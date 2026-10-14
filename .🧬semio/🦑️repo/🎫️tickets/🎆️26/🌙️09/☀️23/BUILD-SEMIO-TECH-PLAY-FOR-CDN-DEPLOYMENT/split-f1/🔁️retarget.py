#!/usr/bin/env python3
"""🔁️ Retargets every consumer of a cut crate to the owner crates it actually names, sources and manifests together.

Reads the consumer census of `🧭️consumer-files.ts`, rewrites old-crate paths in each consumer's selected closure with the
configured ordered rules (`{alias}` is the dependency key the consumer names the old crate by), then replaces the old
dependency row with one row per owner crate the rewritten sources name (`{alias}::` still left over means the
`"rest"` owner, kept under the consumer's own alias). Workspace-inherited rows get matching `[workspace.dependencies]`
rows in the owning workspace manifest. Rows nothing names any more are dropped.

usage: 🔁️retarget.py <config.json> plan|apply
config: {"consumers": <census json>, "old": <old package>, "rules": [[<regex with {alias}>, <replacement>]],
         "rest": <owner package for leftover alias paths>, "owners": {<package>: <repo-relative package dir>},
         "skip": [<consumer package>]}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))


def toml_rows(text):
    """🧾️ Yields (table, key, start, end) for every single-line `key = …` row and `[x.dependencies.key]` table."""
    table = None
    for m in re.finditer(r"(?m)^(\[[^\]\n]+\])[ \t]*$|^([A-Za-z0-9_-]+)[ \t]*=[ \t]*(.*)$", text):
        if m.group(1):
            table = m.group(1)
            sub = re.match(r"\[((?:workspace\.)?(?:dev-|build-)?dependencies)\.([A-Za-z0-9_-]+)\]", table)
            if sub:
                end = text.find("\n[", m.end())
                yield (sub.group(1), sub.group(2), m.start(), len(text) if end < 0 else end + 1, "table")
        else:
            yield (table, m.group(2), m.start(), m.end(), "row")


def rel(from_dir, owner_dir):
    return os.path.relpath(os.path.join(REPO, owner_dir), os.path.join(REPO, from_dir))


def main():
    config_path, mode = sys.argv[1], sys.argv[2]
    config = json.load(open(config_path, encoding="utf-8"))
    consumers = json.load(open(os.path.join(HERE, config["consumers"]), encoding="utf-8"))
    old, owners, rest = config["old"], config["owners"], config["rest"]
    idents = {package.replace("-", "_"): package for package in owners}
    workspace_needs = {}
    for row in consumers:
        if row["package"] in config.get("skip", []):
            continue
        aliases = [alias for alias, package in row["names"].items() if package == old]
        old_ident = old.replace("-", "_")
        used, changed, alias_rest = set(), {}, False
        for file in row["files"]:
            path = os.path.join(REPO, file)
            if not os.path.exists(path):
                continue
            text = open(path, encoding="utf-8").read()
            new = text
            for alias in aliases:
                for pattern, replacement in config["rules"]:
                    new = re.sub(pattern.replace("{alias}", re.escape(alias)), replacement, new)
            code = re.sub(r"//[^\n]*", "", new)
            for alias in aliases:
                if alias != old_ident and re.search(r"(?<![A-Za-z0-9_:])" + re.escape(alias) + r"::", code):
                    alias_rest = True
                    used.add(rest)
            for ident, package in idents.items():
                if re.search(r"(?<![A-Za-z0-9_])" + ident + r"::", code):
                    used.add(package)
            if new != text:
                changed[path] = new
        print(f"{row['package']}: files {len(changed)}, owners {sorted(used)}{' (alias kept for ' + rest + ')' if alias_rest else ''}")
        manifest_path = os.path.join(REPO, row["manifest"])
        manifest = open(manifest_path, encoding="utf-8").read()
        manifest_dir = os.path.dirname(row["manifest"])
        edits = []
        for table, key, start, end, kind in toml_rows(manifest):
            if key.replace("-", "_") not in aliases or table is None or "dependencies" not in table or table.startswith("[workspace") or table == "workspace.dependencies":
                continue
            line = manifest[start:end]
            inherited = "workspace = true" in line or "workspace=true" in line
            keep_alias = key.replace("-", "_") != old_ident and alias_rest
            out = []
            for package in sorted(used, key=lambda p: list(owners).index(p)):
                name = key if (package == rest and keep_alias) else package
                if inherited:
                    out.append(f"{name} = {{ workspace = true }}")
                    workspace_needs.setdefault(row["manifest"], set()).add((name, package))
                else:
                    spec = f'path = "{rel(manifest_dir, owners[package])}"' + (f', package = "{package}"' if name != package else "")
                    out.append(f"{name} = {{ {spec} }}")
            edits.append((start, end, "\n".join(out) + ("\n" if line.endswith("\n") and out else "")))
        for start, end, rep in sorted(edits, reverse=True):
            manifest = manifest[:start] + rep + manifest[end:]
            if not rep:
                manifest = re.sub(r"\n\n\n+", "\n\n", manifest)
        if mode == "apply":
            for path, new in changed.items():
                open(path, "w", encoding="utf-8").write(new)
            open(manifest_path, "w", encoding="utf-8").write(manifest)
    print("workspace rows needed:", {k: sorted(v) for k, v in workspace_needs.items()})
    json.dump({k: sorted(v) for k, v in workspace_needs.items()}, open(os.path.join(HERE, "🗑️generated", "workspace-needs.json"), "w"), ensure_ascii=False, indent=1)


main()
