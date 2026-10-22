# 🧭 Root-length sensitivity and candidate-rename savings for the audit report (read-only, stdout only)
import subprocess

REPO = "/Users/ueli/Documents/semio"
EXEC_OUT = "\\execroot\\_main\\bazel-out\\x64_windows-fastbuild\\bin\\"
BRIEF_BAZEL = "C:\\Users\\abcdefgh\\_bazel_x\\abcdef12" + EXEC_OUT
REAL_BAZEL = "C:\\Users\\abcdefgh\\_bazel_x\\" + "0" * 32 + EXEC_OUT
VS16 = "\uFE0F"


def u16(s):
    return len(s.encode("utf-16-le")) // 2


def main():
    raw = subprocess.run(["git", "-C", REPO, "ls-files", "-z"], capture_output=True, check=True).stdout.decode("utf-8")
    paths = [p for p in raw.split("\0") if p]
    rel = [p.replace("/", "\\") for p in paths]
    for root in ("C:\\Users\\abcdefgh\\semio\\", "C:\\Users\\abcdefgh1\\semio\\", "C:\\Users\\firstname\\src\\semio\\"):
        r = u16(root)
        n = sum(1 for x in rel if r + u16(x) > 256)
        m = sum(1 for x in rel if r + u16(x) > 259)
        print(f"root={r}u16 '{root}': over256={n} over259(MAX_PATH)={m}")

    def count(transform, prefix):
        return sum(1 for x in rel if u16(prefix) + u16(transform(x)) > 256)

    noop = lambda x: x
    nofe = lambda x: x.replace(VS16, "")
    snap = lambda x: x.replace(VS16, "").replace("🔖" + "\uFE0F" + "1\\", "").replace("🪆" + "\uFE0F" + "subsets\\", "")
    print(f"brief prefix ({u16(BRIEF_BAZEL)}u16): baseline={count(noop, BRIEF_BAZEL)} noVS16={count(nofe, BRIEF_BAZEL)} noVS16+noStandardsLayers={count(snap, BRIEF_BAZEL)}")
    print(f"real 32-hex prefix ({u16(REAL_BAZEL)}u16): baseline={count(noop, REAL_BAZEL)} noVS16={count(nofe, REAL_BAZEL)} noVS16+noStandardsLayers={count(snap, REAL_BAZEL)}")
    tk = [x for x in rel if x.split("\\")[2].endswith("tickets")] if False else []
    print(f"plugin-tree share of brief-prefix overflow: {sum(1 for p, x in zip(paths, rel) if p.startswith('✏️s/🔌️plugins/') and u16(BRIEF_BAZEL) + u16(x) > 256)}")
    print(f"ticket-tree share of brief-prefix overflow: {sum(1 for p, x in zip(paths, rel) if p.startswith('.🧬semio/') and u16(BRIEF_BAZEL) + u16(x) > 256)}")


if __name__ == "__main__":
    main()
