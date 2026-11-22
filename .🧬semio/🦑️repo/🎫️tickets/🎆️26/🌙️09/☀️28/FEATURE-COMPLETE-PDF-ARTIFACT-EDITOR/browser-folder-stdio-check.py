"""🧪 Compile the exact staged stdio helper against the production-compatible native disk closure."""
import os
from pathlib import Path
import re
import runpy
import subprocess

ticket = Path(__file__).resolve().parent
root = next(parent for parent in ticket.parents if (parent / "Cargo.toml").is_file())
output = ticket / "🗑️generated/browser-first-archive"
output.mkdir(parents=True, exist_ok=True)
readback = runpy.run_path(str(ticket / "native-archive-readback-stage.py"))
live_path = next(root.glob("🧰*/🛍*/💻*/🔨*/🏪*/🔄*/🖥*/📥*/🔄*/🦀*.rs"))
live = live_path.read_text()
binary = output / ("semio-folder-journal.exe" if os.name == "nt" else "semio-folder-journal")
prefix = ["repo", "tools", "exec", "--light"]

def source(name):
    return (ticket / name).read_text()

def extract(text, start_token, closing="\n}"):
    start = text.index(start_token)
    return text[start:text.index(closing, start) + len(closing)]

def syntax_only(code):
    code = re.sub(r"^//!.*$|^#\[value\(.*\)\]$|^ *#\[serde\(.*\)\]$", "", code, flags=re.MULTILINE)
    return re.sub(r",?\s*(?:serde|semio_framework_value(?:_derive)?)::\w+", "", code)

found = list((root / ".cache/b/r/m/u/l").glob("*/prototype/build/*/*/out/libsemio_framework_hash-*.rlib"))
assert found, "the focused closure requires the already built production hash library"
hash_library = found[0]
cache = hash_library.parents[3]
directories = {path.parent for path in cache.glob("*/*/out/*") if path.suffix in (".rmeta", ".rlib", ".dylib", ".so", ".dll")}

generated_native = ticket / "🗑️generated/native-archive-readback"
constants = (generated_native / "records.rs").read_text().split("pub const DOCUMENT_ARCHIVE_MAXIMUM_BYTES")[0]
folder = "\n".join([
    constants, "#[derive(Clone)]",
    extract(live, "pub struct FolderEventLogStorage {"), "#[derive(Clone)]", extract(live, "struct FolderEvent {"),
    extract(live, "struct FolderEventReader<'a> {"), extract(live, "impl<'a> FolderEventReader<'a> {"),
    extract(live, "fn folder_writer(folder:"), "impl FolderEventLogStorage {",
    *(extract(live, anchor, "\n    }") for anchor in (
        "    pub fn new(folder:", "    pub fn event_path(", "    fn checksum(", "    fn encode_text(",
        "    fn encode_event(", "    fn encode_data(", "    fn decode_event(", "    fn refuse_repository_root(",
        "    fn append_checked(", "    fn append_checked_with_sync(", "    fn scan_tail(", "    fn events_from(",
        "    pub async fn write_archive(", "    async fn append_archive(",
    )),
    live[live.index("    #[cfg(unix)]\n    pub(crate) fn sync_directory"):live.index("    /// 🩹")],
    """fn event_pair(event: &FolderEvent) -> Result<(Vec<u8>, Vec<u8>), VcsError> {
        if event.kind == DOCUMENT_ARCHIVE_PUT_EVENT {
            let carrier = block_on(decode_document_archive_bytes(&event.primary))?;
            return Ok((carrier.parent_pack, carrier.parent_spr));
        }
        Ok((event.primary.clone(), event.secondary.clone()))
    }""", "}",
])
module_sources = {
    "folder": folder,
    "archive_readback": (generated_native / "readback.rs").read_text(),
    "receipt": syntax_only(source("native-first-archive-receipt.rs").split("\nmod u64_decimal")[0]).replace("use protocol::wire::", "use crate::wire::"),
    "first_archive_exchange": source("native-first-archive-exchange.rs").replace("use protocol::wire::", "use crate::wire::"),
    "wire_records": syntax_only(source("natural-pdf-wire-records.rs")),
    "disk": source("native-first-archive-disk.rs"),
    "entry": source("browser-folder-stdio.rs").replace("fn main() -> ExitCode", "pub fn child_main() -> ExitCode"),
}
for name, code in module_sources.items():
    (output / f"{name}.rs").write_text(code.lstrip().split("\n", 1)[1] if code.lstrip().startswith("//!") else code.strip(), encoding="utf-8")
entry = output / "closure.rs"
entry.write_text(source("browser-folder-stdio-proof.rs"), encoding="utf-8")
toolchain = re.search(r'^channel = "([^"]+)"', (root / "rust-toolchain.toml").read_text(), re.M).group(1)
args = ["--edition=2021", "-O", str(entry), "--extern", f"semio_framework_hash={hash_library.with_suffix('.rmeta')}", "--extern", f"semio_framework_hash={hash_library}"]
args.extend(f"-Ldependency={directory}" for directory in sorted(directories))
args.extend(["-o", str(binary)])
arg_file = output / "native.args"
arg_file.write_text("\n".join(args) + "\n", encoding="utf-8")
subprocess.run(prefix + ["rustup", "--", "run", toolchain, "rustc", "@" + str(arg_file)], cwd=root, check=True)
