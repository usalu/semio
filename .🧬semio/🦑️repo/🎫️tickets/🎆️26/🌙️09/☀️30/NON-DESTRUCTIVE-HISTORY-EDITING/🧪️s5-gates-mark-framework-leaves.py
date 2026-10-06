"""🚪️ Declares the two parameterless framework leaves withdraw-only (design §22.20; coordinator decision 10:22, `📓️s4-gates-report.md` § S5.9).

`sign-out` (os config) and `update-node-ports` (workflow) carry no input: their descriptors gain `"editable": false`, so the derive
answers `input_schema() == None`, refuses `with_input_value`, and the `inputless` rule of `schema mutation-inputs` no longer fails them.

    python3 🧪️s5-gates-mark-framework-leaves.py --root <repository root> [--apply | --restore]
"""
import json
import os
import shutil
import sys

LEAVES = [
    ("🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🚪️sign-out/🔣️.json", '    "json-schema"\n  ]\n}', '    "json-schema"\n  ],\n  "editable": false\n}'),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/🧬️mutations/🔄update-node-ports/🔣️.json", '"requiredLanguageSurfaces":["rust","json-schema","text","binary"]}', '"requiredLanguageSurfaces":["rust","json-schema","text","binary"],"editable":false}'),
]
BACKUP = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "s5-gates", "exec", "pre-mark")


def refuse(message):
    """🛑️ Fails closed: prints why and exits 2 before anything is written."""
    print(f"[mark-framework-leaves] REFUSED: {message}", file=sys.stderr)
    sys.exit(2)


def main():
    arguments = sys.argv[1:]
    root = (arguments[arguments.index("--root") + 1] if "--root" in arguments and arguments.index("--root") + 1 < len(arguments) else "").rstrip("/")
    if root == "" or any(not os.path.isfile(os.path.join(root, path)) for path, _, _ in LEAVES):
        refuse("--root must name a directory that holds both leaf descriptors")
    if "--restore" in arguments:
        if any(not os.path.isfile(os.path.join(BACKUP, path)) for path, _, _ in LEAVES):
            refuse(f"no complete pre-wave copy under {BACKUP}")
        for path, _, _ in LEAVES:
            shutil.copyfile(os.path.join(BACKUP, path), os.path.join(root, path))
        print(f"[mark-framework-leaves] {len(LEAVES)} descriptor(s) restored under {root}")
        return
    planned = {}
    for path, old, new in LEAVES:
        text = open(os.path.join(root, path), encoding="utf-8").read()
        before = json.loads(text)
        if "editable" in before or text.count(old) != 1:
            refuse(f"{path} already states editable, or its tail moved")
        after = text.replace(old, new)
        if json.loads(after) != {**before, "editable": False}:
            refuse(f"{path} would change more than the editable key")
        planned[path] = after
    if "--apply" in arguments:
        for path, text in planned.items():
            os.makedirs(os.path.dirname(os.path.join(BACKUP, path)), exist_ok=True)
            shutil.copy2(os.path.join(root, path), os.path.join(BACKUP, path))
            with open(os.path.join(root, path), "w", encoding="utf-8") as handle:
                handle.write(text)
    print(f"[mark-framework-leaves] {len(planned)} descriptor(s) under {root}: {'WRITTEN' if '--apply' in arguments else 'planned (check only; pass --apply)'}")


main()
