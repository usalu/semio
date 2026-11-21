"""🔧 Compile the staged scanline proof with the actual first-party DEFLATE and paged value code."""

import os
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated" / "paged-png"
CACHE = Path(sys.argv[1])
OUTPUT.mkdir(parents=True, exist_ok=True)

prefix = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2021", "-O"]
value = []
for name in ("semio_framework_value", "semio_framework_value_persistent"):
    library = next(CACHE.glob(f"*/*/out/lib{name}-*.rlib"))
    value += ["--extern", f"{name}={library.with_suffix('.rmeta')}", "--extern", f"{name}={library}"]
paths = [f"-Ldependency={directory}" for directory in sorted({path.parent for path in CACHE.glob("*/*/out/*") if path.suffix in (".rmeta", ".rlib", ".dylib", ".dll", ".so")})]
deflate = OUTPUT / "libsemio_framework_deflate.rlib"
deflate_source = Path("🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust/🦀️.rs")
exe = OUTPUT / ("png-probe.exe" if os.name == "nt" else "png-probe")

def run(arguments, name):
    """🧾 Keep long local rustc invocation arguments in the ticket-generated area."""
    response = OUTPUT / name
    response.write_text("\n".join(arguments) + "\n", encoding="utf8")
    subprocess.run([*prefix, "@" + str(response)], check=True)

run(["--crate-type", "rlib", "--crate-name", "semio_framework_deflate", str(deflate_source), "-o", str(deflate), *value, *paths], "deflate.rsp")
run(["--crate-name", "png_plane_probe", str(TICKET / "paged-png-planes-probe.rs"), "-o", str(exe),
     "--extern", f"semio_framework_deflate={deflate}", *value, *paths], "probe.rsp")
