"""🧪 Links checked native Save with the repository's actual protocol and descriptor packages."""

from pathlib import Path
import json
from runpy import run_path
from shutil import copyfile
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated/native-wgpu-save"
STAGE = OUT / "bridge"
CACHE = Path(sys.argv[1]).resolve()
PREFIX = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]

run_path(str(TICKET / "📜️native-wgpu-save-stage.py"), run_name="__main__")
entries = json.loads((STAGE / "receipt.json").read_bytes())
for item in entries:
    if item["source"].endswith(".rs"):
        subprocess.run([*PREFIX, "-Zparse-crate-root-only", "--edition=2021", str(STAGE / item["source"])], check=True)

def staged(name):
    """📌 Select one exact production source from the guarded receipt."""
    [entry] = [item for item in entries if item["file"].endswith("/" + name)]
    return (STAGE / entry["source"]).as_posix()

source = OUT / "law.rs"
source.write_text(f'''extern crate self as semio_plugin;
pub use semio_plugin_decl_media::ArtifactOutputChunks;
pub mod app {{
    pub use semio_plugin_decl_app::MediaArtifactDescriptor;
    pub use semio_framework::manifest::MediaWireFormat;
}}
#[path="{staged("🧮️declaration.rs")}"] mod declaration;
#[path="{staged("📥️driver.rs")}"] mod driver;
#[path="{staged("📤️sink.rs")}"] mod sink;
#[path="{staged("📎️reference.rs")}"] mod reference;
#[path="{(TICKET / "📜️native-wgpu-save-protocol.rs").as_posix()}"] mod protocol_test;
''', encoding="utf8")
artifacts = list(CACHE.glob("*/*/out/*"))
[kernel] = [file for file in artifacts if file.name.startswith("libsemio_os_kernel-") and file.suffix == ".rlib"]
keys = dict(json.loads(kernel.with_suffix(".rmeta.semio-deps").read_bytes())["dependencies"])
metadata = OUT / "deps"
metadata.mkdir(exist_ok=True)
for file in artifacts:
    if file.suffix not in (".rmeta", ".so", ".dll", ".dylib"):
        continue
    link = metadata / file.name
    if not link.exists():
        try:
            link.symlink_to(file.resolve())
        except OSError:
            copyfile(file, link)
binary = OUT / ("checked-save.exe" if sys.platform == "win32" else "checked-save")
args = ["--edition=2021", "--test", "--crate-name=native_wgpu_save", "-Copt-level=1",
        "-Awarnings", str(source), "-o", str(binary), f"-Ldependency={metadata}"]
crates = ("semio_os_kernel", "semio_os_spr", "semio_plugin_decl_media", "semio_plugin_decl_app",
          "semio_framework", "semio_framework_value", "semio_framework_async")
if "--correlation-only" in sys.argv[2:]:
    marker = "pub(super) fn one_save_reply("
    _, definition = Path(staged("📥️driver.rs")).read_text(encoding="utf8").split(marker)
    test_name = "fn accepts_only_the_correlated_reply_from_the_kernel_frame_batch("
    law = (TICKET / "📜️native-wgpu-save-protocol.rs").read_text(encoding="utf8")
    _, fixture = law.split(test_name)
    source = OUT / "correlate.rs"
    source.write_text(f"""use semio_os_spr::{{AppFrame, AppDocumentIdentity}};
mod driver {{
use super::AppFrame;
{marker}{definition}
}}
fn owner() -> AppDocumentIdentity {{
    AppDocumentIdentity {{ app_instance_id: 51, base_revision: 0x2030_4050_6070_8010, parent_document_id: Some("pdf/fixture".into()) }}
}}
#[test]
{test_name}{fixture}
""", encoding="utf8")
    args[args.index(str(OUT / "law.rs"))] = str(source)
    binary = binary.with_name("correlate" + binary.suffix)
    args[args.index("-o") + 1] = str(binary)
    crates = ("semio_os_spr",)
for name in crates:
    candidates = [path for path in artifacts if path.name.startswith(f"lib{name}-") and path.suffix == ".rlib"]
    if len(candidates) > 1:
        candidates = [path for path in candidates if path.name[3:-5] in keys]
    [path] = candidates
    args.extend(("--extern", f"{name}={path.with_suffix('.rmeta')}", "--extern", f"{name}={path}"))
args.extend(f"-Ldependency={path}" for path in sorted({path.parent for path in artifacts if path.suffix == ".rlib"}))
rsp = OUT / "checked-save.rsp"
rsp.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run([*PREFIX, f"@{rsp}"], check=True)
subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(binary), "--nocapture"], check=True)
