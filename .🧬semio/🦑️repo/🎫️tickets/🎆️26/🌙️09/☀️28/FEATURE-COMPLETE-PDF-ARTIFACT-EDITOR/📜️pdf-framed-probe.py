from pathlib import Path
import subprocess
import sys

T = Path(__file__).resolve().parent
OUT = T / "🗑️generated" / "pdf-framed"
OUT.mkdir(parents=True, exist_ok=True)
CACHE = Path(".cache/b/r/m/u/l")
FRAMEWORK = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl")
BANK = FRAMEWORK / "🫴️receiving/🧪️tests/🧰️bank.rs"
COMMON = ["semio_framework_replication_codec", "semio_framework_replication_error",
          "semio_framework_dsl_record", "semio_framework_value",
          "semio_framework_value_native_owned", "semio_framework_pack_json", "serde_json"]
target = ["--target=wasm32-wasip2", "-Cpanic=abort", "--emit=metadata", "--crate-type=lib"] if "--wasi" in sys.argv else ["--test"]
lane = CACHE / "pc5147e"
build = lane / ("wasm32-wasip2" if "--wasi" in sys.argv else "") / "prototype/build"
ext = ".rmeta"
paths = {p.parent for p in build.glob("*/*/out/*") if p.suffix in (ext, ".so", ".dylib")}
paths.update(p.parent for p in lane.glob("prototype/build/*/*/out/*") if p.suffix in (".so", ".dylib"))
args = []
for name in COMMON:
    found = sorted(build.glob(f"*/*/out/lib{name}-*{ext}"))
    if len(found) != 1: raise ValueError((name, found))
    args += ["--extern", f"{name}={found[0]}"]
    if "--wasi" not in sys.argv:
        args += ["--extern", f"{name}={found[0].with_suffix('.rlib')}"]
source = OUT / "probe.rs"
source.write_text('#[path = "' + str(BANK.resolve()) + '"]\nmod bank;\n' +
    (T / "📜️pdf-framed-probe.rs").read_text(encoding="utf8") +
    '\ninclude!(r"' + str(T / "📜️pdf-framed-receiving.rs") + '");\n', encoding="utf8")
out = OUT / ("probe.rmeta" if "--wasi" in sys.argv else "probe")
argfile = OUT / ("wasi.args" if "--wasi" in sys.argv else "native.args")
argfile.write_text("\n".join(["--edition=2021", *target, "-o", str(out), str(source), *args,
    *(f"-Ldependency={path}" for path in sorted(paths))]) + "\n", encoding="utf8")
subprocess.run(["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc","@"+str(argfile)],check=True)
if "--wasi" not in sys.argv:
    subprocess.run([str(out)],check=True)
