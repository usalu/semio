#!/usr/bin/env python3
"""Independent measurement of the Bazel path lengths of the course closure (read only).

usage: measure.py <checkout-root> <closure-packages.json> <out.json>
Reads the tracked file list with `git ls-files`, selects the closure packages, and prints the longest
source path, longest basename and directory count, then the Bazel path families per OS for the CURRENT
layout (`s/<repo path>`) and the ALIAS layout (`a/<sha256(dir)[:10]>/<name>`).
"""
import hashlib, json, os, subprocess, sys

root, packages, out = sys.argv[1], sys.argv[2], sys.argv[3]
u16 = lambda s: len(s.encode("utf-16-le")) // 2
u8 = lambda s: len(s.encode("utf-8"))
tracked = subprocess.run(["git", "-C", root, "ls-files", "-z"], capture_output=True, check=True).stdout.decode("utf-8").split("\0")
tracked = [p for p in tracked if p and not p.startswith(".🧬semio/🦑️repo/🎫️tickets/") and os.path.lexists(os.path.join(root, p))]
roots = list(json.load(open(packages)).values())
closure = [p for p in tracked if any(p.startswith(r + "/") for r in roots)]
alias = lambda p: "a/" + hashlib.sha256(os.path.dirname(p).encode()).hexdigest()[:10] + "/" + os.path.basename(p)
real = lambda p: "s/" + p

def stat(files):
    return {
        "files": len(files),
        "directories": len({os.path.dirname(p) for p in files}),
        "longestRealUnits": max(u16(real(p)) for p in files),
        "longestRealBytes": max(u8(real(p)) for p in files),
        "longestAliasUnits": max(u16(alias(p)) for p in files),
        "longestAliasBytes": max(u8(alias(p)) for p in files),
        "longestBasenameUnits": max(u16(os.path.basename(p)) for p in files),
        "longestBasename": max(files, key=lambda p: u16(os.path.basename(p))).split("/")[-1],
        "longestReal": max(files, key=lambda p: u16(real(p))),
    }

result = {"closure": stat(closure), "repository": stat(tracked)}
hosts = {
    "macOS arm64 /Users/ueli/Documents/semio": ("/Users/ueli/Documents/semio", "/", "darwin_arm64-fastbuild", True),
    "devcontainer /workspaces/semio": ("/workspaces/semio", "/", "aarch64-fastbuild", True),
    "Linux /home/ueli/Documents/semio": ("/home/ueli/Documents/semio", "/", "k8-fastbuild", True),
    "Windows C:\\Users\\ueli\\Documents\\semio": ("C:\\Users\\ueli\\Documents\\semio", "\\", "x64_windows-fastbuild", False),
    "Windows C:\\Users\\firstname.lastname\\source\\repos\\semio": ("C:\\Users\\firstname.lastname\\source\\repos\\semio", "\\", "x64_windows-fastbuild", False),
}
task, file32 = "t_" + "0" * 16, "0" * 32
rows = {}
for name, (checkout, sep, configuration, runfiles) in hosts.items():
    ob = checkout + sep + sep.join([".cache", "b", "p", "m", "bazel", "outputs", "server", "0" * 16])
    def join(*parts): return sep.join(parts)
    bin_ = join("execroot", "_main", "bazel-out", configuration, "bin")
    entry = {"checkout": u16(checkout), "outputBase": u16(ob)}
    for view, label, longest in (("real", real, max(closure, key=lambda p: u16(real(p)))), ("alias", alias, max(closure, key=lambda p: u16(alias(p))))):
        lab = label(longest).replace("/", sep)
        fam = {
            "source": u16(join(ob, "execroot", "_main", lab)),
            "sandbox-source": u16(join(ob, "sandbox", "processwrapper-sandbox", "123456", "execroot", "_main", lab)),
        }
        if runfiles: fam["runfiles-source"] = u16(join(ob, bin_, task + ".runfiles", "_main", lab))
        entry[view] = fam
    entry["snapshot"] = u16(join(ob, bin_, task + ".snapshot", "files", file32))
    if runfiles: entry["runfiles-snapshot"] = u16(join(ob, bin_, task + ".runfiles", "_main", task + ".snapshot", "files", file32))
    rows[name] = entry
result["hosts"] = rows
json.dump(result, open(out, "w"), ensure_ascii=False, indent=1)
print(json.dumps(result["closure"], ensure_ascii=False, indent=1))
for name, row in rows.items(): print(name, json.dumps(row, ensure_ascii=False))
