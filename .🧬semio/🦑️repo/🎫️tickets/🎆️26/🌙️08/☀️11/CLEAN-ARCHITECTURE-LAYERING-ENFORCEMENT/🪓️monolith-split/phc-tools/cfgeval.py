#!/usr/bin/env python3
"""🔀️ cfg predicate evaluator for the plugin-host carve: a fixed native macOS aarch64 environment where `test` is a parameter, wasm32, windows and wasip2 are off and every feature is on."""
import re

TOKEN = re.compile(r'\s*([A-Za-z_][A-Za-z0-9_]*|"[^"]*"|[(),=])')
ENV = {
    'target_arch': {'aarch64'},
    'target_os': {'macos'},
    'target_family': {'unix'},
    'target_env': {''},
    'target_pointer_width': {'64'},
    'target_vendor': {'apple'},
    'target_endian': {'little'},
}
FLAGS = {'unix', 'debug_assertions'}


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
        val = toks[i + 1].strip('"')
        i += 2
        if name in ENV:
            return val in ENV[name], i
        if name == 'feature':
            return True, i
        return False, i
    if name == 'test':
        return test, i
    return name in FLAGS, i


def holds(pred, test=False):
    toks = tokenize(pred)
    if not toks:
        return True
    v, _ = parse(toks, 0, test)
    return v
