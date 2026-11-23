import subprocess, sys, os, collections

OLD = r"C:\git\semio"
NEW = r"C:\semio"


def git(repo, *args, inp=None):
    r = subprocess.run(["git", "-c", "core.quotepath=false", "-C", repo, *args], input=inp, capture_output=True)
    return r.stdout


def parse_raw(out):
    parts = out.split(b"\0")
    i = 0
    res = []
    while i < len(parts) and parts[i]:
        meta = parts[i].decode().split()
        st = meta[4][0]
        if st in "RC":
            old = parts[i + 1]
            new = parts[i + 2]
            i += 3
            res.append(("D", meta[2], b"0" * 40, old))
            res.append(("A", meta[2], meta[3], new))
        else:
            res.append((st, meta[2], meta[3], parts[i + 1]))
            i += 2
    return res


def hash_files(paths):
    # raw hash of worktree files in NEW
    inp = b"\n".join(os.path.join(NEW, p.decode()).encode() for p in paths)
    out = git(NEW, "hash-object", "--no-filters", "--stdin-paths", inp=inp)
    return out.decode().split()


def main(base, commit, outfile):
    raw = git(OLD, "diff-tree", "-r", "--raw", "-z", "--no-renames", base, commit)
    changes = parse_raw(raw)
    present = []
    for st, oldb, newb, p in changes:
        if st != "D" and os.path.isfile(os.path.join(NEW, p.decode())):
            present.append(p)
    hashes = dict(zip(present, hash_files(present))) if present else {}
    cls = collections.Counter()
    lines = []
    for st, oldb, newb, p in changes:
        full = os.path.join(NEW, p.decode())
        exists = os.path.isfile(full)
        if st == "D":
            c = "a-deleted-already" if not exists else ("b-still-present(delete missing)" if hashes.get(p) is None else "?")
            if exists:
                c = "c-or-b-delete-not-applied"
        elif not exists:
            c = "d-only-in-commit(missing in new)"
        else:
            h = hashes[p]
            c = "a-identical" if h == newb else ("b-new-unchanged-since-base" if h == oldb else "c-diverged")
        cls[c] += 1
        lines.append(f"{c}\t{st}\t{p.decode()}")
    with open(outfile, "w", encoding="utf-8") as f:
        f.write("\n".join(sorted(lines)) + "\n")
    print(base[:8], commit[:8], dict(cls))


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2], sys.argv[3])
