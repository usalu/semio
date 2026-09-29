"""🗂️ S20 fault classes: writes the REVIEWED classes (`.🧬semio/🌐hub/s14-s20-sets/class/family-*.json`) into the pass-1
faults overlay — every app declaration `.fault("code", LocalizedLabel…)` becomes `.fault("code", FaultClass::X, LocalizedLabel…)`
(the `FaultClass` path mirrors the site's `LocalizedLabel` path; a bare one gains `FaultClass` in the `use` group that imports
`LocalizedLabel`), and every framework catalog line `{"code": "…", "en": …}` becomes `{"code": "…", "class": "…", "en": …}`.
A declaration whose code has no reviewed class is reported, never guessed. Convergent: a declaration or catalog entry that
already names a class gets the reviewed one (re-run after every review).
Usage: python3 class-apply.py [--dry-run]"""
from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
LISTS = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-sets/class")
CATALOG = OVERLAY / "🧰️framework/🔨️modules/⚠️diagnostic/🗂️catalog/🔣️.json"
PLUGINS = OVERLAY / "✏️s/🔌️plugins"
VARIANT = {"input-invalid": "InputInvalid", "precondition-failed": "PreconditionFailed", "conflict": "Conflict", "permission-denied": "PermissionDenied",
           "unavailable": "Unavailable", "cancelled": "Cancelled", "internal": "Internal"}
DECLARATION = re.compile(r'\.fault\(\s*"(?P<code>[^"]+)",\s*(?P<qual>(?:[A-Za-z_]\w*::)*)LocalizedLabel::')
CLASSED = re.compile(r'\.fault\(\s*"(?P<code>[^"]+)",\s*(?P<qual>(?:[A-Za-z_]\w*::)*)FaultClass::\w+,\s*')
IMPORT = re.compile(r"^(?P<head>[ \t]*(?:pub(?:\([^)]*\))? )?use [\w:]+::\{)(?P<group>[^;{}]*\bLocalizedLabel\b[^;{}]*)\};", re.M)


def classes() -> dict[tuple[str, str], str]:
    out: dict[tuple[str, str], str] = {}
    for path in sorted(LISTS.glob("family-*.json")):
        for entry in json.loads(path.read_text()):
            assert entry["class"] in VARIANT, (path.name, entry)
            out[(entry["owner"], entry["code"])] = entry["class"]
    return out


def main(dry_run: bool) -> None:
    table = classes()
    missing: list[str] = []
    changed_files = 0
    sites = 0
    for current, dirs, names in os.walk(PLUGINS):
        dirs[:] = [d for d in dirs if d not in {"node_modules", "target", "🤖️generated"}]
        for name in names:
            if name != "🦀️.rs":
                continue
            path = Path(current) / name
            text = path.read_text()
            if ".fault(" not in text:
                continue
            owner = str(path.relative_to(PLUGINS)).split("/")[0]
            bare = False

            def insert(match: re.Match) -> str:
                nonlocal bare, sites
                cls = table.get((owner, match.group("code")))
                if cls is None:
                    missing.append(f"{owner} {match.group('code')} {path.relative_to(OVERLAY)}")
                    return match.group(0)
                bare = bare or not match.group("qual")
                sites += 1
                return f'.fault("{match.group("code")}", {match.group("qual")}FaultClass::{VARIANT[cls]}, {match.group("qual")}LocalizedLabel::'

            def update(match: re.Match) -> str:
                nonlocal sites
                cls = table.get((owner, match.group("code")))
                if cls is None:
                    missing.append(f"{owner} {match.group('code')} {path.relative_to(OVERLAY)}")
                    return match.group(0)
                sites += 1
                return f'.fault("{match.group("code")}", {match.group("qual")}FaultClass::{VARIANT[cls]}, '

            converted = CLASSED.sub(update, DECLARATION.sub(insert, text))
            if bare and not re.search(r"\bFaultClass\b[^:]", converted.split("fn ", 1)[0]):
                imported = IMPORT.subn(lambda match: match.group(0) if re.search(r"\bFaultClass\b", match.group("group")) else f"{match.group('head')}{match.group('group').replace('LocalizedLabel', 'FaultClass, LocalizedLabel', 1)}}};", converted, count=1)
                converted = imported[0]
                if imported[1] == 0:
                    imported = re.subn(r"^([ \t]*(?:pub(?:\([^)]*\))? )?use [\w:]+::)LocalizedLabel;", r"\1{FaultClass, LocalizedLabel};", converted, count=1, flags=re.M)
                    converted = imported[0]
                if imported[1] == 0:
                    missing.append(f"import {path.relative_to(OVERLAY)}")
            if converted != text:
                changed_files += 1
                if not dry_run:
                    path.write_text(converted)
    catalog_table = {code: cls for (owner, code), cls in table.items() if owner == "framework"}
    lines = CATALOG.read_text().split("\n")
    for index, line in enumerate(lines):
        match = re.match(r'^(\s*\{"code": "(?P<code>[^"]+)", )(?:"class": "[a-z-]+", )?("en": )', line)
        if match is None:
            continue
        cls = catalog_table.get(match.group("code"))
        if cls is None:
            missing.append(f"catalog {match.group('code')}")
            continue
        lines[index] = f'{match.group(1)}"class": "{cls}", "en": ' + line[match.end():]
    json.loads("\n".join(lines))
    if not dry_run:
        CATALOG.write_text("\n".join(lines))
    print(f"declarations={sites} files={changed_files} unclassified/import-misses={len(missing)}")
    for entry in missing[:60]:
        print("  missing", entry)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])
