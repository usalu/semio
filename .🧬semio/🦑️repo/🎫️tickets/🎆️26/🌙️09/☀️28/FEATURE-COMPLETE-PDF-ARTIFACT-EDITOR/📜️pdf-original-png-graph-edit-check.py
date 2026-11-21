"""🔬 Type-check the paid PDF PNG edit assembler against governed WASI libraries."""
import json
from pathlib import Path
import shutil
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUT = GENERATED / "raw-png-edit"
BASE = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base")
DOMAIN = BASE / "✏️editor/📄️document/🖼️image-authoring"
OWNER = BASE / "🚪️io/📦️foreign-artifacts/🦀️.rs"
IMPORT = "semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts"
CRATES = (
    "semio_framework_value", "semio_framework_hash", "semio_framework_artifact_reference", "semio_framework_pixels",
    "semio_os_kernel",
    "semio_plugin", "semio_s_artifact_stdio_pdf_mut_family_8", "semio_s_artifact_stdio_pdf_editor_document",
)


def materialize(overlay):
    """🧩 Keep actual PDF wire types and named writer logic while isolating the new assembly."""
    owner = overlay[str(OWNER)]
    start = owner.index("pub const fn artifact_reference_bytes")
    end = owner.index("/// 🛂️ Native admission", start)
    definitions = "\n".join(line for line in owner.splitlines() if line.startswith("const RAW_"))
    native = "use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};\n" + definitions + "\n"
    native += owner[start:end] + owner[owner.index("pub mod png_source {"):]
    (OUT / "native.rs").write_text(native)
    writer = overlay[str(DOMAIN / "🧱️graph-writer.rs")].replace(IMPORT, "crate::native")
    (OUT / "writer.rs").write_text(writer)
    placement = overlay[str(DOMAIN / "🏞️graph-placement.rs")]
    start = placement.index("/// 📍")
    end = placement.index("/// 🖼️ Opens", start)
    imports = "use super::*;\nuse crate::native::png_source::PngPlane;\n"
    (OUT / "placement.rs").write_text(imports + placement[start:end])
    author = overlay[str(DOMAIN / "🦀️.rs")]
    start = author.index("fn place_named_image_bodies(")
    end = author.index("#[path", start)
    (OUT / "named.rs").write_text(author[start:end])
    assembler = (TICKET / "📜️pdf-original-png-graph-edit.rs").read_text().replace(IMPORT, "crate::native")
    (OUT / "edit.rs").write_text(assembler)
    entry = OUT / "matrix.rs"
    entry.write_text("""
pub use semio_s_artifact_stdio_pdf_mut_family_8::standards;
mod native;
mod authoring {
    use crate::standards::v1_7::subsets::base::schema::{mutations::*, snapshot::*};
    use semio_plugin::Fault;
    use semio_s_artifact_stdio_pdf_editor_document::{ImagePlacement, fault, check_image_placement};
    include!("named.rs");
    include!("children.rs");
}
""")
    (OUT / "children.rs").write_text("""
mod writer;
pub use writer::PngGraphWriter;
mod placement;
pub use placement::place_graph_png_named;
pub mod edit;
""".replace("mod writer;", '#[path = "writer.rs"] mod writer;').replace("mod placement;", '#[path = "placement.rs"] mod placement;').replace("pub mod edit;", '#[path = "edit.rs"] pub mod edit;'))
    return entry


def library(cache, crate, extension, identity):
    """🪪 Match each duplicated native prerequisite to the registered PDF compiler cohort."""
    paths = list(cache.glob(f"*/*/out/lib{crate}-*{extension}"))
    if len(paths) != 1:
        expected = {name for name, _ in identity.get("dependencies", [])}
        paths = [path for path in paths if path.stem.removeprefix("lib") in expected]
    (path,) = paths
    return path


def add_format(source, overlay, mounted):
    """🎞️ Mount the real staged receiver and PDF decoder interfaces around the assembly type check."""
    modules = {
        "bridge": "📜️original-media-command-bridge.rs",
        "command_owner": "📜️original-media-command-owner.rs",
        "guest": "📜️original-media-guest-reloan.rs",
        "ingress": "📜️original-media-ingress.rs",
        "receiver": "📜️original-media-paged-receiver.rs",
    }
    header = source.read_text()
    if mounted:
        header += """
mod raw_media {
    pub use picker_carrier::guest::OriginalGuestMediaInput;
    pub use picker_carrier::original_media::OriginalMediaLoan;
    pub const RAW_MEDIA_BYTES: usize = semio_framework_replication_command_ingress::COMMAND_MAXIMUM_BYTES;
    pub type RawMediaTarget = [u64; 8];
}
"""
    else:
        for module, name in modules.items():
            header += f'\n#[path = "{(TICKET / name).as_posix()}"] mod {module};\n'
        header += """
    mod raw_media {
        pub use crate::guest::OriginalGuestMediaInput;
        pub use crate::ingress::OriginalMediaLoan;
        pub const RAW_MEDIA_BYTES: usize = semio_framework_replication_command_ingress::COMMAND_MAXIMUM_BYTES;
        pub type RawMediaTarget = [u64; 8];
    }
    """
    header += "\npub mod picker_testing { pub use crate::authoring::format::PngFormatWork; }\n"
    source.write_text(header)
    for original, target in ("🎞️decode-original.rs", "decode.rs"), ("📬️format-work.rs", "format.rs"):
        text = overlay[str(DOMAIN / original)].replace(IMPORT, "crate::native").replace("semio_plugin::original_media", "crate::raw_media")
        (OUT / target).write_text(text)
    children = OUT / "children.rs"
    children.write_text(children.read_text() + '\n#[path = "decode.rs"] mod decode;\nuse decode::RawPngWork;\n'
                        + '#[path = "format.rs"] pub mod format;\nuse edit::PngGraphEdit;\n')


def main():
    """🧪 Type-check the governed WASI prerequisites or run the exact native fixture."""
    cache = Path(sys.argv[1]).resolve()
    native = "--native-law" in sys.argv
    OUT.mkdir(parents=True, exist_ok=True)
    mounted = "--mounted-contract" in sys.argv
    format_check = "--format" in sys.argv or mounted
    overlay_name = "raw-png-format-overlay.json" if format_check else "raw-png-consumers-overlay.json"
    overlay = json.loads((GENERATED / overlay_name).read_text())["candidate"]
    source = materialize(overlay)
    if format_check:
        assert not native
        add_format(source, overlay, mounted)
    directories = set(cache.glob("*/*/out"))
    flags = ["--edition=2021", "--crate-name=pdf_raw_png_edit", str(source)]
    identity = {}
    if native:
        (root,) = cache.glob("*/*/out/libsemio_s_artifact_stdio_pdf_editor_document-*.rmeta.semio-deps")
        identity = json.loads(root.read_text())
        (OUT / "law.rs").write_bytes((TICKET / "📜️pdf-original-png-graph-edit-law.rs").read_bytes())
        (OUT / "input.png").write_bytes((GENERATED / "raw-image-oracle" / "large-picked.png").read_bytes())
        source.write_text(source.read_text() + "\nmod law;\n")
        target = OUT / ("law.exe" if sys.platform == "win32" else "law")
        flags += ["--test", "-o", str(target)]
    else:
        metadata = OUT / ("md-" + cache.parents[2].name)
        metadata.mkdir(exist_ok=True)
        for directory in sorted(directories):
            for artifact in directory.glob("*.rmeta"):
                link = metadata / artifact.name
                if not link.exists():
                    try:
                        link.symlink_to(artifact)
                    except (NotImplementedError, OSError):
                        shutil.copyfile(artifact, link)
        directories.update((cache.parents[2] / "prototype" / "build").glob("*/*/out"))
        target = OUT / ("libpdf_raw_png_edit.rmeta" if mounted else "lib.rmeta")
        flags += ["--target=wasm32-wasip2", "--crate-type=lib", "--emit=metadata", "-o", str(target), "-L", f"dependency={metadata}"]
    crates = CRATES + ("semio_framework_job_testing_caller",) if native else CRATES
    if format_check:
        crates += ("semio_framework_job", "semio_framework_value_persistent", "semio_framework_replication_command_ingress")
        pixels = GENERATED / "raw-png" / ("type-" + cache.parents[2].name)
        directories.add(pixels)
    if mounted:
        carrier = GENERATED / "picker-carrier"
        directories.add(carrier)
        flags += ["--extern", f"picker_carrier={carrier / 'libpicker_carrier.rmeta'}"]
    for crate in crates:
        artifact = pixels / "libsemio_framework_pixels.rmeta" if format_check and crate == "semio_framework_pixels" else library(cache, crate, ".rlib" if native else ".rmeta", identity)
        assert artifact.is_file(), artifact
        if native:
            flags += ["--extern", f"{crate}={artifact.with_suffix('.rmeta')}"]
        flags += ["--extern", f"{crate}={artifact}"]
    for directory in sorted(directories):
        flags += ["-L", f"dependency={directory}"]
    response = OUT / "rustc.rsp"
    response.write_text("\n".join(flags) + "\n")
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "@" + str(response)], check=True)
    if native:
        subprocess.run([str(target), "--nocapture"], check=True)


if __name__ == "__main__":
    main()
