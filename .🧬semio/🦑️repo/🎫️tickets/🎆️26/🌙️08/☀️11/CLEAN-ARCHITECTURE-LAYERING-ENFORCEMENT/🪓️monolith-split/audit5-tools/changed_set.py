"""📋️ Build the changed set: git status (uncommitted and untracked) plus paths of the last-day commits, existing files only."""
import os, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
GEN = sys.argv[1]
status = open(os.path.join(GEN, "audit5r-status-nofsm.txt"), encoding="utf-8").read().splitlines()
st_paths, st_untracked, st_deleted, st_modified = set(), set(), set(), set()
for line in status:
    code, path = line[:2], line[3:]
    if " -> " in path:
        path = path.split(" -> ")[-1]
    if code == "??":
        st_untracked.add(path)
    elif "D" in code:
        st_deleted.add(path)
    else:
        st_modified.add(path)
    st_paths.add(path)
commits = []
for c in ["36fbe9da786", "1bae6115bc5", "f856dff0813"]:
    out = subprocess.run(["git", "--no-optional-locks", "-c", "core.quotepath=false", "show", "--format=", "--name-status", c], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout
    for row in out.splitlines():
        parts = row.split("\t")
        if len(parts) >= 2:
            commits.append((c, parts[-1]))
commit_paths = set(p for _, p in commits)
union = st_paths | commit_paths
exist = sorted(p for p in union if os.path.isfile(os.path.join(ROOT, p)))
missing = sorted(p for p in union if not os.path.isfile(os.path.join(ROOT, p)))
open(os.path.join(GEN, "audit5r-changed-set.txt"), "w", encoding="utf-8").write("\n".join(exist) + "\n")
open(os.path.join(GEN, "audit5r-changed-missing.txt"), "w", encoding="utf-8").write("\n".join(missing) + "\n")
open(os.path.join(GEN, "audit5r-untracked.txt"), "w", encoding="utf-8").write("\n".join(sorted(st_untracked)) + "\n")
open(os.path.join(GEN, "audit5r-status-modified.txt"), "w", encoding="utf-8").write("\n".join(sorted(st_modified)) + "\n")
print("status", len(st_paths), "untracked", len(st_untracked), "modified", len(st_modified), "deleted", len(st_deleted))
print("commit rows", len(commits), "unique commit paths", len(commit_paths), "union", len(union), "existing", len(exist), "missing", len(missing))
rs = [p for p in exist if p.endswith(".rs")]
cargo = [p for p in exist if p.endswith("Cargo.toml")]
print("existing .rs", len(rs), "existing Cargo.toml", len(cargo))
