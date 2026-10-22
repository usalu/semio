# 📏 Measure tracked path lengths for Windows checkout and Bazel output estimates (read-only, stdout only)
import subprocess, collections

REPO = "/Users/ueli/Documents/semio"
WIN_ROOT = "C:\\Users\\abcdefgh\\semio\\"
BAZEL_PREFIX = "C:\\Users\\abcdefgh\\_bazel_x\\abcdef12\\execroot\\_main\\bazel-out\\x64_windows-fastbuild\\bin\\"
SOURCE_EXT = {".rs", ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".go", ".py", ".c", ".h", ".hpp", ".cc", ".cpp",
              ".wgsl", ".css", ".html", ".vue", ".bzl", ".java", ".kt", ".swift", ".cs", ".sh", ".ps1"}


def u16(s):
    return len(s.encode("utf-16-le")) // 2


def in_tickets(path):
    c = path.split("/")
    return len(c) > 3 and c[0].endswith("semio") and c[1].endswith("repo") and c[2].endswith("tickets")


def dirs_of(path):
    c = path.split("/")[:-1]
    return ["/".join(c[:i]) for i in range(1, len(c) + 1)]


def main():
    raw = subprocess.run(["git", "-C", REPO, "ls-files", "-s", "-z"], capture_output=True, check=True).stdout.decode("utf-8")
    rows = []
    for rec in raw.split("\0"):
        if not rec:
            continue
        meta, path = rec.split("\t", 1)
        mode, oid, _ = meta.split(" ")
        win = WIN_ROOT + path.replace("/", "\\")
        bz = BAZEL_PREFIX + path.replace("/", "\\")
        rows.append((path, oid, mode, len(win), u16(win), len(bz), u16(bz)))

    print(f"tracked={len(rows)} win_root_cp={len(WIN_ROOT)} win_root_u16={u16(WIN_ROOT)} bazel_prefix_cp={len(BAZEL_PREFIX)} bazel_prefix_u16={u16(BAZEL_PREFIX)}")
    print(f"max_cp={max(r[3] for r in rows)} max_u16={max(r[4] for r in rows)} max_bazel_u16={max(r[6] for r in rows)}")
    print(f"paths_cp_ne_u16={sum(1 for r in rows if r[3] != r[4])} symlinks={sum(1 for r in rows if r[2] == '120000')} submodules={sum(1 for r in rows if r[2] == '160000')}")
    for th in (256, 240):
        print(f"over{th}: cp={sum(1 for r in rows if r[3] > th)} u16={sum(1 for r in rows if r[4] > th)} | bazel_u16={sum(1 for r in rows if r[6] > th)}")
    print(f"newly_over256_when_bazel_added={sum(1 for r in rows if r[4] <= 256 < r[6])}")
    print(f"newly_over240_when_bazel_added={sum(1 for r in rows if r[4] <= 240 < r[6])}")

    print("\n== top-level / depth-2 groups over 240 (u16) ==")
    g2 = collections.Counter()
    g2_256 = collections.Counter()
    for r in rows:
        if r[4] > 240:
            c = r[0].split("/")
            g2["/".join(c[:2]) if len(c) > 2 else c[0]] += 1
        if r[4] > 256:
            c = r[0].split("/")
            g2_256["/".join(c[:2]) if len(c) > 2 else c[0]] += 1
    for k, v in g2.most_common(25):
        print(f"{v:6d} over240  {g2_256.get(k, 0):6d} over256  {k}")

    print("\n== top 50 longest paths (u16) ==")
    for r in sorted(rows, key=lambda r: -r[4])[:50]:
        print(f"{r[4]:4d}u16 {r[3]:4d}cp {r[5]:4d}bz {r[0]}")

    print("\n== directory prefixes ranked by number of over-240 files beneath ==")
    dc = collections.Counter()
    dl = {}
    for r in rows:
        if r[4] > 240:
            for d in dirs_of(r[0]):
                dc[d] += 1
                dl[d] = u16(d) + 0
    for d, n in dc.most_common(45):
        print(f"{n:6d} {dl[d]:4d}u16 {d}")

    print("\n== path segments ranked by total over-240 contribution (segment u16 x count) ==")
    seg = collections.Counter()
    segl = {}
    for r in rows:
        if r[4] > 240:
            for s in set(r[0].split("/")[:-1]):
                seg[s] += 1
                segl[s] = u16(s)
    for s, n in sorted(seg.items(), key=lambda kv: -kv[1] * segl[kv[0]])[:30]:
        print(f"{n:6d} files {segl[s]:3d}u16 seg={s}")

    print("\n== tickets folder ==")
    tk = [r for r in rows if in_tickets(r[0])]
    print(f"tickets_tracked={len(tk)} tickets_over240={sum(1 for r in tk if r[4] > 240)} tickets_over256={sum(1 for r in tk if r[4] > 256)}")
    by_oid = collections.defaultdict(list)
    for r in rows:
        if not in_tickets(r[0]):
            by_oid[r[1]].append(r[0])
    dup = []
    for r in tk:
        if r[1] in by_oid:
            dup.append((r[0], by_oid[r[1]][0]))
    print(f"tickets_exact_copy_of_nontickets_blob={len(dup)}")
    for a, b in dup[:60]:
        print(f"  COPY {a}\n       <- {b}")
    srcx = [r[0] for r in tk if any(r[0].endswith(e) for e in SOURCE_EXT)]
    print(f"tickets_source_extension_files={len(srcx)}")
    ext = collections.Counter(("." + r[0].rsplit(".", 1)[-1]) for r in tk if "." in r[0].split("/")[-1])
    print("  by ext:", dict(ext.most_common(20)))
    for p in srcx[:80]:
        print(f"  SRC {p}")


if __name__ == "__main__":
    main()
