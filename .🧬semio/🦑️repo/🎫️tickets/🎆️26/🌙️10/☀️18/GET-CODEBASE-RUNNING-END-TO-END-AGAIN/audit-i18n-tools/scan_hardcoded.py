#!/usr/bin/env python3
# 🔎️ Read-only scan: hard-coded English prose in user-facing sites of the repo build library.
# Usage: python3 scan_hardcoded.py <file-list> <repo-root> <out.jsonl>
import json
import re
import sys
from collections import Counter, defaultdict

LIB_PREFIX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
LITERAL = r'''("(?:[^"\\\n]|\\.)*"|'(?:[^'\\\n]|\\.)*'|`(?:[^`\\]|\\.)*`)'''
SITE_PATTERNS = [
    ("error", re.compile(r"(?:new\s+|throw\s+)?\bError\(\s*" + LITERAL)),
    ("console", re.compile(r"\bconsole\.(?:log|error|warn|info|debug|table|group)\(\s*" + LITERAL)),
    ("stream", re.compile(r"\b(?:stdout|stderr)\.write\(\s*" + LITERAL)),
    ("prop", re.compile(r"\b(?:message|text|summary|title|label|description|hint|reason|prompt|help|notice|warning|detail|details|usage|synopsis|notes?|progress|status|heading)\s*:\s*" + LITERAL)),
]
PROSE_WORD = re.compile(r"[A-Za-z]{3,}")
CODEISH = re.compile(r"[\\{}()\[\]<>=;]|^\s*/|\bimport\b|\bexport\b|\bfunction\b|\breturn\b")


def unquote(lit):
    body = lit[1:-1]
    return re.sub(r"\$\{[^}]*\}", "X", body)


def is_prose(text):
    if CODEISH.search(text) and not re.search(r"[a-z]{3,}\s+[a-z]{2,}", text):
        return False
    tokens = text.split()
    if len(tokens) < 2:
        return False
    if not PROSE_WORD.search(text):
        return False
    return bool(re.search(r"[A-Za-z]{3,}\s+[A-Za-z]{2,}", text))


def is_comment(stripped):
    return stripped.startswith(("//", "/*", "*", "*/")) or stripped.startswith("/**")


def main():
    files_list, root, out = sys.argv[1], sys.argv[2], sys.argv[3]
    rows = []
    with open(files_list, encoding="utf-8") as fh:
        paths = [line.strip() for line in fh if line.strip()]
    for rel in paths:
        if not rel.endswith((".ts", ".mts", ".mjs", ".cjs")):
            continue
        full = root + "/" + rel
        try:
            with open(full, encoding="utf-8") as fh:
                lines = fh.read().split("\n")
        except (OSError, UnicodeDecodeError):
            continue
        short = rel[len(LIB_PREFIX):] if rel.startswith(LIB_PREFIX) else rel
        for number, line in enumerate(lines, 1):
            if is_comment(line.strip()):
                continue
            seen_spans = set()
            for kind, pattern in SITE_PATTERNS:
                for match in pattern.finditer(line):
                    literal = match.group(1)
                    if literal.startswith("`") and "\n" in literal:
                        continue
                    text = unquote(literal)
                    if not is_prose(text):
                        continue
                    span = match.span(1)
                    if span in seen_spans:
                        continue
                    seen_spans.add(span)
                    rows.append({
                        "file": short,
                        "line": number,
                        "kind": kind,
                        "text": text[:200],
                        "module": short.split("/")[0] + ("/" + short.split("/")[1] if "/" in short else ""),
                        "inline_bilingual": bool(re.search(r"\b(en|de)\s*:", line)),
                    })
    with open(out, "w", encoding="utf-8") as fh:
        for row in rows:
            fh.write(json.dumps(row, ensure_ascii=False) + "\n")
    by_kind = Counter(row["kind"] for row in rows)
    by_top = Counter(row["module"].split("/")[0] for row in rows)
    print("total", len(rows))
    print("by_kind", dict(by_kind))
    for key, value in by_top.most_common():
        print(value, key)


if __name__ == "__main__":
    main()
