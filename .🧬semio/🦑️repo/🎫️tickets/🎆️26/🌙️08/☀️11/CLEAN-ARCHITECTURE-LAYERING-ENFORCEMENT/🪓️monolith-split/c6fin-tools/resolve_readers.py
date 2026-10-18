"""📖 C5G relative-path reader census: every string literal ending in `.rs` (or naming one of the five split directories) in tracked/untracked text files is resolved against the file's directory and the repo root; hits are those that resolve to a split root file or its directory.
usage: resolve_readers.py"""
import os, re, subprocess, sys
R = "/Users/ueli/Documents/semio"
PL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host"
ROOTS = {
    PL + "/⚡️effects/🦀️.rs": "he",
    PL + "/🎭️guest-runtime/🦀️.rs": "hg",
    PL + "/🧭️routers/🦀️.rs": "hr",
    PL + "/🧵️shard/🦀️.rs": "hs",
    PL + "/🧪️tests/🔬️guest-cold-relay/🦀️.rs": "hc",
}
DIRS = {os.path.dirname(k): v for k, v in ROOTS.items()}
out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-co", "--exclude-standard", "-z"], cwd=R, capture_output=True).stdout.decode("utf-8", "replace").split("\0")
exts = (".ts", ".tsx", ".mjs", ".js", ".py", ".sh", ".json", ".jsonc", ".md", ".toml", ".rs")
LIT = re.compile(r'["\'`]([^"\'`\n]*\.rs)["\'`]')
SEG = re.compile(r'["\'`](⚡️effects|🎭️guest-runtime|🧭️routers|🧵️shard|🔬️guest-cold-relay)(/[^"\'`\n]*)?["\'`]')
hits = []
for f in out:
    if not f or f.startswith(".🧬semio") or f.endswith("Cargo.lock") or not f.endswith(exts):
        continue
    if "node_modules" in f or "/target/" in f or "/dist/" in f:
        continue
    p = os.path.join(R, f)
    try:
        t = open(p, encoding="utf-8").read()
    except Exception:
        continue
    d = os.path.dirname(f)
    for i, line in enumerate(t.split("\n"), 1):
        for m in LIT.finditer(line):
            lit = m.group(1)
            for base in (d, ""):
                n = os.path.normpath(os.path.join(base, lit))
                if n in ROOTS:
                    hits.append((f, i, "file", ROOTS[n], lit))
                    break
        for m in SEG.finditer(line):
            hits.append((f, i, "segment", m.group(1), m.group(0)))
for h in hits:
    print("%s:%d [%s %s] %s" % h)
print("hits", len(hits))
