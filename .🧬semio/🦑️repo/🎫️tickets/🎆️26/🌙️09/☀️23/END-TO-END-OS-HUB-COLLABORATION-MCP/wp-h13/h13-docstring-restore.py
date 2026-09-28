#!/usr/bin/env python3
"""📎️ H13 prepared patch (window 3, guest freeze): the 19:15 Check In law was inserted between the 👁️🔒 docstring of
`viewer_rejects_every_contract_mutating_verb` and its function, so that docstring now heads the codec law and the viewer law
has none. Moves the four 👁️🔒 lines back above the viewer law. Idempotent; `--dry-run` reports without writing."""
import sys
from pathlib import Path

PATH = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs")
VIEWER_DOC = (
    "/// 👁️🔒 Contract §2.3 clause 1/2 — WITH TEETH: dispatches the eight frozen mutating verbs\n"
    "/// through the full `VcsArtifactApp<ViewerApp<V>>` runtime path (`handle_action` for the\n"
    "/// seven string actions, `import_media` for the eighth) and asserts every one comes back\n"
    "/// `Fault { origin: FaultOrigin::Framework, code: FaultCode::new(\"viewer.read-only\"), .. }`.\n"
)
CODEC_DOC_HEAD = "/// 📌️ A hub Check In validates the pair it folded through the guest codec's `print-mirror`"
VIEWER_FN = "#[semio_framework_async_macros::async_test]\nasync fn viewer_rejects_every_contract_mutating_verb() {\n"


def main() -> int:
    dry = "--dry-run" in sys.argv
    text = PATH.read_text(encoding="utf-8")
    if text.count(VIEWER_FN) != 1:
        print(f"problem: viewer law anchor found {text.count(VIEWER_FN)}x")
        return 1
    if VIEWER_DOC + VIEWER_FN in text and VIEWER_DOC + CODEC_DOC_HEAD not in text:
        print("already applied")
        return 0
    if text.count(VIEWER_DOC + CODEC_DOC_HEAD) != 1:
        print("problem: misplaced 👁️🔒 docstring not found exactly once before the codec law")
        return 1
    patched = text.replace(VIEWER_DOC + CODEC_DOC_HEAD, CODEC_DOC_HEAD, 1).replace(VIEWER_FN, VIEWER_DOC + VIEWER_FN, 1)
    print(f"{'would move' if dry else 'moved'} the 👁️🔒 docstring back above viewer_rejects_every_contract_mutating_verb")
    if not dry:
        PATH.write_text(patched, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main())
