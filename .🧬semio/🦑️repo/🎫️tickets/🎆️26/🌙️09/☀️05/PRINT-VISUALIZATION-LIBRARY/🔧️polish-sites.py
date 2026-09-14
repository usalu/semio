#!/usr/bin/env python3
"""📋 Dump the definition sites that still lack an emoji docstring, with a body preview."""
import os
import re
import sys

LATEX = "C:/git/semio/\U0001f9f0\ufe0fframework/\U0001f6cd\ufe0fproducts/\U0001f4d3\ufe0fprint/\U0001f58b\ufe0flatex"

DEFHEAD = re.compile(
    r"^\\(cs_new[a-z_]*:(?:Npn|Nn|Nx|cpn|cn)|cs_set[a-z_]*:(?:Npn|Nn|Nx|cpn|cn)"
    r"|NewDocumentCommand|NewDocumentEnvironment|DeclareDocumentCommand"
    r"|keys_define:nn|SemioVizFamily|semio_viz_family_define:nn"
    r"|prg_new_[a-z_]*:[A-Za-z]+|semio_viz_layout_define:nn)\b"
)
EMOJI = re.compile("[\U0001F000-\U0001FAFF\u2190-\u21FF\u2300-\u27BF\u2B00-\u2BFF\uFE0F\u3030\u203C\u2049]")


SKIPPABLE = re.compile(
    r"^\\(tl|seq|prop|fp|int|bool|clist|box|dim|str|skip|muskip|coffin)_(new|const):[Nc]\b"
    r"|^\\cs_generate_variant:Nn\b|^\\prg_generate_conditional_variant:Nnn\b"
)


def documented(lines, index):
    """📝 True when the definition on line `index` is covered by an emoji docstring above it."""
    cursor = index - 1
    while cursor >= 0:
        text = lines[cursor].strip()
        if text.startswith("%region") or text.startswith("%endregion"):
            return False
        if text.startswith("%"):
            if EMOJI.search(text):
                return True
            cursor -= 1
            continue
        if text == "" or SKIPPABLE.match(text):
            cursor -= 1
            continue
        return False
    return False


def main():
    for fn in sys.argv[1:]:
        p = os.path.join(LATEX, fn)
        lines = open(p, encoding="utf-8").read().split("\n")
        print("### " + fn)
        region = ""
        for i, ln in enumerate(lines):
            s = ln.strip()
            if s.startswith("%region"):
                region = s[7:].strip()
            if DEFHEAD.match(ln):
                if documented(lines, i):
                    continue
                body = " ".join(x.strip() for x in lines[i:i + 4])
                print("%d\t[%s]\t%s" % (i + 1, region, body[:220]))
        print()


main()
