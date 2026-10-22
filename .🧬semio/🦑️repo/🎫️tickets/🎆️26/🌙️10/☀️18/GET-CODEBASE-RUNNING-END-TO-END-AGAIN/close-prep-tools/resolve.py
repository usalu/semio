import re, os, sys, json, unicodedata, collections
ROOT = "/Users/ueli/Documents/semio"
S = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-2BFGEq/a55b014a-59e5-4214-ad10-dbd1b1cb936d/scratchpad"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN")
OUT = os.path.join(TICKET, "🗑️generated")
def norm(s):
    s = unicodedata.normalize("NFC", s)
    return "".join(ch for ch in s if ch not in "️‍")
repo = [p for p in open(S + "/repo-files.bin", "rb").read().decode("utf-8", "surrogateescape").split("\0") if p]
by_norm = {}
NORM = {}
by_base = collections.defaultdict(list)
for p in repo:
    n = norm(p); NORM[p] = n
    by_norm[n] = p
    by_base[n.rsplit("/", 1)[-1]].append((n, p))
files = [l.strip() for l in open(S + "/r1022.txt") if l.strip()]
EXT = r"(?:ts|tsx|js|mjs|cjs|rs|json|toml|lock|md|py|sh|yml|yaml|go|bzl|bazel|css|html|wgsl|swift|kt|cs|ps1|cmd|txt|tex|svg|gradle|xml|proto|graphql|sql|mdx|cfg|ini|dmg)"
pat = re.compile(r"(?<![\w.\-/])((?:[^\s`'\"()<>,|;=\[\]{}]+/)+[^\s`'\"()<>,|;=\[\]{}/]+\.%s)(?!\w)" % EXT)
EDIT = re.compile(r"(?i)\b(edit|edits|edited|changed|change[sd]?|live|fix|fixed|fixes|added|create[sd]?|new|patch(?:ed)?|wrote|written|rewr\w*|removed|applied|port(?:ed)?|repoint\w*|carve[sd]?|split|moved|renamed|touched|pub|DONE|stage[sd]?)\b")
HEAD_EDIT = re.compile(r"(?i)changed|edit|live|fix|files|applied|what changed|port|change|repair|done|carve|split|patch|result|edits")
hits = collections.defaultdict(lambda: {"edit": set(), "mention": set()})
for f in files:
    head = ""
    for line in open(os.path.join(TICKET, f), encoding="utf-8", errors="replace"):
        if line.startswith("#"):
            head = line
        ctx = "edit" if (EDIT.search(line) or HEAD_EDIT.search(head)) else "mention"
        for m in pat.finditer(line):
            hits[m.group(1).strip("./")][ctx].add(f)
def clean_segments(p):
    return [s for s in p.split("/") if s not in ("", "…")]
def resolve(p):
    if p.startswith("C/") or p.startswith("private/") or p.startswith("semio-source:") or "tmp/" in p:
        return ("ticket-or-scratch", None)
    if p.startswith(("cache/", "node_modules/", "lib/src/", "pkgs/", "k/", "vendor/")):
        return ("not-repo-or-external", None)
    n = norm(p.replace("…/", ""))
    if n in by_norm:
        return ("exact", by_norm[n])
    segs = clean_segments(p)
    for k in range(0, len(segs) - 1):
        suf = "/".join(segs[k:])
        sn = norm(suf)
        cands = [q for (qn, q) in by_base.get(sn.rsplit("/", 1)[-1], []) if qn.endswith("/" + sn) or qn == sn]
        if len(cands) == 1:
            return ("suffix", cands[0])
        if len(cands) > 1:
            return ("ambiguous:%d" % len(cands), None)
    return ("unresolved", None)
rows = []
for p in sorted(hits):
    h = hits[p]
    kind, target = resolve(p)
    exists = bool(target) and os.path.exists(os.path.join(ROOT, target))
    rows.append({"ref": p, "kind": kind, "target": target, "exists": exists,
                 "edit_in": sorted(h["edit"]), "mention_in": sorted(h["mention"])})
json.dump(rows, open(OUT + "/close-prep-resolved.json", "w"), ensure_ascii=False, indent=1)
c = collections.Counter(r["kind"].split(":")[0] for r in rows)
print("total refs", len(rows), dict(c))
print("edit-context resolved-exists", sum(1 for r in rows if r["target"] and r["exists"] and r["edit_in"]))
print("edit-context resolved-missing", sum(1 for r in rows if r["target"] and not r["exists"] and r["edit_in"]))
print("edit-context unresolved", sum(1 for r in rows if not r["target"] and r["edit_in"] and r["kind"] in ("unresolved",) ))
