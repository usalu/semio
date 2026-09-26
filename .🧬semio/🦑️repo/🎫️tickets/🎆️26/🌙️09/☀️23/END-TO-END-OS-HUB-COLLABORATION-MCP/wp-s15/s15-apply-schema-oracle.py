#!/usr/bin/env python3
"""S15: routes every draft-07 Ajv construction of the os tests, the os/renderer package scripts and the renderer laws that hand-registered
`x-semio-*` vendor keywords through the one shared oracle `💻️os/🧪️tests/🧬️schema-oracle` (`semioSchemaAjvV1`). Idempotent."""
import os
import re
import sys

OS = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os"
ORACLE = os.path.join(OS, "🧪️tests/🧬️schema-oracle/🟦️.ts")
FILES = [
    "🧪️tests/🗄️plugin-module-store/🟦️.ts",
    "🧪️tests/🔍️plugin-module-resolution/🟦️.ts",
    "🧪️tests/🧩️execution-target-module-resolution/🟦️.ts",
    "🧪️tests/🔌️document-link-shortage/🟦️.ts",
    "🧪️tests/🧪️backbone-envelope-io/🟦️.ts",
    "🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts",
    "🧪️tests/🧩️plugin-module-bundle/🟦️.ts",
    "🧪️tests/🗃️persistence-data-class/🟦️.ts",
    "🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts",
    "🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts",
    "🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts",
    "🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts",
    "📦️packages/🟦️typescript/📜️script.ts",
    "📦️packages/🦀️rust/📜️script.ts",
]

for rel in FILES:
    path = os.path.join(OS, rel)
    source = open(path, encoding="utf-8").read()
    before = source
    source = re.sub(r'^[ \t]*const \{ default: Ajv \} = await import\("ajv"\);\n', "", source, flags=re.M)
    spec = os.path.relpath(ORACLE, os.path.dirname(path))
    spec = spec if spec.startswith(".") else "./" + spec
    source = re.sub(r'^([ \t]*)const Ajv = \(await import\("ajv"\)\)\.default;$', lambda match: f'{match.group(1)}const {{ semioSchemaAjvV1 }} = await import("{spec}");', source, flags=re.M)
    source = re.sub(r'\bnew Ajv\(', "semioSchemaAjvV1(", source)
    source = re.sub(r'\.addKeyword\("x-semio-[A-Za-z-]+"\)', "", source)
    source = re.sub(r'\s*\.addKeyword\(\{ keyword: "x-semio-[A-Za-z-]+", schemaType: "[a-z]+" \}\)', "", source)
    source = re.sub(r'^[ \t]*\w+\.addKeyword\(\{ keyword: "x-semio-[A-Za-z-]+", schemaType: "[a-z]+" \}\);\n', "", source, flags=re.M)
    body = re.sub(r'^import .*$', "", source, flags=re.M)
    uses_ajv = re.search(r'(:\s*Ajv\b|<Ajv\b|\bAjv\.|typeof Ajv\b|\bnew Ajv\b|\bAjv\[|\bAjv\))', body) is not None
    if not uses_ajv:
        source = re.sub(r'^import Ajv from "ajv";\n', "", source, flags=re.M)
        source = re.sub(r'^import Ajv, \{ (type [^}]*) \} from "ajv";$', r'import { \1 } from "ajv";', source, flags=re.M)
    if source != before and "semioSchemaAjvV1 }" not in source and "semioSchemaAjvV1," not in source:
        lines = source.split("\n")
        imports = [index for index, line in enumerate(lines[:120]) if line.startswith("import ") or line.startswith('} from "')]
        at = imports[-1] + 1 if imports else next(index for index, line in enumerate(lines) if re.match(r'(export |type |const |function |async function )', line))
        lines.insert(at, f'import {{ semioSchemaAjvV1 }} from "{spec}";')
        source = "\n".join(lines)
    if source != before:
        open(path, "w", encoding="utf-8").write(source)
        print(f"rewrote {rel} (Ajv still referenced: {uses_ajv})")
    else:
        print(f"unchanged {rel}", file=sys.stderr)
