"""🧪 Mount the production Completed tests with their original, narrow native owners."""
from pathlib import Path
import json
import os

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
framework = root / "🧰️framework"
plugin = framework / "🛍️products/💻️os/🔨️modules/🔌️plugin"
completed = plugin / "⚛️reactor/🔄️turn/📥️receiving/📦️completed"
fixture = ticket / "🗑️generated" / "fault"
fixture.mkdir(parents=True, exist_ok=True)


def relative(path):
    return json.dumps(Path(os.path.relpath(path, fixture)).as_posix(), ensure_ascii=False)


owners = {
    "semio-framework": framework,
    "semio-framework-job": framework / "🔨️modules/🧵️job",
    "semio-framework-trace": framework / "🔨️modules/⏱️trace",
    "semio-framework-value": framework / "🔨️modules/🌱️value",
    "semio-plugin-abi": plugin / "🚪️io",
    "semio-plugin-actor-host": plugin / "🌐host",
    "semio-plugin-reactor-receiving-completed": completed,
    "semio-plugin-testing-caller": plugin / "🧪️testing/🎟️caller",
}
manifest = [
    'cargo-features = ["trim-paths"]', "[package]", 'name = "completed-fault-native-reference"', 'version = "0.1.0"',
    'edition = "2021"', "[workspace]", 'resolver = "3"', "[lib]", 'path = "lib.rs"',
    "[dev-dependencies]", 'serde_json = "1.0.140"',
]
for name, path in owners.items():
    manifest.append(f'{name} = {{ path = {relative(path / "📦️packages/🦀️rust")} }}')
(fixture / "Cargo.toml").write_text("\n".join(manifest) + "\n", encoding="utf-8")
lock = fixture / "Cargo.lock"
if not lock.exists():
    lock.write_bytes((root / "Cargo.lock").read_bytes())
(fixture / "lib.rs").write_text(
    '#![cfg(test)]\n#![allow(unused_imports)]\n'
    'extern crate self as semio_plugin_reactor_turn;\n'
    'extern crate semio_plugin_testing_caller as semio_plugin_testing_laws;\n'
    'use semio_plugin_abi::actor_input_receipt::{OriginalActorInputFailureIdentity, '
    'OriginalActorInputFailureView, OriginalJsonOutputFailure, retire_step};\n'
    f'#[path = {relative(completed / "🧪️tests/🦀️.rs")}]\nmod completed;\n',
    encoding="utf-8",
)
