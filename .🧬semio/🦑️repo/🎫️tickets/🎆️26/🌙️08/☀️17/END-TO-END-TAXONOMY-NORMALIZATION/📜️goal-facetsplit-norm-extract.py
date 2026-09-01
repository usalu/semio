import subprocess, sys, re, os

ROOT = "/Users/ueli/Documents/semio"
PIN = "bb06c41f73f0122fbed315b7487428b976f99921"
DRY = "--dry" in sys.argv
LIMIT = None
for a in sys.argv:
    if a.startswith("--limit="):
        LIMIT = int(a.split("=",1)[1])

worklist_path = os.path.join(os.path.dirname(__file__), "worklist_sorted.txt")
with open(worklist_path, encoding="utf-8") as f:
    leaves = [l.rstrip("\n") for l in f if l.strip()]
if LIMIT:
    leaves = leaves[:LIMIT]

def git_show(path):
    r = subprocess.run(["git", "show", f"{PIN}:{path}"], cwd=ROOT, capture_output=True)
    if r.returncode != 0:
        return None
    return r.stdout.decode("utf-8")

diff_sig_re = re.compile(r'(    fn diff\(&self, base: [^\{]*\{\n)([ \t]*)[^\n]*\n(    \})')
inv_sig_re = re.compile(r'(    fn inverse\(&self, base: [^\{]*\{\n)([ \t]*)[^\n]*\n(    \})')

report = []
skipped = []

for leaf in leaves:
    mdir = os.path.dirname(leaf)  # relative, no trailing slash
    pinned_diff_path = f"{mdir}/🔺️diff/🦀️component.rs"
    pinned_inv_path = f"{mdir}/↩️inverse/🦀️component.rs"

    diff_content = git_show(pinned_diff_path)
    inv_content = git_show(pinned_inv_path)

    used_fallback = []
    full_leaf_path = os.path.join(ROOT, leaf)
    with open(full_leaf_path, encoding="utf-8") as f:
        current = f.read()

    if diff_content is None or inv_content is None:
        used_fallback.append(leaf)
        skipped.append(leaf)
        continue

    diff_content = diff_content.replace("::mutation::", "::")
    inv_content = inv_content.replace("::mutation::", "::")

    new_diff_path = os.path.join(ROOT, mdir, "🔺️diff", "🦀️.rs")
    new_inv_path = os.path.join(ROOT, mdir, "↩️inverse", "🦀️.rs")

    # locate start of Diff region in current leaf, truncate from there
    idx = current.find("//#region 🔖️Diff")
    if idx == -1:
        skipped.append(leaf + " (NO_DIFF_REGION_MARKER_AT_EDIT_TIME)")
        continue
    head = current[:idx].rstrip("\n") + "\n"

    m1 = diff_sig_re.search(head)
    if not m1:
        skipped.append(leaf + " (DIFF_SIG_NOT_MATCHED)")
        continue
    head = diff_sig_re.sub(lambda m: f"{m.group(1)}{m.group(2)}super::diff::diff(self, base)\n{m.group(3)}", head, count=1)

    m2 = inv_sig_re.search(head)
    if not m2:
        skipped.append(leaf + " (INVERSE_SIG_NOT_MATCHED)")
        continue
    head = inv_sig_re.sub(lambda m: f"{m.group(1)}{m.group(2)}super::inverse::inverse(self, base)\n{m.group(3)}", head, count=1)

    report.append(leaf)

    if not DRY:
        os.makedirs(os.path.dirname(new_diff_path), exist_ok=True)
        os.makedirs(os.path.dirname(new_inv_path), exist_ok=True)
        with open(new_diff_path, "w", encoding="utf-8") as f:
            f.write(diff_content)
        with open(new_inv_path, "w", encoding="utf-8") as f:
            f.write(inv_content)
        with open(full_leaf_path, "w", encoding="utf-8") as f:
            f.write(head)

print(f"processed={len(report)} skipped={len(skipped)} dry={DRY}")
for s in skipped:
    print("SKIP:", s)
