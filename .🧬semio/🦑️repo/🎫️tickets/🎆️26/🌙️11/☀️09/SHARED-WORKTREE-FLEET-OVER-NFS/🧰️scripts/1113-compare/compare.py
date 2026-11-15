import os, subprocess, hashlib, json, datetime, stat, collections

DOC = "/Users/ueli/Documents/semio"
SH = "/Users/Shared/semio"
S = "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/🧰️scripts/1113-compare"


def lsf(root):
    out = subprocess.run(
        ["git", "--no-optional-locks", "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
        cwd=root, capture_output=True, check=True).stdout
    return set(p for p in out.split(b"\0") if p)


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def dkey(p):
    parts = p.split(b"/")[:-1][:3]
    return b"/".join(parts).decode("utf-8", "backslashreplace") or "(root)"


def dec(p):
    return p.decode("utf-8", "backslashreplace")


doc_set = lsf(DOC)
sh_set = lsf(SH)
both = doc_set & sh_set
only_doc = doc_set - sh_set
only_sh = sh_set - doc_set

res = collections.Counter()
reasons = collections.Counter()
newer = collections.Counter()
dir_differ = collections.Counter()
dir_only_doc = collections.Counter()
dir_only_sh = collections.Counter()
differ_list = []
missing = collections.Counter()

for p in both:
    a = os.path.join(DOC.encode(), p)
    b = os.path.join(SH.encode(), p)
    try:
        sa = os.lstat(a)
    except FileNotFoundError:
        missing["doc_missing_on_disk"] += 1
        continue
    try:
        sb = os.lstat(b)
    except FileNotFoundError:
        missing["shared_missing_on_disk"] += 1
        continue
    ka = "link" if stat.S_ISLNK(sa.st_mode) else "file" if stat.S_ISREG(sa.st_mode) else "other"
    kb = "link" if stat.S_ISLNK(sb.st_mode) else "file" if stat.S_ISREG(sb.st_mode) else "other"
    diff = None
    if ka != kb:
        diff = "type"
    elif ka == "link":
        if os.readlink(a) != os.readlink(b):
            diff = "link_target"
    elif ka == "file":
        if sa.st_size != sb.st_size:
            diff = "size"
        elif sha(a) != sha(b):
            diff = "content"
    else:
        diff = None if sa.st_size == sb.st_size else "size"
    if diff is None:
        res["same"] += 1
        continue
    res["differ"] += 1
    reasons[diff] += 1
    dir_differ[dkey(p)] += 1
    if sa.st_mtime_ns > sb.st_mtime_ns:
        newer["doc_newer"] += 1
    elif sa.st_mtime_ns < sb.st_mtime_ns:
        newer["shared_newer"] += 1
    else:
        newer["equal_mtime"] += 1
    differ_list.append({"path": dec(p), "reason": diff, "doc_size": sa.st_size, "sh_size": sb.st_size,
                        "doc_mtime": datetime.datetime.fromtimestamp(sa.st_mtime).isoformat(),
                        "sh_mtime": datetime.datetime.fromtimestamp(sb.st_mtime).isoformat()})

cutoff_10 = datetime.datetime(2026, 11, 10).timestamp()
cutoff_09 = datetime.datetime(2026, 11, 9).timestamp()
only_doc_rows = []
only_doc_newer = []
for p in only_doc:
    a = os.path.join(DOC.encode(), p)
    try:
        st = os.lstat(a)
    except FileNotFoundError:
        missing["only_doc_missing_on_disk"] += 1
        continue
    dir_only_doc[dkey(p)] += 1
    row = {"path": dec(p), "size": st.st_size,
           "mtime": datetime.datetime.fromtimestamp(st.st_mtime).isoformat()}
    only_doc_rows.append(row)
    if st.st_mtime >= cutoff_10:
        only_doc_newer.append(row)

for p in only_sh:
    b = os.path.join(SH.encode(), p)
    dir_only_sh[dkey(p)] += 1

only_doc_newer.sort(key=lambda r: r["mtime"], reverse=True)
only_doc_09 = sum(1 for r in only_doc_rows if r["mtime"] >= datetime.datetime(2026, 11, 9).isoformat())

summary = {
    "doc_total": len(doc_set), "shared_total": len(sh_set), "union": len(doc_set | sh_set),
    "only_doc": len(only_doc), "only_shared": len(only_sh), "both": len(both),
    "both_result": dict(res), "differ_reasons": dict(reasons), "newer": dict(newer),
    "missing": dict(missing),
    "only_doc_newer_ge_2026_11_10": len(only_doc_newer),
    "only_doc_ge_2026_11_09": only_doc_09,
    "top_dirs_differ": dir_differ.most_common(30),
    "top_dirs_only_doc": dir_only_doc.most_common(30),
    "top_dirs_only_shared": dir_only_sh.most_common(30),
}
with open(os.path.join(S, "compare.json"), "w", encoding="utf-8") as f:
    json.dump({"summary": summary, "only_doc_newer": only_doc_newer,
               "differ": differ_list}, f, ensure_ascii=False, indent=1)
print(json.dumps(summary, ensure_ascii=False, indent=1))
