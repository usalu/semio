#!/usr/bin/env python3
from pathlib import Path
import re

ROOT = Path("/Users/ueli/Documents/semio")
KEEP = {
    "module.pack.pack",
    "module.protocol.protocol",
    "module.vcs.vcs",
}
MACRO = re.compile(r"\n+dsl_core::fault_from_thiserror!\([^;]+;\n?$")

removed = 0
for path in list(ROOT.rglob("📦️lib.rs")):
    text = path.read_text(encoding="utf-8")
    if "fault_from_thiserror!" not in text:
        continue
    new = MACRO.sub("", text)
    if new != text:
        path.write_text(new.rstrip() + "\n", encoding="utf-8")
        removed += 1

print("stripped macros from", removed, "files")
