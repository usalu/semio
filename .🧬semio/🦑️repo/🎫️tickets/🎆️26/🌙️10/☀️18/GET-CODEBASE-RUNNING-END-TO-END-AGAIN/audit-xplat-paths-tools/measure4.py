# 🧮 Bazel prefix scenarios: brief's 8-hex output base vs real 32-hex MD5 output base, short user root, source link `s` (read-only, stdout only)
import subprocess

REPO = "/Users/ueli/Documents/semio"
WIN_ROOT = "C:\\Users\\abcdefgh\\semio\\"
EXEC_OUT = "\\execroot\\_main\\bazel-out\\x64_windows-fastbuild\\bin\\"
EXEC_SRC = "\\execroot\\_main\\s\\"
SCENARIOS = {
    "brief (8-hex base under _bazel_x)": "C:\\Users\\abcdefgh\\_bazel_x\\abcdef12" + EXEC_OUT,
    "real 32-hex base under _bazel_x": "C:\\Users\\abcdefgh\\_bazel_x\\" + "0" * 32 + EXEC_OUT,
    "short root C:\\b with 32-hex base": "C:\\b\\" + "0" * 32 + EXEC_OUT,
}


def u16(s):
    return len(s.encode("utf-16-le")) // 2


def main():
    raw = subprocess.run(["git", "-C", REPO, "ls-files", "-z"], capture_output=True, check=True).stdout.decode("utf-8")
    paths = [p for p in raw.split("\0") if p]
    rel = [u16(p.replace("/", "\\")) for p in paths]
    print(f"rel max u16={max(rel)} rel>232={sum(1 for r in rel if r > 232)} (root 24 + rel max = {24 + max(rel)})")
    for name, pre in SCENARIOS.items():
        pl = u16(pre)
        tot = [pl + r for r in rel]
        print(f"[{name}] prefix={pl}u16 >256={sum(1 for t in tot if t > 256)} >259={sum(1 for t in tot if t > 259)} max={max(tot)}")
    pre = "C:\\Users\\abcdefgh\\_bazel_x\\" + "0" * 32 + EXEC_SRC
    pl = u16(pre)
    tot = [pl + r for r in rel]
    print(f"[source link s, real 32-hex base] prefix={pl}u16 >256={sum(1 for t in tot if t > 256)} >259={sum(1 for t in tot if t > 259)} max={max(tot)}")
    budget = [256 - u16(SCENARIOS['real 32-hex base under _bazel_x'])]
    print(f"rel budget for outputs with real base = {budget[0]}u16")


if __name__ == "__main__":
    main()
