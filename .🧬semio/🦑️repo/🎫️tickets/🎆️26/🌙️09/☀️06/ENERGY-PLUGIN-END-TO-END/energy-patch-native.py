"""🧪 Test actual energy patch sources through their lower-level protocol owner."""
from pathlib import Path
import json
import os
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = Path.cwd()
generated = ticket / "🗑️generated"
workspace = generated / "patch-workspace"
workspace.mkdir(exist_ok=True)
energy = root / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff"
packages = {}
for manifest in root.rglob("Cargo.toml"):
    if "🗑️generated" in manifest.parts or ".git" in manifest.parts:
        continue
    source = manifest.read_text()
    for name in ["semio-framework-mutation-diff", "semio-framework-value", "semio-framework-value-native-owned", "semio-framework-value-derive", "semio-framework-pack-json", "semio-s-artifact-energy-model-diff-splice"]:
        if ('name = "' + name + '"') in source or ('name="' + name + '"') in source:
            packages[name] = manifest.parent

entries = ['[package]', 'name="semio-s-artifact-energy-model-diff-patch"', 'version="0.0.0"', 'edition="2021"', '[workspace]', '[lib]', 'path=' + json.dumps(str(energy / "🩹️patch/🦀️.rs"), ensure_ascii=False), '[dependencies]']
for name, path in packages.items():
    alias = "semio-framework-os-kernel" if name == "semio-framework-mutation-diff" else name
    entries.append(alias + '={package=' + json.dumps(name) + ',path=' + json.dumps(str(path), ensure_ascii=False) + '}')
entries += ['[[test]]', 'name="energy-patch-native"', 'path=' + json.dumps(str(ticket / "energy-patch-native.rs"), ensure_ascii=False)]
(workspace / "Cargo.toml").write_text("\n".join(entries) + "\n")
env = os.environ.copy()
env.update({"ENERGY_IDENTITY_FIXTURE": str(energy / "🧪️tests/🔬️unit/identity.json"), "ENERGY_SLOTS_FIXTURE": str(energy / "🧪️tests/🔬️unit/slots.json")})
raise SystemExit(subprocess.call([sys.executable, str(ticket / "📜️energy-cargo.py"), "test", "--manifest-path", str(workspace / "Cargo.toml"), "--test", "energy-patch-native"], env=env))
