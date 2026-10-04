#!/usr/bin/env python3
"""🎭️ S3-NORM: norm mutation-leaf directories whose emoji renders as text get their emoji presentation (U+FE0F).

`verify taxonomy report` flags `path-emoji-presentation` on 20 leaves (en1991 17, din4108 2, en1998 1): the leading code point
has no `Emoji_Presentation` and the name carries no U+FE0F (`🏔change-north-german-lowland-snow`). The same rule as
`🧰️framework/🔨️modules/🪪️identity/🛣️path/🟦️.ts` (`pathEmojiStatuteFindings`, kind `presentation`) finds them here; each leaf
directory and its fixture mirror move to `<emoji>U+FE0F<kind>`, the descriptor `emoji` follows, and every git-visible
reference in the norm plugin and the schema catalog is rewritten (exact name, not followed by a kind character). Generated
descriptors (`🌎️hub/…/🛂️.descriptor.semio`) are left to `describe`.

Usage: python3 🧪️s3-norm-leaf-emoji-presentation.py [--check]
"""
import json
import re
import subprocess
import sys
import unicodedata
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
NORM = "✏️s/🔌️plugins/📕️norm"
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any"
TREES = ("🧬️schema/🧬️mutations", "🧫️fixtures/🧬️mutations")
CATALOG = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md"]
TEXT_PRESENTATION = {"🌡", "🏷", "🛢", "🌬", "⚙", "🏗", "🛡", "❄", "🗺", "🛣", "⏱", "♨", "🕰", "🏔", "🏙", "⚖"}


def leaves() -> dict[tuple[str, str], str]:
    found = {}
    for artifact in sorted((ROOT / NORM / "🗿️artifacts").iterdir()):
        for tree in TREES:
            base = artifact / SUBSET / tree
            if not base.is_dir():
                continue
            for leaf in base.iterdir():
                first = leaf.name[0]
                if leaf.is_dir() and first in TEXT_PRESENTATION and leaf.name[1:2] != "️":
                    found[(artifact.name, leaf.name)] = first + "️" + leaf.name[1:]
    return found


def main() -> int:
    check = "--check" in sys.argv
    renames = leaves()
    names = {old: new for (_, old), new in renames.items()}
    print(f"leaf names={len(names)}")
    if check:
        return 1 if names else 0
    for (artifact, old), new in sorted(renames.items()):
        for tree in TREES:
            source = ROOT / NORM / "🗿️artifacts" / artifact / SUBSET / tree / old
            if source.is_dir():
                target = source.with_name(new)
                assert not target.exists(), target
                source.rename(target)
        descriptor = ROOT / NORM / "🗿️artifacts" / artifact / SUBSET / TREES[0] / new / "🔣️.json"
        text = descriptor.read_text()
        assert json.loads(text)["emoji"] == old[0], descriptor
        descriptor.write_text(text.replace(f'"emoji": "{old[0]}"', f'"emoji": "{old[0]}️"'))
    pattern = re.compile("|".join(re.escape(old) for old in sorted(names, key=len, reverse=True)) + r"(?![a-z0-9-])")
    listed = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", NORM, *CATALOG], cwd=ROOT, capture_output=True, check=True).stdout.decode()
    rewritten = 0
    for relative in filter(None, listed.split("\0")):
        path = ROOT / unicodedata.normalize("NFC", relative)
        if not path.is_file() or path.suffix in {".semio"} and "descriptor" in path.name:
            continue
        try:
            text = path.read_text()
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        updated = pattern.sub(lambda match: names[match.group(0)], text)
        if updated != text:
            path.write_text(updated)
            rewritten += 1
    print(f"moved={len(renames)} rewritten-files={rewritten}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
