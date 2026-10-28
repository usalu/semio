"""🧭 [isov] mirror-only bisect helper: `partial.py revert|apply <stage-dir> <root>` flips only the rows of a staged batch that are exactly `new` (revert) or exactly `base` (apply) and skips drifted rows, so a landed batch can be switched on and off on a mirror."""
import hashlib, json, os, shutil, sys
verb, stage, root = sys.argv[1], os.path.abspath(sys.argv[2]), os.path.abspath(sys.argv[3])
assert root.startswith("/private/tmp/m/"), "mirror only"
rows = json.load(open(os.path.join(stage, "manifest.json"), encoding="utf8"))
def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None
done = skipped = 0
for rel, row in rows.items():
    target = os.path.join(root, rel); live = digest(target)
    src, want, put = (("base", "new", "base") if verb == "revert" else ("files", "base", "new"))
    if live != row[want]: skipped += 1; continue
    source = os.path.join(stage, "base" if verb == "revert" else "files", row["file"])
    if not os.path.exists(source): skipped += 1; continue
    tmp = target + ".partial"; shutil.copyfile(source, tmp); os.chmod(tmp, os.stat(target).st_mode & 0o777); os.replace(tmp, target); done += 1
print(verb, "flipped", done, "skipped", skipped)
