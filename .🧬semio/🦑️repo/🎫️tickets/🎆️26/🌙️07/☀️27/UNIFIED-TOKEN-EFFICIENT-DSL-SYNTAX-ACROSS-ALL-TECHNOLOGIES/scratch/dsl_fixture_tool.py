#!/usr/bin/env python3
"""Generic old-dialect DSL fixture tokenizer/parser/re-emitter used to hand-regenerate
puzzle/{2d,3d,5d} example fixtures onto the unified kebab-case + SoA-table syntax.
Scratch tool for ticket UNIFIED-TOKEN-EFFICIENT-DSL-SYNTAX-ACROSS-ALL-TECHNOLOGIES — not part of the
shipped codebase, lives only in the ticket folder.
"""
import re
import sys

TOKEN_RE = re.compile(r'''
    (?P<WS>\s+)
  | (?P<STR>"(?:[^"\\]|\\.)*")
  | (?P<NUM>-?\d+\.\d+(?:[eE][+-]?\d+)?|-?\d+(?:[eE][+-]?\d+)?)
  | (?P<LBRACE>\{)
  | (?P<RBRACE>\})
  | (?P<LBRACKET>\[)
  | (?P<RBRACKET>\])
  | (?P<COMMA>,)
  | (?P<EQUALS>=)
  | (?P<IDENT>[A-Za-z_][A-Za-z0-9_./-]*)
''', re.VERBOSE)

def tokenize(text):
    pos = 0
    toks = []
    while pos < len(text):
        m = TOKEN_RE.match(text, pos)
        if not m:
            raise ValueError(f"lex error at {pos}: {text[pos:pos+40]!r}")
        kind = m.lastgroup
        val = m.group()
        pos = m.end()
        if kind == 'WS':
            continue
        toks.append((kind, val))
    toks.append(('EOF', ''))
    return toks

class P:
    def __init__(self, toks):
        self.toks = toks
        self.pos = 0
    def peek(self):
        return self.toks[self.pos]
    def peek_at(self, n):
        return self.toks[min(self.pos + n, len(self.toks) - 1)]
    def advance(self):
        t = self.toks[self.pos]
        if self.pos < len(self.toks) - 1:
            self.pos += 1
        return t
    def expect(self, kind):
        t = self.advance()
        if t[0] != kind:
            raise ValueError(f"expected {kind}, got {t} near pos {self.pos}")
        return t
    def at_attr_key(self):
        if self.peek()[0] == 'IDENT' and self.peek_at(1)[0] == 'EQUALS':
            return self.peek()[1]
        return None

def parse_scalar(p):
    k, v = p.advance()
    if k == 'STR':
        return ('str', v)
    if k == 'NUM':
        return ('num', v)
    if k == 'IDENT':
        return ('ident', v)
    raise ValueError(f"expected scalar, got {(k,v)}")

def parse_value(p, field_shapes, key):
    """field_shapes: dict old_key -> one of 'scalar','block','record','value_json',
    ('list_record', subfields), or ('tuple', n)."""
    shape = field_shapes.get(key, 'scalar')
    if shape == 'block':
        p.expect('LBRACE')
        rec = parse_record(p, field_shapes[key + '__fields'])
        p.expect('RBRACE')
        return ('block', rec)
    if shape == 'record':
        rec = parse_record(p, field_shapes[key + '__fields'])
        return ('record', rec)
    if isinstance(shape, tuple) and shape[0] == 'list_record':
        p.expect('LBRACKET')
        items = []
        while p.peek()[0] != 'RBRACKET':
            items.append(parse_record(p, shape[1]))
        p.expect('RBRACKET')
        return ('list_record', items)
    if isinstance(shape, tuple) and shape[0] == 'tuple':
        n = shape[1]
        items = []
        for i in range(n):
            items.append(parse_scalar(p))
            if i < n - 1:
                p.expect('COMMA')
        return ('tuple', items)
    if shape == 'value_json':
        # opaque nested json-ish value (braces/brackets/scalars) — capture raw token span
        return ('raw', capture_raw_value(p))
    return parse_scalar(p)

def capture_raw_value(p):
    # Captures one balanced value (scalar, or bracketed/braced structure) as token list, for opaque passthrough.
    depth = 0
    out = []
    k, v = p.peek()
    if k in ('LBRACE', 'LBRACKET'):
        open_k = k
        close_k = 'RBRACE' if k == 'LBRACE' else 'RBRACKET'
        while True:
            k2, v2 = p.advance()
            out.append((k2, v2))
            if k2 == open_k:
                depth += 1
            elif k2 == close_k:
                depth -= 1
                if depth == 0:
                    break
    else:
        out.append(p.advance())
    return out

def parse_record(p, field_shapes):
    """Generic 'flat AoS' record parse: consume key=value / key{...} pairs from a fresh remaining-keys
    set, stop at repeat/unknown key or bracket close — mirrors dsl_schema::parse_record_body's dedup
    termination so multi-record flat blobs split the same way the real engine does. Block-shaped
    fields (`#[dsl(block)]`) print bare (`key { ... }`, no `=`) — matched by lookahead instead."""
    remaining = set(k for k in field_shapes if not k.endswith('__fields'))
    order = []
    values = {}
    while True:
        # Bare block field: `key {` with no `=`.
        blockable = [k for k in remaining if field_shapes.get(k) == 'block' and p.peek()[0] == 'IDENT' and p.peek()[1] == k and p.peek_at(1)[0] == 'LBRACE']
        if blockable:
            key = blockable[0]
            remaining.discard(key)
            p.advance()
            values[key] = parse_value(p, field_shapes, key)
            order.append(key)
            continue
        key = p.at_attr_key()
        if key is None or key not in remaining:
            break
        remaining.discard(key)
        p.advance()
        p.expect('EQUALS')
        values[key] = parse_value(p, field_shapes, key)
        order.append(key)
    return {'__order__': order, **values}

def fmt_num(raw):
    return raw

def kebab(key):
    return key.replace('_', '-')

def fmt_scalar_value(v, bare_ok=True):
    kind, val = v
    if kind == 'num':
        return val
    if kind == 'ident':
        return val
    if kind == 'str':
        return val  # already quoted from source
    raise ValueError(f"unexpected scalar kind {kind}")

def print_flat_record(rec, field_shapes, indent=''):
    parts = []
    for key in rec['__order__']:
        v = rec[key]
        newkey = kebab(key)
        if v[0] == 'block':
            parts.append(f"{newkey} {{ {print_flat_record(v[1], field_shapes[key + '__fields'])} }}")
        elif v[0] == 'record':
            parts.append(f"{newkey}= {print_flat_record(v[1], field_shapes[key + '__fields'])}")
        elif v[0] == 'list_record':
            inner = ' '.join(print_flat_record(item, field_shapes[key][1]) for item in v[1])
            parts.append(f"{newkey}=[ {inner} ]")
        elif v[0] == 'tuple':
            inner = ','.join(fmt_scalar_value(x) for x in v[1])
            parts.append(f"{newkey}= {inner}")
        elif v[0] == 'raw':
            parts.append(f"{newkey}={render_raw(v[1])}")
        else:
            parts.append(f"{newkey}={fmt_scalar_value(v)}")
    return ' '.join(parts)

def render_raw(toks):
    out = []
    for k, v in toks:
        out.append(v)
    # naive join with spaces; opaque passthrough content, not required to be pretty
    return ' '.join(out)

if __name__ == '__main__':
    pass
