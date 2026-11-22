"""🧪 Verify the production-shaped bounded exchange against Python struct."""
import os
from pathlib import Path
import re
import runpy
import struct
import subprocess
import tempfile

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").exists())
runpy.run_path(str(ticket / "native-first-archive-exchange-stage.py"))
out = ticket / "🗑️generated/native-first-archive-exchange"
prefix = ["repo", "tools", "exec", "--light"]
subprocess.run(prefix + ["git", "--", "apply", "--check", str(out / "first-archive-mounted.patch")], cwd=root, check=True)
signed = [42, 0x0020000000000002, 0x0020000000000003, 0x0020000000000005]
def request(phase, offset=0, length=0):
    return struct.pack("<8sIQQQBIH", b"SEMIOFA1", *signed, phase, offset, length)
with tempfile.TemporaryDirectory(dir=out) as directory:
    directory = Path(directory)
    wire_types = (ticket / "natural-pdf-wire-records.rs").read_text()
    wire_types = re.sub(r"^//!.*$|^#\[(?:serde|value)\(.*\)\]$", "", wire_types, flags=re.MULTILINE)
    wire_types = re.sub(r",?\s*(?:serde|semio_framework_value(?:_derive)?)::\w+", "", wire_types)
    exchange = (ticket / "native-first-archive-exchange.rs").read_text().split("\n", 1)[1]
    exchange = exchange.replace("use protocol::wire::", "use crate::protocol::wire::")
    sources = {
        "WIRE_TYPES": wire_types.encode(),
        "EXCHANGE": exchange.encode(),
        "REQUESTS": b"".join([request(1), request(2, 4096), request(3, 4096, 693), request(4), request(5), request(6)]),
        "REPLY": b"SEMIOFR1" + request(1)[8:] + struct.pack("<BIQ32s", 1, 2130273, 7, bytes([5] * 32)) + b"pdf-document",
    }
    env = os.environ.copy()
    for name, contents in sources.items():
        path = directory / name
        path.write_bytes(contents)
        env["SEMIO_ARCHIVE_" + name] = str(path)
    channel = re.search(r'^channel = "([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.MULTILINE).group(1)
    binary = directory / ("proof.exe" if os.name == "nt" else "proof")
    subprocess.run(prefix + ["rustup", "--", "run", channel, "rustc", "--edition", "2024", "--test",
        str(ticket / "native-first-archive-exchange-law.rs"), "-o", str(binary)], cwd=root, env=env, check=True)
    subprocess.run([binary], cwd=root, check=True)
