#!/usr/bin/env python3
"""🧩️ S17 extension census: per extension, the committed descriptor projection (`🔣️.json`) vs the source declarations.

Reports per extension: plugin id, package id, own version vs crate version, `extends`, dependency pins (exact `=X.Y.Z`?),
apps (id, dialect kind), topic contributions, commands, artifact kinds, and every label/description en+de gap.
usage: python3 s17-extension-census.py [--json OUT]
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGINS = ROOT / "✏️s" / "🔌️plugins"


def localized_ok(label):
    if not isinstance(label, dict):
        return False
    native = label.get("native") or {}
    return bool((native.get("en") or "").strip()) and bool((native.get("de") or "").strip())


def crate_meta(ext_dir):
    cargo = (ext_dir / "📦️packages" / "🦀️rust" / "Cargo.toml").read_text()
    name = re.search(r'^name\s*=\s*"([^"]+)"', cargo, re.M).group(1)
    version = re.search(r'^version\s*=\s*"([^"]+)"', cargo, re.M)
    extends = re.search(r'^extends\s*=\s*"([^"]+)"', cargo, re.M)
    package = re.search(r'^package\s*=\s*"([^"]+)"', cargo, re.M)
    return name, version.group(1) if version else "workspace", extends.group(1) if extends else None, package.group(1) if package else None


def source_decls(ext_dir):
    text = (ext_dir / "🦀️.rs").read_text()
    bundles = re.findall(r'ExtensionBundle::new\(([^,]+),\s*([^,]+),\s*"([^"]+)"\)', text)
    deps = re.findall(r'depends_on\("([^"]+)",\s*([^)]*\)?)\)', text)
    describes = len(re.findall(r"\.action_describe\(", text))
    return bundles, deps, describes


def census(ext_dir):
    parent = ext_dir.parent.parent.name
    name, crate_version, extends, package = crate_meta(ext_dir)
    bundles, src_deps, describes = source_decls(ext_dir)
    row = {"parent": parent, "extension": ext_dir.name, "crate": name, "crateVersion": crate_version, "cargoExtends": extends, "cargoPackage": package, "sourceBundles": [b[2] for b in bundles], "sourceDeps": [f"{d[0]}:{d[1]}" for d in src_deps], "sourceDescribes": describes}
    projection = ext_dir / "🔣️.json"
    descriptor = ext_dir / "🛂️.descriptor.semio"
    row["descriptorMtime"] = descriptor.stat().st_mtime if descriptor.exists() else None
    if not projection.exists():
        row["error"] = "no 🔣️.json"
        return row
    d = json.loads(projection.read_text())
    m = d.get("manifest", {})
    row.update({"role": d.get("role"), "packageId": d.get("packageId"), "pluginId": m.get("pluginId"), "version": m.get("version"), "dependencies": m.get("dependencies", []), "execution": d.get("execution")})
    row["exactPins"] = all(str(dep.get("version", "")).startswith("=") for dep in m.get("dependencies", []))
    row["dependsOnParent"] = any(dep.get("pluginId") == parent_id(parent) for dep in m.get("dependencies", []))
    apps, gaps, actions = [], [], 0
    for app in m.get("apps", []):
        apps.append({"id": app.get("id"), "kind": (app.get("dialect") or {}).get("artifactKind")})
        if not localized_ok(app.get("label")):
            gaps.append(f"app {app.get('id')} label")
        for action in app.get("actions", []):
            actions += 1
            if not localized_ok(action.get("label")):
                gaps.append(f"action {action.get('id')} label")
            if not localized_ok(action.get("description")):
                gaps.append(f"action {action.get('id')} description")
        for wk in app.get("windowKinds", []):
            if not localized_ok(wk.get("label")):
                gaps.append(f"window {wk.get('id')} label")
    for command in m.get("commands", []):
        if not localized_ok(command.get("label")):
            gaps.append(f"command {command.get('id')} label")
        if not localized_ok(command.get("description")):
            gaps.append(f"command {command.get('id')} description")
    row.update({"apps": apps, "actions": actions, "commands": len(m.get("commands", [])), "artifactKinds": m.get("artifactKinds", []), "topics": [t.get("topic") for t in m.get("topicContributions", [])], "gaps": gaps})
    return row


def parent_id(parent_dir_name):
    return re.sub(r"^[^a-z]+", "", parent_dir_name)


def main():
    rows = [census(ext) for ext in sorted(PLUGINS.glob("*/🧩️extensions/*")) if (ext / "📦️packages" / "🦀️rust" / "Cargo.toml").exists()]
    for r in rows:
        pins = ",".join(f"{dep['pluginId']}{dep['version']}" for dep in r.get("dependencies", []))
        apps = ";".join(f"{a['id']}" for a in r.get("apps", []))
        print(f"{r['parent']}/{r['extension']} crate={r['crate']} v={r.get('version')} pkg={r.get('packageId')} role={r.get('role')} pins=[{pins}] exact={r.get('exactPins')} parentDep={r.get('dependsOnParent')} apps=[{apps}] actions={r.get('actions')} cmds={r.get('commands')} topics={r.get('topics')} kinds={r.get('artifactKinds')} srcDescribes={r['sourceDescribes']} gaps={len(r.get('gaps', []))} {r.get('gaps', [])[:6]}")
    if len(sys.argv) > 2 and sys.argv[1] == "--json":
        Path(sys.argv[2]).write_text(json.dumps(rows, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
