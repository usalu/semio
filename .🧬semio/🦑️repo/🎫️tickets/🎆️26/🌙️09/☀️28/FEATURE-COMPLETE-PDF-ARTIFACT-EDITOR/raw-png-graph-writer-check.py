"""🧪 Compiles the exact writer with framework effects and a reduced COS fixture shell."""

import hashlib
import json
from pathlib import Path
import runpy
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated/raw-png-writer"
STAGE = runpy.run_path(str(TICKET / "raw-png-graph-stage.py"))
CONSUMERS = "--consumers" in sys.argv
WASI = "--wasi" in sys.argv
if CONSUMERS:
    combined = runpy.run_path(str(TICKET / "raw-png-graph-consumers-stage.py"))
    combined["stage"]()
    overlay = combined["OUT"]
else:
    STAGE["stage"]()
    overlay = STAGE["PROBE"]
OUT.mkdir(parents=True, exist_ok=True)
payload = (OUT.parent / "large-picked/large-picked.png").read_bytes()
(OUT / "identity").write_bytes(hashlib.sha256(payload).digest())
candidates = json.loads(overlay.read_text(encoding="utf8"))["candidate"]
resource = candidates[str(STAGE["OWNER"])]
original = resource[resource.index("pub fn artifact_reference("):resource.index("/// 🛂️ Native admission")]
definitions = "\n".join(line for line in resource.splitlines() if line.startswith("const RAW_"))
native = 'use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};\n' + definitions + "\n" + original
native += resource[resource.index("pub mod png_source {"):]
(OUT / "native.rs").write_text(native, encoding="utf8")
(OUT / "writer.rs").write_text(candidates[str(STAGE["WRITER"])].replace(
    "semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::png_source",
    "crate::native::png_source"), encoding="utf8")

entry = OUT / "harness.rs"
entry.write_text(f"""
pub mod standards {{
    pub mod v1_7 {{ pub mod subsets {{ pub mod base {{
        pub mod schema {{ pub mod snapshot {{ pub use crate::{{PdfDictEntry, PdfImage, PdfImageBody, PdfIndirectObject, PdfObject, PdfStreamFilter}}; }} }}
        pub mod io {{ pub mod error {{ pub use crate::{{PResult, PdfEngineError}}; }} }}
    }}}}}}
}}
pub type PResult<T> = Result<T, PdfEngineError>;
pub enum PdfEngineError {{ Malformed(String) }}
pub enum PdfStreamFilter {{ Flate {{ predictor: Option<()> }} }}
pub enum PdfObject {{ Int(i64), Name(String), Str(Vec<u8>), Text(String), Dict(Vec<PdfDictEntry>), Stream {{ dict: Vec<PdfDictEntry>, data: Vec<u8>, filters: Vec<PdfStreamFilter> }} }}
impl PdfObject {{
    pub fn dict_get(&self, name: &str) -> Option<&Self> {{
        match self {{ Self::Dict(dict) | Self::Stream {{ dict, .. }} => dict.iter().find(|entry| entry.key == name).map(|entry| &entry.value), _ => None }}
    }}
    pub fn as_name(&self) -> Option<&str> {{ if let Self::Name(name) = self {{ Some(name) }} else {{ None }} }}
}}
pub struct PdfDictEntry {{ pub key: String, pub value: PdfObject }}
impl PdfDictEntry {{ pub fn new(key: &str, value: PdfObject) -> Self {{ Self {{ key: key.into(), value }} }} }}
#[derive(Clone, Copy)]
pub struct ObjRef {{ pub num: u32, pub gen: u16 }}
pub struct PdfIndirectObject {{ pub id: ObjRef, pub value: PdfObject }}
pub enum PdfImageBody {{ Artifact {{ reference: semio_framework_artifact_reference::ArtifactRef }}, Samples }}
pub struct PdfImage {{ pub body: PdfImageBody }}
pub struct InsertObject {{ pub id: ObjRef, pub value: PdfObject, pub index: Option<usize>, pub admitted_stream_roles: Option<()> }}
pub enum PdfMutation {{ InsertObject(InsertObject) }}
mod native;
mod writer;
#[path = "{TICKET / 'raw-png-graph-writer-law.rs'}"]
mod laws;
""", encoding="utf8")

cache = Path(sys.argv[1]).resolve()
target = OUT / "probe"
args = ["--test", "--edition=2021", str(entry), "-o", str(target)]
if WASI:
    args += ["--target=wasm32-wasip2", "--emit=metadata"]
    dependencies = OUT / "deps"
    dependencies.mkdir(exist_ok=True)
    host = cache.parents[2] / "prototype/build"
    for folder, extensions in ((cache, (".rmeta",)), (host, (".so", ".dylib", ".dll"))):
        for path in folder.glob("*/*/out/*"):
            target_link = dependencies / path.name
            if path.suffix in extensions and not target_link.exists():
                target_link.symlink_to(path)
    args += [f"-Ldependency={dependencies}"]
for spec in sys.argv[2:]:
    if "=" not in spec:
        continue
    name, key = spec.split("=", 1)
    [meta] = cache.glob(f"*/*/out/lib{name}-{key}.rmeta")
    args += ["--extern", f"{name}={meta}"]
    if not WASI:
        args += ["--extern", f"{name}={meta.with_suffix('.rlib')}"]
if not WASI:
    args += [f"-Ldependency={path}" for path in sorted({
        p.parent for p in cache.glob("*/*/out/*") if p.suffix in (".rlib", ".rmeta", ".so", ".dylib", ".dll")
    })]
response = OUT / "rustc.args"
response.write_text("\n".join(args) + "\n", encoding="utf8")
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{response}"], check=True)
if not WASI:
    subprocess.run(["repo", "capacity", "run", "--memory=256", "--", str(target), "--nocapture"], check=True)
else:
    print("[DEBUG] Type-checked the staged writer and bounded cold PNG decoder on WASI")
