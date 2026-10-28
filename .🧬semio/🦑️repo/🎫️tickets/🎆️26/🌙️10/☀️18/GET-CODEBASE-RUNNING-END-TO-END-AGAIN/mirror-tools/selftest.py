#!/usr/bin/env python3
"""🧪 selftest.py: language-agnostic laws of mirror.py on a throw-away git repository (never the live checkout), with `diff -rq` as the independent oracle. Usage: python3 -I selftest.py   (exit 0 = all laws hold)"""
import hashlib, json, os, shutil, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
MIRROR = os.path.join(HERE, "mirror.py")
SCRATCH = tempfile.mkdtemp(prefix="mt-")
REPO = os.path.join(SCRATCH, "repo")
HOME = os.path.join(SCRATCH, "sm")
ENV = dict(os.environ, SEMIO_REPO=REPO, SEMIO_MIRROR_HOME=HOME)
failures = []


def check(name, condition, detail=""):
    print(("ok   " if condition else "FAIL ") + name + ("" if condition else "  " + detail))
    if not condition:
        failures.append(name)


def mirror(*args, expect=None):
    done = subprocess.run([sys.executable, "-I", MIRROR, *args], env=ENV, capture_output=True, text=True)
    return done.returncode, done.stderr + done.stdout


def write(rel, text):
    path = os.path.join(REPO, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as handle:
        handle.write(text)


def build_repo():
    os.makedirs(REPO)
    write("AGENTS.md", "rules\n")
    write("📋️project.json", "{}\n")
    write("package.json", "{}\n")
    write("tsconfig.json", "{}\n")
    write("sub/one/📋️project.json", "{}\n")
    write("sub/one/a.ts", 'import { b } from "../two/b"\nexport const a = b\n')
    write("sub/one/data.json", '{"k":1}\n')
    write("sub/two/📋️project.json", "{}\n")
    write("sub/two/b.ts", "export const b = 2\n")
    write("other/📋️project.json", "{}\n")
    write("other/c.ts", "export const c = 3\n")
    write("sub/one/new-untracked.txt", "x\n")
    managed = os.path.join(SCRATCH, "managed-nm")
    os.makedirs(os.path.join(managed, "pkg"))
    write_abs(os.path.join(managed, "pkg", "index.js"), "module.exports=1\n")
    os.symlink(os.path.join(REPO, "sub/one"), os.path.join(managed, "ws-one"))
    os.symlink(managed, os.path.join(REPO, "node_modules"))
    with open(os.path.join(REPO, ".gitignore"), "w") as handle:
        handle.write("node_modules\n")
    git = ["git", "-C", REPO, "-c", "user.name=t", "-c", "user.email=t@t"]
    subprocess.run(git + ["init", "-q"], check=True)
    subprocess.run(git + ["add", "--", "AGENTS.md", "📋️project.json", "package.json", "tsconfig.json", ".gitignore", "sub/one/📋️project.json", "sub/one/a.ts", "sub/one/data.json", "sub/two", "other"], check=True)
    write("sub/one/a.ts", 'import { b } from "../two/b"\nexport const a = b + 1\n')


def write_abs(path, text):
    with open(path, "w") as handle:
        handle.write(text)


def sha_tree(root, skip=("node_modules",)):
    out = {}
    for base, dirs, names in os.walk(root):
        dirs[:] = [d for d in dirs if d not in skip]
        for name in names:
            path = os.path.join(base, name)
            out[os.path.relpath(path, root)] = hashlib.sha256(open(path, "rb").read()).hexdigest() if not os.path.islink(path) else "link"
    return out


def escaping_links(root):
    bad = []
    real_repo = os.path.realpath(REPO)
    for base, dirs, names in os.walk(root):
        for name in dirs + names:
            path = os.path.join(base, name)
            if os.path.islink(path):
                target = os.path.realpath(path)
                if target == real_repo or target.startswith(real_repo + os.sep):
                    bad.append(path)
    return bad


def main():
    build_repo()
    code, text = mirror("create", "m1", "--only", "sub/one")
    root = os.path.join(HOME, "m1")
    check("create succeeds", code == 0, text)
    check("closure pulled in the imported project sub/two", os.path.isfile(os.path.join(root, "sub/two/b.ts")), text)
    check("closure left out unrelated project other/", not os.path.exists(os.path.join(root, "other")), text)
    check("uncommitted edit rides along", "b + 1" in open(os.path.join(root, "sub/one/a.ts")).read())
    check("untracked non-ignored file rides along", os.path.isfile(os.path.join(root, "sub/one/new-untracked.txt")))
    oracle = subprocess.run(["diff", "-rq", os.path.join(REPO, "sub/one"), os.path.join(root, "sub/one")], capture_output=True, text=True)
    check("diff -rq oracle: sub/one identical to live", oracle.returncode == 0, oracle.stdout)
    check("no symlink in the mirror reaches the live tree", escaping_links(root) == [], str(escaping_links(root)))
    link = os.path.join(root, "node_modules", "ws-one")
    check("absolute workspace link is rewritten into the mirror", os.path.islink(link) and os.path.realpath(link) == os.path.realpath(os.path.join(root, "sub/one")), os.path.realpath(link))
    live_before = sha_tree(REPO)
    code, text = mirror("run", "m1", "--", sys.executable, "-I", "-c", "open('sub/one/a.ts','w').write('changed\\n'); open('sub/one/made.txt','w').write('n\\n')")
    check("run in the mirror exits 0 and the guard is clean", code == 0 and "guard clean" in text, text)
    check("live tree untouched by a mirror write", sha_tree(REPO) == live_before)
    code, text = mirror("diff", "m1")
    check("diff lists the modified and the added file", "M sub/one/a.ts" in text and "A sub/one/made.txt" in text, text)
    code, text = mirror("run", "m1", "--no-seal", "--", sys.executable, "-I", "-c", f"open({REPO!r}+'/sub/one/leak.txt','w').write('x')")
    check("write-through (unsealed) fails loudly with exit 97", code == 97 and "leak.txt" in text, f"{code} {text}")
    os.unlink(os.path.join(REPO, "sub/one/leak.txt"))
    if sys.platform == "darwin":
        code, text = mirror("run", "m1", "--", sys.executable, "-I", "-c", f"open({REPO!r}+'/sub/one/leak2.txt','w').write('x')")
        check("write-through is refused by the seal (macOS)", not os.path.exists(os.path.join(REPO, "sub/one/leak2.txt")) and code != 0, f"{code} {text}")
    code, text = mirror("run", "m1", "--no-seal", "--", sys.executable, "-I", "-c", f"import os; os.utime({REPO!r}+'/sub/two/b.ts', None)")
    check("a touched live file is detected as modified", code == 97 and "M sub/two/b.ts" in text, f"{code} {text}")
    code, text = mirror("run", "m1", "--no-seal", "--peers-ok", "--", sys.executable, "-I", "-c", f"import os; os.utime({REPO!r}+'/sub/two/b.ts', None)")
    check("--peers-ok downgrades to a warning", code == 0 and "GUARD WARN" in text, f"{code} {text}")
    os.makedirs(os.path.join(SCRATCH, "stage", "files"))
    new = "export const b = 99\n"
    write_abs(os.path.join(SCRATCH, "stage", "files", "f000"), new)
    base = hashlib.sha256(open(os.path.join(REPO, "sub/two/b.ts"), "rb").read()).hexdigest()
    manifest = {"sub/two/b.ts": {"file": "f000", "base": base, "new": hashlib.sha256(new.encode()).hexdigest()}}
    write_abs(os.path.join(SCRATCH, "stage", "manifest.json"), json.dumps(manifest))
    code, text = mirror("create", "m2", "--only", "sub/one", "--batch", os.path.join(SCRATCH, "stage"))
    check("manifest batch applies in the mirror, not in live", code == 0 and open(os.path.join(HOME, "m2", "sub/two/b.ts")).read() == new and open(os.path.join(REPO, "sub/two/b.ts")).read() != new, text)
    manifest["sub/two/b.ts"]["base"] = "0" * 64
    write_abs(os.path.join(SCRATCH, "stage", "manifest.json"), json.dumps(manifest))
    code, text = mirror("create", "m3", "--only", "sub/one", "--batch", os.path.join(SCRATCH, "stage"))
    check("stale batch base is refused (exit 3)", code == 3, f"{code} {text}")
    code, text = mirror("create", "m4", "--only", "sub/one", "--batch", f"python3 -I -c \"open('sub/two/b.ts','w').write('cmd')\" && test {{root}} = {HOME}/m4")
    check("command batch runs inside the mirror with {root}", code == 0 and open(os.path.join(HOME, "m4", "sub/two/b.ts")).read() == "cmd", f"{code} {text}")
    code, text = mirror("drop", "m1", "m2", "m3", "m4", "--sync")
    check("drop removes the named mirrors", code == 0 and not [n for n in os.listdir(HOME) if not n.startswith(".")], text)
    code, text = mirror("drop", "--sync")
    check("drop without names is refused", code == 2, text)
    shutil.rmtree(SCRATCH, ignore_errors=True)
    print("FAILED: " + ", ".join(failures) if failures else "all laws hold")
    return 1 if failures else 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    finally:
        shutil.rmtree(SCRATCH, ignore_errors=True)
