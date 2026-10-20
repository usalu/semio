import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
TARGET = ROOT + "/🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts"
files = [l.strip() for l in open(C + "/tst4-tools/envelope-gated.txt", encoding="utf-8") if l.strip()]
for extra in sys.argv[1:]:
    files += [l.strip() for l in open(extra, encoding="utf-8") if l.strip()]
seen = set()
queue = [os.path.join(ROOT, f) for f in files]
cands = []
imp = re.compile(r'''(?:from|import)\s*["'](\.{1,2}/[^"']+)["']''')
while queue:
    p = queue.pop()
    if p in seen or not os.path.isfile(p):
        continue
    seen.add(p)
    t = open(p, encoding="utf-8").read()
    if "receiveScriptProcessInvocation(" in t:
        cands.append(p)
    for m in imp.finditer(t):
        q = os.path.normpath(os.path.join(os.path.dirname(p), m.group(1)))
        if ("/🧪️tests/" in q or "/🧪️testing/" in q) and q.endswith((".ts", ".mjs")) and "/🔨️modules/🏃️process/" not in q:
            queue.append(q)
call = re.compile(r"(?<![\w.])(?:\w+\.)?receiveScriptProcessInvocation\(\s*process\.env\s*,\s*")
impline = re.compile(r'import\s*(type\s*)?\{([^}]*)\}\s*from\s*"([^"]*routing/🟦️\.ts)"\s*;?')
out = []
other = []
for p in sorted(cands):
    rel = os.path.relpath(p, ROOT)
    t = open(p, encoding="utf-8").read()
    calls = call.findall(t)
    total = len(re.findall(r"receiveScriptProcessInvocation\(", t))
    if total != len(calls):
        other.append((rel, total, len(calls)))
        continue
    edits = []
    for old in sorted(set(calls)):
        edits.append({"old": old, "new": "withBuildTestInvocationV1(", "count": calls.count(old)})
    m = impline.search(t)
    names = [n.strip() for n in m.group(2).split(",") if n.strip()] if m and not m.group(1) else []
    if m and not m.group(1) and "receiveScriptProcessInvocation" not in names:
        m = None
    names2 = [n for n in names if n != "receiveScriptProcessInvocation"]
    if not m:
        first = t.split("\n", 1)[0]
        relpath = os.path.relpath(TARGET, os.path.dirname(p)).replace(os.sep, "/")
        if "withBuildTestInvocationV1" not in t:
            edits.append({"old": first, "new": first + '\nimport {withBuildTestInvocationV1} from "%s";' % relpath, "count": 1})
        out.append({"file": rel, "edits": edits})
        continue
    relpath = os.path.relpath(TARGET, os.path.dirname(p)).replace(os.sep, "/")
    new_import = 'import {withBuildTestInvocationV1} from "%s";' % relpath
    if "withBuildTestInvocationV1" in t:
        new_import = ""
    keep = ('import {%s} from "%s";' % (",".join(names2), m.group(3))) if names2 else ""
    sep = "\n" if (keep and new_import) else ""
    edits.append({"old": m.group(0), "new": keep + sep + new_import, "count": 1})
    out.append({"file": rel, "edits": edits})
json.dump(out, open(C + "/env-pass2-tools/staged/patches/R.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(out), "files patched;", "skipped:", other)
for o in out: print(o["file"][-110:], sum(e["count"] for e in o["edits"][:-1]))
