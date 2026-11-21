"""🔐 Type-checks the live WGPU kernel interface owners with governed captured dependencies."""
from pathlib import Path
import shlex
import subprocess
import sys

ticket = Path(__file__).resolve().parent
out = ticket / "🗑️generated/native-wgpu-save/kmeta"
out.mkdir(exist_ok=True)
log = ticket / "🗑️generated/native-wgpu-save" / sys.argv[1]
rows = log.read_text(encoding="utf8", errors="replace").splitlines()
rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
built = {}
for owner in ("kernel_protocol", "kernel_retained", "kernel_pool"):
    name = "semio_renderer_wgpu_" + owner
    needle = "--crate-name " + name + " "
    calls = [row for row in rows if "Running " in row and needle in row]
    args = shlex.split(calls[-1].strip().removeprefix("Running").strip().strip(chr(96)))
    args = args[args.index("--crate-name"):]
    source = next(i for i, arg in enumerate(args) if arg.startswith("ROOT-") or arg.endswith(".rs"))
    assert Path(args[source]).is_file(), args[source]
    args = [arg for i, arg in enumerate(args) if not (arg.startswith(("incremental=", "-Cincremental="))
        or arg == "-C" and i + 1 < len(args) and args[i + 1].startswith("incremental="))]
    args = ["--emit=metadata" if arg.startswith("--emit=") else arg for arg in args]
    args[args.index("--out-dir") + 1] = str(out)
    args.insert(args.index("--out-dir") + 2, "-Ldependency=" + str(out))
    for index, arg in enumerate(args):
        if index <= 1 or args[index-1] != "--extern" or "=" not in arg:
            continue
        key, _, old = arg.partition("=")
        if key in built:
            args[index] = f"{key}={built[key]}"
        else:
            assert Path(old).is_file(), old
    rsp = out / f"{owner}.rsp"
    rsp.write_text("\n".join(args)+"\n",encoding="utf8")
    result = subprocess.run([*rustc, f"@{rsp}"], text=True, capture_output=True)
    (out / f"{owner}.log").write_text("[DEBUG] " + name + "\n" + result.stdout + result.stderr,encoding="utf8")
    if result.returncode:
        print("\n".join(line[:900] for line in result.stderr.splitlines() if "error" in line)[-4500:])
        raise SystemExit(result.returncode)
    [built[name]] = list(out.glob(f"lib{name}-*.rmeta"))
    print("[DEBUG] Current native WGPU " + name + " typechecked with the captured real governed dependencies")
