#!/usr/bin/env python3
"""🔗️ WP4d — makes every cross-document `$ref` under `🧰️framework/🔨️modules` address an export.

Two mechanical rules from `📋️execution-contract.md` §A, applied together so the tree is never half
migrated:

1. A reference that names its OWN document's `$id` is a module-internal reference written the long
   way. It becomes a bare JSON pointer (`#/definitions/<helper>`), which the contract declares legal
   and the harness resolves by pointer.
2. A reference into ANOTHER module's private `definitions` addresses no export. The referenced helper
   is promoted to a named `$defs` export of the module that owns it, every reference to it (internal
   and cross-document, repository-wide, JSON and TypeScript) is repointed at the export, and the
   private helper ceases to exist — so exactly one name addresses one contract.

Reads and writes the working tree; prints one line per edit. Usage: wp4d-framework-refs.py [--dry]
"""
import json
import os
import re
import subprocess
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))

# 🏷️ module file → { private helper: exported name }. The export name is the helper's PascalCase, except
# where the module already exports that name for a different contract (`framework.actor.lifetime` exports
# `Lifetime`, the lifecycle-message union, so the coordinate triple takes the Rust type's own name
# `ActorInstanceLifetime`).
PROMOTIONS = {
    "🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema/🔣️.json": {"resources": "Resources"},
    "🧰️framework/🔨️modules/🎭️actor/📃️page/🧬️schema/🔣️.json": {"word": "Word"},
    "🧰️framework/🔨️modules/🎭️actor/📤️return/🧬️schema/🔣️.json": {"u64": "U64", "pageReceipt": "PageReceipt", "result": "Result"},
    "🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🧬️schema/🔣️.json": {"transportRequestSequence": "TransportRequestSequence"},
    "🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧬️schema/🔣️.json": {"u64": "U64", "lifetime": "InstanceLifetime"},
    "🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧬️schema/🔣️.json": {
        "generation": "Generation",
        "identity": "Identity",
        "page": "Page",
        "state": "State",
        "domainPatch": "DomainPatch",
    },
}

TEXT_SUFFIXES = (".json", ".ts", ".tsx", ".rs", ".mjs", ".js")
PARTITION = "🧰️framework/🔨️modules/"


def read(rel):
    """📖️ UTF-8 text, or `None` for a binary file or an index entry with no working-tree file."""
    try:
        with open(os.path.join(ROOT, rel), encoding="utf-8") as handle:
            return handle.read()
    except (UnicodeDecodeError, FileNotFoundError):
        return None


def write(rel, text, dry):
    if dry:
        return
    with open(os.path.join(ROOT, rel), "w", encoding="utf-8") as handle:
        handle.write(text)


def repository_files():
    """🗂️ Every tracked text file. Untracked generated trees are rewritten by their generator, never here."""
    listing = subprocess.run(["git", "-C", ROOT, "ls-files", "-z"], check=True, capture_output=True).stdout
    return [rel for rel in listing.decode("utf-8").split("\0") if rel.endswith(TEXT_SUFFIXES)]


def main():
    dry = "--dry" in sys.argv
    files = sorted(repository_files())
    ids = {rel: json.loads(read(rel))["$id"] for rel in PROMOTIONS}
    cross = {f"{ids[rel]}#/definitions/{helper}": f"{ids[rel]}#/$defs/{exported}" for rel, table in PROMOTIONS.items() for helper, exported in table.items()}
    changed = 0

    # 1️⃣ Repository-wide: every reference into a promoted helper now names the export.
    for rel in files:
        text = read(rel)
        if text is None:
            continue
        after = text
        for old, new in cross.items():
            after = after.replace(old, new)
        if after != text:
            changed += 1
            print(f"[wp4d] export-addressed refs → {rel}")
            write(rel, after, dry)

    # 2️⃣ Every schema module in the partition: a reference to the module's own `$id` is internal.
    for rel in files:
        if not (rel.startswith(PARTITION) and rel.endswith("/🧬️schema/🔣️.json")):
            continue
        text = read(rel)
        if text is None:
            continue
        own = json.loads(text).get("$id")
        if not isinstance(own, str):
            continue
        after = text.replace(f"{own}#/definitions/", "#/definitions/")
        if after != text:
            changed += 1
            print(f"[wp4d] self-$id refs → bare pointers in {rel}")
            write(rel, after, dry)

    # 3️⃣ The owning modules: the helper becomes the export, internal pointers follow it, and the
    #    private name is gone — one contract, one address.
    for rel, table in PROMOTIONS.items():
        text = read(rel)
        for helper, exported in table.items():
            text = re.sub(rf'"#/definitions/{helper}"', f'"#/$defs/{exported}"', text)
        document = json.loads(text)
        for helper, exported in table.items():
            document.setdefault("$defs", {})[exported] = document["definitions"].pop(helper)
        if len(document.get("definitions", {})) == 0:
            document.pop("definitions", None)
        rendered = json.dumps(document, ensure_ascii=False, indent=2) + "\n"
        if rendered != read(rel):
            changed += 1
            print(f"[wp4d] {rel}: exports {sorted(table.values())}")
            write(rel, rendered, dry)

    print(f"[wp4d] {changed} file(s) {'would change' if dry else 'changed'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
