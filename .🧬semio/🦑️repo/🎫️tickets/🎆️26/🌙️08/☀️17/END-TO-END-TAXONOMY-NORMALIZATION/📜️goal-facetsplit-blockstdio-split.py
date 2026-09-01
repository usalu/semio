#!/usr/bin/env python3
"""📜️ Undo the diff/inverse inlining for block+stdio mutation leaves.

Splits //#region 🔖️Diff / //#region 🔖️Inverse blocks (optionally wrapped in
//#region 🔺️diff / //#region ↩️inverse) out of a mutation's direct leaf 🦀️.rs
into sibling 🔺️diff/🦀️.rs and ↩️inverse/🦀️.rs files, rewrites the leaf's
MutationKind impl to delegate to them, and rewires the plugin's glue.rs to
mount the new facet modules (mirroring the ✏️s/🔌️plugins/🌿️vcs exemplar).
"""
import re
import sys

DIFF_INNER = re.compile(r'[ \t]*//#region 🔖️Diff\n(.*?)//#endregion 🔖️Diff\n', re.DOTALL)
INV_INNER = re.compile(r'[ \t]*//#region 🔖️Inverse\n(.*?)//#endregion 🔖️Inverse\n', re.DOTALL)
DIFF_OUTER = re.compile(r'\n*[ \t]*//#region 🔺️diff\n(.*?)//#endregion 🔺️diff\n', re.DOTALL)
INV_OUTER = re.compile(r'\n*[ \t]*//#region ↩️inverse\n(.*?)//#endregion ↩️inverse\n', re.DOTALL)


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
            name = it.split(' as ')[-1].strip()
            names.append(name)
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


def extract_region(text, inner_re, outer_re):
    """Returns (inner_block_text_with_markers, leaf_text_with_region_removed) or (None, text)."""
    om = outer_re.search(text)
    if om:
        outer_full = om.group(0)
        inner_m = inner_re.search(om.group(1))
        inner_full = inner_m.group(0) if inner_m else None
        new_text = text[:om.start()] + '\n' + text[om.end():]
        return inner_full, new_text
    im = inner_re.search(text)
    if im:
        inner_full = im.group(0)
        start = im.start()
        end = im.end()
        # eat one extra trailing blank line for tidiness
        new_text = text[:start] + text[end:]
        return inner_full, new_text
    return None, text


def get_use_block(text):
    lines = []
    for line in text.split('\n'):
        s = line.strip()
        if s.startswith('use '):
            lines.append(line)
    return lines


def get_orig_doc(git_show_text):
    if git_show_text is None:
        return None
    lines = git_show_text.split('\n')
    doc_lines = []
    for line in lines:
        if line.startswith('//!'):
            doc_lines.append(line)
        elif doc_lines:
            break
    return '\n'.join(doc_lines) if doc_lines else None


def process(path, orig_diff_doc, orig_inv_doc, dry=False):
    with open(path, 'r', encoding='utf-8') as f:
        text = f.read()

    diff_block, text2 = extract_region(text, DIFF_INNER, DIFF_OUTER)
    inv_block, text3 = extract_region(text2, INV_INNER, INV_OUTER)

    if diff_block is None and inv_block is None:
        return None

    use_lines = get_use_block(text)

    struct_m = re.search(r'pub struct (\w+)', text)
    struct_name = struct_m.group(1) if struct_m else None

    import os
    d = os.path.dirname(path)
    result = {'diff_written': False, 'inverse_written': False}

    if diff_block is not None:
        needed = needed_use_lines(use_lines, diff_block)
        if orig_diff_doc:
            doc = orig_diff_doc
        elif struct_name:
            doc = f'//! 🔺️ Diff for `{struct_name}`.'
        else:
            doc = '//! 🔺️ Diff facet.'
        parts = [doc, '']
        parts.extend(needed)
        if needed:
            parts.append('')
        parts.append(diff_block.rstrip('\n'))
        content = '\n'.join(parts) + '\n'
        os.makedirs(os.path.join(d, '🔺️diff'), exist_ok=True)
        outp = os.path.join(d, '🔺️diff', '🦀️.rs')
        if not dry:
            with open(outp, 'w', encoding='utf-8') as f:
                f.write(content)
        result['diff_written'] = True

    if inv_block is not None:
        needed = needed_use_lines(use_lines, inv_block)
        if orig_inv_doc:
            doc = orig_inv_doc
        elif struct_name:
            doc = f'//! ↩️ Inverse for `{struct_name}`.'
        else:
            doc = '//! ↩️ Inverse facet.'
        parts = [doc, '']
        parts.extend(needed)
        if needed:
            parts.append('')
        parts.append(inv_block.rstrip('\n'))
        content = '\n'.join(parts) + '\n'
        os.makedirs(os.path.join(d, '↩️inverse'), exist_ok=True)
        outp = os.path.join(d, '↩️inverse', '🦀️.rs')
        if not dry:
            with open(outp, 'w', encoding='utf-8') as f:
                f.write(content)
        result['inverse_written'] = True

    # Now rewrite delegate calls in the remaining leaf text (text3)
    final = text3
    if diff_block is not None:
        final = re.sub(r'(?m)^(\s*)diff\((.*)\)\s*$', r'\1super::diff::diff(\2)', final, count=1)
    if inv_block is not None:
        final = re.sub(r'(?m)^(\s*)inverse\((.*)\)\s*$', r'\1super::inverse::inverse(\2)', final, count=1)

    # prune now-unused use lines from the leaf (their referents moved to the facet files)
    body_wo_use = '\n'.join(l for l in final.split('\n') if not l.strip().startswith('use '))
    kept = set(needed_use_lines(use_lines, body_wo_use))
    pruned_lines = []
    for line in final.split('\n'):
        if line.strip().startswith('use ') and line not in kept:
            continue
        pruned_lines.append(line)
    final = '\n'.join(pruned_lines)

    # collapse 3+ blank lines to 2, trim trailing whitespace-only blank run at EOF to single newline
    final = re.sub(r'\n{3,}', '\n\n', final)
    final = final.rstrip('\n') + '\n'

    if not dry:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(final)

    result['leaf_changed'] = (final != text)
    return result


if __name__ == '__main__':
    mode = sys.argv[1] if len(sys.argv) > 1 else 'run'
    worklist_file = sys.argv[2]
    with open(worklist_file, 'r', encoding='utf-8') as f:
        paths = [l.rstrip('\n') for l in f if l.strip()]

    ok, skipped = 0, []
    for p in paths:
        r = process(p, None, None, dry=(mode == 'dry'))
        if r is None:
            skipped.append(p)
        else:
            ok += 1
    print(f'processed={ok} skipped={len(skipped)}')
    for s in skipped:
        print('SKIP', s)
