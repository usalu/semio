import re, sys, os

root = "/Users/ueli/Documents/semio"
listfile = sys.argv[1]
outfile = sys.argv[2]

diff_re = re.compile(r'^pub (async )?fn diff\(', re.M)
inv_re = re.compile(r'^pub (async )?fn inverse\(', re.M)

with open(listfile, encoding='utf-8') as f:
    files = [l.strip() for l in f if l.strip()]

inlined = []
for rel in files:
    path = os.path.join(root, rel)
    d = os.path.dirname(path)
    try:
        with open(path, encoding='utf-8') as fh:
            content = fh.read()
    except Exception as e:
        print(f"ERR reading {rel}: {e}", file=sys.stderr)
        continue
    has_diff = bool(diff_re.search(content))
    has_inv = bool(inv_re.search(content))
    diff_dir = os.path.isdir(os.path.join(d, "🔺️diff"))
    inv_dir = os.path.isdir(os.path.join(d, "↩️inverse"))
    is_inlined = (has_diff and not diff_dir) or (has_inv and not inv_dir)
    if is_inlined:
        inlined.append(rel)

with open(outfile, 'w', encoding='utf-8') as f:
    for rel in inlined:
        f.write(rel + "\n")

print(f"total={len(files)} inlined={len(inlined)}")
