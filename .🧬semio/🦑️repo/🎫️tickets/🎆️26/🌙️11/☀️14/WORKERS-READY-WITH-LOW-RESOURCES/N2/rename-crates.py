"""✂️ Renames the crates of rename-map.json in every manifest, lock, source and script of the worktree so their compiler outputs fit the 256 character bound below the longest canonical root; locks and sorted dependency tables keep their canonical order."""
import json, os, re, subprocess, sys, tempfile
root, mapping = sys.argv[1], json.load(open(sys.argv[2], encoding="utf8"))
apply_changes = "--apply" in sys.argv
dash = {old: new for old, new in mapping.items()}
under = {old.replace("-", "_"): new.replace("-", "_") for old, new in mapping.items()}
def compile_names(table, extra):
    names = sorted(table, key=len, reverse=True)
    return re.compile(r"(?<![A-Za-z0-9_" + extra + r"])(" + "|".join(re.escape(name) for name in names) + r")(?![A-Za-z0-9_" + extra + r"])")
dash_pattern, under_pattern = compile_names(dash, "-"), compile_names(under, "")
description = re.compile(r'(description = "Component crate of )([^:"]+): ([^"]*)(")')
def substitute(text, path):
    def parent(match):
        base, label = match.group(2), match.group(3)
        full = base + "-" + label
        if full in dash and dash[full].startswith(base + "-"):
            return match.group(1) + base + ": " + dash[full][len(base) + 1:] + match.group(4)
        return match.group(0)
    text = description.sub(parent, text)
    text = dash_pattern.sub(lambda match: dash[match.group(1)], text)
    return under_pattern.sub(lambda match: under[match.group(1)], text)
renamed_names = set(dash.values())
def resort_lock(text):
    head, *blocks = text.split("\n[[package]]\n")
    parsed = []
    for block in blocks:
        name = re.search(r'^name = "([^"]+)"', block, re.M).group(1)
        version = re.search(r'^version = "([^"]+)"', block, re.M).group(1)
        def sort_dependencies(match):
            entries = re.findall(r'^ "([^"]+)",$', match.group(2), re.M)
            if not any(entry.partition(" ")[0] in renamed_names for entry in entries):
                return match.group(0)
            return match.group(1) + "".join(' "' + entry + '",\n' for entry in sorted(entries, key=lambda entry: entry.partition(" ")[0])) + match.group(3)
        block = re.sub(r'(^dependencies = \[\n)((?: "[^"]+",\n)+)(\]$)', sort_dependencies, block, flags=re.M)
        parsed.append((name, version, block))
    renamed = {new for new in dash.values()}
    kept = [row for row in parsed if row[0] not in renamed]
    for row in sorted((row for row in parsed if row[0] in renamed), key=lambda row: (row[0], row[1])):
        position = next((index for index, other in enumerate(kept) if (other[0], other[1]) > (row[0], row[1])), len(kept))
        kept.insert(position, row)
    return "\n[[package]]\n".join([head] + [row[2] for row in kept])
section = re.compile(r"^\[(?:workspace\.)?(?:dev-|build-)?dependencies\]$|^\[target\..*dependencies\]$")
single = re.compile(r"^[A-Za-z0-9_-]+ = .*[\"}\]]$")
def resort_manifest(before, after):
    lines, output, index = after.split("\n"), [], 0
    original = before.split("\n")
    if len(original) != len(lines):
        return after
    while index < len(lines):
        output.append(lines[index])
        if section.match(lines[index]):
            end = index + 1
            while end < len(lines) and lines[end] and not lines[end].startswith("["):
                end += 1
            body, old_body = lines[index + 1:end], original[index + 1:end]
            keys = [line.split(" = ")[0] for line in old_body]
            if body and all(single.match(line) for line in old_body) and keys == sorted(keys):
                body = sorted(body, key=lambda line: line.split(" = ")[0])
            output.extend(body)
            index = end
            continue
        index += 1
    return "\n".join(output)
patterns = [name for old in mapping for name in (old, old.replace("-", "_"))]
listing = open(sys.argv[sys.argv.index("--files-from") + 1], encoding="utf8").read().split("\0") if "--files-from" in sys.argv else subprocess.run(["git", "grep", "--untracked", "-z", "-l", "-I", "-F", "-e", patterns[0]] + sum([["-e", p] for p in patterns[1:]], []) + ["--", ".", ":!.🧬semio", ":!.cache"], cwd=root, capture_output=True).stdout.decode("utf8").split("\0")
changed = []
for relative in filter(None, listing):
    path = os.path.join(root, relative)
    if os.path.islink(path) or not os.path.isfile(path):
        continue
    before = open(path, encoding="utf8", newline="").read()
    after = substitute(before, path)
    if relative.endswith("Cargo.lock"):
        after = resort_lock(after)
    elif relative.endswith("Cargo.toml"):
        after = resort_manifest(before, after)
    if after != before:
        changed.append(relative)
        if apply_changes:
            handle, temporary = tempfile.mkstemp(dir=os.path.dirname(path))
            with os.fdopen(handle, "w", encoding="utf8", newline="") as out:
                out.write(after)
            os.chmod(temporary, os.stat(path).st_mode & 0o7777)
            os.replace(temporary, path)
print(len(changed), "files")
for relative in changed:
    print(relative)
