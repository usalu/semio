#!/usr/bin/env python3
"""🪪️ W2-R energy §10 (coordinator decision 2026-09-30): the outcome vocabulary gains the state-dependent `Error` codes
`mutation.target-referenced` and `mutation.target-mismatch`, and the per-plugin `mutation.id-mismatch` (fem 2d/3d, a
`replace-` whose record renames the target it selects) collapses into `mutation.target-mismatch` at `Error` level —
Rust helpers, diff docs, leaf/unit tests, committed fixture outcomes and the Python second implementations, all at once.
Idempotent: a second run changes nothing."""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
ROOT = "✏️s/🔌️plugins/🏗️fem"
OLD, NEW = "mutation.id-mismatch", "mutation.target-mismatch"
RUST = [
    (re.compile(r"\bid_mismatch\b"), "target_mismatch"),
    (re.compile(r"\bfn (replace_\w+)_rename_is_id_mismatch\b"), r"fn \1_rename_is_target_mismatch"),
    (re.compile(r'protocol::MutationOutcome::fatal\((\s*)"mutation\.id-mismatch"'), r'protocol::MutationOutcome::error(\1"mutation.target-mismatch"'),
    (re.compile(r"`mutation\.id-mismatch` \(Fatal\)"), "`mutation.target-mismatch` (Error)"),
    (re.compile(r"`mutation\.id-mismatch`, FATAL — renaming"), "`mutation.target-mismatch`, an Error — renaming"),
    (re.compile(r"`mutation\.id-mismatch`, FATAL\."), "`mutation.target-mismatch`, an Error."),
    (re.compile(r'"a replacement that renames its target is an identity breach — Fatal, and no merge policy may absorb it"'), '"a replacement that renames its target contradicts the target it selects — the state-dependent Error"'),
    (re.compile(r'"a rename through a replace is an identity breach no merge policy may absorb"'), '"a rename through a replace contradicts the target it selects — the state-dependent Error"'),
    (re.compile(r'renaming a record is an identity breach, the same Fatal level a duplicate identity raises"'), 'renaming a record contradicts the target the replace selects, the state-dependent Error"'),
    (re.compile(r"/// 🚦️ Level discipline\. `mutation\.duplicate-id`, `mutation\.id-mismatch` and `mutation\.invariant`\n/// are `Fatal`: they say the PAYLOAD is wrong, so no merge policy may absorb them and no later base\n/// can make them right\. `mutation\.target-missing` and `mutation\.target-referenced` are `Error`:\n/// they say this BASE cannot host the payload, which a different base may well be able to\."),
     "/// 🚦️ Level discipline. `mutation.duplicate-id` and `mutation.invariant` are `Fatal`: they say the PAYLOAD is\n/// wrong, so no merge policy may absorb them and no later base can make them right. `mutation.target-missing`,\n/// `mutation.target-referenced` and `mutation.target-mismatch` (a `replace-` whose record renames the target it\n/// selects) are `Error`: they say this BASE cannot host the payload as it stands."),
    (re.compile(r"/// 🪪️ The `mutation\.id-mismatch` refusal — a `replace-` selects its target by `id` and carries a\n/// whole new record; a new record under a DIFFERENT id would silently rename the row and orphan\n/// every reference to it, so it is a `Fatal` identity breach, the level `mutation\.duplicate-id`\n/// already uses for the other half of the identity contract\."),
     "/// 🪪️ The `mutation.target-mismatch` refusal — a `replace-` selects its target by `id` and carries a\n/// whole new record; a new record under a DIFFERENT id would silently rename the row and orphan\n/// every reference to it, so the payload contradicts the target it selects: an `Error`, the level of\n/// every state-dependent refusal (`mutation.target-missing`, `mutation.target-referenced`)."),
]
PYTHON = [
    ('DUPLICATE_ID, ID_MISMATCH, INVARIANT = "mutation.duplicate-id", "mutation.id-mismatch", "mutation.invariant"', 'DUPLICATE_ID, TARGET_MISMATCH, INVARIANT = "mutation.duplicate-id", "mutation.target-mismatch", "mutation.invariant"'),
    ("fatal(ID_MISMATCH,", "error(TARGET_MISMATCH,"),
    ("`mutation.id-mismatch` for a replace that renames its", "`mutation.target-mismatch` for a replace that renames its"),
]


def rust(text):
    lines = text.split("\n")
    for at, line in enumerate(lines):
        if f'"{OLD}"' in line and "assert" in line:
            lines[at] = line.replace(OLD, NEW)
            for follow in range(at + 1, min(at + 3, len(lines))):
                lines[follow] = lines[follow].replace("protocol::Severity::Fatal", "protocol::Severity::Error")
    text = "\n".join(lines)
    for pattern, replacement in RUST:
        text = pattern.sub(replacement, text)
    return text


def outcome(text):
    document = json.loads(text)
    if document.get("code") == OLD:
        document["code"] = NEW
    for message in document.get("messages", []):
        if message.get("code") == OLD:
            message["code"] = NEW
            message["level"] = "error"
    return json.dumps(document, indent=2, ensure_ascii=False) + "\n"


def python(text):
    for old, new in PYTHON:
        text = text.replace(old, new)
    return text


def main():
    check = "--check" in sys.argv
    changed, left = [], []
    for directory, _, files in os.walk(os.path.join(REPO, ROOT)):
        for name in files:
            path = os.path.join(directory, name)
            if name == "🦀️.rs":
                rewrite = rust
            elif name == "🐍️.py":
                rewrite = python
            elif name == "🔣️.json" and "/🎯️outcome/" in path:
                rewrite = outcome
            else:
                continue
            source = open(path, encoding="utf-8").read()
            if OLD not in source and "ID_MISMATCH" not in source and "id_mismatch" not in source:
                continue
            result = rewrite(source)
            if result != source:
                changed.append(path)
                if not check:
                    open(path, "w", encoding="utf-8").write(result)
            if OLD in result or "ID_MISMATCH" in result:
                left.append(path)
    for path in changed:
        print("changed", os.path.relpath(path, REPO))
    for path in left:
        print("LEFT", os.path.relpath(path, REPO))
    print(f"changed={len(changed)} left={len(left)}{' (check only)' if check else ''}")


if __name__ == "__main__":
    main()
