import os
import re
import subprocess
import sys
from pathlib import Path


def main() -> int:
    ticket = Path(__file__).resolve().parent
    generated = ticket / "🗑️generated"
    ownership = Path(sys.argv[1]).resolve()
    compiler = ["repo", "tools", "exec", "rustup", "--", "run", sys.argv[2], "rustc"]
    environment = {**os.environ, "SEMIO_LANGUAGE": "en", "RUSTC_ICE": str(generated)}
    metadata = subprocess.check_output(
        [*compiler, "-Zls=root", str(ownership.with_suffix(".rmeta"))],
        env=environment,
        text=True,
    )
    names = re.findall(r"^\d+ ([\w]+-[\da-f]+) hash ", metadata, re.MULTILINE)
    artifacts = {
        artifact.stem.removeprefix("lib"): artifact
        for artifact in ownership.parents[3].glob("*/*/out/*")
        if artifact.suffix in {".rlib", ".so", ".dll", ".dylib"}
    }
    parent_directories = sorted({ownership.parent, *(artifacts[name].parent for name in names if name in artifacts)})
    target = generated / ("original-lifecycle-phase-loan.exe" if os.name == "nt" else "original-lifecycle-phase-loan")
    options = [
        "--edition=2021",
        *(f"-Ldependency={directory}" for directory in parent_directories),
    ]
    libraries = {"semio_plugin_ownership": ownership}
    for crate in ["semio_framework", "semio_framework_job", "semio_framework_value"]:
        name = next(name for name in names if name.split("-")[0] == crate)
        libraries[crate] = artifacts[name]
    for crate, artifact in libraries.items():
        options.extend([f"--extern={crate}={artifact.with_suffix('.rmeta')}", f"--extern={crate}={artifact}"])
    if "--canonical" in sys.argv[3:]:
        module = Path.cwd() / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏛️ownership/⚠️failure/🦀️.rs"
        source = generated / "scoped-lifecycle.rs"
        source.write_text(
            'pub use semio_plugin_ownership::bounded_ownership::{plugin_grant_funds, PluginLifecycleStep};\n'
            f'#[path = r#"{module.as_posix()}"#]\npub mod lifecycle;\n',
            encoding="utf-8",
        )
        helper = generated / "libscoped_lifecycle.rlib"
        compile_options = [*options, "--crate-name=scoped_lifecycle", "--crate-type=rlib", "-o", str(helper), str(source)]
        arguments = generated / "scoped-lifecycle.args"
        arguments.write_text("\n".join(compile_options), encoding="utf-8")
        subprocess.run([*compiler, "@" + str(arguments)], env=environment, check=True)
        options.extend(["--cfg=canonical_lifecycle", f"--extern=scoped_lifecycle={helper}"])
    options.extend(["--crate-name=original_lifecycle_phase_loan", "--test"])
    options.extend(["-o", str(target), str(ticket / "original-lifecycle-phase-loan.rs")])
    arguments = generated / "original-lifecycle-phase-loan.args"
    arguments.write_text("\n".join(options), encoding="utf-8")
    subprocess.run([*compiler, "@" + str(arguments)], env=environment, check=True)
    return subprocess.run([str(target)], env=environment, check=False).returncode


if __name__ == "__main__":
    raise SystemExit(main())
