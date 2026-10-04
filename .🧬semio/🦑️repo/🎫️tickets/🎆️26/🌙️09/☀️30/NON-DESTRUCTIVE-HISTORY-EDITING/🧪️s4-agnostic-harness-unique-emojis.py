#!/usr/bin/env python3
"""🎨️ Gives every docstring of the G12 harness (`🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs`) a unique leading emoji (AGENTS docstring
rule; S4-AGNOSTIC owns the file). Each replacement is anchored on the docstring's exact first words, count-asserted, one write, the file
re-read immediately before; idempotent. Usage: [--apply]."""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
REPLACEMENTS = [
    ("/// 🌿️ Downstream operations tried per leaf", "/// 🌊️ Downstream operations tried per leaf"),
    ("/// 🌿️ The new-alternative half of a scenario", "/// 🌳️ The new-alternative half of a scenario"),
    ("/// 🎛️ Changes tried per leaf before it counts", "/// 🔢️ Changes tried per leaf before it counts"),
    ("/// 🧫️ Every committed fixture case under `root`", "/// 🗄️ Every committed fixture case under `root`"),
    ("/// ⚖️ The outcome of one acceptance scenario", "/// 🏁️ The outcome of one acceptance scenario"),
    ("/// ⚖️ One full scenario for `case`", "/// 🎭️ One full scenario for `case`"),
    ("/// ⚖️ LAW (design §16.3): a representative editable leaf of the app", "/// 🧑‍⚖️ LAW (design §16.3): a representative editable leaf of the app"),
    ("/// ⚖️ LAW (design §20.15): a document survives save → fresh load", "/// 🔂️ LAW (design §20.15): a document survives save → fresh load"),
    ("/// ⚖️ LAW (design §16.3, the editable-everything gate at runtime)", "/// 🩺️ LAW (design §16.3, the editable-everything gate at runtime)"),
    ("/// 🌱️ A registered instance whose document is `base`", "/// 🪴️ A registered instance whose document is `base`"),
    ("/// 🔍️ Where a reloaded document's view first departs", "/// 🧐️ Where a reloaded document's view first departs"),
    ("/// 🔎️ The outcome of a representative-leaf search", "/// 📊️ The outcome of a representative-leaf search"),
    ("/// 💾️ `app`'s document saved as its recursive archive", "/// 📀️ `app`'s document saved as its recursive archive"),
    ("/// 🖼️ The [`AcceptanceDocumentView`] of `app`.", "/// 📸️ The [`AcceptanceDocumentView`] of `app`."),
    ("/// 🔗️ LAW (design §16.3): every document leaf payload schema of this editor", "/// ⛓️ LAW (design §16.3): every document leaf payload schema of this editor"),
    ("/// ⏪️ Wires [`assert_history_edits_end_to_end`]", "/// 🔌️ Wires [`assert_history_edits_end_to_end`]"),
    ("/// ⏪️ LAW (design §16.3): a representative editable leaf of this editor", "/// ⏮️ LAW (design §16.3): a representative editable leaf of this editor"),
    ("/// 💾️ Wires [`assert_documents_reload_identically`]", "/// 🛠️ Wires [`assert_documents_reload_identically`]"),
]


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        before = handle.read()
    after = before
    for old, new in REPLACEMENTS:
        if new in after and old not in after:
            continue
        if after.count(old) != 1:
            sys.exit(f"ANCHOR {after.count(old)}: {old}")
        after = after.replace(old, new)
    if after == before:
        print("unchanged")
        return
    if "--apply" not in sys.argv:
        print("WOULD rewrite", sum(old in before for old, _ in REPLACEMENTS), "docstrings")
        return
    with open(PATH, encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE")
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(after)
    print("WROTE")


if __name__ == "__main__":
    main()
