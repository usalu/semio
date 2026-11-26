"""🏭️ Verify Process registration retains both native surfaces in separate compiler units."""

import json
import re
from pathlib import Path


hub = Path("🌎️hub")
owner = hub / "🧩️compositions/🏭️process"
root = (owner / "🦀️.rs").read_text()
lock = (hub / "Cargo.lock").read_text()
workspace = (hub / "Cargo.toml").read_text()
router = (owner / "📦️packages/🦀️rust/Cargo.toml").read_text()
fixture = json.loads((owner / "🧪️tests/🧫️fixtures/registration.json").read_text())
roles = {"editor": "✏️", "viewer": "👁️"}
packages = set()
for app in fixture["apps"]:
    role = app["role"]
    assert app["id"] == f"s.process.process3d@1/*#{role}"
    assert app["originalFactory"] is True
    package = f"semio-hub-process-{role}-app"
    assert package not in packages, package
    packages.add(package)
    app_root = owner / "🗃️apps" / f"{roles[role]}process-{role}"
    manifest = app_root / "📦️packages/🦀️rust/Cargo.toml"
    assert manifest.is_file(), manifest
    assert f'name = "{package}"' in manifest.read_text(), manifest
    assert f'"{manifest.parent.relative_to(hub).as_posix()}"' in workspace
    assert f'name = "{package}"' in lock
    assert f"{package} = {{ workspace = true }}" in router
    assert "OriginalVcsApp<" in (app_root / "🦀️.rs").read_text()
    assert f"use {package.replace('-', '_')}::" in root
    assert re.search(rf"\.{role}_mutation_roster::<", root)
    assert re.search(rf"\.native_{role}::<", root)

fleet = root.partition("pub enum ProcessApps: PluginApp {")[2].partition("}")[0]
assert "OriginalVcsApp<" not in fleet, fleet
assert all(f"Process3d{role.title()}(Process{role.title()}Apps)" in fleet for role in roles)
assert 'name = "semio_hub_process_router"' in router
print("Process native registration resolves to two independently owned app crates")
