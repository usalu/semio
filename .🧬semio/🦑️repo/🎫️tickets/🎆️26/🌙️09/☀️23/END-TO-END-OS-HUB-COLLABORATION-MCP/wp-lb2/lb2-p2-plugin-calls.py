#!/usr/bin/env python3
"""🔑️ LB2 prepared patch p2b — one `assert_every_registered_app_reads_only_declared_arguments` call per plugin (p2's SDK
law), so every app every plugin registers is proven to read only the arguments its verbs declare. Plugins whose root
mounts `🧪️tests/🔬️surface` get one test appended there; process and forms (no surface tests yet) get the mount, the
file and the SDK's `artifact-app-testing` dev feature; stdio (its `plugin()` lives in `🔌️plugin/`) gets an integration
test `🧪️tests/🔑️declared-arguments` + `[[test]] declared_arguments`.
Usage: lb2-p2-plugin-calls.py [--dry-run | --write] [--root <repo-or-overlay root>]"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
PLUGINS = ROOT / "✏️s/🔌️plugins"
MARKER = "fn every_registered_app_reads_only_its_declared_arguments()"
DOC = """/// 🔑️ Every app this plugin registers reads only the arguments its verbs declare — SDK law
/// `artifact_app_laws::assert_every_registered_app_reads_only_declared_arguments` (a bridge reading an undeclared key
/// silently runs on its default in every shell and agent lane, because `effective_action_args` drops undeclared keys).
"""
TEST = DOC + """#[test]
fn every_registered_app_reads_only_its_declared_arguments() {
    assert!(semio_framework_plugin::artifact_app_laws::assert_every_registered_app_reads_only_declared_arguments({plugin}) > 0);
}
"""
MOUNT = """
//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests
"""
changed, problems, skipped = {}, [], []
for plugin_dir in sorted(path for path in PLUGINS.iterdir() if path.is_dir()):
    root = plugin_dir / "🦀️.rs"
    if not root.exists():
        continue
    source = root.read_text(encoding="utf-8")
    surface = plugin_dir / "🧪️tests/🔬️surface/🦀️.rs"
    if plugin_dir.name == "🗄️stdio":
        law = plugin_dir / "🧪️tests/🔑️declared-arguments/🦀️.rs"
        manifest = plugin_dir / "📦️packages/🦀️rust/Cargo.toml"
        text = "//! 🔑️ Every app the stdio plugin registers reads only the arguments its verbs declare.\n\n" + TEST.replace("{plugin}", "semio_s_plugin_stdio::plugin")
        if law.exists():
            skipped.append(str(law.relative_to(ROOT)))
        else:
            changed[law] = text
        cargo = manifest.read_text(encoding="utf-8")
        if 'name = "declared_arguments"' not in cargo:
            anchor = '[[test]]\nname = "shipped_fleet"\n'
            if cargo.count(anchor) != 1:
                problems.append(f"{manifest.relative_to(ROOT)}: shipped_fleet [[test]] anchor")
            else:
                changed[manifest] = cargo.replace(anchor, '[[test]]\nname = "declared_arguments"\npath = "../../🧪️tests/🔑️declared-arguments/🦀️.rs"\n\n' + anchor, 1)
        continue
    if source.count("\npub fn plugin() -> Result<Plugin<") != 1:
        problems.append(f"{root.relative_to(ROOT)}: no single `pub fn plugin()`")
        continue
    call = TEST.replace("{plugin}", "super::plugin")
    if surface.exists():
        current = surface.read_text(encoding="utf-8")
        if MARKER in current:
            skipped.append(str(surface.relative_to(ROOT)))
            continue
        if 'path = "🧪️tests/🔬️surface/🦀️.rs"' not in source:
            problems.append(f"{root.relative_to(ROOT)}: surface tests exist but are not mounted")
            continue
        changed[surface] = current.rstrip("\n") + "\n\n" + call
        continue
    changed[surface] = call
    changed[root] = source.rstrip("\n") + "\n" + MOUNT
    manifest = plugin_dir / "📦️packages/🦀️rust/Cargo.toml"
    cargo = manifest.read_text(encoding="utf-8")
    normal = next((line for line in cargo.splitlines() if line.startswith("semio-framework-plugin = ")), None)
    if normal is None or cargo.count("[dev-dependencies]\n") != 1:
        problems.append(f"{manifest.relative_to(ROOT)}: SDK dependency or [dev-dependencies] anchor")
        continue
    if "artifact-app-testing" not in cargo:
        dev = normal.replace('features = ["component-guest"]', 'features = ["artifact-app-testing"]')
        changed[manifest] = cargo.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + dev + "\n", 1)
for problem in problems:
    print("PROBLEM", problem)
print(f"root={ROOT} files={len(changed)} skipped={len(skipped)} problems={len(problems)} write={WRITE}")
for path in changed:
    print("  ", path.relative_to(ROOT))
if WRITE and not problems:
    for path, content in changed.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")
    print("written")
sys.exit(1 if problems else 0)
