import json, sys
S = 1_000_000
OLD, EDGE, JUST, NEW = S - 5000, S - 2000, S - 2001, S + 10

def facts(code, syscall, path, probe, change, platform="posix", root="/w"):
    return {"code": code, "syscall": syscall, "path": path, "platform": platform, "root": root, "startedAtMs": S, "probe": probe, "changeMs": change}

def want(disposition, reason, cause):
    return {"disposition": disposition, "reason": reason, "cause": cause}

D = []
def d(id, f, w): D.append({"id": id, "facts": f, "expected": w})
A = "/w/src/a.ts"
d("enoent-absent-changed", facts("ENOENT", "lstat", A, "absent", NEW), want("persistent", "vanished", "absent"))
d("enoent-present-changed", facts("ENOENT", "lstat", A, "present", NEW), want("transient", "vanished", "present"))
d("enoent-absent-old", facts("ENOENT", "lstat", A, "absent", OLD), want("none", None, "unchanged"))
d("enoent-boundary-equal", facts("ENOENT", "lstat", A, "absent", EDGE), want("persistent", "vanished", "absent"))
d("enoent-boundary-just-before", facts("ENOENT", "lstat", A, "absent", JUST), want("none", None, "unchanged"))
d("enoent-no-witness", facts("ENOENT", "lstat", A, "absent", None), want("none", None, "unchanged"))
d("enotdir-absent", facts("ENOTDIR", "open", A, "absent", NEW), want("persistent", "vanished", "absent"))
d("enotdir-present", facts("ENOTDIR", "open", A, "present", NEW), want("transient", "vanished", "present"))
d("eisdir-present", facts("EISDIR", "read", A, "present", NEW), want("persistent", "identity", "shape"))
d("eisdir-present-unchanged", facts("EISDIR", "read", A, "present", OLD), want("none", None, "unchanged"))
d("eisdir-absent", facts("EISDIR", "read", A, "absent", NEW), want("persistent", "vanished", "absent"))
d("estale-present", facts("ESTALE", "stat", A, "present", NEW), want("transient", "identity", "present"))
d("eloop-present", facts("ELOOP", "lstat", A, "present", NEW), want("persistent", "identity", "shape"))
d("eacces-posix", facts("EACCES", "open", A, "present", NEW), want("none", None, "code"))
d("eacces-win32", facts("EACCES", "open", "C:\\w\\src\\a.ts", "present", None, "win32", "C:\\w"), want("transient", "busy", "busy"))
d("ebusy-win32", facts("EBUSY", "open", "C:\\w\\src\\a.ts", "present", OLD, "win32", "C:\\w"), want("transient", "busy", "busy"))
d("ebusy-posix", facts("EBUSY", "open", A, "present", NEW), want("none", None, "code"))
d("eperm-win32", facts("EPERM", "lstat", "C:\\w\\src\\a.ts", "unknown", None, "win32", "C:\\w"), want("transient", "busy", "busy"))
d("emfile-posix", facts("EMFILE", "open", A, "present", OLD), want("transient", "busy", "busy"))
d("enfile-posix", facts("ENFILE", "lstat", A, "absent", None), want("transient", "busy", "busy"))
d("enfile-outside", facts("ENFILE", "open", "/etc/passwd", "present", NEW), want("none", None, "outside"))
d("eperm-posix", facts("EPERM", "lstat", A, "present", NEW), want("none", None, "code"))
for code in ["ENOSPC", "EEXIST", "ENOTEMPTY", "EINVAL", "ECONNRESET", "EAGAIN"]:
    d("other-" + code.lower(), facts(code, "open", A, "absent", NEW), want("none", None, "code"))
for call in ["rename", "copyfile", "unlink", "mkdir", "rmdir", "symlink", "write", "utimes"]:
    d("write-" + call, facts("ENOENT", call, A, "absent", NEW), want("none", None, "write"))
for call in ["open", "lstat", "stat", "fstat", "scandir", "opendir", "readdir", "readlink", "realpath", "access", "read", "readFile"]:
    d("read-" + call, facts("ENOENT", call, A, "absent", NEW), want("persistent", "vanished", "absent"))
d("syscall-unknown-is-read", facts("ENOENT", None, A, "absent", NEW), want("persistent", "vanished", "absent"))
d("relative-path", facts("ENOENT", "lstat", "src/a.ts", "absent", NEW), want("none", None, "relative"))
d("command-not-found", facts("ENOENT", "spawn", "bun", "absent", NEW), want("none", None, "write"))
d("outside-root", facts("ENOENT", "lstat", "/etc/passwd", "absent", NEW), want("none", None, "outside"))
d("outside-sibling-prefix", facts("ENOENT", "lstat", "/w2/x", "absent", NEW), want("none", None, "outside"))
d("root-itself", facts("ENOENT", "lstat", "/w", "absent", NEW), want("persistent", "vanished", "absent"))
d("dotdot-name-inside", facts("ENOENT", "lstat", "/w/..foo/a", "absent", NEW), want("persistent", "vanished", "absent"))
d("no-root", facts("ENOENT", "lstat", A, "absent", NEW, root=None), want("none", None, "root"))
for segment in [".cache", ".git", "\u26a1\ufe0fcache", "bazel-out", "bazel-bin", "bazel-testlogs"]:
    d("owned-" + segment.encode("ascii", "backslashreplace").decode(), facts("ENOENT", "lstat", "/w/x/" + segment + "/y", "absent", NEW), want("none", None, "owned"))
d("not-owned-lookalike", facts("ENOENT", "lstat", "/w/x/.cache2/y", "absent", NEW), want("persistent", "vanished", "absent"))
d("probe-unknown", facts("ENOENT", "lstat", A, "unknown", NEW), want("none", None, "probe"))
d("win32-absent-changed", facts("ENOENT", "lstat", "C:\\w\\src\\a.ts", "absent", NEW, "win32", "C:\\w"), want("persistent", "vanished", "absent"))
d("win32-case-insensitive-inside", facts("ENOENT", "lstat", "c:\\W\\Src\\a.ts", "absent", NEW, "win32", "C:\\w"), want("persistent", "vanished", "absent"))
d("win32-relative", facts("ENOENT", "lstat", "src\\a.ts", "absent", NEW, "win32", "C:\\w"), want("none", None, "relative"))
d("win32-other-drive", facts("ENOENT", "lstat", "D:\\w\\a.ts", "absent", NEW, "win32", "C:\\w"), want("none", None, "outside"))
d("win32-owned", facts("ENOENT", "lstat", "C:\\w\\.git\\HEAD", "absent", NEW, "win32", "C:\\w"), want("none", None, "owned"))

def entry(directory, mtime, ctime):
    return {"directory": directory, "mtimeMs": mtime, "ctimeMs": ctime}
O = []
def o(id, platform, path, lstat, probe, change): O.append({"id": id, "platform": platform, "path": path, "lstat": lstat, "expected": {"probe": probe, "changeMs": change}})
o("present-file-newest-own", "posix", "/w/a/b.txt", {"/w/a/b.txt": entry(False, 100, 300), "/w/a": entry(True, 200, 250)}, "present", 300)
o("present-file-newest-parent", "posix", "/w/a/b.txt", {"/w/a/b.txt": entry(False, 100, 120), "/w/a": entry(True, 900, 800)}, "present", 900)
o("absent-parent-exists", "posix", "/w/a/b.txt", {"/w/a/b.txt": "absent", "/w/a": entry(True, 500, 400)}, "absent", 500)
o("absent-grandparent", "posix", "/w/a/b/c.txt", {"/w/a/b/c.txt": "absent", "/w/a/b": "absent", "/w/a": entry(True, 100, 700)}, "absent", 700)
o("absent-ancestor-is-file", "posix", "/w/a/b/c.txt", {"/w/a/b/c.txt": "absent", "/w/a/b": entry(False, 650, 640), "/w/a": entry(True, 10, 20)}, "absent", 650)
o("own-unknown", "posix", "/w/a/b.txt", {"/w/a/b.txt": "unknown", "/w/a": entry(True, 500, 400)}, "unknown", None)
o("ancestor-unknown", "posix", "/w/a/b.txt", {"/w/a/b.txt": "absent", "/w/a": "unknown", "/w": entry(True, 5, 5)}, "unknown", None)
o("nothing-exists", "posix", "/w/a/b.txt", {"/w/a/b.txt": "absent", "/w/a": "absent", "/w": "absent", "/": "absent"}, "absent", None)
o("win32-absent", "win32", "C:\\w\\a\\b.txt", {"C:\\w\\a\\b.txt": "absent", "C:\\w\\a": entry(True, 321, 123)}, "absent", 321)
o("win32-present", "win32", "C:\\w\\a\\b.txt", {"C:\\w\\a\\b.txt": entry(False, 5, 6), "C:\\w\\a": entry(True, 7, 8)}, "present", 8)

T = []
def t(id, platform, text, expected): T.append({"id": id, "platform": platform, "text": text, "expected": expected})
t("node-lstat", "posix", "ENOENT: no such file or directory, lstat '/w/src/a.ts'", {"code": "ENOENT", "syscall": "lstat", "path": "/w/src/a.ts"})
t("node-with-prefix", "posix", "Error: ENOENT: no such file or directory, open '/w/x.json'\n    at readFileSync (node:fs:441:20)", {"code": "ENOENT", "syscall": "open", "path": "/w/x.json"})
t("bun-properties", "posix", "error: ENOENT: no such file or directory, open '/w/x.json'\n  errno: -2,\n  syscall: \"open\",\n  code: \"ENOENT\",\n  path: \"/w/x.json\"\n", {"code": "ENOENT", "syscall": "open", "path": "/w/x.json"})
t("bun-properties-only", "posix", "{\n  syscall: \"scandir\",\n  code: \"ENOTDIR\",\n  path: \"/w/a\"\n}", {"code": "ENOTDIR", "syscall": "scandir", "path": "/w/a"})
t("rename-two-paths", "posix", "ENOENT: no such file or directory, rename '/w/a' -> '/w/b'", {"code": "ENOENT", "syscall": "rename", "path": "/w/a"})
t("relative-then-absolute", "posix", "ENOENT: no such file or directory, open 'x' while reading '/w/y'", {"code": "ENOENT", "syscall": "open", "path": "/w/y"})
t("relative-only", "posix", "ENOENT: no such file or directory, lstat 'src/a.ts'", None)
t("spawn", "posix", "Error: spawn bun ENOENT", None)
t("no-code", "posix", "something failed at '/w/a'", None)
t("code-without-path", "posix", "ECONNRESET: socket hang up", None)
t("eslint-is-not-a-code", "posix", "ESLint: 9.1.0 '/w/a.ts'", None)
t("win32-busy", "win32", "EBUSY: resource busy or locked, open 'C:\\w\\a.ts'", {"code": "EBUSY", "syscall": "open", "path": "C:\\w\\a.ts"})
t("win32-relative-path", "win32", "ENOENT: no such file or directory, lstat 'src\\a.ts'", None)
t("tail-in-the-middle", "posix", "line one\nstack\nENOENT: no such file or directory, readlink '/w/link'\nmore text", {"code": "ENOENT", "syscall": "readlink", "path": "/w/link"})

t("old-error-outside-window", "posix", "ENOENT: no such file or directory, lstat '/w/old'\n" + "x" * 5000, None)
t("error-inside-window", "posix", "x" * 5000 + "\nENOENT: no such file or directory, lstat '/w/new'", {"code": "ENOENT", "syscall": "lstat", "path": "/w/new"})
json.dump({"startedAtMs": S, "decide": D, "observe": O, "text": T}, open(sys.argv[1], "w", encoding="utf-8"), indent=1, ensure_ascii=False)
print(len(D), len(O), len(T))
