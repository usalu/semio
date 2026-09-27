#!/usr/bin/env python3
"""🐫️ LB-P1 (prepared, lands after W3 announces 7800 on B3 — rule 30 forbids guest edits during the rebuild): the brep
snapshot's `BrepCurve`, `BrepCurve2` and `BrepSurface` carry `#[value(tag = "kind", rename_all = "camelCase")]` and
a doc comment claiming the container `rename_all` also cases struct-variant members. Since 09-12 the value derive
follows serde (`🌱️value/✨️derive/🧪️tests/🐫️variant-field-casing`): container `rename_all` cases VARIANT names only;
members follow `rename_all_fields`. So Rust decodes and projects `radius_major`/`control_points`/`half_angle` while
the brep schema (`📸️snapshot/🔣️.json`: `radiusMajor`, `controlPoints`, `halfAngle`), the Python reference and the
case's doc strings say camelCase — measured: `test-parity --case 🧊️mutate-semio-brep` Rust subject 7 rows
"the planned mutation payload must decode: curve.missing field `radius_major`" (`wp-lb/generated/parity-lb-1.txt`).
Fix: `rename_all_fields = "camelCase"` on the three enums and the comments state the real rule.
Usage: lb-p1-brep-field-casing.py --dry-run | --write"""
import sys
from pathlib import Path

PATH = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🦀️.rs")
write = "--write" in sys.argv
if not write and "--dry-run" not in sys.argv:
    sys.exit(__doc__)
source = PATH.read_text(encoding="utf-8")
RULE = (
    "/// 🔣️ Container `rename_all` cases the variant names (`ellipse`, `nurbs`); `rename_all_fields` cases every\n"
    "/// struct-variant member (`radiusMajor`, `controlPoints`), exactly as the brep schema (`📸️snapshot/🔣️.json`)\n"
    "/// and serde state them (`🌱️value/✨️derive/🧪️tests/🐫️variant-field-casing`).\n"
)
EDITS = [
    ("/// 🔣️ Unlike `serde`, this derive's own `rename_all` already applies to BOTH the variant name AND\n"
     "/// every struct-variant member name (see `🌱️value/✨️derive`'s module docs) — no separate\n"
     "/// `rename_all_fields` needed; `radiusMajor`/`controlPoints` etc. come out correctly from\n"
     "/// `rename_all = \"camelCase\"` alone.\n"
     "#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]\n"
     "#[value(tag = \"kind\", rename_all = \"camelCase\")]\n"
     "pub enum BrepCurve {\n",
     RULE + "#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]\n"
     "#[value(tag = \"kind\", rename_all = \"camelCase\", rename_all_fields = \"camelCase\")]\n"
     "pub enum BrepCurve {\n"),
    ("#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]\n"
     "#[value(tag = \"kind\", rename_all = \"camelCase\")]\n"
     "pub enum BrepCurve2 {\n",
     "#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]\n"
     "#[value(tag = \"kind\", rename_all = \"camelCase\", rename_all_fields = \"camelCase\")]\n"
     "pub enum BrepCurve2 {\n"),
    ("/// 🔣️ Same story as `BrepCurve` above — this derive's `rename_all` already covers struct-variant\n"
     "/// member names, so a bare `rename_all = \"camelCase\"` is enough.\n"
     "#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]\n"
     "#[value(tag = \"kind\", rename_all = \"camelCase\")]\n"
     "pub enum BrepSurface {\n",
     "/// 🔣️ Cased like [`BrepCurve`]: variant names by `rename_all`, members by `rename_all_fields`.\n"
     "#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]\n"
     "#[value(tag = \"kind\", rename_all = \"camelCase\", rename_all_fields = \"camelCase\")]\n"
     "pub enum BrepSurface {\n"),
]
problems = []
for old, new in EDITS:
    if source.count(old) != 1:
        problems.append(f"expected 1× {old.splitlines()[-1]!r}, found {source.count(old)}")
        continue
    source = source.replace(old, new)
print(f"files={0 if problems else 1} hunks={len(EDITS) - len(problems)} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    PATH.write_text(source, encoding="utf-8")
