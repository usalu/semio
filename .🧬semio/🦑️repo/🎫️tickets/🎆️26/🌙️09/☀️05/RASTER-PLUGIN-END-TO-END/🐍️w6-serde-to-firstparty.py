#!/usr/bin/env python3
"""🐍️ W6 — rewrites raster's test-profile JSON oracle off `serde_json` onto the first-party
`dsl::json` surface (`from_json_str`/`parse`/`from_dsl_value`/`value_eq_ignoring_object_order`),
the shape `✏️s/🔌️plugins/🏭️process/…/🧬️mutations/🏭create-machine/🧪️tests/…/🦀️.rs` already carries.

Raster's snapshot/mutation/diff types never derived `Serialize`/`Deserialize` (its owned-map field
type is forbidden from carrying one at all), so the 12 fixture-test mounts W2 wired up could not
compile. Run once; idempotent — a second run finds no `serde_json` left."""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
RASTER = ROOT / "✏️s/🔌️plugins/🖨️raster"

RULES = [
    (re.compile(r"let (\w+): serde_json::Value = serde_json::from_str\((\w+)\)\.expect\((\"[^\"]*\")\);"), r"let \1 = dsl::json::parse(\2).expect(\3);"),
    (re.compile(r"let reencoded = serde_json::to_value\(&decoded\)\.expect\(\"[^\"]*\"\);"), "let reencoded = dsl::json::from_dsl_value(&dsl::ToValue::to_value(&decoded));"),
    (re.compile(r"let reencoded = serde_json::to_value\(mutation\(\)\)\.expect\(\"[^\"]*\"\);"), "let reencoded = dsl::json::from_dsl_value(&dsl::ToValue::to_value(&mutation()));"),
    (re.compile(r"let encoded = serde_json::to_value\(produced\.diff\(\)\)\.expect\(\"[^\"]*\"\);"), "let encoded = dsl::json::from_dsl_value(&dsl::ToValue::to_value(produced.diff()));"),
    (re.compile(r"assert_eq!\(reencoded, original, "), "assert!(dsl::json::value_eq_ignoring_object_order(&reencoded, &original), "),
    (re.compile(r"assert_eq!\(encoded, committed, "), "assert!(dsl::json::value_eq_ignoring_object_order(&encoded, &committed), "),
    (re.compile(r"serde_json::from_str\("), "dsl::json::from_json_str("),
    (re.compile(r"serde_json::Value::as_str"), "dsl::json::Value::as_str"),
    (re.compile(r"serde_json::Value::as_array"), "dsl::json::Value::as_array"),
]


def main() -> int:
    touched = []
    for path in sorted(RASTER.rglob("🦀️.rs")):
        source = path.read_text(encoding="utf8")
        if "serde_json" not in source:
            continue
        rewritten = source
        for pattern, replacement in RULES:
            rewritten = pattern.sub(replacement, rewritten)
        if rewritten == source:
            continue
        path.write_text(rewritten, encoding="utf8")
        touched.append(path.relative_to(ROOT))
    for path in touched:
        print(f"[rewrote] {path}")
    remaining = [str(path.relative_to(ROOT)) for path in sorted(RASTER.rglob("🦀️.rs")) if "serde_json" in path.read_text(encoding="utf8")]
    for path in remaining:
        print(f"[remains] {path}")
    print(f"[summary] rewrote={len(touched)} remaining={len(remaining)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
