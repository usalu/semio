#!/usr/bin/env python3
# 🔓️ Widen named definitions to `pub`: column-0 items, methods of impl blocks and struct fields (the narrowest change a cross-crate use demands).
import re, sys
from common import *


def widen_items(text, names):
    n = 0
    for name in names:
        pat = re.compile(r"(?m)^(\s*)((?:async |const |unsafe |extern \"[^\"]*\" )*(?:fn|const|static|struct|enum|type|trait|mod) (?:mut )?)" + re.escape(name) + r"\b")
        def sub(m):
            nonlocal n
            n += 1
            return m.group(1) + "pub " + m.group(2) + name
        text = pat.sub(sub, text)
    return text, n


def widen_fields(text, struct, fields):
    m = re.search(r"(?m)^(?:pub(?:\([^)]*\))? )?struct " + re.escape(struct) + r"\b[^{;]*\{", text)
    assert m, struct
    i = m.end()
    depth = 1
    j = i
    while depth:
        c = text[j]
        depth += (c == "{") - (c == "}")
        j += 1
    body = text[i:j - 1]
    for f in fields:
        body, k = re.subn(r"(?m)^(\s*)(?<!pub )" + re.escape(f) + r"\s*:", lambda mm: mm.group(1) + "pub " + f + ":", body)
        assert k == 1, (struct, f, k)
    return text[:i] + body + text[j - 1:]


def apply(path, names=(), struct_fields=()):
    t = read(path)
    t, n = widen_items(t, names)
    for s, fs in struct_fields:
        t = widen_fields(t, s, fs)
    write(path, t)
    return n
