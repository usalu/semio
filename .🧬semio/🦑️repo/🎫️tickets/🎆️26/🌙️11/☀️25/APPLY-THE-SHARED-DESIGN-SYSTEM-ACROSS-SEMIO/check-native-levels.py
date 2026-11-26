"""🦀 Run focused native tests with the pinned compiler when the fleet launcher is unavailable."""
import os
from pathlib import Path
import subprocess
import sys
ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
out = ticket / "🗑️generated/r"
out.mkdir(parents=True, exist_ok=True)
tool = lambda name: subprocess.check_output(["rustup", "which", name], cwd=root, text=True).strip()
env = {**os.environ, "RUSTC":tool("rustc"), "RUSTC_WRAPPER":"", "RUSTDOC":tool("rustdoc"), "CARGO_TARGET_DIR":str(out/"t"), "CARGO_BUILD_BUILD_DIR":str(out/"b"), "CARGO_PROFILE_TEST_DEBUG":"0","CARGO_PROFILE_DEV_DEBUG":"0","CARGO_INCREMENTAL":"0","RUSTFLAGS":"-Zthreads=1"}
if sys.argv[1] == "semio-renderer-wgpu-ui-engine-tests":
    bun_output = subprocess.check_output(
        ["repo", "tools", "exec", "bun", "--", "-e", "console.log(process.execPath)"],
        cwd=root,
        text=True,
    )
    bun = Path(bun_output.strip().splitlines()[-1])
    if not bun.is_file():
        sys.exit(f"Repo returned an invalid Bun executable: {bun}")
    env["PATH"] = os.pathsep.join([str(bun.parent), os.environ.get("PATH", "")])
from shutil import which
paths = subprocess.check_output([which("rg") or "rg", "-l", "-F", f'name = "{sys.argv[1]}"', "-g", "Cargo.toml"], cwd=root, text=True).splitlines()
manifest = next(root / path for path in paths if (root / path).read_text().split("[package]", 1)[-1].split("[", 1)[0].find(f'name = "{sys.argv[1]}"') >= 0)
store = root / ".cache/b/r/m/cargo/home"
if (store / "registry").is_dir():
    env["CARGO_HOME"] = str(store)
with (out/(sys.argv[1]+".log")).open("w") as log:
    p = subprocess.run([tool("cargo"),"test","--manifest-path",str(manifest),"--locked","--offline","--jobs","6","-p",sys.argv[1],"--lib",*sys.argv[2:]],cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT)
print((out/(sys.argv[1]+".log")).read_text()[-5000:])
sys.exit(p.returncode)
