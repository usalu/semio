"""🔁️ Re-points rust consumers from the facade path semio_framework_2d::compute to the owner crate semio_framework_2d_compute. Journal records sha before/after per file."""
import hashlib, json, os, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
OLD, NEW = "semio_framework_2d::compute", "semio_framework_2d_compute"
ROOTS = ["🧰️framework", "✏️s", "🌎️hub", "🏢️semio-tech", "🎓️teaching", "👴️leutwiler", "🧪️tests", "🧬️schema", "♻️mit-bestand"]
SKIP = ("📍️ownership/🧪️tests/",)
def git(*a):
    return subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", *a], cwd=ROOT, capture_output=True, text=True).stdout
tracked = git("grep", "-l", "-F", OLD, "--", *ROOTS).split("\n")
untracked = [p for p in git("ls-files", "--others", "--exclude-standard", "--", *ROOTS).split("\n") if p.endswith(".rs")]
cands = sorted({p for p in tracked + untracked if p.endswith(".rs") and not any(s in p for s in SKIP)})
apply = "--apply" in sys.argv
journal, total = {}, 0
for p in cands:
    full = os.path.join(ROOT, p)
    if not os.path.isfile(full):
        continue
    raw = open(full, "rb").read()
    text = raw.decode("utf8")
    n = text.count(OLD)
    if not n:
        continue
    new = text.replace(OLD + "::", NEW + "::")
    left = new.count(OLD)
    if left:
        new = new.replace(OLD, NEW)
        print("BARE", p, left)
    total += n
    journal[p] = [hashlib.sha256(raw).hexdigest()[:16], hashlib.sha256(new.encode()).hexdigest()[:16], n]
    if apply:
        if open(full, "rb").read() != raw:
            print("CHANGED-UNDER-US", p); continue
        open(full, "wb").write(new.encode("utf8"))
print(len(journal), "files", total, "sites", "APPLIED" if apply else "dry-run")
if apply:
    json.dump(journal, open(F + "/2dc-tools/journal-rs.json", "w"), ensure_ascii=False, indent=0)
else:
    json.dump(sorted(journal), open(F + "/2dc-tools/candidates-rs.json", "w"), ensure_ascii=False, indent=0)
