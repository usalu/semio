"""🪪️ Add `actorIdentityGrant: WORKER_LAW_ACTOR_IDENTITY_GRANT` to each actor-config literal that the diagnostics name."""
import os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from tt3_blocks import blocks

base, snap = sys.argv[1], sys.argv[2]
apply = len(sys.argv) > 3 and sys.argv[3] == "apply"
NAME = "WORKER_LAW_ACTOR_IDENTITY_GRANT"

def skip_string(s, i):
    q = s[i]; i += 1
    while s[i] != q:
        if s[i] == "\\": i += 1
        i += 1
    return i + 1

def match_brace(s, i):
    assert s[i] == "{"
    depth = 0
    while True:
        c = s[i]
        if c in "\"'`": i = skip_string(s, i); continue
        if c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return i
        i += 1

files = {}
for b in blocks(base):
    if "actorIdentityGrant" in b["text"] and b["code"] in ("TS2345", "TS2741", "TS2322"):
        files.setdefault(b["file"], []).append((b["line"], b["col"]))

for path, sites in sorted(files.items()):
    text = open(path, encoding="utf8").read()
    starts = [0]
    for m in re.finditer("\n", text): starts.append(m.end())
    inserts = []
    for line, col in sites:
        offset = starts[line - 1] + col - 1
        brace = text.find("{", offset) if text[offset] != "{" else offset
        end = match_brace(text, brace)
        literal = text[brace:end + 1]
        if "documentId" not in literal[:600] or "schema" not in literal[:900] or "actorIdentityGrant" in literal:
            print("SKIP", path[-60:], line, col, literal[:80].replace("\n", " ")); continue
        inserts.append((brace, end))
    inserts = sorted(set(inserts), reverse=True)
    print(len(inserts), "of", len(sites), path[-70:])
    if not apply or not inserts: continue
    os.makedirs(snap, exist_ok=True)
    shutil.copy(path, os.path.join(snap, re.sub(r"[^A-Za-z0-9]+", "_", path)[-120:]))
    for brace, end in inserts:
        body = text[brace + 1:end]
        multiline = "\n" in body
        stripped = body.rstrip()
        if multiline:
            lastline = stripped.split("\n")[-1]
            indent = re.match(r"\s*", lastline).group(0)
            comma = "" if stripped.endswith(",") else ","
            insert_at = brace + 1 + len(stripped)
            text = text[:insert_at] + comma + "\n" + indent + f"actorIdentityGrant: {NAME}," + text[insert_at:]
        else:
            comma = "" if stripped.endswith(",") or stripped.strip() == "" else ","
            insert_at = brace + 1 + len(stripped)
            text = text[:insert_at] + comma + f" actorIdentityGrant: {NAME}" + text[insert_at:]
    imp = re.search(r'^import \{([^}]*)\} from "([^"]*worker-law-fakes[^"]*)";?$', text, re.M)
    if imp and NAME not in imp.group(1):
        names = imp.group(1).rstrip()
        sep = "" if names.endswith(",") else ","
        text = text[:imp.start()] + f'import {{{names}{sep} {NAME} }} from "{imp.group(2)}";' + text[imp.end():]
    elif not imp:
        print("NOIMPORT", path)
    open(path, "w", encoding="utf8").write(text)
