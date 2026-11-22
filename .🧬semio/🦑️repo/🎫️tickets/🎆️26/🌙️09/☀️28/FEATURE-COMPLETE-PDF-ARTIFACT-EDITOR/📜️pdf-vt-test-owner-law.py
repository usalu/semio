"""🧪 Run the two existing VT codec laws against the repository's WASI codec package."""

from pathlib import Path
import subprocess

generated = Path(__file__).resolve().parent / "🗑️generated"
lanes = Path(".cache/b/r/m/u/l")
artifact = "libsemio_s_artifact_stdio_pdf_mut_vt-*.rlib"
owners = sorted(lanes.glob(f"*/wasm32-wasip2/prototype/build/*/*/out/{artifact}"),
                key=lambda path: path.stat().st_mtime, reverse=True)
assert owners, "No repository-built WASI VT package is available"
test = generated / "vt-trim-box-test-owner.rs"
component = generated / "vt-trim-box-test-owner.wasm"
args = generated / "vt-trim-box-test-owner.args"
for owner in owners:
    build = owner.parents[3]
    target_dirs = {file.parent.resolve() for file in build.glob("*/*/out/*.rlib")}
    host = build.parents[2] / "prototype" / "build"
    host_dirs = {file.parent.resolve() for ext in ("dylib", "so", "dll") for file in host.glob(f"*/*/out/*.{ext}")}
    args.write_text("\n".join([
        "--edition=2021", "--target=wasm32-wasip2", "-Cpanic=abort",
        str(test), "-o", str(component),
        f"--extern=semio_s_artifact_stdio_pdf_mut_vt={owner.with_suffix('.rmeta').resolve()}",
        f"--extern=semio_s_artifact_stdio_pdf_mut_vt={owner.resolve()}",
        *(f"-Ldependency={path}" for path in sorted(target_dirs | host_dirs)),
    ]) + "\n", encoding="utf8")
    compile = subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                              "@" + str(args)], capture_output=True, text=True)
    if not compile.returncode:
        print(f"[DEBUG] Linked original trim-box codec laws to {owner.parent.parent.name}", flush=True)
        subprocess.run(["repo", "tools", "exec", "jco", "--", "run", str(component),
                        "--jco-dir", str(generated / "vt-trim-box-test-owner-run")], check=True)
        break
    print(compile.stderr[-2500:], flush=True)
else:
    raise SystemExit("Repository-built VT codec dependencies did not link")

