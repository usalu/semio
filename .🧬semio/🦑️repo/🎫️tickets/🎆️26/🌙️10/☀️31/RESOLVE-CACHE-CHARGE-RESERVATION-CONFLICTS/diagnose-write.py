import os
import stat
from pathlib import Path

script = Path(__file__).resolve()
target = script.parents[7] / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🎟️reservation/🧪️tests/🟦️.ts"
print("exists", target.exists())
print("len", len(str(target)))
mode = target.stat().st_file_attributes if hasattr(target.stat(), "st_file_attributes") else target.stat().st_mode
print("attrs", hex(target.stat().st_file_attributes))
print("mode", oct(target.stat().st_mode))
try:
    with target.open("r+b") as handle:
        handle.seek(0)
        head = handle.read(16)
        print("r+ ok", head[:8])
except OSError as error:
    print("r+ failed", error)
try:
    fd = os.open(str(target), os.O_RDWR)
    os.close(fd)
    print("os.open ok")
except OSError as error:
    print("os.open failed", error)
