"""🗂️ S20 fault classes: the retext candidates the class reviewers found (📓️wp-fh5.md, 📓️wp-fh7.md) — texts whose advice
contradicts what the raise site checks get words that match their class. App declarations are rewritten in place on the
pass-1 faults overlay (the `.fault("code", FaultClass::…, LocalizedLabel::native("en", "de"))` of every app that declares the
code); framework catalog entries get an override block in FH1's text table (`wp-fh1/catalog_texts.py`), which `fh1-catalog.py`
writes into the catalog. Idempotent. Usage: python3 class-retext.py [--dry-run]"""
from __future__ import annotations

import os
import re
import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
TEXTS = Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1/catalog_texts.py")
APP = {
    "equation-command-json-decode": ("The command does not match the equation editor; reload the document.", "Der Befehl passt nicht zum Gleichungseditor; laden Sie das Dokument neu."),
    "equation-work-extent-overflow": ("The change could not be processed because of an internal error; reload the document.", "Die Änderung konnte wegen eines internen Fehlers nicht verarbeitet werden; laden Sie das Dokument neu."),
    "flow-retained-extension-item-capacity": ("The flow already holds as many extensions as it can; remove an extension first.", "Der Flow enthält bereits so viele Erweiterungen wie möglich; entfernen Sie zuerst eine Erweiterung."),
    "flow-retained-preview-off-item-capacity": ("As many previews as possible are already switched off; switch one on first.", "Es sind bereits so viele Vorschauen wie möglich ausgeschaltet; schalten Sie zuerst eine wieder ein."),
    "puzzle3d-kind-weight-changed-owner": ("The weight change lost its editor state because of an internal error; reload the puzzle.", "Die Gewichtsänderung hat wegen eines internen Fehlers ihren Editorzustand verloren; laden Sie das Puzzle neu."),
    "puzzle5d-kind-weight-changed-owner": ("The weight change lost its editor state because of an internal error; reload the puzzle.", "Die Gewichtsänderung hat wegen eines internen Fehlers ihren Editorzustand verloren; laden Sie das Puzzle neu."),
    "remodeling.retained.route": ("This command reached the wrong part of the Remodeling editor; reload the document.", "Dieser Befehl hat den falschen Teil des Remodeling-Editors erreicht; laden Sie das Dokument neu."),
}
CATALOG = {
    "artifact-envelope.ingress-credits": ("The document is larger than the editor can receive at once; open a smaller document.", "Das Dokument ist größer, als der Editor auf einmal empfangen kann; öffnen Sie ein kleineres Dokument."),
    "artifact-inference.unavailable": ("The inference service stopped because of an internal error; reload the app.", "Der Auswertungsdienst wurde wegen eines internen Fehlers angehalten; laden Sie die App neu."),
}
MARK = "# ── S20 session 15: class-review retexts (texts that contradicted what the raise site checks) ──"


def literal(text: str) -> str:
    return text.replace("\\", "\\\\").replace('"', '\\"')


def main(dry_run: bool) -> None:
    hits = {code: 0 for code in APP}
    for current, dirs, names in os.walk(OVERLAY / "✏️s/🔌️plugins"):
        dirs[:] = [d for d in dirs if d not in {"node_modules", "target", "🤖️generated"}]
        for name in names:
            if name != "🦀️.rs":
                continue
            path = Path(current) / name
            text = path.read_text()
            if not any(f'.fault("{code}"' in text for code in APP):
                continue
            new = text
            for code, (en, de) in APP.items():
                pattern = re.compile(r'(\.fault\("' + re.escape(code) + r'",\s*(?:[A-Za-z_]\w*::)*FaultClass::\w+,\s*(?:[A-Za-z_]\w*::)*LocalizedLabel::native\()"(?:[^"\\]|\\.)*",\s*"(?:[^"\\]|\\.)*"\)')
                new, count = pattern.subn(lambda match: f'{match.group(1)}"{literal(en)}", "{literal(de)}")', new)
                hits[code] += count
            if new != text and not dry_run:
                path.write_text(new)
    print("app declarations rewritten:", hits)
    assert all(hits.values()), hits
    table = TEXTS.read_text()
    if MARK not in table:
        block = "\n" + MARK + "\n" + "".join(f'T["{code}"] = ("{literal(en)}", "{literal(de)}")\n' for code, (en, de) in CATALOG.items())
        print("catalog overrides appended:", list(CATALOG))
        if not dry_run:
            TEXTS.write_text(table.rstrip("\n") + "\n" + block)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])
