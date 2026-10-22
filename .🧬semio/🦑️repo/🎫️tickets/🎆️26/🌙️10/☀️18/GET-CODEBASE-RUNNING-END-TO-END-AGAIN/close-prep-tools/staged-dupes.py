import subprocess, os, collections
ROOT = "/Users/ueli/Documents/semio"
TICKET = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN")
entries = [l.strip() for l in open(os.path.join(TICKET, "🗑️generated/close-prep-staged-entries.txt")) if l.strip()]
files = []
for e in entries:
    for dp, dn, fn in os.walk(os.path.join(TICKET, e)):
        for f in fn:
            files.append(os.path.join(dp, f))
inp = "\n".join(files) + "\n"
out = subprocess.run(["git", "hash-object", "--no-filters", "--stdin-paths"], input=inp, capture_output=True, text=True, cwd=ROOT).stdout.split()
staged_hash = dict(zip(files, out))
tracked = subprocess.run(["git", "ls-files", "-s", "-z"], capture_output=True, cwd=ROOT).stdout.decode("utf-8", "surrogateescape").split("\0")
blobs = set()
for t in tracked:
    if t:
        blobs.add(t.split()[1])
dup = [f for f, h in staged_hash.items() if h in blobs]
print("staged files", len(files), "byte-identical to a tracked blob", len(dup))
