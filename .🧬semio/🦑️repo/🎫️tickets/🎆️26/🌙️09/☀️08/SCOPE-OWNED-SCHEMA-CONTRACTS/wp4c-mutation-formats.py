#!/usr/bin/env python3
"""🏷️ WP4c — honest `x-semio-formats` on mutation `📝️text` facet documents, and removal of the
`🚪️io/🧬️mutations/📝️text` schema documents the ledger row 107 decision retires.

Two subcommands, both idempotent and both refusing rather than guessing:

    formats [--apply]   annotate every `🧬️schema/🧬️mutations/📝️text/🔣️.json` root export with the
                        formats it actually exists in. The sibling `🦀️.rs`/`🔗️.graphql`/`🛰️.proto`
                        of a text facet carry the codec and the grammar for the mutation enum, not a
                        same-named entity, so the export lives in JSON Schema and in TypeScript only.
                        The script MEASURES that per document (it never assumes the pair) and refuses
                        a document whose measured format set differs from an annotation already there.

    io [--apply]        delete `🚪️io/🧬️mutations/📝️text/🔣️.json`, the schema document of a transport
                        collection that is not a schema module (ledger row 107). A document that is
                        not an empty stub is only deleted when its own union is already restated by
                        the subset's `🧬️schema/🧬️mutations/🔣️.json` aggregate, branch for branch.

https://json-schema.org/draft-07/schema
"""

from __future__ import annotations

import json
import os
import re
import sys

REPO = os.path.dirname(os.path.abspath(__file__)).split("/.\U0001f9ecsemio/")[0]
KEYWORD = "x-semio-formats"
JSON_FORMAT = "\U0001f523️jsonschema"
TS_FORMAT = "\U0001f7e6️typescript"
RUST_FORMAT = "\U0001f980️rust"
GRAPHQL_FORMAT = "\U0001f517️graphql"
PROTO_FORMAT = "\U0001f6f0️protobuf"
GRAPHQL_KEYWORDS = ("type", "input", "enum", "interface", "union", "scalar")
PLUGINS = os.path.join(REPO, "✏️s/\U0001f50c️plugins")
STDIO = "\U0001f5c4️stdio"


def read(path: str) -> str | None:
    try:
        with open(path, encoding="utf8") as handle:
            return handle.read()
    except FileNotFoundError:
        return None


def write_json(path: str, document: dict) -> None:
    with open(path, "w", encoding="utf8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def text_facet_dirs(under: str, parent: str) -> list[str]:
    found = []
    for base, dirs, _files in os.walk(under):
        dirs[:] = [d for d in dirs if not d.startswith(".")]
        if os.path.basename(base) != "\U0001f4dd️text":
            continue
        mutations = os.path.dirname(base)
        if os.path.basename(mutations) != "\U0001f9ec️mutations":
            continue
        if os.path.basename(os.path.dirname(mutations)) != parent:
            continue
        if STDIO in base:
            continue
        found.append(base)
    return sorted(found)


def measured_formats(directory: str, export: str) -> list[str]:
    name = re.escape(export)
    present = [JSON_FORMAT]
    rust = read(os.path.join(directory, "\U0001f980️.rs"))
    graphql = read(os.path.join(directory, "\U0001f517️.graphql"))
    proto = read(os.path.join(directory, "\U0001f6f0️.proto"))
    typescript = read(os.path.join(directory, "\U0001f7e6️.ts"))
    if rust is not None and re.search(rf"^\s*pub\s+(?:struct|enum|type)\s+{name}\b", rust, re.M):
        present.append(RUST_FORMAT)
    if graphql is not None and re.search(rf"^\s*(?:{'|'.join(GRAPHQL_KEYWORDS)})\s+{name}\b", graphql, re.M):
        present.append(GRAPHQL_FORMAT)
    if proto is not None and re.search(rf"^\s*message\s+{name}\b", proto, re.M):
        present.append(PROTO_FORMAT)
    if typescript is not None and re.search(rf"^\s*export\s+(?:interface|type|const|class)\s+{name}\b", typescript, re.M) and re.search(rf"^\s*export\s+(?:(?:async\s+)?function|const|let|declare\s+function)\s+parse{name}\b", typescript, re.M):
        present.append(TS_FORMAT)
    return present


def command_formats(apply: bool) -> int:
    documents = 0
    changed = 0
    refused = []
    complete = 0
    for directory in text_facet_dirs(PLUGINS, "\U0001f9ec️schema"):
        path = os.path.join(directory, "\U0001f523️.json")
        raw = read(path)
        if raw is None:
            continue
        document = json.loads(raw)
        export = document.get("title")
        if not isinstance(export, str) or not re.fullmatch(r"[A-Z][A-Za-z0-9]*", export):
            refused.append(f"{path}: root export keyword `title` is {json.dumps(export, ensure_ascii=False)}, not a PascalCase export id")
            continue
        documents += 1
        formats = measured_formats(directory, export)
        if len(formats) == 5:
            complete += 1
            continue
        declared = document.get(KEYWORD)
        if declared is not None and list(declared) != formats:
            refused.append(f"{path}: declares {declared} and the files carry {formats}")
            continue
        if declared == formats:
            continue
        document[KEYWORD] = formats
        changed += 1
        if apply:
            write_json(path, document)
    print(f"documents={documents} annotated={changed} alreadyCompleteInEveryFormat={complete} refused={len(refused)}")
    for line in refused:
        print(f"  {line}")
    return 1 if refused else 0


def aggregate_branch_ids(subset: str) -> set[str]:
    path = os.path.join(subset, "\U0001f9ec️schema/\U0001f9ec️mutations/\U0001f523️.json")
    raw = read(path)
    if raw is None:
        return set()
    document = json.loads(raw)
    found = set()

    def walk(node) -> None:
        if isinstance(node, dict):
            ref = node.get("$ref")
            if isinstance(ref, str):
                found.add(ref.split("#", 1)[0])
            for value in node.values():
                walk(value)
        elif isinstance(node, list):
            for value in node:
                walk(value)

    walk(document)
    return found


def command_io(apply: bool) -> int:
    deleted = 0
    refused = []
    for directory in text_facet_dirs(PLUGINS, "\U0001f6aa️io"):
        path = os.path.join(directory, "\U0001f523️.json")
        raw = read(path)
        if raw is None:
            continue
        document = json.loads(raw)
        subset = os.path.dirname(os.path.dirname(os.path.dirname(directory)))
        keys = set(document) - {"$schema", "$id", "title", "type"}
        if keys:
            branches = aggregate_branch_ids(subset)
            kinds = {branch.get("title") for branch in document.get("oneOf", []) if isinstance(branch, dict)}
            covered = {kind for kind in kinds if kind and any(re.sub(r"(?<!^)(?=[A-Z])", "-", kind).lower() + "/schema.json" in branch for branch in branches)}
            if not kinds or covered != kinds:
                refused.append(f"{path}: carries {sorted(keys)} and the subset aggregate does not restate {sorted(kinds - covered)}")
                continue
        deleted += 1
        if apply:
            os.remove(path)
    print(f"deleted={deleted} refused={len(refused)}")
    for line in refused:
        print(f"  {line}")
    return 1 if refused else 0


def main() -> int:
    argv = sys.argv[1:]
    apply = "--apply" in argv
    command = next((entry for entry in argv if not entry.startswith("--")), None)
    if command == "formats":
        return command_formats(apply)
    if command == "io":
        return command_io(apply)
    print(__doc__)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
