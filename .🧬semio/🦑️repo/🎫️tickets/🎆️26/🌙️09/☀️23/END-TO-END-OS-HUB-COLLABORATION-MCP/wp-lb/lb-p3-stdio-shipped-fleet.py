#!/usr/bin/env python3
"""🧯️ LB-P3 (prepared for the coordinator, 27 12:0x — NOT applied: rule 30): the chain's wasm-dev rustc of
`semio-s-plugin-stdio` reached an 85 GB footprint (single CGU) vs ~1.8 GB in session 12. Root: the Codex stdio rollout
(`🔌️plugin/🦀️.rs` 01:15, `📦️packages/🦀️rust/Cargo.toml` 01:19, auto-committed 11:30 as `6b8089dcb21`) deleted the
`full-app-catalog` split, so the SHIPPED component (`plugin-root` → `component-app-assembly`) now closes `StdioApps`
over 176 apps (88 subsets × editor/viewer) instead of the 18 of the nine text/data subsets — every app monomorphises
its own `VcsArtifactApp<EditorApp<E>/ViewerApp<V>, Members>` runtime, retained tool-job factories and (new in the
same rollout) one `SnapshotEditToolJobFactory<E>` per editor: ~10× the code in one crate, exactly what the removed
comment warned about ("the full fleet blows `wasm-component-ld`'s 1 000 000-function ceiling").
This patch restores the shipped/library split of `50c97b20513` (09-21) on the current code: `component-app-assembly`
= the 7 text/data artifact crates (18 apps), `full-app-catalog` = every subset (the library fleet native hosts and
tests use); the rollout's full enum + `register_apps` stay as the `full-app-catalog` fleet; the 79 playground rows of
subsets the component does not ship are dropped (their launch rows go with the next launch generation).
Usage: lb-p3-stdio-shipped-fleet.py --dry-run | --write"""
import re
import subprocess
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
CARGO = "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml"
PLUGIN = "✏️s/🔌️plugins/🗄️stdio/🔌️plugin/🦀️.rs"
BASE = "50c97b20513"
write = "--write" in sys.argv
if not write and "--dry-run" not in sys.argv:
    sys.exit(__doc__)
old_cargo = subprocess.run(["git", "show", f"{BASE}:{CARGO}"], cwd=REPO, capture_output=True, text=True, check=True).stdout
old_plugin = subprocess.run(["git", "show", f"{BASE}:{PLUGIN}"], cwd=REPO, capture_output=True, text=True, check=True).stdout
cargo = (REPO / CARGO).read_text(encoding="utf-8")
plugin = (REPO / PLUGIN).read_text(encoding="utf-8")
problems = []

features_old = re.search(r"^# 🧩️ What the SHIPPED wasm component assembles.*?^full-artifact-catalog = ", old_cargo, re.M | re.S)
features_new = re.search(r"^# 🧩️ One complete editor/viewer fleet.*?^full-artifact-catalog = ", cargo, re.M | re.S)
if features_old is None or features_new is None:
    problems.append(f"feature block old={features_old is not None} new={features_new is not None}")
else:
    cargo = cargo[: features_new.start()] + features_old.group(0) + cargo[features_new.end():]

shipped = {m.group(1) for m in re.finditer(r'\[\[package\.metadata\.semio\.playground\]\]\nvariant = "([^"]+)"', old_cargo)}
rows = list(re.finditer(r'\[\[package\.metadata\.semio\.playground\]\]\nvariant = "([^"]+)"\n(?:(?!\[\[)[^\n]*\n)*?(?=\[\[|\n\[|\Z)', cargo))
dropped = [m for m in rows if m.group(1) not in shipped]
if len(rows) != 88 or len(dropped) != 79:
    problems.append(f"playground rows {len(rows)} / to drop {len(dropped)} (expected 88 / 79)")
else:
    for m in reversed(dropped):
        end = m.end() + 1 if cargo[m.end(): m.end() + 1] == "\n" else m.end()
        cargo = cargo[: m.start()] + cargo[end:]
header_old = re.search(r"^# 🎮️ The stdio playgrounds:.*?\n(?=\[\[package\.metadata\.semio\.playground\]\])", old_cargo, re.M | re.S)
header_new = re.search(r"^# 🎮️ Each editor subset owns a playground;.*?\n(?=\[\[package\.metadata\.semio\.playground\]\])", cargo, re.M | re.S)
if header_old is None or header_new is None:
    problems.append(f"playground header old={header_old is not None} new={header_new is not None}")
else:
    cargo = cargo[: header_new.start()] + header_old.group(0) + cargo[header_new.end():]

SHIPPED = re.compile(r"semio_s_artifact_stdio_(csv|tsv|txt|json|xml|md|html)::")
full_enum = "// 🗃️ Complete runtime app fleet shared by native hosts and the shipped component.\ndyn_enum_close! {\n"
full_register = "/// 🗃️ Registers all 88 editor/viewer subsets for every supported host.\nfn register_apps("
if plugin.count(full_enum) != 1 or plugin.count(full_register) != 1:
    problems.append(f"assembly anchors enum={plugin.count(full_enum)} register={plugin.count(full_register)}")
else:
    enum_start = plugin.index(full_enum)
    enum_end = plugin.index("\n}\n", enum_start) + 3
    variants = [line for line in plugin[enum_start:enum_end].split("\n") if SHIPPED.search(line)]
    register_start = plugin.index(full_register)
    register_end = plugin.index("\n}\n", register_start) + 3
    registrations = [line for line in plugin[register_start:register_end].split("\n") if line.startswith("    builder = builder.") and SHIPPED.search(line)]
    if len(variants) != 18 or len(registrations) != 18:
        problems.append(f"shipped fleet: {len(variants)} variants / {len(registrations)} registrations (expected 18 / 18)")
    else:
        shipped_enum = (
            "// 🗃️ Closed runtime app fleet the SHIPPED component assembles: the nine text/data document subsets, 18 apps.\n"
            "// Every app monomorphises the whole app machinery and every registered app is live code in the component, so\n"
            "// the 176-app library fleet above cannot be the component (single-CGU rustc 85 GB, `wasm-component-ld`'s\n"
            "// 1 000 000-function ceiling).\n"
            "#[cfg(not(feature = \"full-app-catalog\"))]\ndyn_enum_close! {\n    pub enum StdioApps: PluginApp {\n" + "\n".join(variants) + "\n    }\n}\n"
        )
        shipped_register = (
            "/// 📄️ Registers the shipped component fleet: csv/tsv/txt/json(any, i-json)/xml(any, valid)/md/html — the nine\n"
            "/// text and data document subsets, 18 apps; the plugin still declares and codec-owns all 36 stdio artifact kinds.\n"
            "#[cfg(not(feature = \"full-app-catalog\"))]\nfn register_apps(mut builder: PluginBuilder<Ready, StdioApps>) -> PluginBuilder<Ready, StdioApps> {\n" + "\n".join(registrations) + "\n    builder\n}\n"
        )
        plugin = (
            plugin[:enum_start]
            + "// 🗃️ Closed runtime app fleet for every stdio editor and viewer surface — the library fleet native hosts\n"
            + "// and tests assemble (`full-app-catalog`).\n#[cfg(feature = \"full-app-catalog\")]\ndyn_enum_close! {\n"
            + plugin[enum_start + len(full_enum): enum_end] + "\n" + shipped_enum + plugin[enum_end:]
        )
        plugin = plugin.replace(full_register, "/// 🗃️ Registers all 88 editor/viewer subsets — the library fleet (`full-app-catalog`).\n#[cfg(feature = \"full-app-catalog\")]\nfn register_apps(")
        plugin = plugin.rstrip("\n") + "\n\n" + shipped_register
print(f"files={0 if problems else 2} dropped_playgrounds={len(dropped)} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    (REPO / CARGO).write_text(cargo, encoding="utf-8")
    (REPO / PLUGIN).write_text(plugin, encoding="utf-8")
