import os, shutil, subprocess, sys
root = "/Users/ueli/Documents/semio"
dest = sys.argv[1]
scopes = sys.argv[2].split(",") if len(sys.argv) > 2 else ["✏️s"]
LEAVES = {"🔣️.json", "🟦️.ts", "🔗️.graphql", "🛰️.proto", "🦀️.rs"}
FACETS = {"🔺️diff", "📸️snapshot", "💡️inferences", "🧬️mutations", "📝️text", "💾️binary"}
out = subprocess.run(["git", "-c", "core.fsmonitor=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", *scopes], cwd=root, capture_output=True).stdout.decode().split("\0")
copied = 0
for rel in sorted(set(out)):
    if not rel:
        continue
    parts = rel.split("/")
    if parts[-1] not in LEAVES or "🧬️schema" not in parts:
        continue
    at = len(parts) - 1 - parts[::-1].index("🧬️schema")
    if not all(part in FACETS for part in parts[at + 1 : -1]):
        continue
    src = os.path.join(root, rel)
    if not os.path.isfile(src):
        continue
    dst = os.path.join(dest, rel)
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copyfile(src, dst)
    copied += 1
print("copied", copied)
