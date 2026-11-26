"""🏗️ Verify the authored FEM app roster routes through independent dispatch crates."""

import json
from pathlib import Path


hub = Path("🌎️hub")
owner = hub / "🧩️compositions/🏗️fem"
router = (owner / "🦀️.rs").read_text()
fleet = router.partition("pub enum FemApps: PluginApp {")[2].partition("}")[0]
manifest = (owner / "📦️packages/🦀️rust/Cargo.toml").read_text()
descriptor = json.loads((owner / "🧩️component/🔣️.json").read_text())
dimensions = {"2d": "◻️", "3d": "🧊️"}
roles = {"editor": "✏️", "viewer": "👁️"}
assert len(descriptor["manifest"]["apps"]) == len(dimensions) * len(roles)
for surface in descriptor["manifest"]["apps"]:
    role = surface["role"]
    dimension = surface["id"].split("@")[0][-2:]
    assert surface["id"] == f"s.fem.fem{dimension}@1/*#{role}"
    owner_package = f"semio-hub-fem-{dimension}-{role}-app"
    source = owner / "🗃️apps" / f"{dimensions[dimension]}{dimension}" / f"{roles[role]}{role}"
    leaf_manifest = source / "📦️packages/🦀️rust/Cargo.toml"
    assert leaf_manifest.is_file(), leaf_manifest
    assert f'name = "{owner_package}"' in leaf_manifest.read_text()
    leaf = (source / "🦀️.rs").read_text()
    assert "VcsArtifactApp<" in leaf
    assert owner_package.replace("-", "_") in fleet
    assert f"{owner_package} = {{ workspace = true }}" in manifest
assert "VcsArtifactApp<" not in fleet
assert 'name = "semio_hub_fem_router"' in manifest
print("FEM authored surfaces resolve through four independently owned app units")
