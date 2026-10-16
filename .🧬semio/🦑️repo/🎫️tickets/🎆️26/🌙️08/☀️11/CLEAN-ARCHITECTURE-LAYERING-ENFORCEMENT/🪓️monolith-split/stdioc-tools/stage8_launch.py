#!/usr/bin/env python3
# 🚀️ Stage 8: registers the six build/check/test routes of every producer in .vscode/launch.json and the seed (one atomic write per file, anchored after the contract gate route).
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(__file__))
from lib import *  # noqa: E402,F401,F403

ANCHOR = '      "name": "⚖️gate🗄️stdio✏️editing🧬️contract",'
GROUPS = [("build", "🏗️build", "4_build", "900.70"), ("check", "🔎️check", "4_gate", "922.70"), ("test", "🧪️test", "4_test", "903.70")]
CONFIGS = [("prototype", "🛠️prototype"), ("release", "🚀️release")]


def routes():
    crates = sorted(CRATES, key=lambda crate: crate["package"])
    out = []
    for target, label, group, band in GROUPS:
        for index, crate in enumerate(crates):
            for offset, (config, config_label) in enumerate(CONFIGS):
                order = float(f"{band}{1 + 2 * index + offset:03d}")
                out.append((f"🦀️ {crate['package']} {label} {config_label}", f"bun repo run {crate['project']}:{target} --configuration={config} --parallel=1 -- --lib --offline", group, order))
    return out


def block(rows):
    parts = []
    for name, command, group, order in rows:
        parts.append("\n".join(['    {', f'      "name": {json.dumps(name, ensure_ascii=False)},', '      "type": "node-terminal",', '      "request": "launch",', f'      "command": {json.dumps(command, ensure_ascii=False)},', '      "cwd": "${workspaceFolder}",', '      "presentation": {', f'        "group": "{group}",', f'        "order": {json.dumps(order)}', '      }', '    },']))
    return "\n".join(parts)


for relative in (".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"):
    path = ROOT + "/" + relative
    text = read(path)
    lines = text.split("\n")
    i = 0
    removed = 0
    while i < len(lines):
        if lines[i] == "    {" and lines[i + 1].startswith('      "name": "🦀️ semio-s-artifact-stdio-contract'):
            end = next(j for j in range(i, len(lines)) if lines[j] == "    },")
            del lines[i:end + 1]
            removed += 1
            continue
        i += 1
    text = "\n".join(lines)
    rows = routes()
    anchor = next(i for i, line in enumerate(lines) if line == ANCHOR)
    end = next(i for i in range(anchor, len(lines)) if lines[i] == "    },")
    lines[end + 1:end + 1] = block(rows).split("\n")
    new = "\n".join(lines)
    write(path, new)
    print(relative, "routes removed", removed, "added", len(rows))
