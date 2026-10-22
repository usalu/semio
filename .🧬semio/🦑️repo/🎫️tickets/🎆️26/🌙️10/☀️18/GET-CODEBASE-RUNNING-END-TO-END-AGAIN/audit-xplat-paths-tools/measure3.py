# 🔬 Breakdowns for the audit report: per-slug Bazel offenders, plugin subtree depth, copies per slug, Windows-invalid names (read-only, stdout only)
import subprocess, collections, re

REPO = "/Users/ueli/Documents/semio"
WIN_ROOT = "C:\\Users\\abcdefgh\\semio\\"
BAZEL_PREFIX = "C:\\Users\\abcdefgh\\_bazel_x\\abcdef12\\execroot\\_main\\bazel-out\\x64_windows-fastbuild\\bin\\"
VS16 = "\uFE0F"
RESERVED = {"CON", "PRN", "AUX", "NUL"} | {f"COM{i}" for i in range(1, 10)} | {f"LPT{i}" for i in range(1, 10)}
BAD_CHARS = re.compile(r'[<>:"|?*\x00-\x1f]')


def u16(s):
    return len(s.encode("utf-16-le")) // 2


def git_bytes(args, stdin=None):
    return subprocess.run(["git", "-C", REPO] + args, input=stdin, capture_output=True, check=True).stdout


def main():
    raw = git_bytes(["ls-files", "-s", "-z"]).decode("utf-8")
    recs = []
    for rec in raw.split("\0"):
        if not rec:
            continue
        meta, path = rec.split("\t", 1)
        mode, oid, _ = meta.split(" ")
        recs.append((path, oid, mode))
    sz = {}
    oids = sorted({r[1] for r in recs})
    out = git_bytes(["cat-file", "--batch-check=%(objectname) %(objectsize)"], stdin=("\n".join(oids) + "\n").encode())
    for line in out.decode().splitlines():
        o, s = line.split(" ")
        sz[o] = int(s)

    print("== Bazel-prefixed >256 per ticket slug (top 15) ==")
    slugc = collections.Counter()
    tot = 0
    for p, o, m in recs:
        c = p.split("/")
        if len(c) > 6 and c[2].endswith("tickets"):
            b = u16(BAZEL_PREFIX + p.replace("/", "\\"))
            if b > 256:
                slugc[c[6]] += 1
                tot += 1
    print(f"total in tickets: {tot}")
    for k, v in slugc.most_common(15):
        print(f"  {v:6d} {k}")

    print("\n== ✏️s/🔌️plugins subtree ==")
    pl = [p for p, o, m in recs if p.startswith("✏️s/🔌️plugins/")]
    lens = [u16(WIN_ROOT + p.replace("/", "\\")) for p in pl]
    lens.sort()
    n = len(lens)
    print(f"files={n} median_u16={lens[n//2]} p90={lens[int(n*0.9)]} p99={lens[int(n*0.99)]} max={lens[-1]}")
    print(f"files with bazel u16 > 256: {sum(1 for p in pl if u16(BAZEL_PREFIX + p.replace('/', chr(92))) > 256)}")
    print(f"files with direct u16 > 240: {sum(1 for l in lens if l > 240)}")
    d3 = collections.Counter("/".join(p.split("/")[:3]) for p in pl)
    print("  top depth-3 dirs by file count:")
    for k, v in d3.most_common(8):
        print(f"    {v:6d} {k}")
    seg = collections.Counter()
    for p in pl:
        for s in p.split("/")[:-1]:
            seg[s] += 1
    print("  most-used segments in plugin tree (count, u16):")
    for s, v in seg.most_common(12):
        print(f"    {v:6d} {u16(s):3d}u16 {s}")

    print("\n== tracked ticket copies of non-ticket source (>=512B exact blob), per slug (top 15) ==")
    nont = {}
    for p, o, m in recs:
        c = p.split("/")
        if not (len(c) > 2 and c[2].endswith("tickets")) and sz.get(o, 0) >= 512:
            nont.setdefault(o, p)
    cp = collections.Counter()
    for p, o, m in recs:
        c = p.split("/")
        if len(c) > 6 and c[2].endswith("tickets") and sz.get(o, 0) >= 512 and o in nont:
            cp[c[6]] += 1
    print(f"total exact copies in tickets: {sum(cp.values())}")
    for k, v in cp.most_common(15):
        print(f"  {v:6d} {k}")

    print("\n== Windows-invalid names in tracked paths ==")
    bad = [p for p, o, m in recs if BAD_CHARS.search(p)]
    res = [p for p, o, m in recs if any(seg.split(".")[0].upper() in RESERVED for seg in p.split("/"))]
    trail = [p for p, o, m in recs if any(seg.endswith((".", " ")) for seg in p.split("/"))]
    print(f"invalid-char paths={len(bad)} reserved-name segments={len(res)} trailing dot/space segments={len(trail)}")
    for p in (bad + res + trail)[:15]:
        print("  ", p)
    print(f"symlinks: {[p for p, o, m in recs if m == '120000'][:10]}")


if __name__ == "__main__":
    main()
