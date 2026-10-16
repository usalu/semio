#!/usr/bin/env python3
# 🧭️ Shared constants of the PLUGIN-WP2 tools: repo paths, the WP2 crate table and the owned source trees.
import os
import re

REPO = "/Users/ueli/Documents/semio"
PLUG_REL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
PLUG = REPO + "/" + PLUG_REL
SPLIT = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = SPLIT + "/🗑️generated"
SCRATCH = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad"
WS = SCRATCH + "/pwp2"
SNAP = WS + "/snap0"
PFX = "semio_framework_plugin_"
PKG_PFX = "semio-framework-plugin-"


def read(path):
    with open(path, encoding="utf8") as handle:
        return handle.read()


def write(path, text):
    os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
    with open(path, "w", encoding="utf8") as handle:
        handle.write(text)


def rs_files(root):
    for base, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in ("node_modules", "dist", "target", "📦️packages")]
        for name in files:
            if name.endswith(".rs"):
                yield os.path.join(base, name)
