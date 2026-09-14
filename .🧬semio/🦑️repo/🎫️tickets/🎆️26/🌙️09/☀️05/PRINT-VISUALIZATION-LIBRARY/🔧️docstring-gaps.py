#!/usr/bin/env python3
"""🔎 Docstring gap checker over EVERY `semio-viz-*.sty` of the print product.

POLISH-2's `docstrings.py` with its owned-file list replaced by a glob, so the FAMILIES-DOMAIN and
FAMILIES-CAPABILITY packages are covered too. A definition at column 0 is documented when the
comment block above it (stepping over blank lines, variable declarations and variant generation)
carries an emoji; `%region` markers are not docstrings. Generated packages are skipped. Usage:

    python docstring-gaps.py            # per-file counts
    python docstring-gaps.py <file>     # every gap site of one file, `line: head`
"""
import glob
import os
import re
import sys

LATEX = glob.glob("C:/git/semio/*framework/*products/*print/*latex")[0]

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


def gaps(path):
    lines = open(path, encoding="utf-8").read().split("\n")
    return [
        (i + 1, lines[i].strip()[:90])
        for i in range(len(lines))
        if DEFHEAD.match(lines[i]) and not documented(lines, i)
    ]


def sources():
    found = sorted(glob.glob(os.path.join(LATEX, "semio-viz*.sty")))
    return [
        path
        for path in found
        if not open(path, encoding="utf-8").readline().startswith("% \U0001f916")
    ]


def main():
    if len(sys.argv) > 1:
        for line, head in gaps(os.path.join(LATEX, sys.argv[1])):
            print("%d: %s" % (line, head))
        return
    total = 0
    for path in sources():
        found = len(gaps(path))
        total += found
        if found:
            print("%5d  %s" % (found, os.path.basename(path)))
    print("%5d  TOTAL over %d files" % (total, len(sources())))


main()
