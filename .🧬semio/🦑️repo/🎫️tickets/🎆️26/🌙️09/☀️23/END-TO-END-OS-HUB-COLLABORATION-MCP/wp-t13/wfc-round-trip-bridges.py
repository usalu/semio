#!/usr/bin/env python3
"""🔁️ Adds `<prefix>_snapshot_json_round_trip` beside the report bridge of the two wfc subsets whose mutate case
declares an `identity-round-trip` scenario (bitmap, wfc3d). Refuses a second run."""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts")
SITES = [("🖼️bitmap", "bitmap", "BitmapSnapshot"), ("🧊️3d", "wfc3d", "Wfc3dSnapshot")]
write = "--write" in sys.argv
problems, edits = [], []
for artifact, prefix, snapshot in SITES:
    path = ROOT / artifact / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"
    text = path.read_text(encoding="utf-8")
    anchor = "}\n//#endregion 🌉️TestBridge\n"
    name = f"{prefix}_snapshot_json_round_trip"
    if name in text or text.count(anchor) != 1:
        problems.append(f"{path}: {name} present or anchor count {text.count(anchor)}")
        continue
    bridge = (
        "}\n\n"
        f"/// 🔁️ Decodes one snapshot through this subset's production JSON codec and re-encodes it — the subject half of the\n"
        "/// case's `identity-round-trip` scenario.\n"
        f"pub fn {name}(text: &str) -> Result<String, String> {{\n"
        f"    let snapshot: {snapshot} = dsl::json::from_json_str(text).map_err(|error| error.to_string())?;\n"
        "    Ok(dsl::json::to_json_string(&snapshot))\n"
        "}\n//#endregion 🌉️TestBridge\n"
    )
    edits.append((path, text.replace(anchor, bridge)))
for path, _ in edits:
    print(("write " if write else "dry-run ") + str(path))
for problem in problems:
    print("problem:", problem)
if problems:
    sys.exit(1)
if write:
    for path, text in edits:
        path.write_text(text, encoding="utf-8")
