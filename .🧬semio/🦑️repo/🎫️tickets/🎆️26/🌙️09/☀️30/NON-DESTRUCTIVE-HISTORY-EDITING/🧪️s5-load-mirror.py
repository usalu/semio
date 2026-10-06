#!/usr/bin/env python3
"""🪞️ S5-LOAD: builds a scratch mirror of the repository in which staged waves of `🧪️s5-load-waves.py` are applied, without
touching the tree — every directory on the way to a file a wave writes is a real directory of the mirror, everything else a
symlink into the repository. A bun law run inside the mirror imports the staged TypeScript and reads the staged corpora, so
a served wave is proven before its `serve` hold.

    python3 🧪️s5-load-mirror.py <wave | part:function> [...]   # applied in order; prints the mirror root

`part:<function>` applies one writes-returning function of the wave script that is no wave of its own (`part:merge_shell_module`).

The mirror lives under `🗑️generated/s5-load/mirror/` and is rebuilt on every call.

History (2026-10-05 06:23): the first version tested `exists()` bottom-up, which is true THROUGH a directory symlink, and
wrote five staged files into the tree. Every mutation now goes through `inside`, which resolves the parent's real path and
refuses anything outside the mirror; directories are made top-down; files are created with `O_EXCL | O_NOFOLLOW`; and the
bytes of every repository file a wave writes are compared before and after the build."""
import hashlib
import os
import pathlib
import shutil
import sys

TICKET = pathlib.Path(__file__).resolve().parent
REPO = TICKET.parents[6]
MIRROR = TICKET / "🗑️generated" / "s5-load" / "mirror"


def load_waves() -> dict:
    source = (TICKET / "🧪️s5-load-waves.py").read_text().replace("\nmain()\n", "\n")
    scope = {"__file__": str(TICKET / "🧪️s5-load-waves.py")}
    exec(compile(source, "waves", "exec"), scope)
    return scope


def inside(path: pathlib.Path) -> str:
    """🛡️ The path as a string, after proving that its parent really is a directory of the mirror."""
    root = os.path.realpath(MIRROR)
    parent = os.path.realpath(path.parent)
    if parent != root and not parent.startswith(root + os.sep):
        raise SystemExit(f"mirror: refusing {path}: its parent resolves to {parent}, outside the mirror")
    return str(path)


def real_directory(relative: pathlib.Path) -> None:
    """📁️ Makes every directory down to `relative` a real directory of the mirror whose other entries link to the repository's."""
    prefixes = [pathlib.Path(*relative.parts[:depth]) for depth in range(0, len(relative.parts) + 1)]
    for prefix in prefixes:
        target = MIRROR / prefix
        if prefix.parts:
            name = inside(target)
            if os.path.islink(name):
                os.unlink(name)
            if not os.path.lexists(name):
                os.mkdir(name)
            elif not os.path.isdir(name):
                raise SystemExit(f"mirror: {target} is not a directory")
        source = REPO / prefix
        for entry in sorted(source.iterdir()) if source.is_dir() else []:
            link = inside(target / entry.name)
            if not os.path.lexists(link):
                os.symlink(entry, link)


def digest(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else "absent"


def main() -> None:
    if not sys.argv[1:]:
        raise SystemExit(__doc__)
    scope = load_waves()
    if os.path.islink(MIRROR):
        raise SystemExit("mirror: the mirror root is a symlink")
    if MIRROR.exists():
        shutil.rmtree(MIRROR)
    MIRROR.mkdir(parents=True)
    overlay: dict = {}
    read_text, exists = pathlib.Path.read_text, pathlib.Path.exists
    pathlib.Path.read_text = lambda self, *args, **kwargs: overlay[self] if self in overlay else read_text(self, *args, **kwargs)
    pathlib.Path.exists = lambda self, *args, **kwargs: self in overlay or exists(self, *args, **kwargs)
    try:
        for wave in sys.argv[1:]:
            writes, _created = scope[wave[5:]]() if wave.startswith("part:") else scope["WAVES"][wave]()
            overlay.update(writes)
    finally:
        pathlib.Path.read_text, pathlib.Path.exists = read_text, exists
    before = {path: digest(path) for path in overlay}
    for path, text in overlay.items():
        relative = path.relative_to(REPO)
        real_directory(relative.parent)
        target = inside(MIRROR / relative)
        if os.path.lexists(target):
            os.unlink(target)
        descriptor = os.open(target, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o644)
        with os.fdopen(descriptor, "w") as handle:
            handle.write(text)
    changed = [str(path.relative_to(REPO)) for path in overlay if digest(path) != before[path]]
    if changed:
        raise SystemExit(f"mirror: REPOSITORY FILES CHANGED DURING THE BUILD: {changed}")
    print(f"{len(overlay)} staged file(s), repository bytes unchanged")
    print(MIRROR)


main()
