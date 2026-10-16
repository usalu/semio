import sys
path, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
count = int(sys.argv[4]) if len(sys.argv) > 4 else 1
data = open(path, encoding="utf-8").read()
assert data.count(old) == count, f"anchor count {data.count(old)} != {count} in {path}"
open(path, "w", encoding="utf-8").write(data.replace(old, new))
print("edited", path, count)
