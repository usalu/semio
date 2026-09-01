import re, os, sys

ROOT = "/Users/ueli/Documents/semio"
SCRATCH = "/private/tmp/claude-501/-Users-ueli-Documents-semio/b3777651-e26e-4d76-aa75-86723494357b/scratchpad"
PINNED = os.path.join(SCRATCH, "pinned_extract")

with open(os.path.join(SCRATCH, "inlined_before.txt"), encoding="utf-8") as f:
    leaves = [l.strip() for l in f if l.strip()]

use_line_re = re.compile(r'^use ([\w:]+?)(?:::\{([\w,\s]+)\})?;\n', re.M)

def find_diff_doc_start(content):
    m = re.search(r'\npub async fn diff\(', content)
    if not m:
        raise ValueError("no diff fn found")
    fn_start = m.start() + 1
    # walk backward over contiguous /// lines directly above
    idx = fn_start
    while True:
        prev_nl = content.rfind('\n', 0, idx - 1)
        line_start = prev_nl + 1
        line = content[line_start:idx]
        if line.startswith('///'):
            idx = line_start
            continue
        break
    return idx

def prune_unused_imports(prefix):
    matches = list(use_line_re.finditer(prefix))
    if not matches:
        return prefix
    block_start = matches[0].start()
    block_end = matches[-1].end()
    after_text = prefix[block_end:]
    new_lines = []
    for m in matches:
        path, group = m.group(1), m.group(2)
        if group:
            items = [x.strip() for x in group.split(',') if x.strip()]
            kept = [it for it in items if re.search(r'\b' + re.escape(it) + r'\b', after_text)]
            if not kept:
                continue
            if len(kept) == 1:
                new_lines.append(f"use {path}::{kept[0]};\n")
            else:
                new_lines.append(f"use {path}::{{{', '.join(kept)}}};\n")
        else:
            item = path.rsplit('::', 1)[-1]
            if re.search(r'\b' + re.escape(item) + r'\b', after_text):
                new_lines.append(f"use {path};\n")
    new_block = ''.join(new_lines)
    return prefix[:block_start] + new_block + prefix[block_end:]

report = []
skipped = []

for leaf in leaves:
    abs_leaf = os.path.join(ROOT, leaf)
    d_rel = os.path.dirname(leaf)
    d_abs = os.path.join(ROOT, d_rel)

    pinned_diff_path = os.path.join(PINNED, d_rel, '🔺️diff', '🦀️component.rs')
    pinned_inv_path = os.path.join(PINNED, d_rel, '↩️inverse', '🦀️component.rs')
    if not (os.path.exists(pinned_diff_path) and os.path.exists(pinned_inv_path)):
        skipped.append((leaf, 'no pinned counterpart'))
        continue

    with open(abs_leaf, encoding='utf-8') as f:
        leaf_content = f.read()

    if 'pub async fn diff(' not in leaf_content:
        skipped.append((leaf, 'no inline diff fn (already restored?)'))
        continue

    with open(pinned_diff_path, encoding='utf-8') as f:
        pinned_diff = f.read()
    with open(pinned_inv_path, encoding='utf-8') as f:
        pinned_inv = f.read()

    new_diff_content = pinned_diff.replace('mutation::', '')
    new_inv_content = pinned_inv.replace('mutation::', '')

    trunc_at = find_diff_doc_start(leaf_content)
    new_leaf_prefix = leaf_content[:trunc_at].rstrip() + "\n"

    if 'diff(self, base)' not in new_leaf_prefix or 'inverse(self, base)' not in new_leaf_prefix:
        skipped.append((leaf, 'delegate call pattern not found'))
        continue
    new_leaf_prefix = new_leaf_prefix.replace('diff(self, base)', 'super::diff::diff(self, base)')
    new_leaf_prefix = new_leaf_prefix.replace('inverse(self, base)', 'super::inverse::inverse(self, base)')

    new_leaf_prefix = prune_unused_imports(new_leaf_prefix)

    diff_dir = os.path.join(d_abs, '🔺️diff')
    inv_dir = os.path.join(d_abs, '↩️inverse')
    os.makedirs(diff_dir, exist_ok=True)
    os.makedirs(inv_dir, exist_ok=True)
    with open(os.path.join(diff_dir, '🦀️.rs'), 'w', encoding='utf-8') as f:
        f.write(new_diff_content)
    with open(os.path.join(inv_dir, '🦀️.rs'), 'w', encoding='utf-8') as f:
        f.write(new_inv_content)
    with open(abs_leaf, 'w', encoding='utf-8') as f:
        f.write(new_leaf_prefix)

    report.append(leaf)

print("processed:", len(report))
print("skipped:", len(skipped))
for s in skipped:
    print(" SKIP:", s)

with open(os.path.join(SCRATCH, "processed.txt"), 'w', encoding='utf-8') as f:
    for r in report:
        f.write(r + '\n')
