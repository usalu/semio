"""🏔️ Publish the reviewed Terrain patch against the exact live inputs."""

from pathlib import Path
from stat import S_IMODE
from tempfile import TemporaryDirectory
import re
import runpy
import sys

ticket = Path(__file__).resolve().parent
root = Path.cwd().resolve()
compose = runpy.run_path(str(ticket / "compose-terrain-resolution.py"))
patch_path = Path(sys.argv[1]) if len(sys.argv) > 1 else ticket / "terrain-rectangular-combined.patch"
patch = patch_path.read_text(encoding="utf-8")
changes = {}
for block in re.split(r"(?m)(?=^--- (?:a/|/dev/null))", patch):
    if not block:
        continue
    old, new, *_ = block.splitlines()
    relative = Path(new.removeprefix("+++ b/"))
    path = root / relative
    if not new.startswith("+++ b/") or relative.is_absolute() or not path.resolve().is_relative_to(root) or len(str(relative)) > 256:
        raise ValueError(f"Unexpected patch target {new}")
    if old not in (f"--- a/{relative}", "--- /dev/null"):
        raise ValueError(f"Patch has different source and destination for {relative}")
    if path.is_symlink() or path in changes:
        raise ValueError(f"Repeated or indirect target {relative}")
    original = path.read_bytes() if path.is_file() else None
    if (old == "--- /dev/null") != (original is None):
        raise ValueError(f"The creation status changed for {relative}")
    target = compose["apply_patch"]((original or b"").decode("utf-8"), block).encode("utf-8")
    if original == target:
        raise ValueError(f"Patch would not change {relative}")
    changes[path] = original, target

generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)


def current_bytes(path):
    return path.read_bytes() if path.is_file() else None


with TemporaryDirectory(prefix="terrain-", dir=generated) as stage:
    stage_dir = Path(stage)
    staged = {}
    for index, (path, (original, target)) in enumerate(changes.items()):
        pending = stage_dir / f"new-{index}"
        pending.write_bytes(target)
        if original is not None:
            pending.chmod(S_IMODE(path.stat().st_mode))
        staged[path] = pending

    modified = []
    try:
        for path, (original, target) in changes.items():
            if current_bytes(path) != original:
                raise RuntimeError(f"Concurrent edit at {path.relative_to(root)}")
        for path, (original, target) in changes.items():
            if current_bytes(path) != original:
                raise RuntimeError(f"Concurrent edit at {path.relative_to(root)}")
            path.parent.mkdir(parents=True, exist_ok=True)
            staged[path].replace(path)
            modified.append(path)
    except BaseException:
        for index, path in reversed(list(enumerate(modified))):
            original, target = changes[path]
            if current_bytes(path) != target:
                continue
            if original is None:
                path.unlink()
            else:
                restore = stage_dir / f"old-{index}"
                restore.write_bytes(original)
                restore.chmod(S_IMODE(path.stat().st_mode))
                restore.replace(path)
        raise

print(f"[DEBUG] Applied {len(changes)} Terrain sources and fixtures from {patch_path.name}.")
