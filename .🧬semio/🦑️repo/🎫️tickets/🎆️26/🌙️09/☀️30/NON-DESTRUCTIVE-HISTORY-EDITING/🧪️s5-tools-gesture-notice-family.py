"""📢️ Fix-forward of `🧪️s5-tools-gesture-notices.py`: the notice fixture's schema admits a closed set of code families and
`toolGesture` is not one of them, so the poisoned-slot refusal joins the admitted `toolTransaction` family —
`toolTransaction.slot-poisoned` — in the runtime, the kernel table, its TS twin and the fixture. Counted anchors, idempotent.

Usage (cwd: repo root): python3 🧪️s5-tools-gesture-notice-family.py [--apply]   (`--apply` under `landing` + `serve`)
"""

import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
OLD, NEW = '"toolGesture.slot-poisoned"', '"toolTransaction.slot-poisoned"'
FILES = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🦀️.rs",
    "🧰️framework/🔨️modules/🎠️kernel/🦀️.rs",
    "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
    "🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🧫️framework-notices/🔣️.json",
]


def main():
    staged, pending = [], 0
    for relative in FILES:
        path = REPO / relative
        text = path.read_text(encoding="utf-8")
        if text.count(OLD) == 1 and text.count(NEW) == 0:
            text, pending = text.replace(OLD, NEW), pending + 1
        elif not (text.count(OLD) == 0 and text.count(NEW) == 1):
            raise SystemExit(f"{relative}: {text.count(OLD)} old / {text.count(NEW)} new site(s)")
        staged.append((path, text))
    if "--apply" in sys.argv:
        for path, text in staged:
            path.write_text(text, encoding="utf-8")
    print(f"{'renamed' if '--apply' in sys.argv else 'would rename'} {pending} site(s) in {len(staged)} files")


if __name__ == "__main__":
    main()
