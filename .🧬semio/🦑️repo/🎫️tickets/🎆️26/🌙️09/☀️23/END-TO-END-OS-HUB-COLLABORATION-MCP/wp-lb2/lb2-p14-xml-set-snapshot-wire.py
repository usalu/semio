#!/usr/bin/env python3
"""📸️ LB2 p14 (window 3, stdio xml): `XmlMutation::SetSnapshot` is a wire mutation like its siblings.

The `set-snapshot` leaf (`🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot`) was added to `XmlMutation` — its descriptor declares
`binaryTag: 7`, `textOpcode: set-snapshot` and the rust/json-schema/text/binary surfaces — but the aggregate surfaces never
learned it: the binary protocol (the ONLY source of op tags) has no `set-snapshot` record, the binary/text rosters and the
grammar's descriptor roster stop at `set-text`, and the aggregate JSON schema's union has no `setSnapshot` arm. Every xml
editor maps every snapshot edit to `SetSnapshot` (`snapshot_edit_set_snapshot`), so every xml detail edit was refused
`snapshot-edit.publication-codec … 📡️.protocol.semio declares no record for 'set-snapshot'` (`editor_catalog` xml_any, scratch
p9-s9). This set completes the four surfaces the leaf declares, from the leaf descriptor itself (tag, opcode, payload schema).

usage: python3 lb2-p14-xml-set-snapshot-wire.py --dry-run | --write | --revert [--root <tree>]
Backups (byte-exact, per root) under `.🧬semio/🌐hub/s14-lb2-backup/p14/<root-hash>/`.
"""
import hashlib, json, os, shutil, sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-backup/p14/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
MUTATIONS = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations"
LEAF = f"{MUTATIONS}/📸️set-snapshot/🔣️.json"
PROTOCOL = f"{MUTATIONS}/💾️binary/📡️.protocol.semio"
BINARY = f"{MUTATIONS}/💾️binary/🦀️.rs"
TEXT = f"{MUTATIONS}/📝️text/🦀️.rs"
GRAMMAR = f"{MUTATIONS}/📝️text/📖️.grammar.semio"
AGGREGATE = f"{MUTATIONS}/🔣️.json"

problems = []


def once(text, old, new, label):
    count = text.count(old)
    if count != 1:
        problems.append(f"{label}: expected 1 anchor, found {count}")
        return text
    return text.replace(old, new)


def leaf():
    descriptor = json.load(open(os.path.join(TREE, LEAF), encoding="utf-8"))
    payload = json.load(open(os.path.join(TREE, os.path.dirname(LEAF), descriptor["payloadSchema"]), encoding="utf-8"))
    if descriptor["semanticKind"] != "set-snapshot" or descriptor["textOpcode"] != "set-snapshot" or not {"text", "binary", "json-schema"} <= set(descriptor["requiredLanguageSurfaces"]):
        problems.append("leaf: descriptor is not the set-snapshot wire leaf")
    return descriptor["binaryTag"], descriptor["aggregateVariant"], payload["$id"]


TAG, VARIANT, PAYLOAD_ID = leaf()
CAMEL = VARIANT[0].lower() + VARIANT[1:]


def protocol(text):
    return once(text, "record set-text tag=6\nfield payload bytes\n", f"record set-text tag=6\nfield payload bytes\nrecord set-snapshot tag={TAG}\nfield payload bytes\n", "protocol: record")


def binary(text):
    return once(text, '("set-text", 6)];\n', f'("set-text", 6), ("set-snapshot", {TAG})];\n', "binary: roster")


def text_codec(text):
    return once(text, '"set-attribute", "set-text"];\n', '"set-attribute", "set-text", "set-snapshot"];\n', "text: roster")


def grammar(text):
    return once(text, "# Direct descriptor identities: set-declaration, set-doctype, insert-element, remove-element, set-attribute, set-text.\n", "# Direct descriptor identities: set-declaration, set-doctype, insert-element, remove-element, set-attribute, set-text, set-snapshot.\n", "grammar: roster")


def aggregate(text):
    document = json.loads(text)
    arms = document["oneOf"]
    if any(arm["properties"]["mutation"].get("const") == CAMEL for arm in arms):
        problems.append("aggregate: setSnapshot arm exists")
        return text
    arms.append({"type": "object", "additionalProperties": False, "required": ["mutation", "payload"], "properties": {"mutation": {"const": CAMEL}, "payload": {"$ref": PAYLOAD_ID}}})
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def plan():
    return {PROTOCOL: protocol, BINARY: binary, TEXT: text_codec, GRAMMAR: grammar, AGGREGATE: aggregate}


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    edits = plan()
    if mode == "--revert":
        for path in edits:
            source = os.path.join(BACKUP, path)
            if os.path.isfile(source):
                shutil.copyfile(source, os.path.join(TREE, path))
                print("restored", path)
        return
    staged = {}
    for path, edit in edits.items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
        after = edit(before)
        if after == before:
            problems.append(f"{path}: unchanged")
        staged[path] = (before, after)
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{len(staged)} files, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            backup = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                open(backup, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
