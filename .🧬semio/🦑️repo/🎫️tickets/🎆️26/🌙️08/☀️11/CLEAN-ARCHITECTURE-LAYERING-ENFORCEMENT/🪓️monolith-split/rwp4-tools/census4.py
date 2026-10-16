# 📊️ WP4 census of the staging crates: files, bytes, lines of every `📥️staging/<crate>` tree (lib part, free items and members; tests are mounted by the old chunks).
import os, sys, glob
root = sys.argv[1]
rows = []
for d in sorted(glob.glob(root + '/📥️staging/*/')):
    fs = sorted(glob.glob(d + '*/🦀️.rs'))
    b = sum(os.path.getsize(f) for f in fs); l = sum(open(f, encoding='utf-8').read().count('\n') for f in fs)
    rows.append((os.path.basename(d.rstrip('/')), len(fs), b, l))
print('| staging folder | files | bytes | lines | within 3500 / 28 / 230000 |'); print('|---|---:|---:|---:|---|')
for n, f, b, l in sorted(rows, key=lambda r: -r[3]):
    print(f'| `{n}` | {f} | {b:,} | {l:,} | {"yes" if f <= 28 and b <= 230000 and l <= 3500 else "NO"} |')
