#!/usr/bin/env python3
"""docstr-check.py FILE...: reports exported declarations without a docstring, docstrings not starting with an emoji and the same leading emoji twice in one file (AGENTS docstring rules)."""
import re, sys, unicodedata

def first_symbol(text):
    cluster = text[:1]
    return cluster if cluster and ord(cluster) > 0x2000 else None

def leading(text):
    out = ""
    for ch in text:
        if ch.isspace():
            break
        out += ch
    return out

for path in sys.argv[1:]:
    src = open(path, encoding="utf8").read()
    lines = src.split("\n")
    seen = {}
    for i, line in enumerate(lines):
        m = re.match(r"\s*/\*\*\s*(\S+)", line)
        if m:
            lead = m.group(1)
            if not (ord(lead[0]) > 0x2000):
                print(f"{path}:{i+1}: docstring without emoji: {lead[:12]}")
            else:
                seen.setdefault(lead, []).append(i + 1)
    for lead, at in seen.items():
        if len(at) > 1:
            print(f"{path}: emoji {lead} used on lines {at}")
    for i, line in enumerate(lines):
        if re.match(r"\s*export\s+(async\s+)?(function|const|interface|type|class)\b", line) or re.match(r"\s*export\s+(async\s+)?function\*", line):
            j = i - 1
            while j >= 0 and not lines[j].strip():
                j -= 1
            if j < 0 or "*/" not in lines[j]:
                print(f"{path}:{i+1}: exported without docstring: {line.strip()[:70]}")
