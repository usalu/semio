#!/usr/bin/env python3
"""🐍️ T3 helper: annotate an oracle test's `🧫️fixtures` load with a generated readonly type.

Usage: python3 🐍️t3-annotate-fixtures.py <test 🟦️.ts> <TypeName> <docstring> [<TypeName> <docstring> …]

Finds every `const <name> = JSON.parse(readFileSync(new URL("…🧫️fixtures/…", import.meta.url), "utf8"));`
in the test file (in source order), derives a `readonly` structural type from the fixture document on
disk, inserts the type aliases above the first exported function and annotates each `const`.

Why an explicit annotation rather than Ajv's `compile<T>` type guard: `assert(validate(fixture))` on an
`any` narrows the binding to `unknown` (Ajv's `ValidateFunction` defaults to `ValidateFunction<unknown>`),
which is what produced the TS18046/TS2698 wall these oracles carried. An annotated `const` states the
shape the schema already enforces at runtime and leaves the assertion doing only its runtime job.
"""
import io
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from importlib import import_module

render = import_module("🐍️t3-fixture-types").render

LOAD = re.compile(
    r'const (?P<name>\w+)\s*=\s*JSON\.parse\(readFileSync\(new URL\(\s*"(?P<url>[^"]*🧫️fixtures[^"]*)"\s*,\s*import\.meta\.url\)\s*,\s*"utf8"\)\)'
)


def main(argv):
    if len(argv) < 4 or len(argv) % 2 != 0:
        raise SystemExit(__doc__)
    path = argv[1]
    names = [(argv[i], argv[i + 1]) for i in range(2, len(argv), 2)]
    source = io.open(path, encoding="utf-8").read()
    loads = list(LOAD.finditer(source))
    if len(loads) != len(names):
        raise SystemExit(f"{path}: {len(loads)} fixture loads but {len(names)} type names given: {[m.group('name') for m in loads]}")

    blocks = []
    for (type_name, doc), match in zip(names, loads):
        fixture_path = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(path)), match.group("url")))
        document = json.load(io.open(fixture_path, encoding="utf-8"))
        blocks.append(f"/** {doc} */\ntype {type_name} = {render(document, 0)};\n")

    for (type_name, _), match in zip(reversed(names), reversed(loads)):
        start, end = match.span()
        replaced = match.group(0).replace(f"const {match.group('name')} =", f"const {match.group('name')}: {type_name} =", 1)
        source = source[:start] + replaced + source[end:]

    anchor = re.search(r"^(/\*\*(?:.|\n)*?\*/\n)?export (?:async )?function ", source, re.M)
    if anchor is None:
        raise SystemExit(f"{path}: no exported function to anchor the type aliases above")
    source = source[: anchor.start()] + "\n".join(blocks) + "\n" + source[anchor.start():]
    io.open(path, "w", encoding="utf-8").write(source)
    print(f"{path}: annotated {len(names)} fixture load(s)")


if __name__ == "__main__":
    main(sys.argv)
