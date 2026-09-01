#!/usr/bin/env python3
"""📜️ Split the non-#region-marked `set-snapshot` leaves (apply/diff/inverse
free-function trio, no payload struct) — same undo-the-inlining contract as
the marker-based split, keyed on function-signature boundaries instead.
"""
import re
import sys
import os


def find_fn_block(text, fn_name):
    """Finds `pub fn <fn_name>(` ... matching closing brace, plus any immediately
    preceding /// or // comment lines. Returns (full_text_incl_comments, start, end) or None."""
    m = re.search(r'^pub (?:async )?fn ' + re.escape(fn_name) + r'\(', text, re.MULTILINE)
    if not m:
        return None
    start = m.start()
    # walk backward over contiguous comment lines directly above
    lines_before = text[:start].split('\n')
    i = len(lines_before) - 1
    # lines_before[-1] is '' (the newline right before start) typically since start is at col 0
    if lines_before and lines_before[-1] == '':
        i -= 1
    comment_start_line = i + 1
    while i >= 0 and (lines_before[i].strip().startswith('//')):
        comment_start_line = i
        i -= 1
    comment_block = '\n'.join(lines_before[comment_start_line:]) if comment_start_line <= len(lines_before) - 1 else ''
    true_start = start - (len('\n'.join(lines_before[comment_start_line:])) + 1) if comment_block else start
    if comment_block:
        true_start = len('\n'.join(lines_before[:comment_start_line])) + (1 if comment_start_line > 0 else 0)
    else:
        true_start = start

    # brace match from first '{' after start
    brace_idx = text.index('{', start)
    depth = 0
    j = brace_idx
    while j < len(text):
        if text[j] == '{':
            depth += 1
        elif text[j] == '}':
            depth -= 1
            if depth == 0:
                break
        j += 1
    end = j + 1
    full = text[true_start:end]
    return full, true_start, end


def import_names(use_line):
    s = use_line.strip()
    s = re.sub(r'^use\s+', '', s)
    s = re.sub(r';\s*$', '', s)
    if s.endswith('::*'):
        return ['\x00GLOB\x00']
    m = re.search(r'\{([^}]*)\}$', s)
    names = []
    if m:
        for it in m.group(1).split(','):
            it = it.strip()
            if not it:
                continue
            names.append(it.split(' as ')[-1].strip())
    else:
        last = s.split('::')[-1]
        names.append(last.split(' as ')[-1].strip())
    return names


def needed_use_lines(use_lines, body):
    out = []
    for line in use_lines:
        names = import_names(line)
        if '\x00GLOB\x00' in names:
            out.append(line)
            continue
        if any(re.search(r'\b' + re.escape(n) + r'\b', body) for n in names):
            out.append(line)
    return out


def get_use_block(text):
    return [l for l in text.split('\n') if l.strip().startswith('use ')]


def process(path):
    with open(path, encoding='utf-8') as f:
        text = f.read()

    diff_found = find_fn_block(text, 'diff')
    inv_found = find_fn_block(text, 'inverse')
    if not diff_found or not inv_found:
        return None

    use_lines = get_use_block(text)
    d = os.path.dirname(path)

    diff_full, d_start, d_end = diff_found
    inv_full, i_start, i_end = inv_found

    # write facet files (independent of removal order)
    for full, sub, label in ((diff_full, '🔺️diff', 'Diff'), (inv_full, '↩️inverse', 'Inverse')):
        needed = needed_use_lines(use_lines, full)
        doc = f'//! {"🔺️" if label == "Diff" else "↩️"} {label} for `set-snapshot`.'
        parts = [doc, '']
        parts.extend(needed)
        if needed:
            parts.append('')
        parts.append(full.rstrip('\n'))
        content = '\n'.join(parts) + '\n'
        outdir = os.path.join(d, sub)
        os.makedirs(outdir, exist_ok=True)
        with open(os.path.join(outdir, '🦀️.rs'), 'w', encoding='utf-8') as f:
            f.write(content)

    # remove both blocks from leaf text (remove later-starting one first to keep offsets valid)
    spans = sorted([(d_start, d_end), (i_start, i_end)], key=lambda x: -x[0])
    new_text = text
    for s, e in spans:
        # eat one leading blank-line run before the block too
        ss = s
        while ss > 0 and new_text[ss - 1] == '\n':
            ss -= 1
        new_text = new_text[:ss] + '\n' + new_text[e:]

    # prune now-unused use lines
    body_wo_use = '\n'.join(l for l in new_text.split('\n') if not l.strip().startswith('use '))
    kept = set(needed_use_lines(use_lines, body_wo_use))
    pruned = []
    for line in new_text.split('\n'):
        if line.strip().startswith('use ') and line not in kept:
            continue
        pruned.append(line)
    new_text = '\n'.join(pruned)

    new_text = re.sub(r'\n{3,}', '\n\n', new_text)
    new_text = new_text.rstrip('\n') + '\n'

    with open(path, 'w', encoding='utf-8') as f:
        f.write(new_text)

    return True


if __name__ == '__main__':
    worklist = sys.argv[1]
    with open(worklist, encoding='utf-8') as f:
        paths = [l.rstrip('\n') for l in f if l.strip()]
    ok, skipped = 0, []
    for p in paths:
        r = process(p)
        if r:
            ok += 1
        else:
            skipped.append(p)
    print(f'processed={ok} skipped={len(skipped)}')
    for s in skipped:
        print('SKIP', s)
