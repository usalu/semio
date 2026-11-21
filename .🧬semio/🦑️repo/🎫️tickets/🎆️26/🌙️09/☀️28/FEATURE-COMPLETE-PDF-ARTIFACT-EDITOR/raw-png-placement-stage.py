"""🧭 Stage the raw PNG preparation and publication cohort against the current PDF document owner."""

from hashlib import sha256
import json
from os.path import relpath
from pathlib import Path
from runpy import run_path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUT = TICKET / "🗑️generated/raw-png/stage"
PDF = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"
AUTHOR = PDF / "✏️editor/📄️document/🖼️image-authoring"
MANIFEST = AUTHOR.parent / "📦️packages/🦀️rust/Cargo.toml"
FRAMEWORK = ROOT / "🧰️framework"
DECODE = run_path(str(TICKET / "raw-png-handoff-stage.py"))
DEP = {
    "semio-framework-pack": "🔨️modules/🎒️pack",
    "semio-framework-pack-format": "🔨️modules/🎒️pack/📐️format",
    "semio-framework-pack-source": "🔨️modules/🎒️pack/🚰️source",
    "semio-framework-artifact-reference": "🔨️modules/🗿️artifact-reference",
    "semio-framework-hash": "🔨️modules/🔏️hash",
    "semio-framework-value": "🔨️modules/🌱️value",
    "semio-framework-value-persistent": "🔨️modules/🌱️value/📋️list/🔗️persistent",
    "semio-os-spr": "🛍️products/💻️os/🔨️modules/📡️spr",
}
SOURCES = {
    "plane_identity": ("🪪️plane.rs", "paged-png-placement-identity.rs"),
    "plane_pack": ("🎒️placement.rs", "paged-png-placement-pack.rs"),
    "plane_rows": ("🧾️rows.rs", "paged-png-placement-rows.rs"),
    "raw_samples": ("🎚️samples.rs", "raw-png-placement-samples.rs"),
    "raw_pack": ("📦️archive.rs", "raw-png-placement-pack.rs"),
    "raw_product": ("📬️publication.rs", "raw-png-placement-product.rs"),
}


def production_changes():
    """📦 Keep all existing mounted editing behavior and append the new dependent owners."""
    author = AUTHOR / "🦀️.rs"
    current = author.read_text(encoding="utf8")
    if current.count("pub use admission::*;") != 1 or "mod raw_product;" in current:
        raise ValueError("PDF image authoring export seam drifted")
    additions = "".join(f'#[path="📥️raw/{file}"] mod {module};\n' for module, (file, _) in SOURCES.items())
    additions += """pub use plane_identity::PlaneFacts;
pub use plane_rows::PngPlacementRows;
pub use raw_samples::{RawPngPlacementSamples, RgbaLoan, PlaneProjection};
pub use raw_pack::{OriginalPack, RawPngPlacementPack};
pub use raw_product::{PDF_RAW_PNG_ARCHIVE_BYTES, PdfRawPngRows, PdfRawPngPublication, prepare_raw_png_rows};
pub use semio_framework_pixels::{RasterImage, png_decoding::{PngDecodeJob, PngDecodeLimits, PngDecodeProgress}};
"""
    yield author, current + "\n" + additions
    dependencies = MANIFEST.read_text(encoding="utf8")
    if dependencies.count("[dependencies]") != 1 or "[dev-dependencies]" in dependencies:
        raise ValueError("PDF image authoring manifest dependency seam drifted")
    for name, location in DEP.items():
        if any(line.startswith(name + " ") for line in dependencies.splitlines()):
            continue
        package = FRAMEWORK / location / "📦️packages/🦀️rust"
        if f'name = "{name}"' not in (package / "Cargo.toml").read_text(encoding="utf8"):
            raise ValueError(f"Wrong owner manifest for {name}")
        relative = Path(relpath(package, MANIFEST.parent)).as_posix()
        dependencies += f'{name} = {{ path = "{relative}" }}\n'
    yield MANIFEST, dependencies
    yield DECODE["LIVE"], DECODE["stage_decoder"](DECODE["LIVE"].read_text(encoding="utf8"))
    for file, incoming in SOURCES.values():
        target = AUTHOR / "📥️raw" / file
        if target.exists():
            raise ValueError(f"PDF raw placement already has an owner for {target}")
        yield target, (TICKET / incoming).read_text(encoding="utf8")


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    entries = []
    for index, (path, source) in enumerate(production_changes()):
        if len(str(path).encode("utf-16-le")) // 2 > 256:
            raise ValueError(f"Path exceeds the portable envelope: {path}")
        staged = OUT / f"{index:02}{path.suffix}"
        staged.write_text(source, encoding="utf8")
        entries.append({
            "file": path.relative_to(ROOT).as_posix(),
            "before": sha256(path.read_bytes()).hexdigest() if path.exists() else None,
            "source": staged.name,
            "after": sha256(staged.read_bytes()).hexdigest(),
        })
    (OUT / "receipt.json").write_text(json.dumps(entries, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"[DEBUG] staged {len(entries)} guarded PDF raw placement production paths")
