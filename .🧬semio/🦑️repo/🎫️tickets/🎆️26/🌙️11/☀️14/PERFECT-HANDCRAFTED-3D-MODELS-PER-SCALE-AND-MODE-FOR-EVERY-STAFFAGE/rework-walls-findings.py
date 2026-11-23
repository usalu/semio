"""🧮 Groups the `volumes-feature` lines of r3-validate (stdin) by scale, list, thickness and id stem."""
import collections, re, sys

groups = collections.OrderedDict()
for line in sys.stdin:
    match = re.search(r'volumes\.(\d+)\.(\w+)\[\d+\]: Solid "([^"]+)" of \w+ is ([\d.]+) mm thin, below ([\d.]+)', line)
    if not match:
        if "volumes-feature" not in line:
            print(line.rstrip()[:300])
        continue
    scale, name, identifier, thin, limit = match.groups()
    stem = re.sub(r"\d+", "#", identifier)
    groups.setdefault((scale, name, stem, thin, limit), []).append(identifier)
for (scale, name, stem, thin, limit), ids in groups.items():
    print(f"1:{scale} {name} {stem} x{len(ids)} {thin} < {limit}")
