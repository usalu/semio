"""📚️ The shared rolled-section catalogue, read from the one source the Rust implementation uses (`SteelSection::from_catalogue` of the en1994 snapshot), so no second copy of the table exists."""

from __future__ import annotations

import os
import re

#: 📍️ The snapshot source holding `SteelSection`, relative to the norm plugin root.
SNAPSHOT = "🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"

#: 🔢 A Rust float literal.
NUMBER = r"-?[0-9][0-9_]*(?:\.[0-9_]+)?(?:e-?[0-9]+)?"


def camel(name):
    """🐫 A snake_case Rust field in its camelCase wire spelling (`w_pl_y_m3` → `wPlYM3`)."""
    head, *tail = name.split("_")
    return head + "".join(part[:1].upper() + part[1:] for part in tail)


def normalised(designation):
    """🔤 A designation in the catalogue's key spelling (`HEB 320` → `HEB320`), like `SteelSection::normalize_designation`."""
    return "".join(designation.split()).upper()


def record(body):
    """🧱 One section record from the `Self { designation: "…".into(), height_m: …, … }` literal body."""
    designation = re.search(r'designation:\s*"([^"]+)"', body).group(1)
    numbers = {camel(name): float(value.replace("_", "")) for name, value in re.findall(r"(\w+):\s*(%s)\b" % NUMBER, body)}
    return {"designation": designation, **numbers}


def section_catalogue(norm_root):
    """📚 Every catalogue section keyed by its normalised designation: the `heb300()` default and each `from_catalogue` match arm."""
    with open(os.path.join(norm_root, SNAPSHOT), encoding="utf-8") as source:
        text = source.read()
    default = re.search(r"pub fn heb300\(\) -> Self \{\s*Self \{(.*?)\}\s*\}", text, re.S)
    arms = re.findall(r'"([A-Z0-9]+)" => Some\(Self \{(.*?)\}\)', text, re.S)
    rows = [record(default.group(1))] + [record(body) for _, body in arms]
    return {normalised(row["designation"]): row for row in rows}
