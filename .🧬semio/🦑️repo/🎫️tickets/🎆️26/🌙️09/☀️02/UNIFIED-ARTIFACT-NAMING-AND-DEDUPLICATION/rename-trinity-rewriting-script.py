import re, os, sys

ROOT = "/Users/ueli/Documents/semio"
SCOPE_DIR = os.path.join(ROOT, "✏️s/🔌️plugins/🔱️trinity")

# Load compound token mapping
mapping = {}
with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/301f96d8-e5ef-4237-8963-4c6d10cd12d4/scratchpad/mapping.txt", encoding="utf-8") as f:
    for line in f:
        line = line.rstrip("\n")
        if not line or " -> " not in line:
            continue
        old, new = line.split(" -> ")
        mapping[old] = new

# Sort tokens longest-first to avoid partial-shadowing issues
tokens_sorted = sorted(mapping.keys(), key=len, reverse=True)
# Build one big regex with word boundaries
token_pattern = re.compile(r'\b(' + '|'.join(re.escape(t) for t in tokens_sorted) + r')\b')

changed_files = []

skip_dirs = {"node_modules", "pkg", "target", ".git"}

for dirpath, dirnames, filenames in os.walk(SCOPE_DIR):
    dirnames[:] = [d for d in dirnames if d not in skip_dirs]
    for fn in filenames:
        path = os.path.join(dirpath, fn)
        try:
            with open(path, "r", encoding="utf-8") as f:
                content = f.read()
        except (UnicodeDecodeError, IsADirectoryError):
            continue
        orig = content

        # Step A: exact hygiene fix for Fix 2 (must precede generic path substitution)
        content = content.replace('text.♻️rewrite', 'text.rewriting')

        # Step B: generic artifact directory path token
        content = content.replace('♻️rewrite', '♻️rewriting')

        # Step C: module path separator form
        content = content.replace('::rewrite::', '::rewriting::')

        # Step D: mod declaration forms
        content = re.sub(r'\bmod rewrite\b', 'mod rewriting', content)

        # Step E: compound identifier token mapping
        content = token_pattern.sub(lambda m: mapping[m.group(1)], content)

        if content != orig:
            with open(path, "w", encoding="utf-8") as f:
                f.write(content)
            changed_files.append(path)

print(f"Changed {len(changed_files)} files")
with open("/private/tmp/claude-501/-Users-ueli-Documents-semio/301f96d8-e5ef-4237-8963-4c6d10cd12d4/scratchpad/changed_files.txt", "w", encoding="utf-8") as f:
    for p in changed_files:
        f.write(p + "\n")
