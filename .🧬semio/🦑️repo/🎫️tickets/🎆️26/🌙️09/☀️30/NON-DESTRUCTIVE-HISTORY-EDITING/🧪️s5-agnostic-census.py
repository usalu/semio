#!/usr/bin/env python3
"""🧮️ S5-AGNOSTIC wiring census: every tracked Rust file that calls `history_edit_acceptance_law!`, `composed_reload_law!` or
`composed_child_history_law!` → its plugin, artifact, crate name, feature list and law set, written to
`🗑️generated/s5-agnostic/census.json` (read by `🧪️s5-agnostic-table.py`) and printed as one TSV line per crate. Reads the git index
(`git grep`, read-only): a recursive disk grep over the plugin trees takes minutes. Usage: no arguments."""
import json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
G = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "s5-agnostic")
LAWS = (("G12", "history_edit_acceptance_law"), ("reload", "composed_reload_law"), ("child", "composed_child_history_law"))


def main():
    os.makedirs(G, exist_ok=True)
    listed = subprocess.run(["git", "-c", "core.quotepath=false", "grep", "-lE", "|".join(name + "!" for _, name in LAWS), "--", "*.rs"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    rows, outside = {}, []
    for path in sorted(filter(None, listed)):
        text = open(os.path.join(ROOT, path), encoding="utf-8").read()
        laws = [law for law, name in LAWS if re.search(r"^\s*(?:[a-z_:]+::)?" + name + r"!\s*\(", text, re.M)]
        match = re.match(r"✏️s/🔌️plugins/([^/]+)/🗿️artifacts/([^/]+)/", path)
        if not match:
            outside.append(path)
            continue
        plugin, artifact = match.groups()
        manifest = os.path.join("✏️s/🔌️plugins", plugin, "🗿️artifacts", artifact, "📦️packages/🦀️rust/Cargo.toml")
        crate, features = None, []
        if os.path.isfile(os.path.join(ROOT, manifest)):
            body = open(os.path.join(ROOT, manifest), encoding="utf-8").read()
            named = re.search(r'^name = "([^"]+)"', body, re.M)
            crate = named.group(1) if named else None
            section = re.search(r"^\[features\]\n(.*?)(?:^\[|\Z)", body, re.M | re.S)
            features = re.findall(r"^([a-z0-9_-]+)\s*=", section.group(1), re.M) if section else []
        row = rows.setdefault((plugin, artifact, crate), {"files": [], "laws": set(), "features": features, "manifest": manifest})
        row["files"].append(path)
        row["laws"].update(laws)
    census = [{"plugin": plugin, "artifact": artifact, "crate": crate, "laws": sorted(row["laws"]), "features": row["features"], "files": row["files"], "manifest": row["manifest"]} for (plugin, artifact, crate), row in sorted(rows.items())]
    json.dump(census, open(os.path.join(G, "census.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for row in census:
        print(f'{row["plugin"]}\t{row["artifact"]}\t{row["crate"]}\t{"+".join(row["laws"])}\t{len(row["files"])}\t{",".join(row["features"])}')
    print(f"{len(census)} crates, {sum(len(row['files']) for row in census)} files, {len({row['plugin'] for row in census})} plugins; outside the plugin trees: {outside}", file=sys.stderr)


if __name__ == "__main__":
    main()
