#!/usr/bin/env python3
"""🔺️ Strip the view-state keys (`resultSourceId`, `resultMode`, `resultModeIndex`, `camera`, `locale`,
`solverResultsJson`, `meshPreviewJson`) that `Fem2dDiff`/`Fem3dDiff` no longer carry from every committed
`🔺️diff/🔣️.json` fixture under the fem plugin, preserving key order and the 2-space layout."""
import json, pathlib, sys
ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem")
KEYS = {"resultSourceId", "resultMode", "resultModeIndex", "camera", "locale", "solverResultsJson", "meshPreviewJson"}
touched = nonnull = 0
for path in ROOT.rglob("🔣️.json"):
    if "node_modules" in path.parts or path.parent.name != "🔺️diff":
        continue
    text = path.read_text(encoding="utf-8")
    data = json.loads(text)
    if not isinstance(data, dict) or not (KEYS & data.keys()):
        continue
    for key in KEYS & data.keys():
        if data[key] is not None:
            nonnull += 1
            print(f"NONNULL {path}: {key}={data[key]!r}", file=sys.stderr)
        del data[key]
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else ""), encoding="utf-8")
    touched += 1
print(f"stripped {touched} diff fixtures, {nonnull} non-null values dropped")
