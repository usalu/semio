"""🧾 Compose the registered PDF raw decoder and paid graph edit postimages."""

import hashlib
import json
from pathlib import Path
import subprocess

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
DOCUMENT = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base") / "✏️editor/📄️document"
MODULE = DOCUMENT / "🖼️image-authoring"
LIBRARY = DOCUMENT / "📦️packages/🦀️rust/Cargo.toml"


def digest(data):
    """🔏 Bind source bytes to the shared worktree's current revision."""
    return hashlib.sha256(data).hexdigest()


def one(text, old, new):
    """🔗 Check exactly one expected producer seam before changing it."""
    assert text.count(old) == 1, old
    return text.replace(old, new)


def stage():
    """🧱 Prepare the paid format modules on the current authenticated graph stage."""
    overlay = json.loads((GENERATED / "raw-png-consumers-overlay.json").read_text())
    candidate = overlay["candidate"]
    preimages = overlay["preimages"]
    for file, prior in preimages.items():
        live = Path(file)
        if prior is None:
            assert not live.exists(), file
        else:
            assert digest(live.read_bytes()) == prior, file
    source = (TICKET / "raw-png-work.rs").read_text()
    source = one(source, "use crate::{guest::OriginalGuestMediaInput, ingress::OriginalMediaLoan, receiver::MediaOrigin};",
                 "use semio_plugin::original_media::{OriginalGuestMediaInput, OriginalMediaLoan};")
    source = one(source, "use semio_framework_job::{", "use semio_plugin::__job::{")
    source = one(source, "use semio_framework_pixels_codecs::{", "use semio_framework_pixels::{")
    source = one(source, "use semio_framework_value::{", "use semio_plugin::__value::{")
    source = one(source, "    origin: Option<MediaOrigin>,\n", "")
    source = one(source, "source: None, origin: None,", "source: None,")
    start = source.index("    fn check_live(")
    end = source.index("    /// 🔑", start)
    source = source[:start] + source[end:]
    assert source.count("        self.check_live(live)?;\n") == 2
    source = source.replace("        self.check_live(live)?;\n", "")
    source = one(source, "            self.origin = Some(MediaOrigin::capture(live)?);\n", "")
    files = {
        MODULE / "🎞️decode-original.rs": source,
        MODULE / "🧩️graph-edit.rs": (TICKET / "📜️pdf-original-png-graph-edit.rs").read_text(),
        MODULE / "📬️format-work.rs": (TICKET / "📜️pdf-original-png-format-work.rs").read_text(),
    }
    for path, source in files.items():
        assert not path.exists() and str(path) not in candidate
        preimages[str(path)] = None
        candidate[str(path)] = source
    root = str(MODULE / "🦀️.rs")
    candidate[root] += '\n#[path = "🎞️decode-original.rs"]\nmod original_decode;\nuse original_decode::RawPngWork;\n#[path = "🧩️graph-edit.rs"]\nmod graph_edit;\nuse graph_edit::PngGraphEdit;\n#[path = "📬️format-work.rs"]\nmod format_work;\npub use format_work::PngFormatWork;\n'
    candidate[str(LIBRARY)] = one(candidate[str(LIBRARY)], "semio-plugin = { workspace = true }",
        "semio-os-kernel = { workspace = true }\nsemio-plugin = { workspace = true }")
    path = GENERATED / "raw-png-format-overlay.json"
    path.write_text(json.dumps(overlay, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("[DEBUG] Staged bounded PNG format and named graph owners", len(candidate))


if __name__ == "__main__":
    stage()
