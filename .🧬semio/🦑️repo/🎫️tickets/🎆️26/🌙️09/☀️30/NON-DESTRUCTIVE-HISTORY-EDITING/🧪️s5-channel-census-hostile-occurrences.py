#!/usr/bin/env python3
"""🔖️ S5-CHANNEL: closes the channel-version census blind spot the v21 bump fell into.

A literal equal to a declared hostile value was always read as hostile, so a +1 bump (old pin = new "previous" hostile row)
left the stale pin of `🧬️catalog-publication` invisible. Schema-first: a consumer that declares `hostileValues` now also
declares `hostileOccurrences`; the census reports a `hostile` finding when the file holds another number of hostile
literals, and the generator refuses to rewrite such a file. Covers the schema, the type, the census + generator, the
portable corpus (2 census cases, 2 admission cases), both laws and the five registered hostile consumers.
Explicit file list (the keys of `ROWS`); every anchor must match exactly its count or nothing is written.
Usage: python3 🧪️s5-channel-census-hostile-occurrences.py [--apply]
"""
from __future__ import annotations

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
OS = "🧰️framework/🛍️products/💻️os"
TOOL = f"{OS}/🔨️modules/🧑‍💻dev/🔖️channel-version"
CONSUMER = '{"ownerRoot":"future","document":{"schema":"semio.os.channel-version-consumers/v1","consumers":[{"path":"wire.json","occurrences":3,"hostileValues":[20,22],"hostileOccurrences":2}]}}'


def census_case(case_id: str, last: int, expected: str) -> str:
    return (
        '    {\n      "id": "%s",\n      "pin": 21,\n      "owners": [\n        {\n          "ownerRoot": "future",\n          "document": {\n'
        '            "schema": "semio.os.channel-version-consumers/v1",\n            "consumers": [\n              {\n                "path": "wire.json",\n'
        '                "occurrences": 3,\n                "hostileValues": [\n                  20,\n                  22\n                ],\n'
        '                "hostileOccurrences": 2\n              }\n            ]\n          }\n        }\n      ],\n      "candidates": [\n        "future/wire.json"\n      ],\n'
        '      "files": {\n        "future/wire.json": "{\\"appChannelVersion\\":20,\\"channelVersion\\":22,\\"executionProtocol\\":%d}"\n      },\n      "expected": %s\n    }'
    ) % (case_id, last, expected)


def admission_case(case_id: str, consumer: str) -> str:
    return (
        '    {\n      "id": "%s",\n      "input": [\n        {\n          "ownerRoot": "future/owner",\n          "document": {\n'
        '            "schema": "semio.os.channel-version-consumers/v1",\n            "consumers": [\n              {\n                "path": "schema.json",\n'
        '                "occurrences": 2,\n%s\n              }\n            ]\n          }\n        }\n      ],\n      "accepted": false,\n      "paths": []\n    },\n'
    ) % (case_id, consumer)


ROWS: dict[str, list[tuple[str, str, int]]] = {
    f"{TOOL}/📣️contributions/🧬️schema/🔣️.json": [
        ('      "required": [\n        "path",\n        "occurrences"\n      ],\n      "properties": {\n        "path": {',
         '      "required": [\n        "path",\n        "occurrences"\n      ],\n      "anyOf": [\n        {\n          "type": "object",\n          "required": [\n            "hostileValues",\n            "hostileOccurrences"\n          ],\n          "properties": {\n            "hostileValues": true,\n            "hostileOccurrences": true\n          }\n        },\n        {\n          "propertyNames": {\n            "enum": [\n              "path",\n              "occurrences",\n              "arbitrary",\n              "guest",\n              "derived"\n            ]\n          }\n        }\n      ],\n      "properties": {\n        "path": {', 1),
        ('        "hostileValues": {\n          "type": "array",\n          "maxItems": 64,',
         '        "hostileOccurrences": {\n          "type": "integer",\n          "minimum": 1,\n          "maximum": 4096\n        },\n        "hostileValues": {\n          "type": "array",\n          "minItems": 1,\n          "maxItems": 64,', 1),
    ],
    f"{TOOL}/📣️contributions/🟦️.ts": [
        ("  hostileValues?: readonly number[];\n", "  hostileValues?: readonly number[];\n  hostileOccurrences?: number;\n", 1),
    ],
    f"{TOOL}/🔍️census/🟦️.ts": [
        ('problem: "unregistered" | "missing" | "occurrences" | "drift"; detail: string', 'problem: "unregistered" | "missing" | "occurrences" | "hostile" | "drift"; detail: string', 1),
        ("/** 🔎️ The census: every candidate file is a registered consumer, holds exactly its declared literals, and every literal that\n * is not a declared hostile value (or arbitrary) equals the pin. */",
         "/** 🔎️ The census: every candidate file is a registered consumer, holds exactly its declared literals and exactly its declared\n * hostile ones (a stale pin that equals a hostile value is otherwise invisible), and every other literal equals the pin. */", 1),
        ("    if (consumer.arbitrary) continue;\n    const drifted = literals.filter(",
         "    if (consumer.arbitrary) continue;\n    const hostile = literals.filter((literal) => (consumer.hostileValues ?? []).includes(literal.value)).length;\n    if (hostile !== (consumer.hostileOccurrences ?? 0)) findings.push({ path: consumer.path, problem: \"hostile\", detail: `declares ${consumer.hostileOccurrences ?? 0} hostile literal(s), holds ${hostile}: a literal equal to a hostile value is a stale pin or an undeclared vector` });\n    const drifted = literals.filter(", 1),
        ("    const text = source.readText(consumer.path);\n    const drifted = channelVersionLiterals(text).filter((literal) => literal.value !== pin && !(consumer.hostileValues ?? []).includes(literal.value));\n",
         "    const text = source.readText(consumer.path);\n    const literals = channelVersionLiterals(text);\n    if (literals.filter((literal) => (consumer.hostileValues ?? []).includes(literal.value)).length !== (consumer.hostileOccurrences ?? 0)) {\n      refused.push(`${consumer.path}: holds another number of hostile literals than declared, set the stale pin literal by hand`);\n      continue;\n    }\n    const drifted = literals.filter((literal) => literal.value !== pin && !(consumer.hostileValues ?? []).includes(literal.value));\n", 1),
        ("/** ✍️ Writes the pin into every drifted literal of every consumer the caller may rewrite: never an arbitrary or hostile\n * literal,", "/** ✍️ Writes the pin into every drifted literal of every consumer the caller may rewrite: never an arbitrary or hostile\n * literal, never a file whose hostile literal count differs from its declaration,", 1),
    ],
    f"{TOOL}/📣️contributions/🧫️fixtures/🔣️.json": [
        ('                "path": "schema.json",\n                "occurrences": 2,\n                "hostileValues": [\n                  13\n                ]\n              }',
         '                "path": "schema.json",\n                "occurrences": 2,\n                "hostileValues": [\n                  13\n                ],\n                "hostileOccurrences": 1\n              }', 1),
        ('    {\n      "id": "deleted-owner-removes-contribution",',
         admission_case("hostile-values-without-their-count-refuse", '                "hostileValues": [\n                  13\n                ]')
         + admission_case("hostile-count-without-values-refuses", '                "hostileOccurrences": 1')
         + '    {\n      "id": "deleted-owner-removes-contribution",', 1),
        ('                "path": "wire.json",\n                "occurrences": 1,\n                "hostileValues": [\n                  13\n                ]\n              }',
         '                "path": "wire.json",\n                "occurrences": 1,\n                "hostileValues": [\n                  13\n                ],\n                "hostileOccurrences": 1\n              }', 1),
        ('        "future/wire.json": "{\\"appChannelVersion\\":7}"\n      },\n      "expected": []\n    }\n  ]\n}',
         '        "future/wire.json": "{\\"appChannelVersion\\":7}"\n      },\n      "expected": []\n    },\n'
         + census_case("stale-pin-equal-to-a-hostile-value-refuses", 20, '[\n        "hostile"\n      ]') + ",\n"
         + census_case("hostile-rows-beside-the-pin-are-admitted", 21, "[]") + "\n  ]\n}", 1),
    ],
    f"{TOOL}/📣️contributions/🧪️tests/🟦️.ts": [
        ("      assert.equal(consumer.arbitrary || admitted, !row.expected.includes(\"drift\"), `${row.id}: independent JSON/AJV pin oracle`);\n",
         "      assert.equal(consumer.arbitrary || admitted, !row.expected.includes(\"drift\"), `${row.id}: independent JSON/AJV pin oracle`);\n      const hostile = values.filter(value => (consumer.hostileValues ?? []).includes(value as number)).length;\n      assert.equal(consumer.arbitrary || hostile === (consumer.hostileOccurrences ?? 0), !row.expected.includes(\"hostile\"), `${row.id}: independent hostile-count oracle`);\n", 1),
    ],
    f"{OS}/🔨️modules/🧑‍💻dev/🧪️tests/🔖️channel-version/🟦️.ts": [
        ('{ path: "hostile.json", occurrences: 1, hostileValues: [13] }', '{ path: "hostile.json", occurrences: 1, hostileValues: [13], hostileOccurrences: 1 }', 1),
    ],
    f"{OS}/🧫️fixtures/📡️channel/📇️consumers.json": [
        ('      "occurrences": 3,\n      "hostileValues": [\n        13\n      ]\n', '      "occurrences": 3,\n      "hostileValues": [\n        13\n      ],\n      "hostileOccurrences": 1\n', 1),
        ('      "occurrences": 3,\n      "hostileValues": [\n        20,\n        22\n      ]\n', '      "occurrences": 3,\n      "hostileValues": [\n        20,\n        22\n      ],\n      "hostileOccurrences": 2\n', 1),
        ('      "occurrences": 1,\n      "hostileValues": [\n        19\n      ]\n', '      "occurrences": 1,\n      "hostileValues": [\n        19\n      ],\n      "hostileOccurrences": 1\n', 1),
        ('      "occurrences": 2,\n      "hostileValues": [\n        13,\n        14\n      ]\n', '      "occurrences": 2,\n      "hostileValues": [\n        13,\n        14\n      ],\n      "hostileOccurrences": 2\n', 1),
    ],
    "🌎️hub/🔖️channel-version/📇️consumers.json": [
        ('      "occurrences": 2,\n      "hostileValues": [\n        13\n      ]\n', '      "occurrences": 2,\n      "hostileValues": [\n        13\n      ],\n      "hostileOccurrences": 2\n', 1),
    ],
}


def main() -> None:
    apply = "--apply" in sys.argv
    if not (ROOT / ".git").exists() or not ROWS:
        raise SystemExit("[DEBUG] not the repo root or no rows: refusing")
    results: dict[str, str] = {}
    failed = False
    for path, rows in ROWS.items():
        target = ROOT / path
        if not target.is_file():
            raise SystemExit(f"[DEBUG] missing file: {path}")
        text = target.read_text()
        print(f"== {path}")
        for index, (old, new, count) in enumerate(rows, 1):
            found = text.count(old)
            failed |= found != count
            print(f"   {index} {'ok' if found == count else 'MISMATCH'} x{found}/{count}: {old.strip().splitlines()[0][:100]}")
            text = text.replace(old, new)
        results[path] = text
    if failed:
        raise SystemExit("[DEBUG] anchors do not match: nothing written")
    if apply:
        for path, text in results.items():
            (ROOT / path).write_text(text)
    print(f"[DEBUG] {len(results)} files {'written' if apply else 'would change (dry run)'}")


if __name__ == "__main__":
    main()
