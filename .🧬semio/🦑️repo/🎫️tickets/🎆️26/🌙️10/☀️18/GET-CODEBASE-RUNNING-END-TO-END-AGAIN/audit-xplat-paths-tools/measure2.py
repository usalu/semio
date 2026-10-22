# 🔎 Refined path-length and ticket source-copy audit (read-only, stdout only)
import subprocess, collections

REPO = "/Users/ueli/Documents/semio"
WIN_ROOT = "C:\\Users\\abcdefgh\\semio\\"
BAZEL_PREFIX = "C:\\Users\\abcdefgh\\_bazel_x\\abcdef12\\execroot\\_main\\bazel-out\\x64_windows-fastbuild\\bin\\"
VS16 = "\uFE0F"
REPO_TOP = {"✏️s", ".🧬semio", "🧰️framework", "♻️mit-bestand", "🌎️hub", ".cursor", "🎓️teaching", "🏢️semio-tech",
            "🧪️tests", "👴️leutwiler", ".storybook", ".github", "holder", ".devcontainer", ".agents", ".codex", "🧫️fixtures"}
SRC_EXT = (".rs", ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".go", ".py", ".c", ".h", ".hpp", ".cc", ".cpp",
           ".wgsl", ".css", ".html", ".vue", ".bzl", ".java", ".kt", ".swift", ".cs", ".sh", ".ps1", ".toml", ".json")


def u16(s):
    return len(s.encode("utf-16-le")) // 2


def ticket_slug_index(comps):
    for i, c in enumerate(comps):
        if c.endswith("tickets") and i + 4 < len(comps) + 1:
            return i + 4  # tickets/YY/MM/DD/SLUG -> index of SLUG
    return None


def git_bytes(args, stdin=None):
    return subprocess.run(["git", "-C", REPO] + args, input=stdin, capture_output=True, check=True).stdout


def main():
    raw = git_bytes(["ls-files", "-s", "-z"]).decode("utf-8")
    rows = []
    for rec in raw.split("\0"):
        if not rec:
            continue
        meta, path = rec.split("\t", 1)
        mode, oid, _ = meta.split(" ")
        rows.append((path, oid, mode))

    # blob sizes for exact-copy filtering
    oids = sorted({r[1] for r in rows})
    sz = {}
    out = git_bytes(["cat-file", "--batch-check=%(objectname) %(objectsize)"], stdin=("\n".join(oids) + "\n").encode())
    for line in out.decode().splitlines():
        o, s = line.split(" ")
        sz[o] = int(s)

    recs = []
    for path, oid, mode in rows:
        win = WIN_ROOT + path.replace("/", "\\")
        bz = BAZEL_PREFIX + path.replace("/", "\\")
        recs.append(dict(p=path, oid=oid, mode=mode, u=u16(win), b=u16(bz), vs=path.count(VS16), ub=u16(win) - path.count(VS16)))

    print(f"tracked={len(recs)} blobsize_known={len(sz)}")
    print(f"win_prefix_u16={u16(WIN_ROOT)} bazel_prefix_u16={u16(BAZEL_PREFIX)} (literal root is 24 units, brief said 25)")
    print("\n== direct checkout (u16) ==")
    for th in (240, 250, 256):
        print(f"  >{th}: {sum(1 for r in recs if r['u'] > th)}")
    print(f"  at exactly 256: {sum(1 for r in recs if r['u'] == 256)}")
    print("  VS16 (U+FE0F) in paths: files=%d" % sum(1 for r in recs if r['vs']))
    print(f"  >256 if FE0F dropped: {sum(1 for r in recs if r['ub'] > 256)} (direct)")
    print("\n== with Bazel output prefix (u16) ==")
    for th in (240, 256, 260):
        print(f"  >{th}: {sum(1 for r in recs if r['b'] > th)}")
    print(f"  newly >256 (was <=256): {sum(1 for r in recs if r['u'] <= 256 < r['b'])}")
    print(f"  newly >256 after dropping FE0F: {sum(1 for r in recs if r['ub'] <= 256 < r['b'] - r['vs'])}")
    print(f"  bazel >256 after dropping FE0F: {sum(1 for r in recs if r['b'] - r['vs'] > 256)}")
    print("  max bazel u16 =", max(r['b'] for r in recs))
    g = collections.Counter()
    for r in recs:
        if r['b'] > 256:
            c = r['p'].split("/")
            g["/".join(c[:2]) if len(c) > 2 else c[0]] += 1
    print("  bazel >256 by depth-2 group:")
    for k, v in g.most_common(15):
        print(f"    {v:6d} {k}")
    gb = collections.Counter()
    for r in recs:
        if r['b'] > 256:
            c = r['p'].split("/")
            if len(c) > 3 and c[0].endswith("semio"):
                gb["tickets/" + c[4] if len(c) > 4 and c[2].endswith("tickets") else "/".join(c[:4])] += 1
    print("  bazel >256 inside .🧬semio/🦑️repo/🎫️tickets, by ticket slug:")
    for k, v in gb.most_common(25):
        print(f"    {v:6d} {k}")

    print("\n== ticket source copies ==")
    ticket = [r for r in recs if r['p'].split("/")[:3][-1:] and len(r['p'].split("/")) > 4 and r['p'].split("/")[2].endswith("tickets")]
    print(f"  tickets tracked: {len(ticket)}")
    nont = collections.defaultdict(list)
    for r in recs:
        if not (len(r['p'].split("/")) > 2 and r['p'].split("/")[2].endswith("tickets")):
            if sz.get(r['oid'], 0) >= 512:
                nont[r['oid']].append(r['p'])
    exact = []
    for r in ticket:
        if sz.get(r['oid'], 0) >= 512 and r['oid'] in nont:
            exact.append((r['p'], nont[r['oid']][0]))
    print(f"  exact byte copies (>=512B) of non-ticket tracked files: {len(exact)}")
    ex_src = [e for e in exact if e[0].endswith(SRC_EXT)]
    print(f"    of which source-type extension: {len(ex_src)}")
    for a, b in ex_src[:40]:
        print(f"    COPY {a}\n         <- {b}")
    slug_c = collections.Counter()
    repo_tree = []
    for r in ticket:
        c = r['p'].split("/")
        si = ticket_slug_index(c)
        if si is None or si >= len(c):
            continue
        slug = c[si]
        rest = c[si + 1:]
        if any(seg in REPO_TOP for seg in rest[:-1]) or any(seg in REPO_TOP for seg in rest[:-1]):
            slug_c[slug] += 1
            repo_tree.append(r['p'])
    print(f"  tracked ticket files that mirror a repo top-level dir (path segment match, any depth): {len(repo_tree)}")
    for k, v in slug_c.most_common(30):
        print(f"    {v:6d} {k}")
    sc = collections.Counter(("." + r['p'].rsplit(".", 1)[-1]) for r in ticket if "." in r['p'].split("/")[-1])
    print("  ticket tracked extensions (top 20):", dict(sc.most_common(20)))
    print("  ticket source-ext tracked files:", sum(1 for r in ticket if r['p'].endswith(SRC_EXT[:-2])))


if __name__ == "__main__":
    main()
