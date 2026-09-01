import os, re, sys

root = "/Users/ueli/Documents/semio"
all_leaves_file = sys.argv[1]
out_file = sys.argv[2]

diff_re = re.compile(r'^pub (async )?fn diff\(', re.M)
inv_re = re.compile(r'^pub (async )?fn inverse\(', re.M)

results = []
with open(all_leaves_file, encoding="utf-8") as f:
    leaves = [l.rstrip("\n") for l in f if l.strip()]

for leaf in leaves:
    full = os.path.join(root, leaf)
    d = os.path.dirname(full)
    try:
        with open(full, encoding="utf-8") as f:
            content = f.read()
    except Exception as e:
        results.append(f"ERROR reading {leaf}: {e}")
        continue
    has_diff_fn = bool(diff_re.search(content))
    has_inv_fn = bool(inv_re.search(content))
    has_diff_dir = os.path.isdir(os.path.join(d, "🔺️diff"))
    has_inv_dir = os.path.isdir(os.path.join(d, "↩️inverse"))
    inlined = (has_diff_fn and not has_diff_dir) or (has_inv_fn and not has_inv_dir)
    if inlined:
        results.append(leaf)

with open(out_file, "w", encoding="utf-8") as f:
    for r in results:
        f.write(r + "\n")

print(f"total_checked={len(leaves)} inlined={len(results)}")
