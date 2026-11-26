"""🦀 Run focused native tests with the pinned compiler when the fleet launcher is unavailable."""
import os
from pathlib import Path
import subprocess
import sys
ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
out = ticket / "🗑️generated/web2/cargo"
out.mkdir(parents=True, exist_ok=True)
tool = lambda name: subprocess.check_output(["rustup", "which", name], cwd=root, text=True).strip()
env = {**os.environ, "RUSTC":tool("rustc"), "RUSTC_WRAPPER":"", "RUSTDOC":tool("rustdoc"), "CARGO_TARGET_DIR":str(out/"t"), "CARGO_BUILD_BUILD_DIR":str(out/"b"), "CARGO_PROFILE_TEST_DEBUG":"0","CARGO_PROFILE_DEV_DEBUG":"0","CARGO_INCREMENTAL":"0","RUSTFLAGS":"-Zthreads=1"}
from shutil import copyfile, which
paths = subprocess.check_output([which("rg") or "rg", "-l", "-F", f'name = "{sys.argv[1]}"', "-g", "Cargo.toml"], cwd=root, text=True).splitlines()
manifest = next(root / path for path in paths if (root / path).read_text().split("[package]", 1)[-1].split("[", 1)[0].find(f'name = "{sys.argv[1]}"') >= 0)
store = root / ".cache/b/r/m/cargo/home"
if (store / "registry").is_dir():
    env["CARGO_HOME"] = str(store)
lock = ["--locked"]
if os.environ.get("SEMIO_DIAGNOSTIC_CARGO_LOCK") == "1":
    mirror = out / "m"
    framework = mirror / "🧰️framework"
    framework.mkdir(parents=True, exist_ok=True)
    for source in root.iterdir():
        if source.name in {framework.name, ".🧬semio", ".git"}:
            continue
        link = mirror / source.name
        if not link.is_symlink():
            link.symlink_to(source, target_is_directory=source.is_dir())
    for source in (root / framework.name).iterdir():
        link = framework / source.name
        if source.name in {"Cargo.toml", "Cargo.lock"}:
            copyfile(source, link)
        elif not link.is_symlink():
            link.symlink_to(source, target_is_directory=source.is_dir())
    manifest = mirror / manifest.relative_to(root)
    lock = []
with (out/(sys.argv[1]+".log")).open("w") as log:
    p = subprocess.run([tool("cargo"),"test","--manifest-path",str(manifest),*lock,"--offline","--jobs","2","-p",sys.argv[1],"--lib",*sys.argv[2:]],cwd=root,env=env,stdout=log,stderr=subprocess.STDOUT)
print((out/(sys.argv[1]+".log")).read_text()[-5000:])
sys.exit(p.returncode)
