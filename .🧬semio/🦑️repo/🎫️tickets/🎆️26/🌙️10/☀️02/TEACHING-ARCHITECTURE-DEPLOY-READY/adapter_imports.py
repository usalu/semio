"""Points the Protocol v2 TypeScript adapters that were written before checkpoint 667 at the test adapter module that
checkpoint moved (`🧰️framework/🔨️modules/🧪️test/🔌️adapter`), spelled as their migrated siblings spell it, and removes
the docstring that checkpoint's codemod duplicated in the adapters it touched.

    python adapter_imports.py          # report
    python adapter_imports.py --write  # rewrite
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
OWNERS = ["🧰️framework/🛍️products/❓️quiz/🧪️tests", "🧰️framework/🛍️products/🐾️pets/🧪️tests"]
OLD = '"../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts"'
NEW = '"../../../../\\uD83D\\uDD28\\uFE0Fmodules/\\uD83E\\uDDEA\\uFE0Ftest/\\uD83D\\uDD0C\\uFE0Fadapter/\\uD83D\\uDFE6\\uFE0F.ts"'
DOCSTRING = re.compile(r"\A(/\*\*.*?\*/\n)\1", re.S)

write = "--write" in sys.argv
for owner in OWNERS:
    for path in sorted((ROOT / owner).glob("*/🟦️.ts")):
        source = path.read_text(encoding="utf-8")
        target = DOCSTRING.sub(r"\1", source.replace(OLD, NEW))
        if target == source:
            continue
        print(path.relative_to(ROOT).as_posix(), "import" if OLD in source else "", "docstring" if DOCSTRING.match(source.replace(OLD, NEW)) else "")
        if write:
            path.write_text(target, encoding="utf-8", newline="\n")
