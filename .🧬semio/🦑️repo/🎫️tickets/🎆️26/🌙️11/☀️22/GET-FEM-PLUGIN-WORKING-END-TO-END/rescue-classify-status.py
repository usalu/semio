import subprocess, sys, os, collections

OLD = r"C:\git\semio"
NEW = r"C:\semio"


def git(repo, *args, inp=None):
    return subprocess.run(["git", "-c", "core.quotepath=false", "-C", repo, *args], input=inp, capture_output=True).stdout


def hashes(root, paths):
    ex = [p for p in paths if os.path.isfile(os.path.join(root, p))]
    out = git(NEW, "hash-object", "--no-filters", "--stdin-paths", inp="\n".join(os.path.join(root, p) for p in ex).encode("utf-8")).decode().split()
    return dict(zip(ex, out))


def main(statusfile, outfile):
    entries = []
    for line in open(statusfile, encoding="utf-8").read().split("\n"):
        if len(line) > 3:
            entries.append((line[:2], line[3:]))
    paths = [p for _, p in entries]
    oh = hashes(OLD, paths)
    nh = hashes(NEW, paths)
    head = {}
    ls = git(OLD, "ls-tree", "-r", "-z", "HEAD").split(b"\0")
    want = set(paths)
    for e in ls:
        if not e:
            continue
        meta, p = e.split(b"\t", 1)
        p = p.decode("utf-8")
        if p in want:
            head[p] = meta.split()[2].decode()
    cls = collections.Counter()
    rows = []
    for st, p in entries:
        o, n, h = oh.get(p), nh.get(p), head.get(p)
        if o is None:
            c = "x-old-missing(deleted?)"
        elif n is None:
            c = "d-only-in-old"
        elif o == n:
            c = "a-identical"
        elif st.strip() == "M" and n == h:
            c = "b-new-unchanged-since-head(change missing)"
        else:
            c = "c-diverged"
        cls[c] += 1
        rows.append(f"{c}\t{st}\t{p}")
    open(outfile, "w", encoding="utf-8").write("\n".join(sorted(rows)) + "\n")
    print(dict(cls))


main(sys.argv[1], sys.argv[2])
