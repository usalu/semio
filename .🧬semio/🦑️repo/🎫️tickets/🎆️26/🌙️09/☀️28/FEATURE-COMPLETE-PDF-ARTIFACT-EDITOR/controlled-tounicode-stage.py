"""🎚️ Stages a direct ToUnicode cursor in the PDF fonts owner."""

from pathlib import Path
import difflib
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "font-unicode"


def stage(base):
    OUT.mkdir(parents=True, exist_ok=True)
    folder = base / "🔨️modules/🔤️fonts/🗺️cmap"
    original = folder / "🦀️.rs"
    child = folder / "🎚️controlled/🦀️.rs"
    anchor = "//#region 🔖️Parsing"
    before = original.read_text()
    assert before.count(anchor) == 1
    staged = before.replace(anchor, '#[path = "🎚️controlled/🦀️.rs"]\nmod controlled;\npub use controlled::PdfDirectToUnicodeCursor;\n\n' + anchor)
    source = (TICKET / "controlled-tounicode.rs").read_text()
    (OUT / "cmap.rs").write_text(staged)
    (OUT / "controlled.rs").write_text(source)
    patches = [
        difflib.unified_diff(before.splitlines(True), staged.splitlines(True), "a/" + str(original), "b/" + str(original)),
        difflib.unified_diff([], source.splitlines(True), "/dev/null", "b/" + str(child)),
    ]
    (OUT / "tounicode.patch").write_text("".join(line for patch in patches for line in patch))


if __name__ == "__main__":
    stage(Path(sys.argv[1]))
