#!/usr/bin/env python3
"""Records the metrics git transcript from the committed recipe with a HERMETIC real git.

Usage: record_transcript.py [--write]   (default: dry run, prints the verdict and the sha map)

The recipe (`🧪️tests/🧫️fixtures/🌱️repository-recipe.json`) names the pinned `git.environment` and `git.config`.
Every git process here runs with: all inherited GIT_* variables dropped (except GIT_EXEC_PATH), HOME / USERPROFILE /
XDG_CONFIG_HOME / GIT_CONFIG_GLOBAL pointing at an empty private directory, the recipe's environment and `-c` config,
explicit author + committer identity and date. The repository is built twice under two different ambient configurations
(a quiet one and the recipe's `git.hostile` one: signing on, CRLF, latin1, merge.log, other identity, other timezone, GIT_CONFIG_COUNT, GIT_DIR);
the commit ids must be identical, otherwise the recording refuses.
"""
import json, os, shutil, subprocess, sys, tempfile
from pathlib import Path

MODULE = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/🧪️tests/🧫️fixtures")
RECIPE = json.loads((MODULE / "🌱️repository-recipe.json").read_text(encoding="utf8"))
PRETTY = "--pretty=format:COMMIT%x09%H%x09%aN%x09%aE%x09%at"
TAIL = ["--numstat", "--first-parent", "--reverse", PRETTY]


def hostile_ambient(root):
    home = Path(root) / "hostile-home"
    home.mkdir(parents=True)
    (home / ".gitconfig").write_text(RECIPE["git"]["hostile"]["config"])
    (home / "global").write_text(RECIPE["git"]["hostile"]["config"])
    return {"HOME": str(home), "USERPROFILE": str(home), "XDG_CONFIG_HOME": str(home), "GIT_CONFIG_GLOBAL": str(home / "global"), **RECIPE["git"]["hostile"]["environment"]}


def environment(home, ambient):
    env = {key: value for key, value in {**os.environ, **ambient}.items() if not key.startswith("GIT_") or key == "GIT_EXEC_PATH"}
    if "GIT_EXEC_PATH" in ambient or "GIT_EXEC_PATH" in os.environ:
        env["GIT_EXEC_PATH"] = ambient.get("GIT_EXEC_PATH", os.environ.get("GIT_EXEC_PATH"))
    home.mkdir(parents=True, exist_ok=True)
    (home / "gitconfig").write_text("")
    env.update(HOME=str(home), USERPROFILE=str(home), XDG_CONFIG_HOME=str(home), GIT_CONFIG_GLOBAL=str(home / "gitconfig"))
    env.update(RECIPE["git"]["environment"])
    return env


class Git:
    def __init__(self, repo, ambient):
        self.repo, self.home, self.ambient = repo, Path(str(repo) + ".home"), ambient

    def __call__(self, args, who=None, when=None, binary=False):
        env = environment(self.home, self.ambient)
        if who is not None:
            env.update(GIT_AUTHOR_NAME=who["name"], GIT_AUTHOR_EMAIL=who["email"], GIT_COMMITTER_NAME=who["name"], GIT_COMMITTER_EMAIL=who["email"], GIT_AUTHOR_DATE=f"{when} +0000", GIT_COMMITTER_DATE=f"{when} +0000")
        config = [part for key, value in RECIPE["git"]["config"].items() for part in ("-c", f"{key}={value}")]
        done = subprocess.run(["git", *config, *args], cwd=self.repo, env=env, capture_output=True, timeout=60)
        if done.returncode:
            raise SystemExit(f"git {args}: {done.stderr.decode()}")
        return done.stdout if binary else done.stdout.decode("utf8")


def build(root, ambient):
    repo = Path(root)
    repo.mkdir(parents=True)
    git = Git(repo, ambient)
    git(["init", "--quiet"])
    git(["symbolic-ref", "HEAD", f"refs/heads/{RECIPE['branch']}"])
    first = RECIPE["commits"][0]["author"]
    for key, value in (("user.name", first["name"]), ("user.email", first["email"]), ("core.quotepath", "true"), ("diff.renames", "true")):
        git(["config", key, value])
    shas = []
    for commit in RECIPE["commits"]:
        for change in commit["changes"]:
            path = change["path"]
            if change.get("delete"):
                git(["rm", "--quiet", "--", path])
                continue
            target = repo / path
            target.parent.mkdir(parents=True, exist_ok=True)
            if change.get("rename"):
                git(["mv", change["rename"], path])
            target.write_bytes(bytes(change["bytes"]) if "bytes" in change else change.get("content", "").encode("utf8"))
            git(["add", "--", path])
        git(["commit", "--quiet", "-m", commit["message"]], commit["author"], commit["when"])
        shas.append(git(["rev-parse", "HEAD"]).strip())
    side = RECIPE["side"]
    git(["checkout", "--quiet", "-b", side["branch"], shas[side["from"]]])
    (repo / side["path"]).write_bytes(side["content"].encode("utf8"))
    git(["add", "--", side["path"]])
    git(["commit", "--quiet", "-m", side["message"]], side["author"], side["when"])
    git(["checkout", "--quiet", RECIPE["branch"]])
    git(["merge", "--quiet", "--no-ff", "--no-edit", "-m", side["mergeMessage"], side["branch"]], side["mergeAuthor"], side["mergeWhen"])
    return git, shas + [git(["rev-parse", "HEAD"]).strip()]


def record(git, shas):
    branch = RECIPE["branch"]
    paths = lambda ref: [p for p in (git(["ls-files", "-z"]) if ref == "" else git(["ls-tree", "-r", "--name-only", "-z", ref])).split("\0") if p]
    logs = {"": git(["log", "--no-merges", *TAIL]), branch: git(["log", "--no-merges", *TAIL, branch]), "🔀️with-merges": git(["log", *TAIL, branch])}
    tracked, blobs = {}, {}
    for ref in [""] + shas:
        tracked[ref] = paths(ref)
        for path in tracked[ref]:
            raw = git(["show", f"{ref}:{path}"], binary=True) if ref else (git.repo / path).read_bytes()
            try:
                blobs[f"{ref}\u0001{path}"] = raw.decode("utf8")
            except UnicodeDecodeError:
                pass
    return {"id": "metrics-recorded-repository", "logs": logs, "tracked": tracked, "blobs": dict(sorted(blobs.items()))}


def main():
    write = "--write" in sys.argv
    scratch = tempfile.mkdtemp(prefix="metrics-d-record-")
    try:
        quiet, shas = build(Path(scratch) / "quiet", {})
        hostile, hostile_shas = build(Path(scratch) / "hostile", hostile_ambient(scratch))
        if shas != hostile_shas:
            raise SystemExit(f"NOT HERMETIC: {shas} != {hostile_shas}")
        print("hermetic: identical commit ids under a quiet and a hostile ambient:", *shas, sep="\n  ")
        fresh = record(quiet, shas)
        again = record(hostile, hostile_shas)
        if fresh != again:
            raise SystemExit("NOT HERMETIC: the recorded transcripts differ")
        print("git", quiet(["--version"]).strip())
        old = json.loads((MODULE / "🎞️git-transcript.json").read_text(encoding="utf8"))
        starts = lambda log: [line.split("\t")[1] for line in log.split("\n") if line.startswith("COMMIT\t")]
        mapping = dict(zip(starts(old["logs"]["🔀️with-merges"]), starts(fresh["logs"]["🔀️with-merges"])))
        remap = lambda text: [text := text.replace(was, now) for was, now in mapping.items()] and text
        print("sha map:", *[f"{was[:8]} -> {now[:8]}" for was, now in mapping.items()], sep="\n  ")
        print("fresh transcript == old transcript with remapped shas (as data):", fresh == json.loads(remap(json.dumps(old, ensure_ascii=False))))
        rendered = json.dumps(fresh, indent=2, ensure_ascii=False) + "\n"
        if not write:
            return
        (MODULE / "🎞️git-transcript.json").write_text(rendered, encoding="utf8")
        for name in ("📤️numstat-deltas.json", "📤️time-buckets.json"):
            path = MODULE / name
            text = path.read_text(encoding="utf8")
            path.write_text(remap(text), encoding="utf8")
            print("remapped", name)
    finally:
        shutil.rmtree(scratch, ignore_errors=True)


main()
