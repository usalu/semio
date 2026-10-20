"""📖️ Shared reader for bun.lock (JSONC with trailing commas), the six lock roots and the member manifests. Standard library only."""
import json, os

ROOTS = [".", "♻️mit-bestand", "✏️s", "🌎️hub", "🎓️teaching", "🏢️semio-tech"]
FIELDS = ["dependencies", "devDependencies", "optionalDependencies", "peerDependencies"]

def strip_trailing_commas(s):
    out, instr, i, n = [], False, 0, len(s)
    while i < n:
        c = s[i]
        if instr:
            out.append(c)
            if c == "\\": out.append(s[i + 1]); i += 1
            elif c == '"': instr = False
        elif c == '"': instr = True; out.append(c)
        elif c == ",":
            j = i + 1
            while j < n and s[j] in " \t\r\n": j += 1
            if not (j < n and s[j] in "}]"): out.append(c)
        else: out.append(c)
        i += 1
    return "".join(out)

def load_lock(path):
    with open(path, encoding="utf-8") as f: return json.loads(strip_trailing_commas(f.read()))

def lock_name(root):
    return "root" if root == "." else root
