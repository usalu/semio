#!/usr/bin/env python3
"""🔀️ Tiny cfg predicate evaluator for dependency scans: `test` is a parameter, every other atom is treated as enabled."""
import re

TOKEN = re.compile(r'\s*([A-Za-z_][A-Za-z0-9_]*|"[^"]*"|[(),=])')


def tokenize(s):
    pos = 0
    out = []
    while pos < len(s):
        m = TOKEN.match(s, pos)
        if not m:
            break
        out.append(m.group(1))
        pos = m.end()
    return out


def parse(toks, i, test):
    name = toks[i]
    i += 1
    if i < len(toks) and toks[i] == '(':
        i += 1
        args = []
        while toks[i] != ')':
            v, i = parse(toks, i, test)
            args.append(v)
            if toks[i] == ',':
                i += 1
        i += 1
        if name == 'all':
            return all(args), i
        if name == 'any':
            return any(args), i
        if name == 'not':
            return not args[0], i
        return True, i
    if i < len(toks) and toks[i] == '=':
        i += 2
        return True, i
    if name == 'test':
        return test, i
    if name in ('windows',):
        return True, i
    return True, i


def holds(pred, test=False):
    toks = tokenize(pred)
    if not toks:
        return True
    v, _ = parse(toks, 0, test)
    return v
