#!/usr/bin/env python3
"""🌎️ Gives the hub workspace root one `[workspace.dependencies]` path row per crate a P8 cut created (from every cut
config in the lane folder), inserted right after its family's aggregator row, and drops rows whose manifest is gone.

usage: 🌎️hub-rows.py [apply]
"""
import glob, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
hub = os.path.join(REPO, "🌎️hub", "Cargo.toml")
text = open(hub, encoding="utf-8").read()
for path in sorted(glob.glob(os.path.join(HERE, "*", "*-cut.json"))):
    config = json.load(open(path, encoding="utf-8"))
    if not os.path.exists(os.path.join(REPO, config["artifact"], "📦️packages")) and "manifest" not in config:
        continue
    agg = config["crate"].replace("_", "-")
    anchor = re.search(r"(?m)^" + re.escape(agg) + r"\s*=\s*\{[^\n]*\}\n", text)
    rows = ""
    for spec in config["crates"].values():
        d = os.path.join(REPO, config["artifact"], spec["dir"])
        if not os.path.exists(os.path.join(d, "Cargo.toml")) or re.search(r"(?m)^" + re.escape(spec["package"]) + r"\s*=", text):
            continue
        rows += f'{spec["package"]} = {{ path = "{os.path.relpath(d, os.path.dirname(hub))}" }}\n'
    if rows:
        if anchor:
            text = text[:anchor.end()] + rows + text[anchor.end():]
        else:
            m = re.search(r"(?m)^\[workspace\.dependencies\]\n", text)
            text = text[:m.end()] + rows + text[m.end():]
        print(agg, rows.count("\n"))
for m in list(re.finditer(r'(?m)^(semio-s-artifact-stdio-[\w-]+)\s*=\s*\{\s*path\s*=\s*"([^"]+)"[^\n]*\n', text)):
    if not os.path.exists(os.path.join(os.path.dirname(hub), m.group(2), "Cargo.toml")):
        print("STALE", m.group(1)); text = text.replace(m.group(0), "")
if len(sys.argv) > 1 and sys.argv[1] == "apply":
    open(hub, "w", encoding="utf-8").write(text)
