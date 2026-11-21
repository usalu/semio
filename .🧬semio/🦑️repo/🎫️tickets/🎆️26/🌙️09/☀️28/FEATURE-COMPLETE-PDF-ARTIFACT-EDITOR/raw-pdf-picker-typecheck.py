"""🔬 Type-check the real retained PDF document owner with the staged original picker API."""

import json
from pathlib import Path
import re
from shutil import copyfile
from runpy import run_path
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
CACHE = Path(sys.argv[1]).resolve()
OUT = TICKET / "🗑️generated/rm-pdf-check"
WASI = CACHE / "wasm32-wasip2/prototype/build"
COHORT = OUT / "deps"
WIRE = TICKET / "🗑️generated/rm-wire/raw-wasip2.rmeta"
CONTRACT_ONLY = "--work-contract" in sys.argv
STAGE = run_path(str(TICKET / "raw-pdf-picker-stage.py"))
STAGE["stage"]()
ROOT = STAGE["ROOT"]
OUT.mkdir(exist_ok=True)
COHORT.mkdir(exist_ok=True)
for native in (WASI, CACHE / "prototype/build"):
    for path in native.glob("*/*/out/*"):
        if path.suffix not in (".rmeta", ".dylib", ".so", ".dll"):
            continue
        link = COHORT / path.name
        if not link.exists():
            if link.is_symlink():
                link.unlink()
            try:
                link.symlink_to(path.resolve())
            except OSError:
                copyfile(path, link)
proposal = json.loads((OUT.parent / "raw-pdf-picker-overlay.json").read_bytes())["candidate"]


def check(name, entry, externs):
    output = OUT / f"lib{name}.rmeta"
    args = ["--target=wasm32-wasip2", "--edition=2021", "--crate-name", name, "--crate-type=rlib", "--emit=metadata",
            "-Cmetadata=raw-picker-stage", str(entry), "-o", str(output), f"-Ldependency={COHORT}", f"-Ldependency={OUT}"]
    for alias, path in externs.items():
        args += ["--extern", f"{alias}={path}"]
    response = OUT / f"{name}.rsp"
    response.write_text("\n".join(args) + "\n", encoding="utf8")
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{response}"], check=True)
    return output


def pinned(name):
    sources = list(WASI.glob(f"*/*/out/lib{name}-*.rmeta"))
    if len(sources) != 1:
        raise ValueError(f"Exact WASI metadata for {name} is unavailable in {WASI}")
    [source] = sources
    return source


def materialize(source, output, redirects, body=None):
    if body is None:
        body = proposal.get(str(source.relative_to(ROOT)))
    if body is None:
        body = source.read_text(encoding="utf8")

    def repath(match):
        target = (source.parent / match[2]).resolve()
        path = redirects.get(target, target)
        if not path.is_file():
            raise ValueError(f"Picker crate module is missing: {path}")
        return f'{match[1]}"{path.as_posix()}"'

    output.write_text(re.sub(r'(#\[path\s*=\s*)"([^"]+)"', repath, body), encoding="utf8")
    return output


if not WIRE.is_file() or not COHORT.is_dir():
    raise ValueError("Run the staged WASI wire and existing PDF metadata cohort gates first")
sdk = OUT.parent / "rm-sdk"
facade = OUT / "facade.rs"
module = (sdk / "🦀️.rs").read_text(encoding="utf8")
module = module.replace('#[path = "🧭️runtime.rs"]\npub mod runtime;\n', "")
module = re.sub(r'(#\[path\s*=\s*)"([^"]+)"', lambda match: f'{match[1]}"{(sdk / match[2]).as_posix()}"', module)
module = module.replace(next(line for line in module.splitlines() if line.startswith("//!")), "", 1)
deps = {name: pinned(name) for name in ("semio_framework", "semio_framework_job", "semio_framework_value", "semio_framework_value_persistent")}
recognized = OUT / "libsemio_framework_replication_command_ingress.rmeta"
copyfile(WIRE, recognized)
deps["semio_framework_replication_command_ingress"] = recognized
if CONTRACT_ONLY:
    api_source = """pub use semio_framework::Fault;
pub use semio_framework::kernel::{Effect, RequestId};
pub use semio_framework_job as __job;
pub use semio_framework_value as __value;
pub use semio_os_kernel as __kernel;
pub use semio_plugin_decl_app::ArtifactApp;
pub use semio_plugin_decl_emission::Emit;
pub use semio_plugin_decl_views::{AppOperationContext, NoConfigMutation, NoDraftMutation};
pub use semio_plugin_retained_work as retained_command;
pub mod app { pub use semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobContext; }
"""
    deps.update({name: pinned(name) for name in ("semio_os_kernel", "semio_plugin_decl_app", "semio_plugin_decl_emission",
                                               "semio_plugin_decl_views", "semio_plugin_retained_work")})
else:
    api_source = "pub use released_plugin::*;\n"
    deps["released_plugin"] = pinned("semio_plugin")
facade.write_text(api_source + "pub mod original_media {\n" + module + "}\n", encoding="utf8")
api = check("semio_plugin", facade, deps)

author = ROOT / STAGE["DOC"] / "🦀️.rs"
work = ROOT / STAGE["WORK"]
worker = OUT / "picker.rs"
worker.write_text(proposal[str(work.relative_to(ROOT))], encoding="utf8")
if CONTRACT_ONLY:
    source = author.read_text(encoding="utf8")
    start = source.index("pub enum ImagePlacement")
    placement = source[source.rfind("#[derive", 0, start):source.index("\n}", start) + 2]
    source = (author.parent / "📥️admission.rs").read_text(encoding="utf8")
    start = source.index("pub struct PngTarget")
    target = source[start:source.index("\npub enum PngInvocation", start)]
    fixture = OUT / "retained-work.rs"
    fixture.write_text(f'''use semio_plugin::{{AppOperationContext, Emit, Fault}};
pub struct PdfSnapshot;
pub struct PdfMutation;
pub type PdfMatrix = [f64; 6];
pub const IMPORT: &str = "import-local-image";
pub const INSERT_OPERATOR: u32 = u32::MAX;
pub const MAX_FILE_BYTES: u32 = 24_546;
{placement}
{target}
pub fn check_image_placement(_: &PdfSnapshot, _: ImagePlacement) -> Result<usize, Fault> {{ unreachable!() }}
pub fn revision_bytes(_: &AppOperationContext) -> [u8; 64] {{ unreachable!() }}
pub fn picker(_: &PdfSnapshot, _: &AppOperationContext, _: PngTarget, _: &str) -> Result<Emit<PdfMutation>, Fault> {{ unreachable!() }}
#[path="{worker.as_posix()}"] pub mod picker_work;
''', encoding="utf8")
    check("pdf_picker_retained_work_contract", fixture, {"semio_plugin": api})
    print("[DEBUG] Type-checked the staged worker on WASI against the real retained-command and declaration crates")
    sys.exit(0)
authored = materialize(author, OUT / "authors.rs", {work: worker})
doc = author.parent.parent / "🦀️.rs"
doc_file = materialize(doc, OUT / "document.rs", {author: authored})
entry = doc.parent / "📦️packages/🦀️rust/🦀️.rs"
actual = materialize(entry, OUT / "entry.rs", {doc: doc_file})
check("semio_s_artifact_stdio_pdf_editor_document", actual, {
    "semio_plugin": api,
    **{name: pinned(name) for name in ("semio_s_artifact_stdio_contract", "semio_framework_pixels",
                                      "semio_s_artifact_stdio_pdf_snapshot_2", "semio_s_artifact_stdio_pdf_mut_family_8")},
})
print("[DEBUG] Type-checked the full staged PDF editor document with the actual mounted retained-command trait on WASI")
