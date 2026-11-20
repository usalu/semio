"""🔌 Run PDF module tests using repository-produced Rust dependencies."""

from itertools import product
import os
from pathlib import Path
import subprocess


def build(root, cache, crate_source, externs):
    generated = root / "🗑️generated"
    generated.mkdir(exist_ok=True)
    binary = generated / ("flate-tests.exe" if os.name == "nt" else "flate-tests")
    packages = []
    for name in externs:
        artifacts = list(cache.glob(f"*/*/out/lib{name}-*.rlib"))
        if not artifacts:
            raise RuntimeError(f"missing repository build backing for {name}")
        packages.append(artifacts)
    search = sorted({f"-Ldependency={path.parent.resolve()}" for path in cache.glob("*/*/out/lib*") if path.suffix in {".rlib", ".rmeta", ".dylib", ".so", ".dll"}})
    base = ["--edition=2021", "--test", "--crate-name=pdf_flate_fixture", str(crate_source), "-o", str(binary)] + search
    response = generated / "flate-test-args"
    for combination in product(*packages):
        arguments = list(base)
        for name, artifact in zip(externs, combination):
            metadata = artifact.with_suffix(".rmeta")
            if metadata.is_file():
                arguments.append(f"--extern={name}={metadata.resolve()}")
            arguments.append(f"--extern={name}={artifact.resolve()}")
        response.write_text("\n".join(arguments) + "\n", encoding="utf-8")
        invocation = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)]
        env = {**os.environ, "RUSTC_ICE": str(generated.resolve())}
        result = subprocess.run(invocation, env=env, capture_output=True, text=True)
        if result.returncode == 0:
            print(result.stdout)
            subprocess.run([str(binary), "--nocapture"], check=True)
            return
        print(result.stderr[-2500:])
    raise RuntimeError("could not link repository-built dependencies into the fixture harness")


if __name__ == "__main__":
    import sys
    root = Path(__file__).resolve().parent
    crate_source = Path(sys.argv[2]) if len(sys.argv) > 2 else root / "📜️flate-tests.rs"
    externs = sys.argv[3:] or ("semio_framework_deflate", "semio_framework_value", "serde_json")
    build(root, Path(sys.argv[1]), crate_source, externs)
