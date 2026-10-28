"""🧫 [isolate] anchored test conversions: `edit(root, rel, pairs, helper=True)` replaces each exact `old` exactly `count` times (exit 2 naming the anchor when a count differs, never guesses) and, with `helper`, imports the shared private repository and opens it once at module level (`const repository=await privateRepositoryWorkspaceV1();`) after the last import. Batch scripts in `edits/` call it: `python3 -I edits/<batch>.py <root>`."""
import os, re, sys
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
HELPER = LIB + "/🧪️testing/🧫️private-repository/🟦️.ts"
OPEN = "const repository=await privateRepositoryWorkspaceV1();"
def relpath(rel, target=HELPER):
    path = os.path.relpath(target, os.path.dirname(rel)).replace(os.sep, "/")
    return path if path.startswith(".") else "./" + path
def last_import_end(text):
    end = 0
    for m in re.finditer(r'^(?:import\b[^;]*?from\s*"[^"]+"(?:\s*with\s*\{[^}]*\})?;|import\s*"[^"]+";)', text, re.M | re.S): end = m.end()
    if end == 0: sys.exit("no import anchor")
    return end
def first_statement(text, after):
    position, comment = 0, None
    for line in text.splitlines(keepends=True):
        if line.startswith(("/*", "//")) and comment is None: comment = position
        elif line[:1] not in ("", " ", "\t", "\n", "}", ")", "*") and not line.startswith(("import", "/*", "//", "export{", "export {")):
            if position < after: return comment if comment is not None else position
            return after
        elif line.startswith("import") or line.strip() == "": comment = None
        position += len(line)
    return after
def edit(root, rel, pairs, helper=True, names=("privateRepositoryWorkspaceV1",)):
    path = os.path.join(root, rel)
    text = open(path, encoding="utf-8").read()
    for old, new, count in pairs:
        found = text.count(old)
        if found != count: sys.stderr.write("ANCHOR %s: %d != %d: %s\n" % (rel[-80:], found, count, old[:120])); sys.exit(2)
        text = text.replace(old, new)
    if helper and OPEN not in text:
        end = last_import_end(text)
        line = "\nimport{%s}from\"%s\";\n%s" % (",".join(names), relpath(rel), OPEN)
        text = text[:end] + line + text[end:]
    with open(path, "w", encoding="utf-8") as f: f.write(text)
    print("edited", rel[-90:])
