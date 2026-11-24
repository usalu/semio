"""⚖️ Two logs of `measure.ts` side by side: `python3 cmp.py <before.log> <after.log>` prints every silhouette value (IoU and largest boundary distance per scale and view) that differs, marks a value beyond its gate with `!`, and the validate and sweep lines of both."""
import re, sys
def read(path):
    out, rest = {}, []
    for line in open(path):
        m = re.match(r"^\S+ 1:(\d+) bbox (.*?)  (.*)$", line.rstrip())
        if m:
            out[(m.group(1), "bbox")] = m.group(2)
            for part in m.group(3).split("  "):
                view, value = part.split(" ", 1)
                out[(m.group(1), view)] = value
        elif re.search(r"silhouette:|validate:|sweep:", line): rest.append(line.rstrip().split(" ", 1)[1])
    return out, rest
b, rb = read(sys.argv[1]); a, ra = read(sys.argv[2])
for key in a:
    if a[key] != b.get(key) or "!" in a[key]: print(f"  1:{key[0]} {key[1]}: {b.get(key)} -> {a[key]}")
for x, y in zip(rb, ra): print(" ", x if x == y else f"{x}  ->  {y}")
