import re, sys, os, json
E = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧩️entities"
MACROS = ("field", "set", "delete", "renaming", "kind")
START = re.compile(r"\b(field|set|delete|renaming|kind)!\s*\(")
def balanced(text, open_at):
    depth = 0; i = open_at; in_str = False
    while i < len(text):
        c = text[i]
        if in_str:
            if c == "\\": i += 1
            elif c == '"': in_str = False
        elif c == '"': in_str = True
        elif c in "([{": depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0: return i
        i += 1
    return -1
def sites(text):
    out = []
    for m in START.finditer(text):
        end = balanced(text, m.end() - 1)
        if end < 0: continue
        body = text[m.end():end]
        name = m.group(1)
        if name in ("set", "delete", "renaming"):
            h = re.match(r"\s*([a-z_0-9]+)\s*::\s*([A-Z]\w*)", body)
            if h: out.append((name, h.group(1), h.group(2)))
        for h in re.finditer(r"=>\s*([a-z_0-9]+)\s*::\s*([A-Z]\w*)\s*(?=[,)]|$)", body):
            out.append((name, h.group(1), h.group(2)))
    return out
if __name__ == "__main__":
    total = 0; files = {}
    for root, _, names in os.walk(E):
        for n in names:
            if n != "🦀️.rs": continue
            p = os.path.join(root, n)
            t = open(p, encoding="utf8").read()
            s = sites(t)
            if s:
                files[p] = sorted({x[1] for x in s}); total += len(s)
    print(total, len(files))
    for p, mods in files.items(): print(p[len(E):], len(mods))
