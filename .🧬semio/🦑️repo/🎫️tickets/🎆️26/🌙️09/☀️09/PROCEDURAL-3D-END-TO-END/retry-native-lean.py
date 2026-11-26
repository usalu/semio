"""🏗️ Resume this ticket's stalled prototype compiler with its issued profile and direct diagnostic files."""
import ctypes
import os
from pathlib import Path
import signal
import subprocess
import sys

pid = int(sys.argv[1])
ticket = Path(__file__).resolve().parent
area = ticket / "🗑️generated/bridge-current/lean"
area.mkdir(exist_ok=True)
libc = ctypes.CDLL(None, use_errno=True)
mib = (ctypes.c_int * 3)(1, 49, pid)
size = ctypes.c_size_t()
assert libc.sysctl(mib, 3, None, ctypes.byref(size), None, 0) == 0
data = ctypes.create_string_buffer(size.value)
assert libc.sysctl(mib, 3, data, ctypes.byref(size), None, 0) == 0
block = data.raw[:size.value]
argc = int.from_bytes(block[:4], sys.byteorder)
parts = block[block.index(b"\0", 4)+1:].lstrip(b"\0").split(b"\0")
args = [os.fsdecode(item) for item in parts[:argc]]
environment = dict(os.fsdecode(item).split("=", 1) for item in parts[argc:] if b"=" in item)
assert Path(args[0]).name == "cargo" and args[1] == "build"
assert any("🌀️procedural/🧩️component" in x for x in args)
assert 'profile.prototype.opt-level="s"' in args
log = ticket / "🗑️generated/bridge-current/fleet-requester.log"
driver = int(sys.argv[2])
assert str(log) in subprocess.check_output(["lsof","-a","-p",str(driver),"-d","1"],text=True)
for ancestor in (pid,):
    os.kill(ancestor,signal.SIGTERM)
for key in ["RUSTC_WRAPPER","RUSTC_WORKSPACE_WRAPPER","CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS","CARGO_BUILD_JOBS"]:
    environment.pop(key, None)
environment["CARGO_TARGET_DIR"] = str(area)
environment["CARGO_BUILD_BUILD_DIR"] = str(Path(".cache/b/r/m/tmp/g3d-lean-n26").resolve())
args = [arg for arg in args if arg != "--message-format=json-render-diagnostics"] + ["-j","2"]
with (area/"build.log").open("w") as log:
    sys.exit(subprocess.run(args,env=environment,stdout=log,stderr=subprocess.STDOUT).returncode)
