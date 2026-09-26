#!/usr/bin/env python3
"""🌉️ Adds `<prefix>_mutation_report_json` to the five wfc mutation roots (F10). Refuses a second run."""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts")
SITES = [
    ("◻️2d", "wfc2d", "Wfc2dSnapshot", "Wfc2dMutation"),
    ("🔲️grid2d", "grid2d", "Grid2dSnapshot", "Grid2dMutation"),
    ("🖼️bitmap", "bitmap", "BitmapSnapshot", "BitmapMutation"),
    ("🧊️3d", "wfc3d", "Wfc3dSnapshot", "Wfc3dMutation"),
    ("🧱️grid3d", "grid3d", "Grid3dSnapshot", "Grid3dMutation"),
]
ANCHOR = "\n//#region 🧪️Tests\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]\nmod tests;\n"
write = "--write" in sys.argv
problems = []
for artifact, prefix, snapshot, mutation in SITES:
    path = ROOT / artifact / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"
    text = path.read_text(encoding="utf-8")
    name = f"{prefix}_mutation_report_json"
    if name in text:
        problems.append(f"{path}: {name} already present")
        continue
    if text.count(ANCHOR) != 1:
        problems.append(f"{path}: test-mount anchor found {text.count(ANCHOR)} times")
        continue
    bridge = (
        "\n//#region 🌉️TestBridge\n"
        f"/// 🌉️ The language-neutral report of one committed specification vector — decoded, diffed, applied and inverted\n"
        f"/// through this subset's production JSON codec and `Mutation` implementation — that the `mutate-{prefix}` case's\n"
        "/// subject half judges with `law::vector`. Its signature names only `str`, so a generated test host reaches it.\n"
        "/// @see store::os_store::test_support::mutation_report_json\n"
        f"pub fn {name}(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {{\n"
        f"    store::os_store::test_support::mutation_report_json::<{snapshot}, {mutation}>(base_json, mutation_json, after_json)\n"
        "}\n"
        "//#endregion 🌉️TestBridge\n"
    )
    print(f"{'write' if write else 'dry-run'} {path}")
    if write:
        path.write_text(text.replace(ANCHOR, bridge + ANCHOR), encoding="utf-8")
for problem in problems:
    print("problem:", problem)
sys.exit(1 if problems else 0)
