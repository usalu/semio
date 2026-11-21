"""🧾 Join the PDF retained PNG job to its concrete factory proof."""

import hashlib
import json
from pathlib import Path
import sys

TICKET = Path(__file__).resolve().parent
MACRO = r'''/// 🏭️ Declares the concrete factory joined to the PDF editor's retained PNG job proof.
#[macro_export]
macro_rules! pdf_local_image_import_factory {
    ($factory:ident, $app:ty) => {
        pub struct $factory {
            keys: [semio_plugin::retained_command::__action_bus::ToolFactoryKey; 1],
        }

        impl $factory {
            pub fn new(controller: &str) -> Self {
                use semio_plugin::retained_command::__action_bus::ToolFactoryKey;
                Self { keys: [ToolFactoryKey::new(controller, $crate::IMPORT)] }
            }
        }

        impl semio_plugin::retained_command::__action_bus::ToolJobFactory for $factory {
            type Payload = semio_plugin::retained_command::ArtifactRetainedCommandPayload<$app>;
            type Job = semio_plugin::retained_command::ArtifactRetainedCommandJob<$app>;

            fn keys(&self) -> &[semio_plugin::retained_command::__action_bus::ToolFactoryKey] { &self.keys }
            fn payload_schema_id(&self) -> &str { "stdio.pdf.local-png-import.v1" }
            fn classification(&self) -> semio_plugin::InteractiveJobClassification { semio_plugin::InteractiveJobClassification::Migrated }
            fn execution_contract(&self) -> semio_plugin::ToolExecutionContract { $crate::CONTRACT }

            semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: $crate::WIRE_BYTES, checkpoint_resume: false);
        }

        impl semio_plugin::ArtifactOwnedToolJobFactory for $factory {
            type Owner = $app;
            const TOOL_IDS: &'static [&'static str] = &[$crate::IMPORT];
            const DOCUMENT_SCHEMA: &'static str = <$app as semio_plugin::ArtifactApp>::DOCUMENT_SCHEMA;
            const PUBLICATION_CONTRACTS: &'static [semio_plugin::ArtifactToolPublicationContract] = &[
                semio_plugin::ArtifactToolPublicationContract { tool_id: $crate::IMPORT, lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact] },
            ];
        }
    };
}
'''


def exact(text, before, after):
    """📌 Replace one guarded source anchor."""
    assert text.count(before) == 1, before
    return text.replace(before, after, 1)


def build(base):
    """🧩 Return the exact owner, adapter and controller postimages."""
    local = base / "✏️editor/📥️local-image/🦀️.rs"
    admission = base / "✏️editor/📄️document/🖼️image-authoring/📥️admission.rs"
    controller = base / "✏️editor/🦀️.rs"
    files = {path: path.read_bytes() for path in (admission, local, controller)}
    source = files[admission].decode("utf8")
    source = exact(source, "use super::{PdfMutation, PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};", "use super::{PdfMutation, PdfSnapshot};")
    source = exact(source, "use semio_plugin::{ArtifactApp, Emit, Fault};", "use semio_plugin::{Emit, Fault};")
    source = exact(source, "use semio_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};\n", "")
    source = exact(source, "use semio_plugin::retained_command::__action_bus::{ToolFactoryKey, ToolJobFactory};\n", "")
    marker = "pub struct PngFactory<A: ArtifactApp> {"
    assert source.count(marker) == 1
    source = source[:source.index(marker)] + MACRO
    adapter = exact(files[local].decode("utf8"), "PngDecodeWork, PngFactory, PngInput", "PngDecodeWork, PngInput")
    adapter = exact(adapter, "pub type Factory = PngFactory<App>;", "semio_s_artifact_stdio_pdf_editor_document::pdf_local_image_import_factory!(PdfLocalImageImport, App);")
    root = exact(files[controller].decode("utf8"), "factory_type: image_import::Factory,", "factory_type: image_import::PdfLocalImageImport,")
    return files, {admission: source.encode("utf8"), local: adapter.encode("utf8"), controller: root.encode("utf8")}


def apply(base):
    """🔐 Check every preimage and write only the reviewed factory repair."""
    original, output = build(base)
    receipt = {}
    for path in output:
        assert len(str(path.resolve()).encode("utf-16-le")) // 2 <= 256, path
        assert path.read_bytes() == original[path], path
    for path, replacement in output.items():
        assert path.read_bytes() == original[path], path
        path.write_bytes(replacement)
        assert path.read_bytes() == replacement, path
        receipt[str(path)] = {"before": hashlib.sha256(original[path]).hexdigest(), "after": hashlib.sha256(replacement).hexdigest()}
    target = TICKET / "🗑️generated" / "pdf-image-factory-release.json"
    target.write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] Verified {len(receipt)} PDF PNG factory postimages")


if __name__ == "__main__":
    apply(Path(sys.argv[1]))
