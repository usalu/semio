"""Lists every include_bytes!/include_str! site of the given oracle artifacts' test units with the artifact-relative target, its existence and sha256. usage: sites.py <out.tsv> <artifact dir>..."""
import os, re, subprocess, sys, hashlib
out = sys.argv[1]
pat = re.compile(r'include_(bytes|str)!\(\s*(concat!\()?"([^"]*)"')
rows = []
for art in sys.argv[2:]:
    files = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "--cached", "--others", "--exclude-standard", "--", art], capture_output=True, text=True, cwd="/Users/ueli/Documents/semio").stdout.splitlines()
    for f in files:
        if not f.endswith(".rs") or "🔮️oracles" not in f: continue
        path = os.path.join("/Users/ueli/Documents/semio", f)
        text = open(path, encoding="utf-8").read()
        for m in pat.finditer(text):
            line = text.count("\n", 0, m.start()) + 1
            rel = m.group(3)
            if m.group(2): rows.append((f, line, m.group(1), "CONCAT", rel, "", "")); continue
            tgt = os.path.normpath(os.path.join(os.path.dirname(path), rel))
            ok = os.path.isfile(tgt)
            sha = hashlib.sha256(open(tgt, "rb").read()).hexdigest() if ok else ""
            rows.append((f, line, m.group(1), os.path.relpath(tgt, os.path.join("/Users/ueli/Documents/semio", art)) if ok else "MISSING", rel, sha, os.path.getsize(tgt) if ok else 0))
open(out, "w").write("\n".join("\t".join(str(c) for c in r) for r in rows) + "\n")
print(len(rows), "sites")
