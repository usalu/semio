import re, sys
ROOT = "/Users/ueli/Documents/semio"
NEW = [
    "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🆔️element-identity/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🐳️dock-skeleton/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🗄️storage-port/🟦️.ts",
]
apply = "--apply" in sys.argv
TAX = f"{ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
PRJ = f"{ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json"
jobs = [(TAX, '"wgpu-frame-worker": {', '"sourceModulePaths": [', ""), (TAX, '"wgpu-frame-worker": {', '"inputPatterns": [', ""), (PRJ, '"namedInputs": {', '"frameWorkerSources": [', "{workspaceRoot}/")]
texts = {}
entry_re = re.compile(r'^(\s*)"([^"]*)"(,?)\s*$')
for path, scope, key, prefix in jobs:
    s = texts.get(path) or open(path, encoding="utf8").read()
    assert s.count(scope) == 1, (path, scope)
    a = s.index(scope)
    i = s.index(key, a)
    body_start = i + len(key)
    depth, k = 1, body_start
    while depth:
        c = s[k]
        depth += (c == "[") - (c == "]")
        k += 1
    close = k - 1
    lines = s[body_start:close].split("\n")
    strs = [(n, entry_re.match(l)) for n, l in enumerate(lines)]
    strs = [(n, m) for n, m in strs if m]
    items = [m.group(2) for _, m in strs]
    stripped = [x[len(prefix):] if prefix and x.startswith(prefix) else x for x in items]
    indent = strs[0][1].group(1)
    for new in NEW:
        if prefix + new in items:
            continue
        after = next((n for (n, m), t in zip(strs, stripped) if t.encode() > new.encode()), None)
        if after is None:
            last = strs[-1][0]
            lines[last] = lines[last] + ","
            lines.insert(last + 1, f'{indent}"{prefix}{new}"')
        else:
            lines.insert(after, f'{indent}"{prefix}{new}",')
        strs = [(n, entry_re.match(l)) for n, l in enumerate(lines)]
        strs = [(n, m) for n, m in strs if m]
        items = [m.group(2) for _, m in strs]
        stripped = [x[len(prefix):] if prefix and x.startswith(prefix) else x for x in items]
    s = s[:body_start] + "\n".join(lines) + s[close:]
    texts[path] = s
    print(path.split("/")[-1], key, "now", len(items))
if apply:
    for path, s in texts.items():
        open(path, "w", encoding="utf8").write(s)
    print("applied")
else:
    print("dry")
