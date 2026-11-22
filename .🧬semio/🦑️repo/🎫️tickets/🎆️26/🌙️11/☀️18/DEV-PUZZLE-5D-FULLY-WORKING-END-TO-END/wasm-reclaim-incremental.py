import fcntl
import json
import os
import re
import shutil
import stat
import sys
from collections import Counter
from contextlib import ExitStack
from pathlib import Path

GIB = 1 << 30
TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
PROTOTYPE = GENERATED / "w/b/wasm32-unknown-unknown/prototype"
INCREMENTAL = PROTOTYPE / "incremental"
BUILD = PROTOTYPE / "build"
PACKAGE = re.compile(r"([A-Za-z0-9_]+)-[a-z0-9]+")
GENERATION = re.compile(r"s-[a-z0-9]+-[a-z0-9]+-[a-z0-9]+")


def available():
    volume = os.statvfs(GENERATED)
    return volume.f_bavail * volume.f_frsize


def fresh_library(crate, cutoff, verify=False):
    owner = BUILD / crate.replace("_", "-")
    if not owner.is_dir():
        return 0
    for unit in owner.iterdir():
        library = unit / "out" / f"lib{crate}-{unit.name}.rlib"
        receipt = unit / "fingerprint" / f"lib-{crate}"
        try:
            artifact = library.stat()
            done = receipt.stat()
        except FileNotFoundError:
            continue
        if not artifact.st_size or not done.st_size or not cutoff <= artifact.st_mtime_ns <= done.st_mtime_ns:
            continue
        if verify:
            dep = unit / "out" / f"{crate}-{unit.name}.d"
            try:
                with library.open("rb") as archive:
                    readable_library = archive.read(8) == b"!<arch>\n"
                readable_dep = b"# checksum:blake3=" in dep.read_bytes()
                readable_receipt = bool(receipt.read_bytes().strip())
                fresh_dep = cutoff <= dep.stat().st_mtime_ns <= done.st_mtime_ns
            except FileNotFoundError:
                continue
            if not (readable_library and readable_dep and readable_receipt and fresh_dep):
                continue
        return done.st_mtime_ns
    return 0

def active_compiler(crate):
    for process in Path("/proc").iterdir():
        if not process.name.isdecimal():
            continue
        try:
            if (process / "comm").read_bytes().strip() != b"rustc":
                continue
            argv = (process / "cmdline").read_bytes().split(b"\0")
            flag = argv.index(b"--crate-name")
            if argv[flag + 1] == crate.encode():
                return True
        except (FileNotFoundError, ProcessLookupError, PermissionError, ValueError, IndexError):
            continue
    return False

def completed_generations(cutoff):
    for package in sorted(INCREMENTAL.iterdir()):
        if not PACKAGE.fullmatch(package.name) or not package.is_dir():
            continue
        for directory in package.iterdir():
            try:
                info = directory.lstat()
            except FileNotFoundError:
                continue
            if GENERATION.fullmatch(directory.name) and stat.S_ISDIR(info.st_mode) and info.st_mtime_ns >= cutoff:
                yield {"package": package.name, "generation": directory.name, "oldestEligibleMtimeNs": str(info.st_mtime_ns)}


def allocated(directory, expected, cutoff, completed_at=0):
    original = directory.lstat()
    if not stat.S_ISDIR(original.st_mode) or original.st_mtime_ns != int(expected["oldestEligibleMtimeNs"]):
        return None
    files = []
    for parent, dirs, names in os.walk(directory):
        for child in dirs:
            if not stat.S_ISDIR((Path(parent) / child).lstat().st_mode):
                return None
        for child in names:
            info = (Path(parent) / child).lstat()
            newest = info.st_mtime_ns > completed_at if completed_at else info.st_mtime_ns >= cutoff
            if not stat.S_ISREG(info.st_mode) or newest:
                return None
            files.append(info)
    if not completed_at and len(files) != expected["files"]:
        return None
    counts = Counter(info.st_ino for info in files)
    inodes = {info.st_ino: info for info in files}
    return sum(info.st_blocks * 512 for inode, info in inodes.items() if info.st_nlink == counts[inode])


def reclaim(entry, cutoff, limit, apply, completed=False):
    parent = INCREMENTAL / entry["package"]
    directory = parent / entry["generation"]
    package = PACKAGE.fullmatch(parent.name)
    if not package or not GENERATION.fullmatch(directory.name) or directory.resolve() != directory:
        return None
    crate = package[1]
    if not fresh_library(crate, cutoff):
        return None
    try:
        names = list(parent.iterdir())
        lockfile = parent / (directory.name.rsplit("-", 1)[0] + ".lock")
        if lockfile not in names or any(path.name.endswith("-working") for path in names):
            return None
        with ExitStack() as held:
            for lock in (path for path in names if path.name.endswith(".lock")):
                fd = os.open(lock, os.O_RDWR | os.O_NOFOLLOW)
                held.callback(os.close, fd)
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                fcntl.lockf(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
            newer_finished = any(
                path != directory
                and GENERATION.fullmatch(path.name)
                and stat.S_ISDIR(path.lstat().st_mode)
                and path.lstat().st_mtime_ns >= cutoff
                for path in parent.iterdir()
            )
            finished = fresh_library(crate, cutoff, completed)
            if not finished or (not completed and not newer_finished):
                return None
            blocks = allocated(directory, entry, cutoff, finished if completed else 0)
            if blocks is None or not 0 < blocks <= limit:
                return None
            if completed and (active_compiler(crate) or any(path.name.endswith("-working") for path in parent.iterdir())):
                return None
            if apply:
                shutil.rmtree(directory)
            return blocks
    except (FileNotFoundError, BlockingIOError):
        return None


generation_epoch = next((name + "-" for name in ("tenth", "ninth", "eighth", "seventh", "sixth") if "--" + name in sys.argv), "")
inventory_name = f"wasm-private-incremental-{generation_epoch}candidates.json"
inventory = json.loads((GENERATED / inventory_name).read_text())
cutoff = int(inventory["cutoffNs"])
before = available()
completed = "--completed" in sys.argv
restore = "--continue-recovery" in sys.argv
below_limit = before < (10 if restore else 7) * GIB if completed else before <= 8 * GIB
apply = "--apply" in sys.argv and below_limit
previous = next((int(arg.removeprefix("--reclaimed-bytes=")) for arg in sys.argv if arg.startswith("--reclaimed-bytes=")), 0)
if not 0 <= previous <= 8 * GIB:
    raise ValueError("Prior reclamation must be between 0 and 8 GiB")
limit = 2 * GIB if "--extra-batch" in sys.argv else 8 * GIB - previous
if completed:
    limit = min(3 * GIB // 2, max(0, 10 * GIB - before))
total = 0
entries = []
candidates = completed_generations(cutoff) if completed else inventory["groups"]
for candidate in candidates:
    if completed and (total >= limit or (apply and available() >= 10 * GIB)):
        break
    if apply and "--extra-batch" in sys.argv and available() >= 10 * GIB:
        break
    blocks = reclaim(candidate, cutoff, limit - total, apply, completed)
    if blocks:
        total += blocks
        entries.append({**candidate, "bytes": blocks})
print(json.dumps({"applied": apply, "beforeBytes": before, "afterBytes": available(), "batchLimitBytes": limit, "allocatedBytes": total, "groups": entries}))
