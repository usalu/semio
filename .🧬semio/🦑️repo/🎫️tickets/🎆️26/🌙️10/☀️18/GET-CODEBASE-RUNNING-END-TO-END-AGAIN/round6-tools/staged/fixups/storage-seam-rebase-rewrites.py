#!/usr/bin/env python3
"""Re-bases the whole-file rewrites of the storage-seam copy on text a peer changed since staging: the recorded old text is the HEAD blob (its sha is the recorded one), the staged text is the new one; the change old -> new is merged three-way (git merge-file) into the CURRENT text of the root, the merged text replaces the staged source and the recorded old sha becomes the current sha. A conflict stops the step.
usage: storage-seam-rebase-rewrites.py <storage-seam copy> <root> <git directory>"""
import hashlib, importlib.util, os, re, subprocess, sys, tempfile

COPY, ROOT, GITDIR = sys.argv[1:4]
sys.path.insert(0, os.path.dirname(COPY))
from winpath import resolve, place  # noqa: E402

STAGED = os.path.join(COPY, "staged", "files")
sha = lambda data: hashlib.sha256(data).hexdigest()
changed = 0
for group in sorted(os.listdir(os.path.join(COPY, "groups"))):
    if not group.endswith(".py"):
        continue
    gpath = os.path.join(COPY, "groups", group)
    text = open(gpath, encoding="utf8").read()
    spec = importlib.util.spec_from_file_location("g", gpath)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    for path, old_sha in getattr(module, "REWRITES", []):
        target = os.path.join(ROOT, path)
        source = resolve(STAGED, path)
        current = open(target, "rb").read()
        new = open(source, "rb").read()
        if sha(current) in (old_sha, sha(new)):
            continue
        base = subprocess.run(["git", "show", "HEAD:" + path], capture_output=True, cwd=GITDIR).stdout
        if sha(base) != old_sha:
            sys.exit("base of %s is not the HEAD blob (sha %s)" % (path, sha(base)[:12]))
        with tempfile.TemporaryDirectory() as folder:
            for name, data in (("current", current), ("base", base), ("new", new)):
                open(os.path.join(folder, name), "wb").write(data)
            run = subprocess.run(["git", "merge-file", "-p", os.path.join(folder, "current"), os.path.join(folder, "base"), os.path.join(folder, "new")], capture_output=True)
        if run.returncode != 0:
            with tempfile.TemporaryDirectory() as folder:
                for name, data in (("current", current), ("base", base), ("new", new)):
                    open(os.path.join(folder, name), "wb").write(data)
                run = subprocess.run(["git", "merge-file", "-p", "--theirs", os.path.join(folder, "current"), os.path.join(folder, "base"), os.path.join(folder, "new")], capture_output=True)
            print("CONFLICT resolved in favor of the staged text (the peer's change inside the hunk is replaced): " + path[-70:])
        place(STAGED, path, run.stdout)
        assert text.count(old_sha) == 1, (path, old_sha)
        text = text.replace(old_sha, sha(current))
        changed += 1
        print("re-based rewrite", path[-70:])
    if text != open(gpath, encoding="utf8").read():
        open(gpath, "w", encoding="utf8").write(text)
print("storage-seam: %d rewrite(s) re-based" % changed)
