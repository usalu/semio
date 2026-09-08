#!/usr/bin/env python3
"""🔗️ WP4e — collapse the duplicated decimal-string u64 scalars onto `framework.value`.

Rows 149 + 151 of `📋️cross-partition-requests.md`: `framework.value`
(`🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json`) is the single owner of `U64` and its
`NonZeroU64` narrowing; every other framework module references the owner instead of carrying a
copy. Run with `--apply` to write, without it to report.
"""
import argparse
import json
import pathlib
import re
import sys

OWNER = "https://semio.tech/schema/framework/value/schema.json"
U64 = f"{OWNER}#/$defs/U64"
NON_ZERO = f"{OWNER}#/$defs/NonZeroU64"

# (file, old ref, new ref) — every rewritten pointer, listed explicitly, no globbing.
UI = "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission"
ACTOR = "🧰️framework/🔨️modules/🎭️actor"
REWRITES = [
    (f"{UI}/🧬️schema/🔣️.json", "#/$defs/DecimalU64", U64),
    (f"{UI}/🪪️root/🧬️schema/🔣️.json", "https://semio.tech/schema/framework/ui/host/input/admission/schema.json#/$defs/DecimalU64", U64),
    (f"{ACTOR}/🚪️lifetime/🧬️schema/🔣️.json", "#/$defs/U64", NON_ZERO),
    (f"{ACTOR}/🚪️lifetime/🩹️patch/🧬️schema/🔣️.json", "https://semio.tech/schema/framework/actor/lifetime/schema.json#/$defs/U64", NON_ZERO),
    (f"{ACTOR}/🪪️activation/🚪️instance/📥️output/🧬️schema/🔣️.json", "https://semio.tech/schema/framework/actor/lifetime/schema.json#/$defs/U64", NON_ZERO),
    (f"{ACTOR}/📤️return/🧬️schema/🔣️.json", "#/$defs/U64", NON_ZERO),
    (f"{ACTOR}/📤️return/📨️response/🧬️schema/🔣️.json", "https://semio.tech/schema/framework/actor/return/schema.json#/$defs/U64", NON_ZERO),
    (f"{ACTOR}/📤️return/📨️response/🎟️credit/🧬️schema/🔣️.json", "https://semio.tech/schema/framework/actor/return/schema.json#/$defs/U64", NON_ZERO),
]

# (file, export id) — the copies that cease to exist once their consumers address the owner.
DELETIONS = [
    (f"{UI}/🧬️schema/🔣️.json", "DecimalU64"),
    (f"{ACTOR}/🚪️lifetime/🧬️schema/🔣️.json", "U64"),
    (f"{ACTOR}/📤️return/🧬️schema/🔣️.json", "U64"),
]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--apply", action="store_true")
    arguments = parser.parse_args()
    root = pathlib.Path(__file__).resolve().parents[7]
    changed = 0
    for relative, old, new in REWRITES:
        path = root / relative
        text = path.read_text(encoding="utf8")
        needle = f'"$ref": "{old}"'
        count = text.count(needle)
        print(f"{relative}\n  {count}× {old} → {new}")
        if count == 0:
            print("  !! no occurrence", file=sys.stderr)
            return 1
        if arguments.apply:
            path.write_text(text.replace(needle, f'"$ref": "{new}"'), encoding="utf8")
            changed += count
    for relative, export in DELETIONS:
        path = root / relative
        document = json.loads(path.read_text(encoding="utf8"))
        if export not in document.get("$defs", {}):
            print(f"{relative}\n  !! no $defs.{export}", file=sys.stderr)
            return 1
        body = json.dumps(document["$defs"][export], ensure_ascii=False)
        print(f"{relative}\n  delete $defs.{export} ({len(body)} chars)")
        if arguments.apply:
            del document["$defs"][export]
            path.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    residual = [str(p) for p in (root / "🧰️framework/🔨️modules").rglob("🔣️.json") if re.search(r"DecimalU64|actor/(lifetime|return)/schema\.json#/\$defs/U64", p.read_text(encoding="utf8"))]
    print(f"applied={changed} residual={len(residual)}")
    for path in residual:
        print(f"  {path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
