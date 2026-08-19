#!/usr/bin/env python3
"""🔬 Sanity pass: for every scanned file, flag any non-comment/non-blank raw line whose
stripped counterpart went fully blank — a proxy for stray comment/string-state corruption."""
import sys
sys.path.insert(0, '/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad')
from dyn_census import walk_rust_files, handle_raw_strings_pre, strip_comments_and_strings

files = walk_rust_files()
bad_files = []
for area, path in files:
    try:
        src = open(path, encoding='utf-8', errors='replace').read()
    except Exception:
        continue
    pre = handle_raw_strings_pre(src)
    stripped = strip_comments_and_strings(pre)
    rl = src.splitlines()
    sl = stripped.splitlines()
    if len(rl) != len(sl):
        bad_files.append((path, 'LINE COUNT MISMATCH', len(rl), len(sl)))
        continue
    first_bad = None
    bad_count = 0
    for i in range(len(rl)):
        raw = rl[i]
        rs = raw.strip()
        if not rs or rs.startswith('//'):
            continue
        strp = sl[i]
        if strp.strip() == '':
            bad_count += 1
            if first_bad is None:
                first_bad = (i+1, raw)
    if bad_count:
        bad_files.append((path, 'BLANKED-CODE-LINES', bad_count, first_bad))

print(f"scanned {len(files)} files, {len(bad_files)} suspicious")
for b in bad_files[:60]:
    print(b)
