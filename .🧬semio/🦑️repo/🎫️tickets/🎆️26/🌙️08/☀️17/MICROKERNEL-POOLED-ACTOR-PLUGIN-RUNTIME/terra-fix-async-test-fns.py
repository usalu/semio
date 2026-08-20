#!/usr/bin/env python3
"""🧵 Converts `#[test] async fn NAME(...) { BODY }` into a sync `#[test] fn` per R4 clause 5 —
`#[test]` cannot run an async fn directly (std has no executor for it), so every such test in this
crate needs either (a) plain `fn` when its body has no `.await` at all, or (b) `fn` whose body is
`block_on_test(async { BODY })` when it does.

STRUCTURAL, NOT NAME-KEYED: this matches on the `#[test]` attribute + `async fn` signature shape,
never on identifier names, and closes each function by matching the closing brace at the SAME
INDENTATION as the `async fn` line — verified safe for this codebase's consistent 4-space rustfmt
style. Every rewrite is verified afterward with `cargo check --all-targets`; this script is not
applied blindly.

Run with --dry-run first to inspect the count/diff before --apply.
"""
import argparse
import re
import sys

TEST_ASYNC_FN_RE = re.compile(r'^(?P<indent>\s*)async fn (?P<name>\w+)\(([^)]*)\)(\s*->\s*[^{]+)?\s*\{\s*$')


def find_matching_close(lines, start_idx, indent):
    """Find the index of the line that is exactly `indent + '}'` closing the fn opened at start_idx."""
    close_line = indent + '}'
    for i in range(start_idx + 1, len(lines)):
        if lines[i] == close_line:
            return i
    return None


def transform_file(path, apply):
    with open(path, encoding='utf-8') as f:
        lines = f.read().split('\n')

    out = []
    i = 0
    n_wrapped = 0
    n_dropped_async = 0
    while i < len(lines):
        line = lines[i]
        m = TEST_ASYNC_FN_RE.match(line)
        # Only treat as a test fn if the immediately preceding non-blank line (within 3 lines up)
        # is `#[test]` (allow doc comments/attributes in between is NOT handled here — checked below).
        is_test = False
        if m:
            for back in range(1, 4):
                j = i - back
                if j < 0:
                    break
                prev = lines[j].strip()
                if prev == '#[test]':
                    is_test = True
                    break
                if prev == '' or prev.startswith('///') or prev.startswith('#['):
                    continue
                break
        if m and is_test:
            indent = m.group('indent')
            name = m.group('name')
            close_idx = find_matching_close(lines, i, indent)
            if close_idx is None:
                print(f"  !! could not find matching close for {name} at line {i+1} in {path}", file=sys.stderr)
                out.append(line)
                i += 1
                continue
            body = lines[i + 1:close_idx]
            body_text = '\n'.join(body)
            has_await = '.await' in body_text
            sig_line = line[:-1].replace('async fn', 'fn', 1)  # strip trailing '{'
            if has_await:
                out.append(sig_line + '{')
                out.append(indent + '    block_on_test(async {')
                for b in body:
                    out.append(('    ' + b) if b.strip() else b)
                out.append(indent + '    });')
                out.append(indent + '}')
                n_wrapped += 1
            else:
                out.append(sig_line + '{')
                out.extend(body)
                out.append(indent + '}')
                n_dropped_async += 1
            i = close_idx + 1
            continue
        out.append(line)
        i += 1

    print(f"{path}: wrapped={n_wrapped} dropped_async_only={n_dropped_async}")
    if apply and (n_wrapped or n_dropped_async):
        with open(path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(out))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('files', nargs='+')
    ap.add_argument('--apply', action='store_true')
    args = ap.parse_args()
    for path in args.files:
        transform_file(path, args.apply)


if __name__ == '__main__':
    main()
