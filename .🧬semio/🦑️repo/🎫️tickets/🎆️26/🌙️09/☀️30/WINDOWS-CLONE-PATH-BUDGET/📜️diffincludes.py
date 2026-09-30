#!/usr/bin/env python3
"""🪟 Show include_str edits against HEAD for files with missing targets."""

import re
import subprocess

FILES = [
    "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🎚️mutate-gis-gismap-1-any-editor-edit-map-config/🦀️.rs",
    "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏋️mutate-en1991-1/🦀️.rs",
    "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-en1992-1/🦀️.rs",
    "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mutate-en1994-1/🦀️.rs",
    "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪶️mutate-en1999-1/🦀️.rs",
    "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫨️mutate-en1998-1/🦀️.rs",
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🎨️mutate-drawing-1-any-style/🦀️.rs",
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🔀️mutate-drawing-1-any-transform/🦀️.rs",
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-drawing-1-any-structure/🦀️.rs",
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪪️mutate-drawing-1-any-metadata/🦀️.rs",
]


def includes(text: str) -> list[str]:
    return re.findall(r'include_str!\(\s*"([^"]+)"', text)


def main() -> None:
    for path in FILES:
        show = subprocess.run(["git", "show", f"HEAD:{path}"], capture_output=True)
        head = show.stdout.decode("utf-8", "replace") if show.returncode == 0 else ""
        work = open(path, encoding="utf-8").read()
        old, new = includes(head), includes(work)
        changed = [(a, b) for a, b in zip(old, new) if a != b]
        print(f"\n{path.split('/')[-2]}")
        print(f"  head {len(old)} work {len(new)} changed_pairs {len(changed)} identical_file {head == work}")
        for a, b in changed[:4]:
            print("  -", a)
            print("  +", b)
        if len(old) != len(new):
            print("  COUNT MISMATCH")


if __name__ == "__main__":
    main()
