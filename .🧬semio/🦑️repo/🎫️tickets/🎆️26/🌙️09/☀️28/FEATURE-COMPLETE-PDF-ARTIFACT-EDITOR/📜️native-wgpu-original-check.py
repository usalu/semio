"""🧪 Verifies guarded WGPU transport with the actual registered kernel's Rust types."""
from hashlib import sha256
import json
from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
STAGE = GENERATED / "native-wgpu-save/original-picker"
RUSTC = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]


def check_stage():
    """🧾 Confirms exact preimages and prepares the real-case runner."""
    receipt = json.loads((STAGE / "receipt.json").read_text())
    installed_path = STAGE.parent / "original-picker-installed.json"
    installed = installed_path.exists()
    if installed: assert json.loads(installed_path.read_text()) == receipt
    later = [STAGE.parent / receipt for receipt in ("original-pacing-installed.json", "extension-query-installed.json")]
    overlays = [{row["file"]: row for row in json.loads(path.read_text())} for path in later if installed and path.exists()]
    for row in receipt:
        target = Path(row["file"])
        expected = row["after" if installed else "before"]
        for newer in overlays:
            if row["file"] in newer:
                assert newer[row["file"]]["before"] == expected
                expected = newer[row["file"]]["after"]
        assert (sha256(target.read_bytes()).hexdigest() if target.exists() else None) == expected, target
        assert sha256((STAGE / row["source"]).read_bytes()).hexdigest() == row["after"], row["file"]
    media = next(row for row in receipt if row["file"].endswith("/🎞️original-media/🦀️.rs"))
    queue = next(row for row in receipt if row["file"].endswith("/🧮️kernel-core/📥️original-media.rs"))
    (STAGE / "media.rs").write_bytes((STAGE / media["source"]).read_bytes())
    (STAGE / "queue.rs").write_bytes((STAGE / queue["source"]).read_bytes())
    (STAGE / "law.rs").write_bytes((TICKET / "📜️native-wgpu-original-law.rs").read_bytes())


def native_args(log):
    """🔗 Selects the completed native kernel and Value artifacts with their governed dependency directories."""
    import re
    import shlex
    lines = log.read_text(errors="replace").splitlines()
    kernel, value = (next(line for line in reversed(lines) if "Running `" in line and f"--crate-name {name} " in line)
                     for name in ("semio_framework_kernel", "semio_framework_value"))
    def artifact(row):
        args = shlex.split(row.strip().removeprefix("Running `").removesuffix("`"))
        name = args[args.index("--crate-name") + 1]
        out = Path(args[args.index("--out-dir") + 1])
        digest = re.search(r"extra-filename=(-[a-f0-9]+)", row).group(1)
        path = out / f"lib{name}{digest}.rlib"
        assert path.is_file(), path
        return name, path
    names = [artifact(row) for row in (kernel, value)]
    cache = names[0][1].parents[3]
    dirs = sorted({p.parent for p in cache.glob("*/*/out/*") if p.suffix in (".rmeta", ".rlib", ".dylib", ".so", ".dll")})
    dirs += [path.parent for _, path in names]
    return [f"--extern={name}={path.with_suffix(suffix)}" for name, path in names for suffix in (".rmeta", ".rlib")] + [f"-Ldependency={p}" for p in dirs]


def main():
    """🎛️ Builds the test against exact repository types and optionally executes the shared cases."""
    check_stage()
    args = ["--edition=2024", "--crate-name=native_original_media_law", str(STAGE / "law.rs")]
    if len(sys.argv) == 1:
        paired = GENERATED / "reserved-types"
        kernel = paired / "libsemio_framework_kernel.rmeta"
        response = (paired / "semio_framework_kernel.rsp").read_text().splitlines()
        value = next(line for line in response if "semio_framework_value=" in line).split("=", 1)[1]
        if '"' in value: value = value.strip('"')
        args += ["--target=wasm32-wasip2", "--emit=metadata", "-o", str(STAGE / "liblaw.rmeta"),
                 f"--extern=semio_framework_kernel={kernel}", f"--extern=semio_framework_value={value}"]
        args += [item for i, item in enumerate(response) if item == "-L" or (i and response[i - 1] == "-L") or item.startswith("-Ldependency=")]
        args += [f"-Ldependency={paired}", f"-Ldependency={GENERATED / 'rm-wire'}"]
    else:
        binary = STAGE / "native-original-law"
        args += ["-o", str(binary), *native_args(Path(sys.argv[1]))]
    rsp = STAGE / "law.rsp"
    rsp.write_text("\n".join(args) + "\n")
    subprocess.run([*RUSTC, f"@{rsp}"], check=True)
    if len(sys.argv) == 1:
        print("[DEBUG] Source checked with real registered kernel and command-ingress WASI metadata")
        return
    fixture = json.loads((TICKET / "original-media-react-cases.json").read_text())
    png = GENERATED / "paged-media-receiver/raw-picker.png"
    loan = fixture["request"]["loan"]
    for case in fixture["cases"]:
        command = ["repo", "capacity", "run", "--memory=256", "--", str(binary), str(png), case["trigger"],
                  *map(str, [loan[key] for key in ("operationId", "generation", "instanceId", "documentId", "baseRevision")]),
                  fixture["seq"], str(fixture["request"]["maximumFileBytes"])]
        result = subprocess.check_output(command, text=True).splitlines()
        assert result[-4:] == [case["expected"], ",".join(case["phase"]), ",".join(map(str, case["reads"])),
                               ",".join(map(str, case["progress"]))], (case["name"], result)
        print("[DEBUG] Passed " + case["name"])


if __name__ == "__main__":
    main()
