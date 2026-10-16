#!/usr/bin/env python3
"""🧩️ Re-points the exact-law groups of TypeScript scripts at the carved crates: package and law path of every law of the monolith crate, from the law map."""
import os, sys, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lawmap, state, plan_host
from carve import REPO

OLD_PKG = 'semio-framework-plugin-host'


def match_close(text, i, open_ch, close_ch):
    """Index of the bracket closing the one at text[i], skipping string and template literals."""
    d = 0
    j = i
    n = len(text)
    while j < n:
        c = text[j]
        if c in '"\'`':
            q = c
            j += 1
            while j < n and text[j] != q:
                if text[j] == '\\':
                    j += 1
                j += 1
        elif c == '/' and text[j:j + 2] == '//':
            j = text.index('\n', j)
            continue
        elif c == open_ch:
            d += 1
        elif c == close_ch:
            d -= 1
            if d == 0:
                return j
        j += 1
    raise ValueError('unbalanced')


def groups(text):
    """Yield (start, end, laws_list or None) of every `{ package: "semio-framework-plugin-host", ... laws: [ ... ] }` object."""
    for m in re.finditer(r'package\s*:\s*"' + re.escape(OLD_PKG) + r'"', text):
        a = text.rfind('{', 0, m.start())
        b = match_close(text, a, '{', '}')
        body = text[a:b + 1]
        lm = re.search(r'laws\s*:\s*\[', body)
        if not lm:
            continue
        s = a + lm.end() - 1
        e = match_close(text, s, '[', ']')
        inner = text[s + 1:e]
        if '${' in inner or 'names.map' in inner:
            yield a, b + 1, None
            continue
        laws = re.findall(r'"((?:[^"\\]|\\.)*)"', inner)
        yield a, b + 1, laws


def rewrite_text(text, table):
    out = []
    pos = 0
    skipped = []
    unmapped = []
    for a, b, laws in groups(text):
        if laws is None:
            skipped.append(text[a:b][:120])
            continue
        by = collections.OrderedDict()
        for law in laws:
            r = lawmap.map_law(table, law) or lawmap.map_law(table, 'component::' + law)
            if r is None:
                unmapped.append(law)
                by.setdefault(OLD_PKG, []).append(law)
                continue
            ident, new = r
            by.setdefault(plan_host.CRATES[ident]['name'], []).append(new)
        ls = text.rfind('\n', 0, a) + 1
        indent = re.match(r'[ \t]*', text[ls:]).group(0)
        multi = '\n' in text[a:b]
        parts = []
        for pkg, ls in by.items():
            if multi:
                lw = ('\n' + indent + '    ').join(f'"{l}",' for l in ls)
                parts.append('{\n' + indent + f'  package: "{pkg}",\n' + indent + '  target: { kind: "lib" },\n' + indent + '  laws: [\n' + indent + '    ' + lw + '\n' + indent + '  ],\n' + indent + '}')
            else:
                parts.append('{ package: "' + pkg + '", target: { kind: "lib" }, laws: [' + ', '.join(f'"{l}"' for l in ls) + '] }')
        out.append(text[pos:a])
        out.append((',\n' + indent if multi else ', ').join(parts))
        pos = b
    out.append(text[pos:])
    return ''.join(out), skipped, unmapped
