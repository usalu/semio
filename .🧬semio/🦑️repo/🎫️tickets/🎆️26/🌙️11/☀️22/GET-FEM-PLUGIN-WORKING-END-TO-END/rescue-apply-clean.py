import subprocess, sys, os, tempfile

OLD = r"C:\git\semio"
NEW = r"C:\semio"
OUTDIR = sys.argv[3]
apply = sys.argv[2] == "apply"


def git(repo, *args):
    return subprocess.run(["git", "-c", "core.quotepath=false", "-C", repo, *args], capture_output=True)


for line in open(sys.argv[1], encoding="utf-8").read().split("\n"):
    if not line.startswith("clean-merge-differs"):
        continue
    p = line.split("\t")[1]
    base = git(OLD, "show", "HEAD:" + p).stdout
    old = open(os.path.join(OLD, p), "rb").read()
    fn = os.path.join(NEW, p)
    new = open(fn, "rb").read()
    d = tempfile.mkdtemp(dir=OUTDIR)
    fb, fo, fnn = (os.path.join(d, x) for x in ("base", "old", "new"))
    open(fb, "wb").write(base)
    open(fo, "wb").write(old)
    open(fnn, "wb").write(new)
    r = subprocess.run(["git", "merge-file", "-p", fnn, fb, fo], capture_output=True)
    if r.returncode != 0:
        print("CONFLICT", p)
        continue
    stat = subprocess.run(["git", "diff", "--no-index", "--numstat", fnn, os.path.join(d, "merged")], capture_output=True) if False else None
    mf = os.path.join(d, "merged")
    open(mf, "wb").write(r.stdout)
    ns = subprocess.run(["git", "diff", "--no-index", "--numstat", fnn, mf], capture_output=True).stdout.decode().split()[:2]
    print("apply" if apply else "dry", ns, p.encode("ascii", "replace").decode()[-70:])
    if apply:
        if open(fn, "rb").read() != new:
            print("CHANGED-UNDER-US", p)
            continue
        open(fn, "wb").write(r.stdout)
