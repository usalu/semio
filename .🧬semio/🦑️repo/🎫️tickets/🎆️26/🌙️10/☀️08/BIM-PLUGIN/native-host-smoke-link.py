from pathlib import Path
import os
import re
import subprocess
import sys

generated = Path(__file__).resolve().parent / "🗑️generated"
room = Path.cwd() / "🌎️hub/🧩️compositions/🏙️bim"
host_build = generated / "hb/debug/build"
native_build = generated / "ntb/debug/build"
exe = generated / "native-host-smoke"
test = room / "🧪️tests/🛂️committed-descriptor/🦀️.rs"
source = generated / "native-host-smoke.rs"
source.write_text(f'#[path = "{test}"]\nmod smoke;\n')
rustc = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
meta = max((host_build / "semio-os-run").glob("*/out/libsemio_os_run-*.rmeta"), key=lambda path: path.stat().st_mtime)
dependencies = subprocess.check_output([rustc, "-Zls=root", str(meta)], text=True)
links = generated / "host-smoke-link"
links.mkdir(exist_ok=True)
args = [rustc, "--test", "--edition=2024", str(source), "-o", str(exe), "-L", f"dependency={links}"]
for build in [host_build, native_build]:
    for file in build.glob("*/*/out/*"):
        if file.suffix not in {".rlib", ".rmeta", ".dylib", ".so", ".a", ".lib"}:
            continue
        link = links / file.name
        if not link.exists():
            try:
                link.symlink_to(file)
            except FileExistsError:
                pass
        if file.suffix in {".a", ".lib"}:
            args.extend(["-L", f"native={file.parent}"])
for name in ["semio_os_run", "semio_framework_async", "semio_framework_async_macros", "semio_framework_job", "semio_plugin_host", "semio_framework_test_fixtures", "serde_json"]:
    selected = re.search(rf"(?m)^\d+ {name}-([a-f0-9]+) hash ", dependencies)
    build_hash = meta.stem.split("-")[-1] if name == "semio_os_run" else (selected[1] if selected else "*")
    files = [file for file in links.glob(f"lib{name}-{build_hash}.*") if file.is_file()]
    if build_hash == "*":
        available = [file for file in files if file.suffix in {".dylib", ".so", ".rlib"}]
        chosen = max(available, key=lambda file: file.stat().st_mtime)
        files = [file for file in files if file.stem == chosen.stem]
    assert files, name
    for file in files:
        args.extend(["--extern", f"{name}={file}"])
env = dict(os.environ, CARGO_MANIFEST_DIR=str(room / "📦️packages/🦀️rust"))
subprocess.run(args, env=env, check=True)
if len(sys.argv) > 1:
    env["SEMIO_BIM_GUEST_COMPONENT"] = str(Path(sys.argv[1]).resolve(strict=True))
    subprocess.run([str(exe), "--nocapture"], env=env, check=True)
