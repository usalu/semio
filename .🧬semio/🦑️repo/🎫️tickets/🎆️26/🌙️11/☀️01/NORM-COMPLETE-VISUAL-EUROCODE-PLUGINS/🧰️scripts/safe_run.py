"""🛡️ Safe runner for every generator script: python3 safe_run.py [--check] [--no-lose] SCRIPT [ARGS...]

Runs SCRIPT as __main__ with write guards on everything under the repository (the ticket folder stays free):
- an existing file is only rewritten when the new content loses none of the old lines (a line counts as kept when an added line contains it, so an appended comma or an inserted field is fine); otherwise the write is skipped and reported as `kept`;
- creating files is allowed; deleting, renaming or moving files and directories is skipped and reported (TH_FORCE=1 lifts it);
- `--check` is a faithful dry run: every write lands in memory (reads see it), nothing touches the disk, the report lists what would be created or changed and the exit status is 1 when there is anything, 0 otherwise. A script with its own native `--check` gets the flag passed through unchanged.
Limits of the memory overlay: directory listings and globbing do not see files created in the same run. Docs: README-generators.md."""
import builtins, io, os, pathlib, runpy, shutil, sys
from collections import Counter

SEMIO = "/home/deck/semio"
TICKET = SEMIO + "/.🧬semio/🦑️repo/🎫️tickets"
CHECK = False
FORCE = bool(os.environ.get("TH_FORCE"))
overlay, created, changed, kept, blocked = {}, [], [], [], []
sys.argv_all = []
_open, _io_open = builtins.open, io.open
_exists, _isfile, _isdir = os.path.exists, os.path.isfile, os.path.isdir


def guarded(path):
    try:
        p = os.path.abspath(os.fspath(path))
    except TypeError:
        return None
    return p if p.startswith(SEMIO + "/") and not p.startswith(TICKET + "/") and "/node_modules/" not in p else None


def subsequence(small, big):
    it = iter(big)
    return all(c in it for c in small)


INT = __import__("re").compile(r"(?<![\w.])\d+(?![\w.])")


def bumped(old, new):
    """Same line, only whole-number counters (table counts, indices, tags) that did not decrease."""
    if INT.sub("#", old) != INT.sub("#", new):
        return False
    return all(int(x) <= int(y) for x, y in zip(INT.findall(old), INT.findall(new)))


def lost(old, new):
    """Old lines (stripped, trailing comma ignored) that no added line carries: contained verbatim, or kept as a subsequence of an added line (text inserted into the line)."""
    norm = lambda t: Counter(l.strip().rstrip(",") for l in t.splitlines() if l.strip())
    a, b = norm(old), norm(new)
    came = list((b - a))
    out = []
    for l in (a - b):
        if l in ("{", "}", "[", "]") or any(l in c for c in came):
            continue
        head = l[:10]
        if any(len(c) > len(l) and head in c and subsequence(l, c) for c in came) or any(bumped(l, c) for c in came if len(c) - len(l) < 12):
            continue
        out.append(l)
    return out


def current(p):
    if p in overlay:
        return overlay[p]
    if _exists(p) and _isfile(p):
        with _io_open(p, "rb") as f:
            data = f.read()
        try:
            return data.decode("utf-8")
        except UnicodeDecodeError:
            return data
    return None


DIFFS = {}


def finish(p, new):
    old = current(p)
    if "--diff" in sys.argv_all and isinstance(old, str) and isinstance(new, str) and old != new:
        import difflib
        DIFFS[p] = (DIFFS.get(p, (old, 0))[0], new)
    if old == new:
        return
    if old is None:
        created.append(p)
    elif isinstance(old, str) and isinstance(new, str) and (gone := lost(old, new)):
        kept.append((p, len(gone), gone[0][:100]))
        return
    else:
        changed.append(p)
    overlay[p] = new


_makedirs = os.makedirs


def commit():
    """All-or-nothing: the buffered writes reach the disk only when none was refused, so a partly applicable run can never leave a half-applied tree behind (which would make the next run apply the rest again)."""
    for p, new in overlay.items():
        _makedirs(os.path.dirname(p), exist_ok=True)
        with _io_open(p, "wb") as f:
            f.write(new.encode("utf-8") if isinstance(new, str) else new)


class Text(io.StringIO):
    def __init__(self, path, initial="", encoding="utf-8"):
        super().__init__(initial)
        self.seek(0, 2)
        self._path, self._done = path, False

    def close(self):
        if not self._done:
            self._done = True
            finish(self._path, self.getvalue())
        super().close()

    def __del__(self):
        try:
            self.close()
        except Exception:
            pass


class Binary(io.BytesIO):
    def __init__(self, path, initial=b""):
        super().__init__(initial)
        self.seek(0, 2)
        self._path, self._done = path, False

    def close(self):
        if not self._done:
            self._done = True
            finish(self._path, self.getvalue())
        super().close()

    def __del__(self):
        try:
            self.close()
        except Exception:
            pass


def safe_open(file, mode="r", *args, **kwargs):
    p = guarded(file) if isinstance(file, (str, os.PathLike)) else None
    if p is None:
        return _io_open(file, mode, *args, **kwargs)
    writing = any(c in mode for c in "wax+")
    binary = "b" in mode
    if not writing:
        if p in overlay:
            v = overlay[p]
            return io.BytesIO(v if isinstance(v, bytes) else v.encode()) if binary else io.StringIO(v if isinstance(v, str) else v.decode())
        return _io_open(file, mode, *args, **kwargs)
    old = current(p) if ("a" in mode or "+" in mode) else None
    if binary:
        return Binary(p, old if isinstance(old, bytes) else (old or "").encode() if old else b"")
    return Text(p, old if isinstance(old, str) else "")


def deny(what):
    def inner(path, *a, **k):
        if guarded(path) is None or FORCE:
            return what(path, *a, **k)
        blocked.append(os.fspath(path))
    return inner


def deny2(what):
    def inner(src, dst, *a, **k):
        if (guarded(src) is None and guarded(dst) is None) or FORCE:
            return what(src, dst, *a, **k)
        blocked.append("%s -> %s" % (os.fspath(src), os.fspath(dst)))
    return inner


def install():
    builtins.open = io.open = safe_open
    os.path.exists = lambda p: (guarded(p) in overlay) or _exists(p)
    os.path.isfile = lambda p: (guarded(p) in overlay) or _isfile(p)
    global _makedirs
    _makedirs, _mkdir = os.makedirs, os.mkdir
    os.makedirs = lambda p, *a, **k: None if guarded(p) else _makedirs(p, *a, **k)
    os.mkdir = lambda p, *a, **k: None if guarded(p) else _mkdir(p, *a, **k)
    pathlib.Path.mkdir = lambda self, *a, **k: None if guarded(self) else _mkdir(self)
    os.remove = os.unlink = deny(os.remove)
    os.rmdir = deny(os.rmdir)
    shutil.rmtree = deny(shutil.rmtree)
    os.rename = deny2(os.rename)
    os.replace = deny2(os.replace)
    shutil.move = deny2(shutil.move)


def report():
    for p in created:
        print("safe_run: %s %s" % ("would create" if CHECK else "creates", p[len(SEMIO) + 1:]))
    for p in changed:
        print("safe_run: %s %s" % ("would change" if CHECK else "changes", p[len(SEMIO) + 1:]))
    if "--diff" in sys.argv_all:
        import difflib
        for p, (old, new) in DIFFS.items():
            print("--- " + p[len(SEMIO) + 1:])
            print("".join(list(difflib.unified_diff(old.splitlines(True), new.splitlines(True), "tree", "run", n=0))[:80]))
    for p, n, first in kept:
        print("safe_run: kept %s (the new content would lose %d lines, e.g. %r)" % (p[len(SEMIO) + 1:], n, first))
    for p in blocked:
        print("safe_run: skipped delete/rename %s" % p)
    sys.stdout.flush()


def main():
    global CHECK
    argv = sys.argv[1:]
    sys.argv_all = argv
    CHECK = "--check" in argv
    argv = [a for a in argv if a not in ("--no-lose", "--diff")]
    script = next(a for a in argv if not a.startswith("--"))
    rest = argv[argv.index(script) + 1:]
    path = os.path.abspath(script)
    native = '"--check"' in _io_open(path, encoding="utf-8").read()
    if CHECK and not native:
        argv.remove("--check")
        rest = argv[argv.index(script) + 1:]
    if CHECK and native:
        CHECK = False
        rest = ["--check"] + [a for a in rest if a != "--check"]
    install()
    sys.argv = [path] + rest
    sys.path.insert(0, os.path.dirname(path))
    code = 0
    try:
        runpy.run_path(path, run_name="__main__")
    except SystemExit as e:
        code = e.code if isinstance(e.code, int) else (0 if e.code is None else 1)
        if isinstance(e.code, str):
            print(e.code, file=sys.stderr)
    import gc
    gc.collect()
    report()
    if CHECK:
        sys.exit(1 if (created or changed) and not kept else 0)
    if kept:
        print("safe_run: aborted, nothing written (%d refused write(s); the script is not additive against this tree)" % len(kept))
        sys.exit(3)
    commit()
    sys.exit(code)


if __name__ == "__main__":
    main()
