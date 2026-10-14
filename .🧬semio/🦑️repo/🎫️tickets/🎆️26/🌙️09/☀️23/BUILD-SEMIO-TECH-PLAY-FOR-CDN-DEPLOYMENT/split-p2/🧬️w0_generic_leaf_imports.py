#!/usr/bin/env python3
"""🧬️ Wave 0 follow-up: imports every leaf payload a generic leaf's `From<…>` bounds name but neither defines nor imports
yet (lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
M = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")
root = open(os.path.join(M, "🦀️.rs"), encoding="utf-8").read()
MODULE = {t: m for m, t in re.findall(r"use super::([a-z_]+)::([A-Za-z]+);", root)}
P = "crate::standards::v1::subsets::any::schema::mutations"
added = 0
for leaf in sorted(os.listdir(M)):
    for rel in ("🦀️.rs", "↩️inverse/🦀️.rs"):
        path = os.path.join(M, leaf, rel)
        if not os.path.isfile(path) or not os.path.isfile(os.path.join(M, leaf, "↩️inverse/🦀️.rs")):
            continue
        text = open(path, encoding="utf-8").read()
        needed = sorted(set(re.findall(r"From<([A-Za-z]+)>", text)))
        missing = []
        for t in needed:
            defined = re.search(r"(?m)^pub (?:struct|enum) " + t + r"\b", text)
            imported = re.search(r"(?m)^use [^;]*\b" + t + r"\b[^;]*;", text)
            if not defined and not imported:
                missing.append(t)
        if not missing:
            continue
        lines = "".join(f"use {P}::{MODULE[t]}::{t};\n" for t in missing)
        first_use = re.search(r"(?m)^use ", text)
        at = first_use.start() if first_use else text.index("\n\n") + 2
        text = text[:at] + lines + text[at:]
        open(path, "w", encoding="utf-8").write(text)
        added += len(missing)
print("imports added", added)
