"""⚖️ Compares two logs of `../MR/measure.ts` (before, after): `python3 silcmp.py <before.log> <after.log>` prints per scale and view the IoU and the largest boundary distance of the silhouette guard where they changed, every failing value, and the validate and sweep lines of the second log."""
import re, sys
def read(path):
    values, tail = {}, []
    for line in open(path):
        m = re.match(r"\S+ 1:(\d+) bbox (.*?)  (.*)", line)
        if m:
            values[m.group(1)] = (m.group(2), dict((v.split()[0], v.split()[1]) for v in m.group(3).strip().split("  ")))
        elif re.search(r"silhouette:|validate:|sweep:|^   ", line):
            tail.append(line.rstrip()[:220])
    return values, tail
before, _ = read(sys.argv[1])
after, tail = read(sys.argv[2])
for scale, (bbox, views) in after.items():
    old = before.get(scale, ("", {}))
    changed = [f"{view} {old[1].get(view)} -> {value}" for view, value in views.items() if old[1].get(view) != value]
    if "!" in bbox or bbox != old[0]:
        changed.append(f"bbox {old[0]} -> {bbox}")
    if changed:
        print(f"1:{scale} " + "; ".join(changed))
print("\n".join(tail))
