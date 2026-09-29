"""🧯️ S20 faults overlay, session-15 rebase: stdio pdf 1.7 page editor — the 44 free-text `fault(…)` refusals LB2's round-2
sets added after the faults overlay dropped the page editor's `fault(message)` helper become `app_fault` codes (+ parameters),
declared en/de in the pdf root's `editor_faults` (existing codes reused where the meaning is the same).
Idempotent (a site whose replacement is present is skipped). Usage: python3 s20-rebase-pdf-page.py [--dry-run]"""
from __future__ import annotations

import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
PDF = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"
PAGE = OVERLAY / f"{PDF}/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🦀️.rs"
ROOT = OVERLAY / f"{PDF}/🦀️.rs"
C = "stdio.pdf.page."


def raise_(code: str, **parameters: str) -> str:
    return f'app_fault("{C}{code}")' + "".join(f'.with_parameter("{name}", {value})' for name, value in parameters.items())


SITES: list[tuple[str, str]] = [
    ('fault(format!("pdf annotation \'{object_id}\' is not addressable"))', raise_("annotation-unaddressable", object="object_id")),
    ('fault(format!("pdf annotation \'{object_id}\' is gone"))', raise_("annotation-missing", object="object_id")),
    ('fault("a list needs numbers")', raise_("number-list-invalid")),
    ('fault("an entry needs a key")', raise_("entry-key-required")),
    ('fault("an entry needs key=value")', raise_("entry-invalid")),
    ('fault(format!("unknown annotation markup \'{other}\'"))', raise_("annotation-markup-unknown", markup="other")),
    ('fault("a form setting needs a name")', raise_("form-setting-name-required")),
    ('fault(format!("pdf form field \'{field_name}\' is gone"))', raise_("form-field-missing", field="field_name")),
    ('fault(format!("unknown form field setting \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("unknown extra dictionary \'{other}\'"))', raise_("extra-dictionary-unknown", dictionary="other")),
    ('fault("a line needs two endings")', raise_("line-endings-invalid")),
    ('fault(format!("unknown annotation kind \'{other}\'"))', raise_("annotation-kind-unknown", kind="other")),
    ('fault(format!("this annotation has no \'{field}\'"))', raise_("setting-unknown", setting="field")),
    ('fault("a line needs four coordinates")', raise_("line-coordinates-invalid")),
    ('fault("a form field needs a name")', raise_("form-field-name-required")),
    ('fault("the document has no form")', raise_("form-missing")),
    ('fault(format!("pdf form field \'{name}\' is gone"))', raise_("form-field-missing", field="name")),
    ('fault(format!("a choice field has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("a text field has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("a button field has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("a signature field has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault("a container field has no value")', raise_("form-field-container-value")),
    ('fault("a line cap is 0, 1, or 2")', raise_("line-cap-invalid")),
    ('fault("a line join is 0, 1, or 2")', raise_("line-join-invalid")),
    ('fault("a resource detail needs owner.field")', raise_("resource-detail-invalid")),
    ('fault("an outline index needs a whole number")', raise_("outline-index-invalid")),
    ('fault("that outline is gone")', raise_("outline-missing")),
    ('fault(format!("an outline has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("pdf image \'{id}\' is gone"))', raise_("image-missing", image="id")),
    ('fault(format!("an image has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("a graphics state has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("pdf font \'{id}\' is gone"))', raise_("font-missing", font="id")),
    ('fault("font descriptor details apply to a simple font")', raise_("font-descriptor-simple-only")),
    ('fault(format!("a font descriptor has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("pdf form \'{id}\' is gone"))', raise_("form-object-missing", form="id")),
    ('fault(format!("unknown group color space \'{other}\'"))', raise_("group-color-space-unknown", colorSpace="other")),
    ('fault(format!("a form has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("pdf shading \'{id}\' is gone"))', raise_("shading-missing", shading="id")),
    ('fault(format!("a shading has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("optional content has no \'{other}\'"))', raise_("setting-unknown", setting="other")),
    ('fault(format!("unknown resource owner \'{other}\'"))', raise_("resource-owner-unknown", owner="other")),
]

DECLARATIONS: list[tuple[str, str, str]] = [
    ("annotation-kind-unknown", "The annotation kind {kind} is not supported; choose another kind.", "Die Anmerkungsart {kind} wird nicht unterstützt; wählen Sie eine andere Art."),
    ("annotation-markup-unknown", "The annotation markup {markup} is not supported; choose another one.", "Die Anmerkungsauszeichnung {markup} wird nicht unterstützt; wählen Sie eine andere."),
    ("entry-invalid", "An entry must have the form key=value; correct it.", "Ein Eintrag muss die Form Schlüssel=Wert haben; korrigieren Sie ihn."),
    ("entry-key-required", "An entry needs a key; enter one.", "Ein Eintrag benötigt einen Schlüssel; geben Sie einen ein."),
    ("extra-dictionary-unknown", "The dictionary {dictionary} is not supported; choose another one.", "Das Wörterbuch {dictionary} wird nicht unterstützt; wählen Sie ein anderes."),
    ("font-descriptor-simple-only", "Font descriptor details can only be set for a simple font; pick such a font.", "Details der Schriftbeschreibung lassen sich nur für eine einfache Schrift festlegen; wählen Sie eine solche Schrift."),
    ("form-field-container-value", "A field that groups other fields has no value of its own; edit one of its fields.", "Ein Feld, das andere Felder gruppiert, hat keinen eigenen Wert; bearbeiten Sie eines seiner Felder."),
    ("form-field-missing", "The form field {field} no longer exists; refresh the document and try again.", "Das Formularfeld {field} existiert nicht mehr; aktualisieren Sie das Dokument und versuchen Sie es erneut."),
    ("form-field-name-required", "A form field needs a name; enter one.", "Ein Formularfeld benötigt einen Namen; geben Sie einen ein."),
    ("form-missing", "The document has no form yet; add a form field first.", "Das Dokument hat noch kein Formular; fügen Sie zuerst ein Formularfeld hinzu."),
    ("form-object-missing", "The form object {form} no longer exists; refresh the document and try again.", "Das Formularobjekt {form} existiert nicht mehr; aktualisieren Sie das Dokument und versuchen Sie es erneut."),
    ("form-setting-name-required", "A form setting needs a name; enter one.", "Eine Formulareinstellung benötigt einen Namen; geben Sie einen ein."),
    ("line-cap-invalid", "A line cap must be 0, 1 or 2; choose one of them.", "Ein Linienende muss 0, 1 oder 2 sein; wählen Sie einen dieser Werte."),
    ("line-coordinates-invalid", "A line needs four coordinates; enter x1, y1, x2 and y2.", "Eine Linie benötigt vier Koordinaten; geben Sie x1, y1, x2 und y2 ein."),
    ("line-endings-invalid", "A line needs exactly two line endings; enter two.", "Eine Linie benötigt genau zwei Linienenden; geben Sie zwei an."),
    ("line-join-invalid", "A line join must be 0, 1 or 2; choose one of them.", "Eine Linienverbindung muss 0, 1 oder 2 sein; wählen Sie einen dieser Werte."),
    ("number-list-invalid", "A list needs comma-separated numbers; correct it.", "Eine Liste benötigt durch Kommas getrennte Zahlen; korrigieren Sie sie."),
    ("outline-index-invalid", "A bookmark position must be a whole number; correct it.", "Eine Lesezeichenposition muss eine ganze Zahl sein; korrigieren Sie sie."),
    ("outline-missing", "That bookmark no longer exists; refresh the document and try again.", "Dieses Lesezeichen existiert nicht mehr; aktualisieren Sie das Dokument und versuchen Sie es erneut."),
    ("resource-detail-invalid", "A resource detail must have the form owner.field; correct it.", "Ein Ressourcendetail muss die Form Besitzer.Feld haben; korrigieren Sie es."),
    ("resource-owner-unknown", "The resource owner {owner} is not supported; choose another one.", "Der Ressourcenbesitzer {owner} wird nicht unterstützt; wählen Sie einen anderen."),
    ("setting-unknown", "The selected item has no setting {setting}; choose another setting.", "Das ausgewählte Element hat keine Einstellung {setting}; wählen Sie eine andere Einstellung."),
]
ANCHOR = '        .fault("stdio.pdf.set-page.conflict",'


def main(dry_run: bool) -> None:
    page = PAGE.read_text()
    for old, new in SITES:
        count = page.count(old)
        if count == 0:
            assert new in page, old
            continue
        page = page.replace(old, new)
        print(f"site ×{count} {new[:90]}")
    root = ROOT.read_text()
    assert root.count(ANCHOR) == 1
    lines = [f'        .fault("{C}{code}", semio_framework_plugin::LocalizedLabel::native("{en}", "{de}"))\n' for code, en, de in DECLARATIONS]
    missing = [line for line, (code, _, _) in zip(lines, DECLARATIONS) if f'.fault("{C}{code}",' not in root]
    root = root.replace(ANCHOR, "".join(missing) + ANCHOR, 1) if missing else root
    print(f"declarations +{len(missing)}")
    if not dry_run:
        PAGE.write_text(page)
        ROOT.write_text(root)


if __name__ == "__main__":
    main("--dry-run" in sys.argv[1:])
