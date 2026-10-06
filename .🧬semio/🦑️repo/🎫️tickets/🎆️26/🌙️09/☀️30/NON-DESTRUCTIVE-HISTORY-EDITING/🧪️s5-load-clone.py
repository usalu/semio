#!/usr/bin/env python3
"""🧬️ S5-LOAD: an isolated copy-on-write clone of the repository's sources with staged waves of `🧪️s5-load-waves.py` applied, so
never-compiled Rust can be type-checked before it reaches the landing train. Every file is its own APFS clone (`clonefile`),
never a link: nothing written in the clone can reach the tree. The clone must lie OUTSIDE the repository (nx and cargo discover
projects by walking it) and is built with private `CARGO_TARGET_DIR` AND `CARGO_BUILD_BUILD_DIR` (rule 66; a clone on the shared
build dir poisons it).

    python3 🧪️s5-load-clone.py <destination outside the repository> [<wave> ...]

Then, in the clone: `CARGO_TARGET_DIR=<d>-target CARGO_BUILD_BUILD_DIR=<d>-build CARGO_BUILD_JOBS=3 cargo check --offline -p … --lib`."""
import ctypes
import os
import pathlib
import shutil
import subprocess
import sys

TICKET = pathlib.Path(__file__).resolve().parent
REPO = TICKET.parents[6]


def main() -> None:
    if len(sys.argv) < 2:
        raise SystemExit(__doc__)
    destination = pathlib.Path(os.path.realpath(sys.argv[1]))
    repo = pathlib.Path(os.path.realpath(REPO))
    if destination == repo or repo in destination.parents or destination in repo.parents:
        raise SystemExit("clone: the destination must lie outside the repository")
    if destination.exists():
        shutil.rmtree(destination)
    destination.mkdir(parents=True)
    tracked = subprocess.run(["git", "ls-files", "-z", "-co", "--exclude-standard"], cwd=repo, capture_output=True, check=True).stdout.split(b"\0")
    ignored = subprocess.run(["git", "ls-files", "-z", "-oi", "--exclude-standard", "--", "🧰️framework", "✏️s", "🌎️hub"], cwd=repo, capture_output=True, check=True).stdout.split(b"\0")
    skip = (b"/node_modules/", b"/dist/", b"/target/", b"/.vite/", b"/.nx/")
    names = [name for name in tracked if name] + [name for name in ignored if name and not any(part in b"/" + name for part in skip) and name.endswith((b".rs", b".toml", b".json", b".wit", b".lock"))]
    libc = ctypes.CDLL(None, use_errno=True)
    made = set()
    cloned = 0
    for name in names:
        source = os.path.join(os.fsencode(repo), name)
        target = os.path.join(os.fsencode(destination), name)
        if not os.path.lexists(source):
            continue
        parent = os.path.dirname(target)
        if parent not in made:
            os.makedirs(parent, exist_ok=True)
            made.add(parent)
        if os.path.lexists(target):
            continue
        if libc.clonefile(source, target, 1) != 0:
            error = ctypes.get_errno()
            if os.path.isdir(source):
                continue
            raise SystemExit(f"clone: clonefile failed for {os.fsdecode(name)}: errno {error}")
        cloned += 1
    source = (TICKET / "🧪️s5-load-waves.py").read_text().replace("\nmain()\n", "\n")
    scope = {"__file__": str(TICKET / "🧪️s5-load-waves.py")}
    exec(compile(source, "waves", "exec"), scope)
    overlay: dict = {}
    read_text, exists = pathlib.Path.read_text, pathlib.Path.exists
    pathlib.Path.read_text = lambda self, *args, **kwargs: overlay[self] if self in overlay else read_text(self, *args, **kwargs)
    pathlib.Path.exists = lambda self, *args, **kwargs: self in overlay or exists(self, *args, **kwargs)
    try:
        for wave in sys.argv[2:]:
            writes, _created = scope["WAVES"][wave]()
            overlay.update(writes)
    finally:
        pathlib.Path.read_text, pathlib.Path.exists = read_text, exists
    for path, text in overlay.items():
        target = destination / path.relative_to(REPO)
        if os.path.realpath(target.parent) != str(destination / path.relative_to(REPO).parent):
            raise SystemExit(f"clone: {target} does not resolve inside the clone")
        target.parent.mkdir(parents=True, exist_ok=True)
        if os.path.lexists(target):
            os.unlink(target)
        target.write_text(text)
    print(f"{cloned} files cloned, {len(overlay)} staged file(s) applied")
    print(destination)


main()
