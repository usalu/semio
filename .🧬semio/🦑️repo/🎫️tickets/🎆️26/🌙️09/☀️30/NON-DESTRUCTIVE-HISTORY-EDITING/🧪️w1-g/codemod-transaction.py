"""🩹 Threads the tool-transaction slot through every Rust call site (ticket NON-DESTRUCTIVE-HISTORY-EDITING, W1-G).

1. `ArtifactCommand::Apply { .. }` / `ArtifactCommand::ApplyInLane { .. }` literals gain `transaction: None`; patterns
   without a rest pattern gain `..`.
2. `begin_apply_batch(..)` / `begin_outbound_apply_batch(..)` calls gain a trailing `None` argument.

Usage: python3 codemod-transaction.py [--write] <file>...   (prints every classified site; writes only with --write)
"""
import re
import sys

LITERAL = re.compile(r"ArtifactCommand::(Apply|ApplyInLane)\s*\{")
CALL = re.compile(r"\.(begin_apply_batch|begin_outbound_apply_batch)\s*\(")


def skip_literal(text, index):
    """Returns the index after the string/char literal starting at `index`, or None when none starts there."""
    if text.startswith('r#', index) or text.startswith('r"', index):
        hashes = 0
        cursor = index + 1
        while text[cursor] == '#':
            hashes += 1
            cursor += 1
        if text[cursor] != '"':
            return None
        terminator = '"' + '#' * hashes
        end = text.index(terminator, cursor + 1)
        return end + len(terminator)
    if text[index] == '"':
        cursor = index + 1
        while text[cursor] != '"':
            cursor += 2 if text[cursor] == '\\' else 1
        return cursor + 1
    if text[index] == "'":
        match = re.match(r"'(\\.[^']*|[^'\\])'", text[index:])
        if match:
            return index + match.end()
    return None


def matching(text, open_index, opening, closing):
    depth = 0
    cursor = open_index
    while cursor < len(text):
        if text.startswith('//', cursor):
            cursor = text.index('\n', cursor)
            continue
        skipped = skip_literal(text, cursor)
        if skipped is not None:
            cursor = skipped
            continue
        char = text[cursor]
        if char in '({[':
            depth += 1
        elif char in ')}]':
            depth -= 1
            if depth == 0:
                if char != closing:
                    raise ValueError(f"unbalanced {opening} at {open_index}")
                return cursor
        cursor += 1
    raise ValueError("unterminated")


def line_of(text, index):
    return text.count('\n', 0, index) + 1


def insertion(text, close_index, addition):
    """Inserts `addition` as the last item before `close_index`, respecting a trailing comma and multi-line layout."""
    cursor = close_index - 1
    while text[cursor] in ' \t\n\r':
        cursor -= 1
    multiline = '\n' in text[cursor:close_index]
    if multiline:
        line_start = text.rindex('\n', 0, cursor) + 1
        indent = re.match(r'[ \t]*', text[line_start:]).group(0)
        separator = '' if text[cursor] == ',' else ','
        terminator = '' if addition == '..' else ','
        return cursor + 1, f"{separator}\n{indent}{addition}{terminator}"
    if text[cursor] == ',':
        return cursor + 1, f" {addition}"
    return cursor + 1, f", {addition}"


def transform(path, write):
    text = open(path, encoding='utf-8').read()
    edits = []
    for match in LITERAL.finditer(text):
        line_start = text.rfind('\n', 0, match.start()) + 1
        if text[line_start:match.start()].lstrip().startswith('//'):
            continue
        open_index = match.end() - 1
        close_index = matching(text, open_index, '{', '}')
        body = text[open_index + 1:close_index]
        after = text[close_index + 1:close_index + 40].lstrip()
        before = text[max(0, match.start() - 40):match.start()]
        if re.search(r'(^|[\s,{])\.\.(\s|$)', body.strip() + ' ') or body.strip() == '..':
            kind = 'pattern-rest'
        elif after.startswith('=>') or (after.startswith('=') and not after.startswith('==')) or after.startswith('|') or after.startswith('if ') or 'matches!(' in before or re.search(r'\blet\s+$', before):
            kind = 'pattern'
        else:
            kind = 'literal'
        if 'transaction' in body:
            kind += '-done'
        print(f"{path}:{line_of(text, match.start())}: {kind}: {' '.join(body.split())[:100]}")
        if kind == 'pattern':
            edits.append(insertion(text, close_index, '..'))
        elif kind == 'literal':
            edits.append(insertion(text, close_index, 'transaction: None'))
    for match in CALL.finditer(text):
        open_index = match.end() - 1
        close_index = matching(text, open_index, '(', ')')
        arguments = text[open_index + 1:close_index]
        print(f"{path}:{line_of(text, match.start())}: call: {' '.join(arguments.split())[:100]}")
        edits.append(insertion(text, close_index, 'None'))
    if write and edits:
        for index, addition in sorted(edits, reverse=True):
            text = text[:index] + addition + text[index:]
        with open(path, 'w', encoding='utf-8') as handle:
            handle.write(text)


write = '--write' in sys.argv
for path in [argument for argument in sys.argv[1:] if argument != '--write']:
    transform(path, write)
