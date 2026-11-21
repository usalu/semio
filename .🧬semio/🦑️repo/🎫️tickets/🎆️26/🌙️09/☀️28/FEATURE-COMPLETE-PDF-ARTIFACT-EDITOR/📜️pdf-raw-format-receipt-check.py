"""🧪 Link the exact raw format receipt laws into one current governed native caller cohort."""

from pathlib import Path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated" / "raw-format-receipt"
MODULES = {
    "bridge": "📜️original-media-command-bridge.rs",
    "command_owner": "📜️original-media-command-owner.rs",
    "guest": "📜️original-media-guest-reloan.rs",
    "ingress": "📜️original-media-ingress.rs",
    "receiver": "📜️original-media-paged-receiver.rs",
    "raw_picker": "raw-picker-receipts.rs",
    "raw_picker_pack": "raw-picker-pack.rs",
    "receipts": "raw-picker-receipts-test.rs",
}
CRATES = {
    "semio_framework_value": "86eb0cf0fad7ae66",
    "semio_framework_value_persistent": "d19346c8b0514097",
    "semio_framework_job": "689697170f9e2e90",
    "semio_framework_job_testing_caller": "382719391b729040",
    "semio_framework_replication_command_ingress": "8cf2689dbcc20a4d",
    "serde_json": "4e790474e9a23a5f",
}


def main():
    """🎟️ Compile and run the existing and prepaid terminal receipt cases."""
    OUTPUT.mkdir(parents=True, exist_ok=True)
    cache = Path(sys.argv[1]).resolve()
    source = OUTPUT / "law.rs"
    original = (TICKET / "🗑️generated" / "raw-png" / "focused.rs").read_text()
    declarations = "\n".join(f'#[path="{(TICKET / file).as_posix()}"] mod {module};' for module, file in MODULES.items())
    origin = Path.cwd() / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🎞️command/📥️custody/🎟️source/🦀️.rs"
    declarations += f'\n#[path="{origin.as_posix()}"] mod command_origin;'
    source.write_text(declarations + "\n" + original[original.index("use semio_framework_value::RetainedCloneGrant;"):])
    binary = OUTPUT / ("law.exe" if sys.platform == "win32" else "law")
    flags = ["--test", "--edition=2021", "--crate-name=raw_format_receipt", str(source), "-o", str(binary)]
    for crate, variant in CRATES.items():
        (library,) = cache.glob(f"*/{variant}/out/lib{crate}-{variant}.rlib")
        for suffix in (".rmeta", ".rlib"):
            flags += ["--extern", f"{crate}={library.with_suffix(suffix)}"]
    for directory in sorted(cache.glob("*/*/out")):
        flags += ["-L", f"dependency={directory}"]
    response = OUTPUT / "rustc.rsp"
    response.write_text("\n".join(flags) + "\n")
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
    subprocess.run([str(binary), "--nocapture"], check=True)


if __name__ == "__main__":
    main()
