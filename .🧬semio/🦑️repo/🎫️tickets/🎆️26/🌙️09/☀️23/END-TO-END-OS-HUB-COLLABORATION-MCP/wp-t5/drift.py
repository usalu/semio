"""Replays the contract's binary-protocol-drift rule on every mutation vocabulary: kinds on disk vs `record <kind> tag=<n>`."""
import os, re, subprocess
root = "/Users/ueli/Documents/semio/"
KEBAB = re.compile(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+$")
protos = subprocess.run(["find", "✏️s", "🧰️framework", "-path", "*🧬️mutations/💾️binary/📡️.protocol.semio", "-not", "-path", "*/target*", "-not", "-path", "*/dist/*"], cwd=root, capture_output=True, text=True).stdout.split("\n")
bad = 0
for p in filter(None, protos):
    vocab = os.path.dirname(os.path.dirname(root + p))
    kinds = {KEBAB.search(d).group(0) for d in os.listdir(vocab) if os.path.isdir(f"{vocab}/{d}") and not os.path.islink(f"{vocab}/{d}") and KEBAB.search(d)}
    if not kinds: continue
    declared = set(re.findall(r"(?m)^record\s+([a-z][a-z0-9-]*)\s+tag=\d+", open(root + p, encoding="utf-8").read()))
    if kinds != declared:
        bad += 1
        print(len(kinds - declared), len(declared - kinds), p.split("🗿️artifacts/")[-1].replace("/🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio", "")[:90])
print("drifting:", bad)
