import pathlib
import re

plugin = pathlib.Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
owners = [plugin / "🧬️guest-contract", plugin / "⚛️reactor/🎟️context"]
actor = "semio-plugin-actor-input"
for owner, expected in zip(owners, (False, True)):
    manifest = owner / "📦️packages/🦀️rust/Cargo.toml"
    dependency = re.search(r'^semio-plugin-actor-input = \{ path = "([^"]+)"', manifest.read_text(), re.M)
    assert (dependency is not None) == expected, manifest
    if expected:
        assert (manifest.parent / dependency.group(1) / "Cargo.toml").is_file()
        assert "pub use semio_plugin_actor_input as original_input;" in (owner / "🦀️.rs").read_text()
abi = plugin / "🚪️io"
manifest = abi / "📦️packages/🦀️rust/Cargo.toml"
receipt = (abi / "📥️actor-input-receipt/🦀️.rs").read_text()
for name in ("semio-framework-job-native-decode", "semio-framework-pack-json-output-receiving"):
    assert name.replace("-", "_") in receipt
    paths = re.findall(r'^' + name + r' = \{ path = "([^"]+)"', manifest.read_text(), re.M)
    assert len(paths) == 1, (name, paths)
    target = (manifest.parent / paths[0] / "Cargo.toml").read_text()
    assert f'name = "{name}"' in target
print("Guest contract and reactor input ownership passed")
