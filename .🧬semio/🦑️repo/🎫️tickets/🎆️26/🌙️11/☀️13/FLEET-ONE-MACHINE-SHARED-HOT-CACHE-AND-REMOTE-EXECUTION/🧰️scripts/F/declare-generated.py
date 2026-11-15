import json, os, re, sys

ROOT = "/Users/Shared/semio"
SCRIPT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📜️script.ts"
SKIP = {"🧰️framework/🤖️generated"}
apply = "--apply" in sys.argv
gens = sorted(g for g in (l.strip() for l in open(sys.argv[1], encoding="utf8")) if g and g not in SKIP)


def command(generated):
    return 'repo \\"%s\\" digest-generated \\"%s\\"' % (SCRIPT, generated)


owners = {}
for g in gens:
    module = os.path.dirname(g)
    for directory, names, files in os.walk(os.path.join(ROOT, module)):
        names[:] = [n for n in names if n not in ("node_modules", "🤖️generated", "dist", "target", ".cache", ".git")]
        if "📋️project.json" in files:
            owners.setdefault(os.path.relpath(directory, ROOT), []).append(g)

changed = 0
for owner, generated in sorted(owners.items()):
    path = os.path.join(ROOT, owner, "📋️project.json")
    text = open(path, encoding="utf8").read()
    items = [('{ "runtime": "%s" }' % command(g)) for g in generated if command(g) not in text]
    if not items:
        continue
    block = ", ".join(items)
    match = re.search(r'("namedInputs"\s*:\s*\{)', text)
    if match:
        inside = text[match.end():]
        default = re.search(r'^\s*"default"\s*:\s*\[(\s*)', inside, re.M)
        if default and not re.match(r'\s*\]', inside[default.end():]):
            at = match.end() + default.end()
            new = text[:at] + block + ", " + text[at:]
        elif default:
            at = match.end() + default.end()
            new = text[:at] + block + text[at:]
        else:
            empty = re.match(r'\s*\}', inside)
            body = '"default": ["{projectRoot}/**/*", "sharedGlobals", %s]' % block
            new = text[:match.end()] + " " + body + ("" if empty else ",") + text[match.end():]
    else:
        first = text.index("{")
        new = text[:first + 1] + '\n  "namedInputs": { "default": ["{projectRoot}/**/*", "sharedGlobals", %s] },' % block + text[first + 1:]
    json.loads(new)
    changed += 1
    if apply:
        open(path, "w", encoding="utf8").write(new)
print(("applied" if apply else "would change"), changed, "of", len(owners), "projects")
