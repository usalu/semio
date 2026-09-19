#!/usr/bin/env python3
"""🧫️ Annotates `await Bun.file(new URL("…🧫️fixtures/…")).json()` bindings with a real fixture type.

`Bun.file().json()` resolves to `any`; the very next line usually runs the binding through an Ajv
`ValidateFunction`, whose default parameter is `unknown`, so `assert(validate(fixture))` *narrows the
binding down to `unknown`* and every later property read fails (TS18046 / TS2698). The cure is not a
cast — it is to state the shape the schema already enforces, so the assertion keeps doing only its
runtime job.

This derives that shape structurally from the fixture document on disk (arrays collapse to the union
of keys/values their rows show, via `🐍️t3-fixture-types.py`) and emits a `readonly` type alias above
the first binding, then annotates each `const`.

Usage: python3 🐍️t3-annotate-bun-fixtures.py [--apply] <test 🟦️.ts> <bindingName>=<TypeName> …
"""
import io
import os
import re
import sys
from importlib import import_module

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
render = import_module("🐍️t3-fixture-types").render

BINDING = re.compile(
    r'const (?P<name>\w+)(?P<annotation>: [^=]+)? = await Bun\.file\(new URL\("(?P<url>[^"]+)", import\.meta\.url\)\)\.json\(\);'
)


def main(argv):
    apply = "--apply" in argv
    positional = [argument for argument in argv[1:] if not argument.startswith("--")]
    if len(positional) < 2:
        raise SystemExit(__doc__)
    path, pairs = positional[0], dict(pair.split("=", 1) for pair in positional[1:])
    source = io.open(path, encoding="utf-8").read()
    matches = [match for match in BINDING.finditer(source) if match.group("name") in pairs]
    found = {match.group("name") for match in matches}
    missing = set(pairs) - found
    if missing:
        raise SystemExit(f"{path}: no Bun.file().json() binding for {sorted(missing)}")

    blocks = []
    for match in matches:
        type_name = pairs[match.group("name")]
        fixture = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(path)), match.group("url")))
        document = __import__("json").load(io.open(fixture, encoding="utf-8"))
        blocks.append(f"/** 🧫️ Shape of `{os.path.relpath(fixture, os.path.dirname(os.path.abspath(path)))}`, the document its JSON Schema validates at load. */\ntype {type_name} = {render(document, 0)};")

    for match in reversed(matches):
        type_name = pairs[match.group("name")]
        replacement = f'const {match.group("name")}: {type_name} = await Bun.file(new URL("{match.group("url")}", import.meta.url)).json();'
        source = source[: match.start()] + replacement + source[match.end():]

    anchor = matches[0].start()
    line_start = source.rfind("\n", 0, anchor) + 1
    source = source[:line_start] + "\n".join(blocks) + "\n" + source[line_start:]
    if apply:
        io.open(path, "w", encoding="utf-8").write(source)
    print(f"{path}: {'annotated' if apply else 'would annotate'} {len(matches)} fixture binding(s): {sorted(pairs)}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
