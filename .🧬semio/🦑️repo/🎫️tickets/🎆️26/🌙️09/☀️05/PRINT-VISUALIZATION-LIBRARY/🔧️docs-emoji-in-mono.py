#!/usr/bin/env python3
"""🙂 DOCS: wraps every emoji inside a \\Key / \\Cs / \\meta argument of 🔓️viz-api.tex in \\E{…}.

The monospaced document font has no emoji coverage, so an emoji written directly into a path is
dropped silently ("Missing character" in the log). \\E switches to the emoji face for that one glyph.
Variation selectors are removed: they are invisible and no font in the stack carries them.
Idempotent — an emoji already inside \\E{…} is left alone.
"""
import io
import os
import re

DOC_DIR = r"C:\git\semio\🧰️framework\🛍️products\📓️print\🧾️template\📊️viz-api"
DOC = "🔓️viz-api.tex"
MACRO = re.compile(r"\\(Key|Cs|meta)\{([^{}]*)\}")
EMOJI = re.compile("[\U0001F000-\U0001FAFF\u2190-\u21FF\u2600-\u27BF\U0001F1E6-\U0001F1FF]\uFE0F?")


def wrap(match: "re.Match[str]") -> str:
    name, body = match.group(1), match.group(2)
    if "\\E{" in body:
        return match.group(0)
    fixed = EMOJI.sub(lambda hit: "\\E{%s}" % hit.group(0).replace("\uFE0F", ""), body)
    return "\\%s{%s}" % (name, fixed)


def main() -> None:
    os.chdir(DOC_DIR)
    source = io.open(DOC, encoding="utf8").read()
    fixed = MACRO.sub(wrap, source)
    if fixed == source:
        print("[DEBUG] no emoji to wrap")
        return
    io.open(DOC, "w", encoding="utf8", newline="\n").write(fixed)
    print("[DEBUG] wrapped %d emoji" % fixed.count("\\E{"))


main()
