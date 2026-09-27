#!/usr/bin/env python3
"""🛬️ T14 window-3 landing, one command per step, compile-atomic. Sets (in order): `f9` (content ids: `f9/content-id.py`,
then the carrier map `f9/apply-map.py` with the map the overlay recorded), `g12` (G12's authoring-seed builder pass — lands
in the SAME pass as F9, it needs `store::content_id`), `h9l` (per-kind labels, `h9l/kind-label-patch.py`). Every set is
dry-run on the live tree first; `--write` snapshots every file the set may touch (backups under
`.🧬semio/🌐hub/s14-t14-land/<set>/`), applies, and records exactly which files changed; `--restore <set>` puts back only
files still byte-equal to what this run wrote (a peer's later edit is never overwritten). Compile gates run separately
through the fleet lanes (`land-w3-gates.sh`).

usage: land-w3.py [--dry-run] | --write <set>… | --restore <set> | --status"""
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent
STATE = ROOT / ".🧬semio/🌐hub/s14-t14-land"
MAPS = sorted((ROOT / ".🧬semio/🌐hub/s14-t14-logs").glob("hold*-f9-map-*.tsv"))
SETS = {
    "f9": [[sys.executable, str(HERE / "f9/content-id.py")], [sys.executable, str(HERE / "f9/apply-map.py"), *sum((["--map", str(path)] for path in MAPS), [])]],
    "g12": [[sys.executable, str(ROOT / ".tmp-ticket/wp-g12/g12-authoring-seed.py")]],
    "h9l": [[sys.executable, str(HERE / "h9l/kind-label-patch.py")]],
}
WRITE_FLAG = {"content-id.py": "--write", "apply-map.py": "--write", "g12-authoring-seed.py": "--write", "kind-label-patch.py": "--apply"}
DRY_FLAG = {"content-id.py": None, "apply-map.py": None, "g12-authoring-seed.py": "--dry-run", "kind-label-patch.py": None}


DIRS = ["✏️s", "🧰️framework", "🌎️hub", "📜️script.ts"]
PRUNE = [arg for name in ("node_modules", "dist", "target", "🗑️generated", "generated", ".venv") for arg in ("-o", "-name", name)][1:]


def snapshot(target: Path) -> Path:
    """🪞️ Copy-on-write clone (`clonefile(2)`) of the source roots right before a set writes: the exact before-content of
    whatever the set changes, at no data cost."""
    import ctypes
    libc = ctypes.CDLL("libc.dylib", use_errno=True)
    libc.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]
    pre = target / "pre"
    pre.mkdir(parents=True, exist_ok=True)
    for entry in DIRS:
        destination = pre / entry
        if not destination.exists() and libc.clonefile(str(ROOT / entry).encode(), str(destination).encode(), 1) != 0:
            raise OSError(ctypes.get_errno(), entry)
    return pre


def changed_since(stamp: Path) -> list:
    found = subprocess.run(["find", *DIRS, "(", *PRUNE, ")", "-prune", "-o", "-newer", str(stamp), "-type", "f", "-print0"], cwd=ROOT, capture_output=True).stdout.decode("utf-8", "replace").split("\0")
    return sorted(filter(None, found))


def run(command: list, flag) -> subprocess.CompletedProcess:
    return subprocess.run(command + ([flag] if flag else []), cwd=ROOT, capture_output=True, text=True)


def dry_run() -> bool:
    clean = True
    for name, commands in SETS.items():
        for command in commands:
            result = run(command, DRY_FLAG[Path(command[1]).name])
            tail = (result.stdout + result.stderr).strip().splitlines()[-3:]
            print(f"[{name}] {Path(command[1]).name} rc={result.returncode}: {' | '.join(tail)}")
            clean &= result.returncode == 0 or (name != "f9" and "content_id" in result.stdout)
    return clean


def write(names: list) -> None:
    for name in names:
        target = STATE / name
        if (target / "manifest.json").exists():
            raise SystemExit(f"[{name}] already landed by this tool ({target}); --restore first")
        target.mkdir(parents=True, exist_ok=True)
        stamp = target / "stamp"
        stamp.write_text(name, encoding="utf-8")
        pre = snapshot(target)
        for command in SETS[name]:
            result = run(command, WRITE_FLAG[Path(command[1]).name])
            print(f"[{name}] {Path(command[1]).name} rc={result.returncode}: {' | '.join((result.stdout + result.stderr).strip().splitlines()[-2:])}")
        manifest = []
        for index, rel in enumerate(changed_since(stamp)):
            before = pre / rel
            if before.is_file():
                (target / f"{index}.before").write_bytes(before.read_bytes())
            (target / f"{index}.after").write_bytes((ROOT / rel).read_bytes())
            manifest.append({"rel": rel, "index": index, "created": not before.is_file()})
        (target / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=1), encoding="utf-8")
        print(f"[{name}] {len(manifest)} files changed, backups + manifest in {target}")


def restore(name: str) -> None:
    target = STATE / name
    manifest = json.loads((target / "manifest.json").read_text(encoding="utf-8"))
    kept = 0
    for row in manifest:
        path = ROOT / row["rel"]
        if not path.exists() or path.read_bytes() != (target / f"{row['index']}.after").read_bytes():
            kept += 1
            print(f"kept (edited since): {row['rel']}")
            continue
        if row["created"]:
            path.unlink()
        else:
            path.write_bytes((target / f"{row['index']}.before").read_bytes())
    print(f"[{name}] restored {len(manifest) - kept}, kept {kept}")


if __name__ == "__main__":
    arguments = sys.argv[1:]
    if not arguments or arguments[0] == "--dry-run":
        sys.exit(0 if dry_run() else 1)
    if arguments[0] == "--write":
        write(arguments[1:])
    elif arguments[0] == "--restore":
        restore(arguments[1])
    elif arguments[0] == "--status":
        for name in SETS:
            manifest = STATE / name / "manifest.json"
            print(name, "landed" if manifest.exists() else "not landed")
