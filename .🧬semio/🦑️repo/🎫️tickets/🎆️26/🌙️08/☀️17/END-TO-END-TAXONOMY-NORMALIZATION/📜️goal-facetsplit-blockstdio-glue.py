#!/usr/bin/env python3
"""📜️ Mount 🔺️diff/↩️inverse facet modules in a plugin's glue.rs, mirroring
the ✏️s/🔌️plugins/🌿️vcs exemplar's `pub mod diff; pub mod inverse; mod component;`
mount order, for every mutation dir that now has split facet files.
"""
import re
import sys


def patch_glue(glue_path, plugin_prefix, mutation_dirs):
    with open(glue_path, 'r', encoding='utf-8') as f:
        text = f.read()
    missing = []
    count = 0
    for d in mutation_dirs:
        if not d.startswith(plugin_prefix):
            missing.append(('BAD-PREFIX', d))
            continue
        rel = d[len(plugin_prefix):]
        leaf_rel = '../../' + rel + '/🦀️.rs'
        diff_rel = '../../' + rel + '/🔺️diff/🦀️.rs'
        inv_rel = '../../' + rel + '/↩️inverse/🦀️.rs'
        pattern = re.compile(r'^([ \t]*)#\[path = "' + re.escape(leaf_rel) + r'"\]\n\1mod component;\n', re.MULTILINE)
        m = pattern.search(text)
        if m:
            indent = m.group(1)
            insertion = (
                f'{indent}#[path = "{diff_rel}"]\n'
                f'{indent}pub mod diff;\n'
                f'{indent}#[path = "{inv_rel}"]\n'
                f'{indent}pub mod inverse;\n'
            )
            text = text[:m.start()] + insertion + text[m.start():]
            count += 1
            continue
        # fallback: flat single-line mount `#[path = "leaf_rel"]\npub mod NAME;\n` with no
        # existing { mod component; pub use component::*; } wrapper — convert it into one.
        flat_pattern = re.compile(r'^([ \t]*)#\[path = "' + re.escape(leaf_rel) + r'"\]\n\1pub mod (\w+);\n', re.MULTILINE)
        fm = flat_pattern.search(text)
        if not fm:
            missing.append(('NO-MOUNT', d))
            continue
        indent = fm.group(1)
        mod_name = fm.group(2)
        inner = indent + '    '
        replacement = (
            f'{indent}#[path = "."]\n'
            f'{indent}pub mod {mod_name} {{\n'
            f'{inner}#[path = "{diff_rel}"]\n'
            f'{inner}pub mod diff;\n'
            f'{inner}#[path = "{inv_rel}"]\n'
            f'{inner}pub mod inverse;\n'
            f'{inner}#[path = "{leaf_rel}"]\n'
            f'{inner}mod component;\n'
            f'{inner}pub use component::*;\n'
            f'{indent}}}\n'
        )
        text = text[:fm.start()] + replacement + text[fm.end():]
        count += 1
    with open(glue_path, 'w', encoding='utf-8') as f:
        f.write(text)
    return count, missing


if __name__ == '__main__':
    glue_path = sys.argv[1]
    plugin_prefix = sys.argv[2]
    dirs_file = sys.argv[3]
    with open(dirs_file, 'r', encoding='utf-8') as f:
        dirs = [l.rstrip('\n') for l in f if l.strip()]
    count, missing = patch_glue(glue_path, plugin_prefix, dirs)
    print(f'mounted={count} missing={len(missing)}')
    for kind, d in missing:
        print(kind, d)
