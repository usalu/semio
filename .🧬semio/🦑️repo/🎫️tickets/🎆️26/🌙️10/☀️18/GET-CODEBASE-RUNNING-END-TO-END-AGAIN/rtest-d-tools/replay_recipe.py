#!/usr/bin/env python3
"""Replays the committed metrics repository recipe against the real git binary (same steps as the TS oracle) and prints the commit shas."""
import json, os, subprocess, sys, tempfile, shutil
from pathlib import Path
recipe = json.loads(Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧪️tests/🧫️fixtures/🌱️repository-recipe.json").read_text(encoding="utf8"))
def git(cwd, args, author=None, when=None):
    env = dict(os.environ)
    if when is not None:
        env.update(GIT_AUTHOR_DATE=f"{when} +0000", GIT_COMMITTER_DATE=f"{when} +0000")
    if author:
        env.update(GIT_AUTHOR_NAME=author["name"], GIT_AUTHOR_EMAIL=author["email"], GIT_COMMITTER_NAME=author["name"], GIT_COMMITTER_EMAIL=author["email"])
    r = subprocess.run(["git", "-c", "commit.gpgsign=false"] + args, timeout=30, cwd=cwd, env=env, capture_output=True)
    if r.returncode: raise SystemExit(r.stderr.decode())
    return r.stdout.decode("utf8")
d = tempfile.mkdtemp()
try:
    git(d, ["init", "--quiet", "--initial-branch=" + recipe["branch"]])
    a = recipe["commits"][0]["author"]
    git(d, ["config", "user.name", a["name"]]); git(d, ["config", "user.email", a["email"]]); git(d, ["config", "core.quotepath", "true"]); git(d, ["config", "diff.renames", "true"])
    for c in recipe["commits"]:
        for ch in c["changes"]:
            if ch.get("delete"):
                git(d, ["rm", "--quiet", "--", ch["path"]]); continue
            t = Path(d) / ch["path"]; t.parent.mkdir(parents=True, exist_ok=True)
            if ch.get("rename"): git(d, ["mv", ch["rename"], ch["path"]])
            if "bytes" in ch: t.write_bytes(bytes(ch["bytes"]))
            else: t.write_text(ch.get("content", ""), encoding="utf8")
            git(d, ["add", "--", ch["path"]])
        git(d, ["commit", "--quiet", "-m", c["message"]], c["author"], c["when"])
        print(git(d, ["rev-parse", "HEAD"]).strip(), c["message"])
    print(git(d, ["--version"]).strip(), git(d, ["cat-file", "-p", "HEAD~4"]))
finally:
    shutil.rmtree(d, ignore_errors=True)
