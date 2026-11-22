import subprocess
from pathlib import Path
import sys

ticket, native = (Path(value).resolve() for value in sys.argv[1:])
generated = ticket / "🗑️generated" / "rectangular-brush"
source = generated / "probe.rs"
source.write_text(r'''
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const TERRAIN_GRID_RESOLUTION: u32 = 33;
mod brush { include!("action.rs"); }

fn main() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("action-fixture.json")).unwrap();
    let decode = |row: &serde_json::Value| from_json_str::<brush::TerrainBrushAction>(&row.to_string(), JsonMemberPolicy::Reject);
    for row in fixture["accepted"].as_array().unwrap() {
        let action = decode(row).unwrap();
        assert!(action.is_valid(), "{row}");
        assert_eq!(action.arguments.value, row["arguments"]["value"].as_f64().unwrap());
    }
    for row in fixture["refused"].as_array().unwrap() {
        assert!(!decode(row).is_ok_and(|action| action.is_valid()), "{row}");
    }
    println!("[DEBUG] Native brush parser retains signed fractional strengths and rejects malformed metadata.");
}
'''.replace('mod brush { include!("action.rs"); }', '#[path = "action.rs"] mod brush;'))
rustc = Path("~/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin/rustc").expanduser()
cache = ticket / "🗑️generated" / "qr"
binary = generated / "probe"
pairs = {
    "semio_framework_pack_json": "4144898bdbda0868",
    "semio_framework_value": "05431580f111efc8",
    "serde_json": "45a81c2a224abcf3",
}
command = [str(rustc), "--crate-name", "gis_brush_precision", "--edition=2024", str(source), "-L", f"dependency={cache}", "-o", str(binary)]
for crate, key in pairs.items():
    for extension in (".rmeta", ".rlib"):
        dependency = cache / f"lib{crate}-{key}{extension}"
        if not dependency.is_file():
            crate_folder = "serde_json" if crate == "serde_json" else crate.replace("_", "-")
            dependency = native / "debug/build" / crate_folder / key / "out" / dependency.name
        assert dependency.is_file(), dependency
        command.extend(("--extern", f"{crate}={dependency}"))
subprocess.run(command, check=True)
subprocess.run([binary], check=True)
