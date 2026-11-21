"""🔬 Type-check the staged PDF image editor against cached production WASI libraries."""
from pathlib import Path
import json
import runpy
import subprocess
import sys

ticket = Path(__file__).resolve().parent
stage = runpy.run_path(str(ticket / "image-authoring-stage.py"))
output = stage["OUTPUT"]
output.mkdir(parents=True, exist_ok=True)
lane = Path(sys.argv[1])
target = lane / "wasm32-wasip2/prototype/build"
original, = (target / "semio-s-artifact-stdio-pdf-page-editor").glob("*/out/*.rmeta")
deps = dict(json.loads(original.with_suffix(".rmeta.semio-deps").read_text(encoding="utf-8"))["dependencies"])
libs = {}
for build in (target, lane / "prototype/build"):
    for item in build.glob("*/*/out/lib*"):
        if item.suffix not in (".rmeta", ".dylib", ".so", ".dll", ".rlib"):
            continue
        stem = item.stem.removeprefix("lib")
        if stem not in deps:
            continue
        if stem not in libs or item.suffix in (".rmeta", ".dylib", ".so", ".dll"):
            libs[stem] = item

def library(package):
    exact = [file for info, file in libs.items() if info.rsplit("-", 1)[0] == package]
    assert len(exact) == 1, (package, exact)
    return exact[0]


options = [
    "repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
    "--crate-name", "pdf_image_stage_check", "--crate-type=lib", "--edition=2024",
    "--target", "wasm32-wasip2", "--emit=metadata", "-o", str(output / "check.rmeta"),
]
for folder in dict.fromkeys(item.parent for item in libs.values()):
    options.extend(["-L", f"dependency={folder}"])
options.extend(["--extern", f"original_pdf={original}"])
aliases = {
    "value_derive": "semio_framework_value_derive",
    "protocol": "semio_os_kernel",
    "store": "semio_os_kernel",
    "dsl": "semio_os_kernel",
    "semio_framework_2d_compute": "semio_framework_2d_compute",
    "semio_framework_artifact_reference": "semio_framework_artifact_reference",
    "semio_framework_diagnostic": "semio_framework_diagnostic",
    "semio_framework_dsl_record": "semio_framework_dsl_record",
    "semio_framework_dsl_record_derive": "semio_framework_dsl_record_derive",
    "semio_framework_value": "semio_framework_value",
    "semio_plugin": "semio_plugin",
    "semio_s_artifact_stdio_contract": "semio_s_artifact_stdio_contract",
    "semio_s_artifact_stdio_pdf_mut_family_8": "semio_s_artifact_stdio_pdf_mut_family_8",
    "original_document": "semio_s_artifact_stdio_pdf_editor_document",
    "semio_ui_contract": "semio_ui_contract",
    "semio_ui_locale": "semio_ui_locale",
    "semio_framework_pixels": "semio_framework_pixels",
}
for alias, package in aliases.items():
    options.extend(["--extern", f"{alias}={library(package)}"])
options.append("-")

authored = stage["SOURCE"].split("\n", 1)[1].replace("use super::*;", "")
button = stage["BUTTON"].split("\n", 1)[1]
decoder = (ticket / "image-decode.rs").read_text(encoding="utf-8").split("\n", 1)[1]
admission = (ticket / "image-pick-admission.rs").read_text(encoding="utf-8").split("\n", 1)[1]
controller = stage["picker_changes"]()[0][2].split("\n")
controller = "\n".join(line for line in controller if not line.startswith("//!"))
controller = controller.replace("📥️local-image/🦀️.rs", (ticket / "image-picker.rs").as_posix())
controller = controller.replace("use crate::editor::pdf17::modes::edit;", "")
controller = controller.replace("use crate::editor::pdf17::modes::edit::windows::main;", "")
natural_start = controller.index("    fn natural_file_codec()")
natural_end = controller.index("    fn import_media(", natural_start)
controller = controller[:natural_start] + controller[natural_end:]
birth_start = controller.index("    fn reference_birth()")
birth_end = controller.index("    /// ✏️ Replaces", birth_start)
controller = controller[:birth_start] + controller[birth_end:]
wrapper = '''
#![allow(dead_code)]
extern crate self as semio_s_artifact_stdio_pdf_editor_document;
pub use authoring::*;
mod standards { pub mod v1_7 { pub mod subsets { pub mod base { pub mod schema {
    pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::{mutations, snapshot};
    pub const PDF_ARTIFACT_SCHEMA_ID: &str = "pdf";
} } } } }
mod authoring {
    use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::{mutations::*, snapshot::*};
    use original_document::fault;
    use semio_plugin::Fault;
''' + authored + '''
    mod decode {
''' + decoder + '''
    }
    pub use decode::*;
    mod admission {
''' + admission + '''
    }
    pub use admission::*;
}
mod button {
''' + button + '''
}
mod editor {\n    pub mod page {
        pub use original_pdf::page::*;
    }
    pub mod pdf17 {
        mod edit {
            pub const PDF17_EDIT_MODE_ID: &str = "edit";
            pub fn definition() -> semio_plugin::ModeDefinition { unimplemented!() }
            pub mod windows { pub mod main {
                use crate::editor::page;
                use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
                pub use crate::editor::page::{BODY_KEY, WINDOW_KIND_ID};
                pub fn definition() -> semio_plugin::WindowKindDefinition { page::window_definition() }
                pub fn render_windowed(snapshot: &PdfSnapshot, _: &semio_plugin::TreeWindows, _: semio_ui_locale::Locale)
                    -> semio_plugin::UiAssemblyResult<semio_ui_contract::BuiltNode> { page::render_window(snapshot) }
            } }
        }
        use edit::windows::main;
''' + controller + '''
    }
}
'''
result = subprocess.run(options, input=wrapper, text=True)
if result.returncode:
    raise SystemExit(result.returncode)
print("[DEBUG] Staged PDF image editor type-checked with production WASI libraries")
