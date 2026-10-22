#!/usr/bin/env python3
"""🧪️ Self-test of make.py and apply.py on a scratch repository (stdlib only; never touches the checkout).
Proves: the codemod fails closed (missing root, empty list, rule without a match, residue after the rows), apply is a dry run by default, refuses on drift, is all-or-nothing (a failing write restores every file), is idempotent and undoable.
usage: selftest.py [--keep]   exit 0 = every check passed"""
import json
import os
import shutil
import stat
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
PY = sys.executable
failures = []


def check(name, condition, detail=""):
    print(("ok   " if condition else "FAIL ") + name + ((" :: " + detail) if detail and not condition else ""))
    if not condition:
        failures.append(name)


def run(*argv, cwd=None):
    return subprocess.run([PY, "-I", *argv], capture_output=True, text=True, cwd=cwd)


def git(root, *argv):
    subprocess.run(["git", "-C", root, *argv], check=True, capture_output=True)


work = tempfile.mkdtemp(prefix="tdm-selftest-")
repo = os.path.join(work, "repo")
os.makedirs(repo)
files = {
    "docs/a.md": "# A\n\n```sh\nbun run dashboard run x:y -- quick\ncargo test\n```\n",
    "app/📋️project.json": '{"targets":{"a":{"command":["bun","./📜️script.ts","build"]},"b":{"command":"repo run c"}}}\n',
    ".🧬semio/🦑️repo/🎫️tickets/x/n.md": "```sh\nbun x\n```\n",
}
for path, body in files.items():
    full = os.path.join(repo, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    open(full, "w", encoding="utf-8").write(body)
git(repo, "init", "-q")
git(repo, "add", "-A")
scope = {
    "version": 1, "summary": {"en": "x", "de": "y"}, "roots": ["docs", "app"], "groups": ["docs", "json-commands"],
    "rules": [
        {"id": "dash", "group": "docs", "files": ["**/*.md"], "pattern": "\\bbun run dashboard run ", "repl": "repo run ", "note": {"en": "x", "de": "y"}},
        {"id": "cargo", "group": "docs", "files": ["**/*.md"], "pattern": "(?m)^cargo test$", "repl": "repo cargo --store=shared --configuration=prototype -- test", "note": {"en": "x", "de": "y"}},
        {"id": "late", "group": "docs", "optional": True, "files": ["**/*.md"], "pattern": "(?<![\\w/.-])bun repo(?![\\w-])", "repl": "repo", "note": {"en": "x", "de": "y"}},
        {"id": "argv", "group": "json-commands", "files": ["**/*.json"], "pattern": "(\\[\\s*)\"bun\"(\\s*,\\s*\"[^\"\\n]+\\.(?:ts|mjs|js)\")", "repl": "\\1\"repo\"\\2", "note": {"en": "x", "de": "y"}},
    ],
}
literals = {"version": 1, "rows": []}
scope_file = os.path.join(work, "scope.json")
literal_file = os.path.join(work, "literals.json")
json.dump(scope, open(scope_file, "w", encoding="utf-8"), ensure_ascii=False)
json.dump(literals, open(literal_file, "w", encoding="utf-8"))
staged = os.path.join(work, "staged")
base = [os.path.join(HERE, "make.py"), "--root", repo, "--scope", scope_file, "--literals", literal_file, "--out", staged, "--no-law"]

result = run(*base)
check("make builds the staged set", result.returncode == 0, result.stderr[-300:])
manifest = json.load(open(os.path.join(staged, "manifest.json"), encoding="utf-8"))
check("rows cover both roots", {row["path"] for row in manifest["rows"]} == {"docs/a.md", "app/📋️project.json"}, str(manifest["rows"]))
check("the ticket folder is never a row", all("tickets" not in row["path"] for row in manifest["rows"]))

broken = dict(scope, roots=["docs", "nope"])
json.dump(broken, open(scope_file, "w", encoding="utf-8"), ensure_ascii=False)
result = run(*base)
check("a missing root refuses", result.returncode == 1 and "holds no tracked file" in result.stderr, result.stderr[-200:])

json.dump(dict(scope, rules=scope["rules"] + [{"id": "idle", "group": "docs", "files": ["**/*.md"], "pattern": "never-there", "repl": "x", "note": {"en": "x", "de": "y"}}]), open(scope_file, "w", encoding="utf-8"), ensure_ascii=False)
result = run(*base)
check("a rule that matches nothing refuses", result.returncode == 1 and "matched nothing" in result.stderr, result.stderr[-200:])

json.dump(dict(scope, rules=scope["rules"][:3]), open(scope_file, "w", encoding="utf-8"), ensure_ascii=False)
result = run(*base)
check("residue after the rows refuses", result.returncode == 1 and "residue" in result.stderr, result.stderr[-300:])

empty = os.path.join(work, "empty.z")
open(empty, "wb").write(b"")
json.dump(scope, open(scope_file, "w", encoding="utf-8"), ensure_ascii=False)
result = run(*base, "--list", empty)
check("an empty file list refuses", result.returncode == 1 and "empty" in result.stderr, result.stderr[-200:])

result = run(*base)
check("rebuild after the negative controls", result.returncode == 0, result.stderr[-300:])
manifest = json.load(open(os.path.join(staged, "manifest.json"), encoding="utf-8"))
check("an optional rule becomes a sweep, not a per-file row", [s["id"] for s in manifest["sweeps"]] == ["late"] and all(r["id"] != "late" for r in manifest["rows"]), str(manifest["sweeps"]))
late = os.path.join(repo, "docs/a.md")
open(late, "a", encoding="utf-8").write("\n```sh\nbun repo run q\n```\n")
files["docs/a.md"] = open(late, encoding="utf-8").read()

apply = [os.path.join(HERE, "apply.py"), "--root", repo, "--staged", staged, "--backup-dir", work]
before = {p: open(os.path.join(repo, p), encoding="utf-8").read() for p in files}
result = run(*apply, "apply", "--allow-frozen")
check("apply is a dry run by default", result.returncode == 1 and "DRY RUN" in result.stdout and all(open(os.path.join(repo, p), encoding="utf-8").read() == before[p] for p in files), result.stdout[-300:])
result = run(*apply, "diff")
check("diff prints the unified change", result.returncode == 0 and "+```sh" not in result.stdout and "-bun run dashboard" in result.stdout and "+repo run x:y" in result.stdout, result.stdout[-400:])

drifted = os.path.join(repo, "docs/a.md")
open(drifted, "w", encoding="utf-8").write(before["docs/a.md"].replace("bun run dashboard run x:y", "bun run dashboard run z:w -- a\nbun run dashboard run x:y"))
result = run(*apply, "apply", "--write")
check("drift refuses and writes nothing", result.returncode == 2 and "DRIFT" in result.stdout and open(os.path.join(repo, "app/📋️project.json"), encoding="utf-8").read() == before["app/📋️project.json"], result.stdout[-300:])
open(drifted, "w", encoding="utf-8").write(before["docs/a.md"])

locked = os.path.join(repo, "docs")
os.chmod(locked, stat.S_IRUSR | stat.S_IXUSR)
result = run(*apply, "apply", "--write")
os.chmod(locked, stat.S_IRWXU)
restored = all(open(os.path.join(repo, p), encoding="utf-8").read() == before[p] for p in files)
leftovers = [name for _, _, names in os.walk(repo) for name in names if name.endswith(".tdm.tmp")]
check("a failing write restores every file (all-or-nothing)", restored and not leftovers and result.returncode != 0, "restored=%s leftovers=%s rc=%s %s" % (restored, leftovers, result.returncode, result.stderr[-200:]))

result = run(*apply, "apply", "--write", "--inject-failure-after", "1")
restored = all(open(os.path.join(repo, p), encoding="utf-8").read() == before[p] for p in files)
check("a failure between renames restores the renamed file", restored and result.returncode != 0 and "injected failure" in result.stderr, "restored=%s rc=%s %s" % (restored, result.returncode, result.stderr[-200:]))

result = run(*apply, "apply", "--write")
check("apply --write writes", result.returncode == 0 and "written 2 file(s)" in result.stdout, result.stdout[-300:])
after = {p: open(os.path.join(repo, p), encoding="utf-8").read() for p in files}
check("the sweep rewrote text staged after the build", "\nrepo run q\n" in after["docs/a.md"] and "bun repo" not in after["docs/a.md"], after["docs/a.md"])
check("the rows rewrote the commands", "repo run x:y -- quick" in after["docs/a.md"] and "repo cargo --store=shared" in after["docs/a.md"] and '"repo","./📜️script.ts"' in after["app/📋️project.json"].replace(" ", ""), after["docs/a.md"])
check("the ticket folder stayed", after[".🧬semio/🦑️repo/🎫️tickets/x/n.md"] == before[".🧬semio/🦑️repo/🎫️tickets/x/n.md"])
result = run(*apply, "check")
check("a second check reports everything applied", result.returncode == 0 and "pending 0" in result.stdout, result.stdout[-200:])
backups = sorted(name for name in os.listdir(work) if name.startswith("backup-"))
result = run(*apply, "--undo", os.path.join(work, backups[-1]), "--write")
check("undo restores the originals", result.returncode == 0 and all(open(os.path.join(repo, p), encoding="utf-8").read() == before[p] for p in files), result.stdout[-200:])
checker = run(os.path.join(HERE, "law", "checker.py"), "--root", repo, "--patterns", os.path.join(HERE, "law", "patterns.json"))
check("the law reports the originals and skips the ticket folder", checker.returncode == 1 and "bun" in checker.stdout and "tickets" not in checker.stdout, checker.stdout[-300:])

if "--keep" not in sys.argv:
    shutil.rmtree(work, ignore_errors=True)
print("failures: %d" % len(failures))
sys.exit(1 if failures else 0)
