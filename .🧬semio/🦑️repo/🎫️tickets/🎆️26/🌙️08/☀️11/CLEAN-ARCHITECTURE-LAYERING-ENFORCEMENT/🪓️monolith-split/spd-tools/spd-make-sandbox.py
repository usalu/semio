"""📦️ Copies the schema leaves of `✏️s`, `🌎️hub` and the schema documents, entry files and manifests of `🧰️framework` into a sandbox outside the repository: `python3 spd-make-sandbox.py <dest>`. Writes only below <dest>."""
import os, shutil, subprocess, sys
root = "/Users/ueli/Documents/semio"
dest = os.path.realpath(sys.argv[1])
assert dest.startswith("/private/tmp/") or dest.startswith("/tmp/") or "scratchpad" in dest, dest
LEAVES = {"🔣️.json", "🟦️.ts", "🔗️.graphql", "🛰️.proto", "🦀️.rs"}
SKIP = ("/🧪️tests/", "/node_modules/", "/🧫️fixtures/", "/🧪️testing/", "/📦️packages/", "/🧬️mutations/", "/🧾️descriptor/")
out = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "✏️s", "🌎️hub", "🧰️framework"], cwd=root, capture_output=True).stdout.decode().split("\0")
copied = 0
for rel in sorted(set(out)):
    if not rel:
        continue
    name = rel.split("/")[-1]
    framework = rel.startswith("🧰️framework/")
    if framework:
        keep = (name in ("🔣️.json", "🟦️.ts", "package.json")) and not any(s in "/" + rel for s in ("/🧪️tests/", "/node_modules/", "/🧫️fixtures/", "/🧪️testing/"))
        facade = "/📦️packages/" in "/" + rel and name.endswith(".ts")
        keep = keep or facade
    else:
        keep = name in LEAVES and "/🧬️schema/" in "/" + rel and not any(s in "/" + rel for s in SKIP)
    if not keep:
        continue
    src = os.path.join(root, rel)
    if not os.path.isfile(src):
        continue
    dst = os.path.join(dest, rel)
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copyfile(src, dst)
    copied += 1
print("copied", copied)
