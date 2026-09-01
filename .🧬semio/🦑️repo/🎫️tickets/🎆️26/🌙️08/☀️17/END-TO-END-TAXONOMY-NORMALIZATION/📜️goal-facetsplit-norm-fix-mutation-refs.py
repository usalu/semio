import re, sys, os

ROOT = "/Users/ueli/Documents/semio"
SC = os.path.dirname(__file__)
DRY = "--dry" in sys.argv

with open(os.path.join(SC, "leaf_alias_map.txt"), encoding="utf-8") as f:
    pairs = [line.rstrip("\n").split("\t") for line in f if line.strip()]

PREFIX = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
artifact_modnames = {}
for leaf, alias in pairs:
    rest = leaf[len(PREFIX):]
    artifact_seg = rest.split("/", 1)[0]
    artifact_root = PREFIX + artifact_seg
    artifact_modnames.setdefault(artifact_root, set()).add(alias)

total_files_changed = 0
total_replacements = 0
changed_files = []

for artifact_root, names in artifact_modnames.items():
    full_artifact_dir = os.path.join(ROOT, artifact_root)
    alt = "|".join(sorted((re.escape(n) for n in names), key=len, reverse=True))
    pat = re.compile(r'\b(' + alt + r')::mutation::')
    for dirpath, dirnames, filenames in os.walk(full_artifact_dir):
        for fn in filenames:
            if not fn.endswith(".rs"):
                continue
            fp = os.path.join(dirpath, fn)
            with open(fp, encoding="utf-8") as f:
                content = f.read()
            if "::mutation::" not in content:
                continue
            new_content, n = pat.subn(r'\1::', content)
            if n > 0:
                total_files_changed += 1
                total_replacements += n
                rel = os.path.relpath(fp, ROOT)
                changed_files.append((rel, n))
                if not DRY:
                    with open(fp, "w", encoding="utf-8") as f:
                        f.write(new_content)

print(f"files_changed={total_files_changed} replacements={total_replacements} dry={DRY}")
for rel, n in changed_files:
    print(f"  {rel}: {n}")
