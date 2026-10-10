import sys, os
root = sys.argv[1]
rows = []
for base, _, files in os.walk(root):
    for name in files:
        path = os.path.join(base, name)
        rows.append((len(path.encode("utf-16-le")) // 2, path))
for n, p in sorted(rows)[-3:]:
    print(n, p[len(root):])
