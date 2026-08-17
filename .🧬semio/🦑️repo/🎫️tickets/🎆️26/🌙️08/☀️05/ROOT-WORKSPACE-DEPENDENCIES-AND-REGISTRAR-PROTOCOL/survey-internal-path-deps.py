import re, os, collections

root = os.getcwd()
manifests = []
for dirpath, dirnames, filenames in os.walk(root):
    if '/target/' in dirpath or dirpath.endswith('/target'):
        dirnames[:] = []
        continue
    if '.git' in dirpath.split(os.sep):
        continue
    for f in filenames:
        if f == 'Cargo.toml':
            manifests.append(os.path.join(dirpath, f))

path_re = re.compile(r'^\s*([A-Za-z0-9_\-]+)\s*=\s*\{[^}]*path\s*=\s*"([^"]+)"[^}]*\}', re.MULTILINE)
name_re = re.compile(r'^\s*name\s*=\s*"([^"]+)"', re.MULTILINE)

counter = collections.Counter()

for m in manifests:
    try:
        text = open(m, encoding='utf-8').read()
    except Exception:
        continue
    base = os.path.dirname(m)
    for match in path_re.finditer(text):
        depname, relpath = match.groups()
        target = os.path.normpath(os.path.join(base, relpath))
        counter[target] += 1

results = []
for target, count in counter.items():
    if count < 6:
        continue
    cargo_toml = os.path.join(target, 'Cargo.toml')
    pkg_name = None
    if os.path.isfile(cargo_toml):
        txt = open(cargo_toml, encoding='utf-8').read()
        # only look in [package] section (before first [dependencies] etc.), take first name= match
        nm = name_re.search(txt)
        if nm:
            pkg_name = nm.group(1)
    rel = os.path.relpath(target, root)
    results.append((count, pkg_name, rel))

results.sort(key=lambda x: -x[0])
print(f"total qualifying (>=6): {len(results)}")
for count, pkg, rel in results:
    print(f"{count}\t{pkg}\t{rel}")

# check for name collisions
names = collections.Counter(r[1] for r in results if r[1])
dups = {n:c for n,c in names.items() if c>1}
print("\nDUPLICATE PACKAGE NAMES:", dups)
missing = [r for r in results if not r[1]]
print("\nMISSING PACKAGE NAME (no Cargo.toml or no name field):", len(missing))
for r in missing:
    print(r)
