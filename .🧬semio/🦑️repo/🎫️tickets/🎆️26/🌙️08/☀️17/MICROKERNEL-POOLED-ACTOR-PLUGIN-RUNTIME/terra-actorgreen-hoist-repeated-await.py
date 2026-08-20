#!/usr/bin/env python3
"""🪡 Diagnostic-scoped repair for R10 residue shape 2 ("awaiting one future repeatedly") as it
shows up ~35 times across `mod tests` in
🧰️framework/🔨️modules/🎭️actor/🦀️component.rs: a constructor call was left un-awaited
(`let mut mailbox = Mailbox::new(4);`), and every later use was written `mailbox.await.method()`
— which is illegal past the first use (awaiting a Future consumes it). The fix per R10's own
residue-shape doc is "fix the constructor, not the uses": hoist the single `.await` onto the `let`
binding and drop it from every later use of that name.

This is NOT a name-keyed bulk awaiter (the thing R10 bans): the scope is a single `async fn` body,
found by brace-matching from each `#[semio_framework_async_macros::async_test]`-tagged item inside
`mod tests`, so `mailbox`/`actor`/`kernel`/etc. in one test can never bleed into another. Within
that bounded scope, a bare identifier match is safe by construction — these are test-local `let`
bindings, not calls into arbitrary production code with std-name collisions.
"""
import re
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🦀️component.rs"

def find_matching_brace(text, open_idx):
    depth = 0
    i = open_idx
    while i < len(text):
        if text[i] == '{':
            depth += 1
        elif text[i] == '}':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("unbalanced braces")

def statement_end(text, start):
    """From `start` (right after a let's `=`), find the index of the `;` that ends the statement,
    respecting (), [], {} nesting."""
    depth = 0
    i = start
    while i < len(text):
        c = text[i]
        if c in '([{':
            depth += 1
        elif c in ')]}':
            depth -= 1
        elif c == ';' and depth == 0:
            return i
        i += 1
    raise ValueError("no statement end found")

LET_RE = re.compile(r'\blet\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*(?::[^=]+?)?=\s*')

def process_fn_body(body):
    """Returns (new_body, edits_count)."""
    edits = 0
    out = []
    pos = 0
    while True:
        m = LET_RE.search(body, pos)
        if not m:
            out.append(body[pos:])
            break
        ident = m.group(1)
        rhs_start = m.end()
        end = statement_end(body, rhs_start)
        rhs = body[rhs_start:end]
        after = body[end + 1:]
        pattern = re.compile(r'\b' + re.escape(ident) + r'\.await\b')
        if pattern.search(after):
            out.append(body[pos:m.start()])
            out.append(body[m.start():rhs_start])
            rhs_stripped = rhs.rstrip()
            trailing_ws = rhs[len(rhs_stripped):]
            if rhs_stripped.endswith('.await'):
                new_rhs = rhs  # already awaited; leave as-is
            else:
                new_rhs = rhs_stripped + '.await' + trailing_ws
                edits += 1
            out.append(new_rhs)
            out.append(';')
            new_after, n = pattern.subn(ident, after)
            edits += n
            out.append(new_after)
            pos = len(body)
            body = body  # after is already consumed to end; but there may be MORE lets inside `after`
            # Since we replaced pattern occurrences textually inside `after` but did not re-scan it
            # for further `let` statements, we must continue scanning from the point right after
            # this statement's `;` within the NEW after-text. Recurse.
            tail_processed, tail_edits = process_fn_body(new_after)
            out[-1] = tail_processed
            edits += tail_edits
            break
        else:
            out.append(body[pos:end + 1])
            pos = end + 1
    return ''.join(out), edits

def main():
    text = open(PATH, encoding='utf-8').read()
    tests_idx = text.index('\nmod tests {')
    marker = '#[semio_framework_async_macros::async_test]\n        async fn '
    total_edits = 0
    fns_touched = 0
    search_pos = tests_idx
    pieces = [text[:tests_idx]]
    cursor = tests_idx
    while True:
        idx = text.find(marker, cursor)
        if idx == -1:
            pieces.append(text[cursor:])
            break
        fn_kw = text.index('async fn ', idx)
        brace_open = text.index('{', fn_kw)
        brace_close = find_matching_brace(text, brace_open)
        pieces.append(text[cursor:brace_open + 1])
        body = text[brace_open + 1:brace_close]
        new_body, edits = process_fn_body(body)
        if edits:
            fns_touched += 1
            total_edits += edits
        pieces.append(new_body)
        pieces.append('}')
        cursor = brace_close + 1
    new_text = ''.join(pieces)
    open(PATH, 'w', encoding='utf-8').write(new_text)
    print(f"functions touched: {fns_touched}, total .await hoist/strip edits: {total_edits}")

if __name__ == '__main__':
    main()
