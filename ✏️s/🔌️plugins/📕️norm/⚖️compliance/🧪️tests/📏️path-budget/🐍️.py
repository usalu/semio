"""📏 No path below the norm plugin exceeds the byte budget, measured as UTF-8 bytes of the path relative to the repository root.

`python -I 🐍️.py` exits non-zero and lists the longest offenders with the byte cost of every segment.

The repository limit is 256 characters for native Windows, macOS and Linux checkouts. UTF-8 bytes bound the UTF-16 units and code points of the same path from above, and a path relative to the repository root excludes the machine-specific checkout prefix, so one number holds everywhere. `BUDGET` keeps 16 bytes of headroom for the checkout prefix and the next kind.

Two independent walkers (`os.scandir` and `pathlib.rglob`) must agree on the path set, and entries that vanish mid-walk (concurrent writers) are tolerated: a disagreement is re-walked until both walkers see the same surviving paths, and only a disagreement about a path that still exists fails. The 240-byte rule is checked on that stable snapshot. Further, `git ls-files` must find every tracked path inside it, so a walker bug cannot hide a long path. Build outputs and dependency folders are not part of the tree.
"""
import os, pathlib, subprocess, sys

BUDGET = 240
LIMIT = 256
SKIPPED = {"target", "node_modules", "__pycache__", ".git", ".venv"}
NORM = pathlib.Path(__file__).resolve().parents[3]
REPO = next(parent for parent in NORM.parents if (parent / "Directory.Build.props").is_file())
SCOPE = NORM.relative_to(REPO).as_posix()


def measure(path):
    return len(path.encode("utf-8"))


def scandir_paths(directory, prefix, scandir=os.scandir):
    found = []
    try:
        with scandir(directory) as entries:
            for entry in entries:
                if entry.name in SKIPPED:
                    continue
                path = prefix + "/" + entry.name
                found.append(path)
                try:
                    descend = entry.is_dir(follow_symlinks=False)
                except FileNotFoundError:
                    continue
                if descend:
                    found.extend(scandir_paths(entry.path, path, scandir))
    except (FileNotFoundError, NotADirectoryError):
        pass
    return found


def rglob_paths(root=None, repo=None):
    root, repo = root or NORM, repo or REPO
    found = []
    try:
        for path in root.rglob("*"):
            relative = path.relative_to(repo)
            if not SKIPPED.intersection(relative.parts):
                found.append(relative.as_posix())
    except (FileNotFoundError, NotADirectoryError):
        pass
    return found


def stable_snapshot(walk, glob, exists, attempts=5):
    """Both walkers' agreed path set with vanished entries dropped; `None` when they still disagree about a surviving path after `attempts` walks."""
    for _ in range(attempts):
        walked, globbed = set(walk()), set(glob())
        if not any(exists(p) for p in walked ^ globbed):
            return sorted(p for p in walked & globbed if exists(p))
    return None


def tracked_paths():
    try:
        out = subprocess.run(["git", "ls-files", "-z", "--", SCOPE], cwd=REPO, capture_output=True, check=True).stdout
    except (OSError, subprocess.CalledProcessError):
        return None
    return [p.decode("utf-8") for p in out.split(b"\0") if p]


def costs(path):
    return [(len(part.encode("utf-8")), part) for part in path.split("/")]


def self_test():
    assert measure("🧬️") == 7 and measure("✅") == 3 and measure("a") == 1
    assert measure("x" * 240) == BUDGET and measure("🧬️" * 35) > BUDGET
    assert measure("é" * 120) == 240 and len("é" * 120) == 120
    assert BUDGET + 16 == LIMIT
    vanishing_test()


def vanishing_test():
    import tempfile
    with tempfile.TemporaryDirectory() as tmp:
        root = pathlib.Path(tmp) / "norm"
        (root / "a").mkdir(parents=True)
        for name in ("a/keep", "a/gone", "b"):
            (root / name).write_text("")
        real = os.scandir

        def racing(directory):
            entries = list(real(directory))
            for gone in ("gone", "a"):
                if (pathlib.Path(directory) / gone).is_file() and gone == "gone":
                    (pathlib.Path(directory) / gone).unlink()
            return _Listing(entries)

        class _Listing(list):
            def __enter__(self):
                return self

            def __exit__(self, *exc):
                return False

        racing_paths = scandir_paths(root, "norm", racing)
        assert "norm/a/keep" in racing_paths and "norm/b" in racing_paths
        assert scandir_paths(root / "missing", "norm") == []
        (root / "a" / "gone").write_text("")
        calls = []

        def walk():
            calls.append(1)
            return scandir_paths(root, "norm")

        def glob():
            found = rglob_paths(root, pathlib.Path(tmp))
            if len(calls) == 1:
                (root / "a" / "gone").unlink()
            return found

        exists = lambda p: os.path.lexists(pathlib.Path(tmp) / p)
        snapshot = stable_snapshot(walk, glob, exists)
        assert snapshot == ["norm/a", "norm/a/keep", "norm/b"], snapshot
        assert stable_snapshot(lambda: ["norm/x"], lambda: [], lambda p: True) is None


def main():
    self_test()
    exists = lambda p: os.path.lexists(REPO / p)
    walked = stable_snapshot(lambda: scandir_paths(NORM, SCOPE), rglob_paths, exists)
    if walked is None:
        print("walkers disagree about paths that still exist after repeated walks")
        return 2
    tracked = tracked_paths()
    seen = set(walked)
    missing = [p for p in (tracked or []) if p not in seen and not SKIPPED.intersection(p.split("/")) and (REPO / p).exists()]
    if missing:
        print("tracked paths the walkers missed:", missing[:5])
        return 2
    over = sorted(((measure(p), p) for p in walked if measure(p) > BUDGET), reverse=True)
    longest = max(walked, key=measure)
    print("paths", len(walked), "longest", measure(longest), "budget", BUDGET, "over", len(over), "git-tracked", "n/a" if tracked is None else len(tracked))
    for size, path in over[:20]:
        print("over budget", size, "bytes", path)
    if over:
        print("segment bytes of the longest:", " ".join("%d:%s" % cost for cost in costs(over[0][1])))
    return 1 if over else 0


if __name__ == "__main__":
    sys.exit(main())
