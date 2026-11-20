"""📥 Verify the mounted editor's downloaded page rotation with MuPDF."""
import json
import sys
from pathlib import Path

import fitz


def verify(source, downloaded, contract):
    with fitz.open(source) as original, fitz.open(downloaded) as edited:
        assert not original.is_repaired and not edited.is_repaired
        assert original.page_count == edited.page_count == contract["expected"]["pageCount"]
        page_index = contract["mutation"]["params"]["index"]
        assert edited[page_index].rotation == contract["expected"]["rotation"]
        for index, (before, after) in enumerate(zip(original, edited)):
            if index != page_index:
                assert after.rotation == before.rotation
            assert after.mediabox == before.mediabox
            assert after.cropbox == before.cropbox
            assert after.get_text() == before.get_text()
        for key in ("title", "author"):
            assert edited.metadata[key] == original.metadata[key]
    print("MuPDF matched page 2 rotation and preserved page 1, geometry, text and metadata")


if __name__ == "__main__":
    source = Path(sys.argv[1])
    contract = json.loads((source.parent.parent / "📄️natural-file-edit/🔣️.json").read_text())
    verify(source, Path(sys.argv[2]), contract)
