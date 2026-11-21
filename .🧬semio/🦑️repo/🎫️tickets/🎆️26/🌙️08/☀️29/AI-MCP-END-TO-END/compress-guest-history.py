import gzip
from hashlib import sha256
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parent / "🗑️generated"
HISTORICAL = (
    "native-guests.log", "native-guests-final.log", "native-guests-note-dep.log", "native-guests-retry.log",
    "native-wfc-opt1.log", "native-wfc-artifact-opt1.log", "native-wfc-boot-refresh.log",
    "native-wfc-source-transition-guest.log", "native-wfc-ui-batch-guest.log", "native-wfc-ui-batch-edge-retry.log",
    "native-note-core-wire-rebuild.log", "native-note-source-transition-guest.log",
    "native-note-ui-batch-guest.log", "native-note-optimized-guest.log",
)


def snapshot(path):
    stat = path.stat()
    return stat.st_ino, stat.st_mtime_ns, stat.st_size


def open_reader(path):
    called = subprocess.run(["/usr/sbin/lsof", "--", str(path)], capture_output=True)
    if called.returncode == 1 and not called.stdout and not called.stderr:
        return False
    if called.returncode == 0:
        return True
    raise RuntimeError(f"lsof could not inspect {path.name}")


def chunks(stream):
    return iter(lambda: stream.read(1024 * 1024), b"")


for name in HISTORICAL:
    source = ROOT / name
    archive = source.with_suffix(".log.gz")
    temporary = archive.with_suffix(".gz.tmp")
    if not source.is_file() or archive.exists() or temporary.exists() or open_reader(source):
        continue
    original = snapshot(source)
    if original[2] < 6_000_000:
        continue
    try:
        expected = sha256()
        with source.open("rb") as inp, gzip.open(temporary, "xb", compresslevel=4) as out:
            for block in chunks(inp):
                expected.update(block)
                out.write(block)
        restored = sha256()
        decoded = 0
        with gzip.open(temporary, "rb") as inp:
            for block in chunks(inp):
                restored.update(block)
                decoded += len(block)
        if restored.digest() != expected.digest() or decoded != original[2]:
            raise ValueError(f"archive did not preserve {source.name}")
        if snapshot(source) != original or open_reader(source):
            raise ValueError(f"historical log became active: {source.name}")
        temporary.replace(archive)
        source.unlink()
        print(f"[DEBUG] {source.name} {original[2]} to {archive.stat().st_size} bytes, decoded SHA-256 {restored.hexdigest()}")
    finally:
        if temporary.exists():
            temporary.unlink()
