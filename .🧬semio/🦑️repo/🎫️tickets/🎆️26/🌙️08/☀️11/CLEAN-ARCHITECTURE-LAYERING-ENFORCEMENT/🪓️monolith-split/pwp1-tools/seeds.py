#!/usr/bin/env python3
# 🌱️ Seeded sensitivity checks of the static scanners: a narrowed item must be re-flagged by defscan, a foreign impl must be flagged by orphan_scan.
import re, sys
from lib import *
import defscan, orphan_scan, defs, wp1files
from crates_wp1 import ROOT

# 1. defscan: narrow `preflight_artifact_identity` of identity back to pub(crate) in memory and scan
path = PLUG + "/🪪️identity/🦀️.rs"
text = read(path)
assert text.count("pub fn preflight_artifact_identity") == 1
seeded = text.replace("pub fn preflight_artifact_identity", "pub(crate) fn preflight_artifact_identity")
orig_read = defscan.read
defscan.read = lambda p: seeded if p == path else orig_read(p)
others = defscan.other_files({path})
rows = defscan.scan_crate([path], others)
flagged = [(d.name, len(defscan.hits(d, others))) for f, d in rows if d.name == "preflight_artifact_identity"]
print("defscan seed (pub fn -> pub(crate) fn):", flagged, "FLAGGED" if flagged and flagged[0][1] else "MISSED")
defscan.read = orig_read
unseeded = [(d.name) for f, d in defscan.scan_crate([path], others) if d.name == "preflight_artifact_identity"]
print("defscan unseeded (must be empty):", unseeded)

# 2. orphan: add `impl std::fmt::Display for Vec<u8> {}` to a temp view of identity
orig = orphan_scan.read
orphan_scan.read = lambda p: (orig(p) + "\nimpl std::fmt::Display for Vec<u8> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Ok(()) } }\n") if p == path else orig(p)
hits = orphan_scan.scan("identity")
print("orphan seed:", [h[2] for h in hits], "FLAGGED" if hits else "MISSED")
orphan_scan.read = orig
print("orphan unseeded:", orphan_scan.scan("identity"))
