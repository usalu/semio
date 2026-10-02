#!/usr/bin/env python3
"""🧭️ Moves the leaf TS oracle tests this WP added under `<leaf>/🧪️tests/🔬️unit/` (where the taxonomy counts every directory as a
mutation scenario and demands a catalog vector for it) to the aggregate's open-pattern case `✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️<leaf>/`,
rewrites their relative imports and the package test lists. Run from the repository root; `--apply` performs it."""

import os
import pathlib
import re
import sys

PLUGINS = {
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets": ("🔀️transform", ["✋️drag-layers", "🧭️rotate-layers", "📐️scale-layers", "📍️drag-path-points"], "✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts"),
    "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets": ("🧱️block", ["🤏️drag-blocks"], "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🟦️typescript/📜️script.ts"),
}
IMPORT = re.compile(r'from "(\.\./[^"]+)"')


def slug(name):
    return re.sub(r"^[^a-z0-9]+", "", name)


def main(apply):
    for subsets, (subset, leaves, package) in PLUGINS.items():
        package_path = pathlib.Path(package)
        package_text = package_path.read_text(encoding="utf-8")
        for leaf in leaves:
            old = pathlib.Path(subsets, subset, "🧬️schema/🧬️mutations", leaf, "🧪️tests/🔬️unit/🟦️.ts")
            new = pathlib.Path(subsets, "✳️any/🧬️schema/🧬️mutations/🧪️tests", f"🧪️{slug(leaf)}", "🟦️.ts")
            text = old.read_text(encoding="utf-8")

            def rebase(match):
                target = os.path.normpath(old.parent / match.group(1))
                assert os.path.exists(target), target
                return f'from "{pathlib.Path(os.path.relpath(target, new.parent)).as_posix()}"'

            moved = IMPORT.sub(rebase, text)
            old_entry = f'"../{subset}/🧬️schema/🧬️mutations/{leaf}/🧪️tests/🔬️unit/🟦️.ts"'
            new_entry = f'"🧬️schema/🧬️mutations/🧪️tests/🧪️{slug(leaf)}/🟦️.ts"'
            assert package_text.count(old_entry) == 1, old_entry
            package_text = package_text.replace(old_entry, new_entry)
            print(old, "->", new)
            if apply:
                new.parent.mkdir(parents=True, exist_ok=False)
                new.write_text(moved, encoding="utf-8")
                old.unlink()
                old.parent.rmdir()
        if apply:
            package_path.write_text(package_text, encoding="utf-8")


if __name__ == "__main__":
    main("--apply" in sys.argv)
