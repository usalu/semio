#!/usr/bin/env python3
"""🦀️ Lists the non-doc comments that sit inside a brace-delimited item of a Rust file (fn/impl/mod bodies): python3 -I rs_comments.py <file>... (a small lexer: strings, raw strings, chars and lifetimes do not count braces)."""
import re, sys
for path in sys.argv[1:]:
    src = open(path, encoding="utf8").read()
    i, n, depth, line = 0, len(src), 0, 1
    stack = []
    found = []
    while i < n:
        c = src[i]
        if c == "\n": line += 1; i += 1; continue
        if src.startswith("//", i):
            j = src.find("\n", i); j = n if j < 0 else j
            text = src[i:j]
            doc = text.startswith("///") and not text.startswith("////") or text.startswith("//!")
            if not doc and depth > 0: found.append((line, "/".join(stack[-2:]) or "-", text.strip()[:90]))
            i = j; continue
        if src.startswith("/*", i):
            j = src.find("*/", i + 2); j = n if j < 0 else j + 2
            text = src[i:j]
            if depth > 0 and not text.startswith("/**") and not text.startswith("/*!"): found.append((line, "/".join(stack[-2:]) or "-", text.strip()[:90]))
            line += text.count("\n"); i = j; continue
        m = re.match(r'r(#*)"', src[i:i + 12])
        if m:
            end = '"' + m.group(1); j = src.find(end, i + len(m.group(0))); j = n if j < 0 else j + len(end)
            line += src[i:j].count("\n"); i = j; continue
        if c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            line += src[i:j].count("\n"); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(?:\\.[^']*|[^\\'])'", src[i:i + 12])
            if m: i += len(m.group(0)); continue
            i += 1; continue
        if c == "{":
            head = src[max(0, i - 160):i]
            k = re.findall(r"\b(fn|impl|mod|struct|enum|trait|unsafe|match|if|else|for|while|loop)\s+(\w+)?", head)
            stack.append(k[-1][0] + ":" + (k[-1][1] or "") if k else "block"); depth += 1
        elif c == "}":
            depth -= 1
            if stack: stack.pop()
        i += 1
    print(path[-60:], "comments inside items:", len(found))
    for f in found[:40]: print("  ", f)
