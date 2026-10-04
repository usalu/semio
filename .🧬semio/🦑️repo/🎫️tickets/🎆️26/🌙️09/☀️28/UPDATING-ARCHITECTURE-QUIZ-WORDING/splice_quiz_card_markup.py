"""🪡️ Replaces the rendered markup of one quiz react screen with a card-grid version kept in a fragment file.

Usage: python splice_quiz_card_markup.py <module file> <start marker> <fragment file> [<import anchor> <import replacement file>]
The file is cut from the first line that equals `<start marker>` to its end and the fragment appended; an optional
import anchor (exact text, unique) is replaced by the content of a second fragment file. Refuses ambiguous markers.
"""

import io
import sys

module, marker, fragment = sys.argv[1], sys.argv[2], sys.argv[3]
text = io.open(module, encoding="utf-8", newline="").read()
if text.count(marker) != 1:
    sys.exit(f"start marker found {text.count(marker)} times")
start = text.index(marker)
text = text[:start] + io.open(fragment, encoding="utf-8").read()
if len(sys.argv) > 5:
    anchor = io.open(sys.argv[4], encoding="utf-8").read().rstrip("\n")
    replacement = io.open(sys.argv[5], encoding="utf-8").read().rstrip("\n")
    if text.count(anchor) != 1:
        sys.exit(f"import anchor found {text.count(anchor)} times")
    text = text.replace(anchor, replacement)
io.open(module, "w", encoding="utf-8", newline="").write(text)
print("spliced", module)
