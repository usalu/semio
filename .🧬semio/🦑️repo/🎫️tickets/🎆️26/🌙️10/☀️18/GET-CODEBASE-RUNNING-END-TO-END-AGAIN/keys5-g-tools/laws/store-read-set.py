# keys5-g: independent Python reading of what `readStoreSourceText` (🧰️framework/🔨️modules/🧪️test/📖️source-text) reads: the store facade root file plus the module files of the carved crates and every lifted directory.
# usage: python3 -I store-read-set.py <repo root>   prints one repository-relative path per line (sorted by UTF-8 bytes)
import os, sys
ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
FILE = "🦀️.rs"
CARVED = ["⚙️engine", "🏗️initialization", "🧰️test-support", "🧩️composition", "🧾️document", "🎚️config", "📜️space-history/🏠️space", "📦️codec/🏭️factory"]
EXCLUDED = {"🧪️tests", "🧪️testing", "📦️packages", "node_modules"}
repo = sys.argv[1]
base = os.path.join(repo, ROOT)
out = [os.path.join(ROOT, FILE)]
for directory, names, files in os.walk(base):
    names[:] = [n for n in names if n not in EXCLUDED]
    if FILE not in files: continue
    path = os.path.join(directory, FILE)
    rel = os.path.relpath(path, repo)
    if rel == out[0]: continue
    inside = os.path.relpath(path, base).split(os.sep)
    carved = any(os.path.relpath(path, base).startswith(c + os.sep) for c in CARVED)
    lifted = "📥️lifted" in inside[:-1]
    if carved or lifted: out.append(rel)
out = sorted(set(out), key=lambda p: p.encode("utf-8"))
sys.stdout.write("\n".join(out) + "\n")
