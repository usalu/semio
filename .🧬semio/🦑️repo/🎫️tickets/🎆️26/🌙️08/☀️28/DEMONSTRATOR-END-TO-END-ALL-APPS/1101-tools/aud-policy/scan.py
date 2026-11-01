import re, json, os, sys
T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
REPO = "/Users/ueli/Documents/semio"
DIFF = os.path.join(T, "🗑️generated/1101/aud-policy-diff.txt")
CODE = re.compile(r"\.(rs|ts|tsx|mts|mjs|cjs|js|jsx|py|sh|go|wit)$")
TESTPATH = re.compile(r"(/tests?/|🧪️tests?/|\.test\.|\.spec\.|_test\.|/test_|tests?\.rs$|🧪️)")
EXCL = re.compile(r"(node_modules|/dist/|/target/|\.cache|🗑️generated|/\.🧬semio/)")
rules = {
 "debug-print": re.compile(r"\[DEBUG\]|\bdbg!\(|\bdebugger;"),
 "unwrap-expect-panic": re.compile(r"\.unwrap\(\)|\.expect\(|\bpanic!\(|\btodo!\(|\bunimplemented!\("),
 "allow-attr": re.compile(r"#!?\[allow\("),
 "comment-in-body": re.compile(r"^\s+//(?!/|!)"),
 "shim-adapter-compat": re.compile(r"(?i)\bshim|adapter|\bcompat(?!ible|ibility)|legacy|deprecat|backwards?[- ]compat|migrat"),
 "default-language": re.compile(r"(unwrap_or\(\s*\"(en|de)\"|\?\?\s*[\"'](en|de)[\"']|\|\|\s*[\"'](en|de)[\"']|(?i:default|fallback)\w*\s*[:=]\s*[\"'](en|de)[\"']|(?i:lang|locale)\w*\s*[:=]\s*[\"'](en|de)[\"']|DEFAULT_(LANG|LOCALE)|defaultLocale|defaultLanguage|default_language|default_locale)"),
}
hits = []
newfiles = []
cur = None; newno = 0; infile = False
with open(DIFF, encoding="utf-8", errors="replace") as f:
    for raw in f:
        line = raw.rstrip("\n")
        if line.startswith("diff --git "):
            cur = None; infile = True
            m = re.match(r"diff --git a/(.*) b/(.*)$", line)
            cur = m.group(2) if m else None
            continue
        if line.startswith("new file mode"):
            if cur: newfiles.append(cur)
            continue
        if line.startswith("+++ "):
            p = line[4:]
            cur = p[2:] if p.startswith("b/") else None
            continue
        if line.startswith("--- ") or line.startswith("index ") or line.startswith("similarity") or line.startswith("rename ") or line.startswith("deleted file"):
            continue
        m = re.match(r"@@ -\d+(?:,\d+)? \+(\d+)(?:,\d+)? @@", line)
        if m:
            newno = int(m.group(1)); continue
        if line.startswith("+") and cur:
            text = line[1:]
            if CODE.search(cur) and not EXCL.search(cur):
                hits.append({"file": cur, "line": newno, "text": text})
            newno += 1
        elif line.startswith(" ") or line == "":
            newno += 1

# test-module boundary per file (current working tree)
boundary_cache = {}
def test_boundary(path):
    if path in boundary_cache: return boundary_cache[path]
    full = os.path.join(REPO, path)
    b = None
    try:
        with open(full, encoding="utf-8", errors="replace") as fh:
            lines = fh.readlines()
        for i, l in enumerate(lines):
            if "#[cfg(test)]" in l:
                b = i + 1; break
    except Exception:
        pass
    boundary_cache[path] = b
    return b

results = {k: [] for k in rules}
for h in hits:
    t = h["text"]; f = h["file"]
    for k, rx in rules.items():
        if not rx.search(t): continue
        if k == "comment-in-body" and not f.endswith((".rs", ".ts", ".tsx", ".mts", ".js", ".mjs", ".cjs")): continue
        if k == "comment-in-body" and re.search(r"^\s+//\s*(https?://|@ts-|eslint|SAFETY)", t): continue
        if k == "unwrap-expect-panic":
            if not f.endswith(".rs"): continue
            if TESTPATH.search(f): continue
            b = test_boundary(f)
            if b is not None and h["line"] >= b: continue
        if k == "debug-print" and f.endswith(".py"):
            pass
        results[k].append(h)

# comment-in-body must be inside a definition: approximate by indent > 0 lines only (already)
newfile_hits = [f for f in newfiles if CODE.search(f) and not EXCL.search(f)]
mig = re.compile(r"(?i)(migrat|codemod|backfill|fixup|upgrade|rewrite)")
migr = [f for f in newfile_hits if mig.search(os.path.basename(f))]
shimfiles = [f for f in newfile_hits if re.search(r"(?i)shim|adapter|compat|legacy", os.path.basename(f))]
out = {"counts": {k: len(v) for k, v in results.items()}, "changed_code_files": len({h["file"] for h in hits}),
       "new_files": len(newfile_hits), "migration_named_new_files": migr, "shim_named_new_files": shimfiles,
       "hits": {k: v for k, v in results.items()}}
with open(os.path.join(T, "🗑️generated/1101/aud-policy-findings.json"), "w", encoding="utf-8") as fh:
    json.dump(out, fh, ensure_ascii=False, indent=1)
print(json.dumps(out["counts"], ensure_ascii=False), "files", out["changed_code_files"], "new", out["new_files"])
print("migration-named new files:", migr)
print("shim-named new files:", shimfiles)
