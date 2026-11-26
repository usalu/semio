"""🧱 Link a fixture generator against the native Cargo-owned BIM libraries."""
from pathlib import Path
import os
import json
import subprocess
import sys
ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
artifacts = generated / "ntb" / "debug" / "build"
deps = generated / "fixture-link"
deps.mkdir(exist_ok=True)
for output in artifacts.glob("*/*/out"):
    for archive in output.iterdir():
        if archive.suffix in (".rlib", ".rmeta", ".dylib", ".so", ".a", ".lib", ".dll"):
            link = deps / archive.name
            try:
                link.symlink_to(archive)
            except FileExistsError:
                if not link.exists() and link.is_symlink() and Path(os.readlink(link)) != archive:
                    temporary = link.with_name(f".{os.getpid()}-{link.name}")
                    temporary.symlink_to(archive)
                    temporary.replace(link)
compiler = subprocess.check_output(["rustup", "which", "rustc"], text=True).strip()
source = os.environ.get("BIM_FIXTURE_LINK_SOURCE", str(ticket / "fixture-export.rs"))
binary = os.environ.get("BIM_FIXTURE_LINK_OUTPUT", str(generated / "fixture-export"))
args = [compiler, "--crate-type", "bin", "--edition=2024", "-Zno-parallel-backend", "-Zthreads=1", "-Cdebuginfo=0", "-Cstrip=debuginfo", source, "-L", "dependency="+str(deps), "-L", "native="+str(deps), "-o", binary]
if sys.platform == "darwin":
    sysroot = Path(subprocess.check_output([compiler, "--print", "sysroot"], text=True).strip())
    host = next(line.split(": ", 1)[1] for line in subprocess.check_output([compiler, "-vV"], text=True).splitlines() if line.startswith("host: "))
    linker = sysroot / "lib/rustlib" / host / "bin/gcc-ld/ld64.lld"
    args.extend(["-Clink-arg=-fuse-ld="+str(linker), "-Clink-arg=-Wl,--threads=1,--read-workers=0"])
captures = generated / "compiler-retry"
invocations = sorted((*captures.glob("n-*.invocation.json"), *captures.glob("n-*.success.json")), key=lambda path:path.stat().st_mtime, reverse=True)
anchor = os.environ.get("BIM_FIXTURE_LINK_ANCHOR")
if anchor:
    invocations.insert(0, Path(anchor))
externs = {}
for path in invocations:
    arguments=json.loads(path.read_text())["arguments"]
    for index, item in enumerate(arguments[:-1]):
        if item == "--extern" and "=" in arguments[index+1]:
            name, arg=arguments[index+1].split("=",1)
            if arg.endswith(".rlib") and Path(arg).is_file():
                externs.setdefault(name,Path(arg))
        if item == "-L" and arguments[index+1].startswith("native="):
            args.extend(["-L",arguments[index+1]])
for item in sys.argv[1:]:
    alias, package = item.split("=")
    choices = list((artifacts/package).glob("*/out/lib"+package.replace("-","_")+"-*.rlib"))
    archive = Path(package).resolve() if package.endswith(".rlib") else externs.get(package.replace("-","_")) or max(choices,key=lambda path:path.stat().st_mtime)
    args.extend(["--extern", alias+"="+str(archive), "--extern", alias+"="+str(archive.with_suffix(".rmeta"))])
subprocess.run(args, check=True)
