import subprocess, sys, os, tempfile, hashlib

OLD = r"C:\git\semio"
NEW = r"C:\semio"
OUT = sys.argv[3]


def git(repo, *args):
    return subprocess.run(["git", "-c", "core.quotepath=false", "-C", repo, *args], capture_output=True)


def main(classes, mode):
    os.makedirs(OUT, exist_ok=True)
    rows = []
    for line in open(classes, encoding="utf-8").read().split("\n"):
        if not line:
            continue
        c, st, p = line.split("\t")
        if c.startswith("c-") and st.strip() == "M":
            rows.append(p)
    res = []
    for i, p in enumerate(rows):
        base = git(OLD, "show", "HEAD:" + p).stdout
        old = open(os.path.join(OLD, p), "rb").read()
        new = open(os.path.join(NEW, p), "rb").read()
        d = tempfile.mkdtemp(dir=OUT)
        fb, fo, fn = (os.path.join(d, x) for x in ("base", "old", "new"))
        open(fb, "wb").write(base)
        open(fo, "wb").write(old)
        open(fn, "wb").write(new)
        r = subprocess.run(["git", "merge-file", "-p", "--diff3", fn, fb, fo], capture_output=True)
        merged = r.stdout
        conflicts = r.returncode
        if conflicts == 0:
            verdict = "already-contained" if merged == new else "clean-merge-differs"
        else:
            verdict = f"conflicts={conflicts}"
        if old == base:
            verdict = "old==base"
        res.append((verdict, p, d))
        if verdict == "clean-merge-differs":
            open(os.path.join(OUT, f"merged-{i:03d}.out"), "wb").write(merged)
            res[-1] = (verdict, p, os.path.join(OUT, f"merged-{i:03d}.out"))
    with open(os.path.join(OUT, "verdicts.tsv"), "w", encoding="utf-8") as f:
        for v, p, d in sorted(res):
            f.write(f"{v}\t{p}\t{d}\n")
    import collections
    print(collections.Counter(v.split("=")[0] for v, _, _ in res))


main(sys.argv[1], sys.argv[2])
