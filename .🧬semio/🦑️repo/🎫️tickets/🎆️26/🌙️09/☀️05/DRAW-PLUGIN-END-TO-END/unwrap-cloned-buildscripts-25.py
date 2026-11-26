from pathlib import Path
import os

root = Path(__file__).parent / "🗑️generated" / "w25"
count = 0
for real in root.rglob("build_script_build.real"):
    wrapper = real.with_suffix("")
    if wrapper.is_file() and b"semio-unit-cache-build-script" in wrapper.read_bytes()[:128]:
        os.replace(real, wrapper)
        count += 1
print(f"[DEBUG] Restored {count} cloned build script executables")
