"""🏷️ S4-TOOLS-B — gives the eight path-budget-truncated energy field leaves precise shorter semantic kinds (design §14,
coordinator decision 10-04): dir name == semanticKind within the 240-byte path budget, every reference at once.

Scheme `change-<record>-<field>` (the energy verb, records `site`, `ground`, `run`; redundant `temperature`/`period` dropped):
`change-ground-{building,shallow,deep}`, `change-run-{start-month,start-day,end-month,end-day,year}` (entities `ground`, `run`).
Renames the eight leaf dirs under `🧬️schema/🧬️mutations` and `🧫️fixtures/🧬️mutations`, then rewrites kebab / camel / Pascal /
snake / display spellings, dir paths and the two SemanticDescriptor entities in every text file of the energy plugin tree and
the two S3-CONTROLS energy generator inputs (never the ticket's reports).
Dry run by default (prints the plan); `--write` applies; idempotent (a second `--write` changes nothing).
"""
import os
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
ENERGY = ROOT / "✏️s/🔌️plugins/🔋️energy"
ANY = ENERGY / "🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
TICKET = pathlib.Path(__file__).resolve().parent
KINDS = [
    ("🌡️", "change-ground-temperature", "change-ground-temperature-building-surface", "change-ground-building", "ground"),
    ("🌱️", "change-ground-temperature", "change-ground-temperature-shallow", "change-ground-shallow", "ground"),
    ("⛏️", "change-ground-temperature", "change-ground-temperature-deep", "change-ground-deep", "ground"),
    ("🛫️", "change-run-period-start", "change-run-period-start-month", "change-run-start-month", "run"),
    ("▶️", "change-run-period-start", "change-run-period-start-day", "change-run-start-day", "run"),
    ("🛬️", "change-run-period-end", "change-run-period-end-month", "change-run-end-month", "run"),
    ("⏹️", "change-run-period-end", "change-run-period-end-day", "change-run-end-day", "run"),
    ("📅️", "change-run-period-year", "change-run-period-year", "change-run-year", "run"),
]
OLD_ENTITY = {"ground": "ground-temperature", "run": "run-period"}


def words(kebab):
    return kebab.split("-")


def spellings(kebab):
    parts = words(kebab)
    pascal = "".join(part.capitalize() for part in parts)
    return {
        "kebab": kebab,
        "camel": parts[0] + "".join(part.capitalize() for part in parts[1:]),
        "pascal": pascal,
        "record": "Changed" + pascal[len("Change"):],
        "snake": "_".join(parts),
        "screaming": "_".join(parts).upper(),
        "display": " ".join(part.capitalize() for part in parts),
    }


def replacements():
    pairs = []
    for emoji, old_dir, old_kind, new_kind, _ in KINDS:
        pairs.append((re.compile(re.escape(emoji + old_dir) + r"(?![a-z0-9-])"), emoji + new_kind))
    for _, _, old_kind, new_kind, _ in sorted(KINDS, key=lambda row: -len(row[2])):
        old, new = spellings(old_kind), spellings(new_kind)
        for form in ("kebab", "camel", "pascal", "record", "snake", "screaming", "display"):
            pairs.append((re.compile(re.escape(old[form]) + r"(?![A-Za-z0-9])"), new[form]))
    return pairs


def entity_fix(text, path):
    for emoji, _, _, new_kind, entity in KINDS:
        if f"/{emoji}{new_kind}/" in str(path) and path.name == "🦀️.rs":
            text = text.replace(f'entity: "{OLD_ENTITY[entity]}", kind: "{new_kind}"', f'entity: "{entity}", kind: "{new_kind}"')
    return text


def main():
    write = "--write" in sys.argv[1:]
    if any(arg != "--write" for arg in sys.argv[1:]):
        sys.exit(f"unknown arguments {sys.argv[1:]}")
    renames = []
    for collection in ("🧬️schema/🧬️mutations", "🧫️fixtures/🧬️mutations"):
        for emoji, old_dir, _, new_kind, _ in KINDS:
            old, new = ANY / collection / (emoji + old_dir), ANY / collection / (emoji + new_kind)
            if old.exists() and old != new:
                renames.append((old, new))
    for old, new in renames:
        print(f"rename {old.relative_to(ROOT)} -> {new.name}")
        if write:
            os.rename(old, new)
    pairs = replacements()
    changed = 0
    inputs = [TICKET / "🧪️s3-controls-energy-field-leaves.py", TICKET / "🧪️s3-controls-energy-field-witnesses.py"]
    for path in [*ENERGY.rglob("*"), *inputs]:
        if True:
            if not path.is_file() or any(part in ("node_modules", "dist", "🗑️generated", "target") for part in path.parts) or path.suffix in (".wasm", ".png", ".jpg", ".epw", ".lock"):
                continue
            if path == pathlib.Path(__file__).resolve():
                continue
            try:
                text = path.read_text(encoding="utf-8")
            except (UnicodeDecodeError, OSError):
                continue
            after = text
            for pattern, value in pairs:
                after = pattern.sub(value, after)
            after = entity_fix(after, path)
            if after != text:
                changed += 1
                print(f"rewrite {path.relative_to(ROOT)}")
                if write:
                    path.write_text(after, encoding="utf-8")
    print(f"{len(renames)} dir renames, {changed} files {'rewritten' if write else 'to rewrite'}")


main()
