"""🔬 Check the mounted picker carrier against the real WASI Source and value ledger."""
from pathlib import Path
from runpy import run_path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "picker-carrier"
FILES = {
    "bridge": "📜️original-media-command-bridge.rs",
    "command_owner": "📜️original-media-command-owner.rs",
    "guest": "📜️original-media-guest-reloan.rs",
    "ingress": "📜️original-media-ingress.rs",
    "receiver": "📜️original-media-paged-receiver.rs",
    "raw_picker": "raw-picker-receipts.rs",
    "publication": "📜️raw-picker-mounted-publication.rs",
}
CRATES = ("semio_framework_job", "semio_framework_replication_command_ingress", "semio_framework_value",
          "semio_framework_value_persistent", "semio_plugin_decl_emission")


def main():
    """🧾 Type-check the concrete receipt handoff, budget accounting and paid cleanup."""
    cache = Path(sys.argv[1]).resolve()
    OUT.mkdir(exist_ok=True)
    library = run_path(str(TICKET / "📜️pdf-original-png-graph-edit-check.py"))["library"]
    root = OUT / "carrier.rs"
    root.write_text('extern crate self as semio_plugin_decl_media;\n'
        + "".join(f'#[path = "{(TICKET / name).as_posix()}"] pub mod {key};\n' for key, name in FILES.items())
        + """pub mod original_media {
    pub use crate::ingress::OriginalMediaLoan;
    pub use crate::raw_picker::FormatReceipt;
    const LIMIT: usize = semio_framework_replication_command_ingress::COMMAND_MAXIMUM_BYTES;
    pub type RawMediaInput = crate::guest::OriginalGuestMediaInput<[u64; 8], LIMIT>;
    pub type RawMediaPicker = crate::raw_picker::RawPicker<[u64; 8], LIMIT>;
}
""")
    dirs = sorted(set(cache.glob("*/*/out")) | set((cache.parents[2] / "prototype/build").glob("*/*/out")))
    target = OUT / "libpicker_carrier.rmeta"
    args = ["--target", "wasm32-wasip2", "--edition=2021", "--crate-name=picker_carrier", "--crate-type=lib",
            "--emit=metadata", "-o", str(target), str(root)]
    for crate in CRATES:
        args += ["--extern", f"{crate}={library(cache, crate, '.rmeta', {})}"]
    for path in dirs:
        args += ["-L", f"dependency={path}"]
    response = OUT / "rustc.rsp"
    response.write_text("\n".join(args) + "\n")
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
                    "rustc", "@" + str(response)], check=True)
    print("[DEBUG] Verified the actual Original picker carrier and paid close against WASI")


if __name__ == "__main__":
    main()
