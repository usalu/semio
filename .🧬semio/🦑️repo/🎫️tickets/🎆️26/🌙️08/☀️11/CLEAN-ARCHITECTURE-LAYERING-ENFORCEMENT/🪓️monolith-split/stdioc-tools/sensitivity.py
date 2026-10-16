#!/usr/bin/env python3
# 🧪️ Seeded sensitivity of the union oracle: each mutation of the built union must add at least one new internal error compared to the clean union.
import os, re, shutil, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
import probe
from lib import *
SCRATCH = os.environ["SCRATCH"]
UNION = SCRATCH + "/union"
SENS = SCRATCH + "/union-sens"
REL = probe.REL
clean = probe.internal_heads(open(f"{SCRATCH}/probe/split-union-test-testing.err").read())
cases = [
    ("private decode_pointer", "✏️editing/🦀️.rs", r"pub fn decode_pointer\(", "fn decode_pointer("),
    ("private check_publication_limits", "✏️editing/🦀️.rs", r"pub fn check_publication_limits<", "fn check_publication_limits<"),
    ("private load_example_effect", "🪟️window-kit/🦀️.rs", r"pub fn load_example_effect<", "fn load_example_effect<"),
    ("private snapshot_edit_source", "🧾️payload-wire/🦀️.rs", r"pub fn snapshot_edit_source<", "fn snapshot_edit_source<"),
    ("facade drops a name", "🦀️.rs", r"definition_from_schema, ", ""),
    ("pub(crate) leak", "📐️part21/🦀️.rs", r"pub fn parse_part21", "pub(crate) fn parse_part21"),
]
for label, relative, pattern, replacement in cases:
    if os.path.exists(SENS):
        shutil.rmtree(SENS)
    shutil.copytree(UNION, SENS, symlinks=True)
    path = SENS + "/" + REL + "/" + relative
    text = read(path)
    changed = re.subn(pattern, replacement, text, count=1)
    if relative == "📐️part21/🦀️.rs":
        changed = (re.sub(r"pub\(in crate::[a-z_0-9]+\) fn parse_part21", "pub(in crate::semio_s_artifact_stdio_contract_part21) fn parse_part21", text) if "pub(crate) fn parse_part21" in text else text, 1)
        changed = (re.sub(pattern, "pub(in crate::semio_s_artifact_stdio_contract_part21) fn parse_part21", text, count=1), 1)
    assert changed[1] >= 1, label
    write(path, changed[0])
    root = SENS + "/union.rs"
    text = read(root).replace(UNION, SENS)
    write(root, text)
    uses = SENS + "/uses.rs"
    stderr = probe.run("union", root, probe.CONFIGS["test-testing"])
    heads = probe.internal_heads(stderr)
    new = {k: v - clean.get(k, 0) for k, v in heads.items() if v - clean.get(k, 0) > 0}
    print(("DETECTED " if new else "MISSED   ") + label, [k[:110] for k in list(new)[:2]])
