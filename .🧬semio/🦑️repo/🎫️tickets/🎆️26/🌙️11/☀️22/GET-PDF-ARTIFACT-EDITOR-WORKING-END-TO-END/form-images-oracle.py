#!/usr/bin/env python3
"""pymupdf oracle for slice 10: the form-images, page-resources and acro-form fixtures (and, with a second argument, a file Save wrote from one of them) must agree with their schema-first specs.

usage: uv run --offline --with pymupdf==1.26.7 python -I form-images-oracle.py <fixtures directory> [<kind> <saved pdf>]
"""
import json
import sys
from pathlib import Path

import pymupdf

KINDS = {"form-images": "🖼️form-images", "page-resources": "🎨️page-resources", "acro-form": "📝️acro-form"}


def check_form_images(path: Path, spec: dict):
    document = pymupdf.open(path)
    assert document.page_count == spec["pages"], document.page_count
    images = {image[0] for page in document for image in page.get_images(full=True)}
    assert images >= {8, 9, 10, 11, 14, 15} or len(images) >= 6, images
    forms = {xref for page in document for xref in (item[0] for item in page.get_xobjects())}
    assert len(forms) >= 2, forms
    pixmap = document[0].get_pixmap()
    assert pixmap.width == 240 and pixmap.height == 240
    return f"{document.page_count} pages, {len(images)} images, {len(forms)} page-level forms"


def check_page_resources(path: Path, spec: dict, saved: bool = False):
    document = pymupdf.open(path)
    assert document.page_count == spec["pages"], document.page_count
    for page, declared, drawings in zip(document, spec["declared"], spec["drawings"]):
        for category, names in ({} if saved else declared).items():
            resources = document.xref_get_key(page.xref, "Resources")
            text = document.xref_object(int(resources[1].split()[0]), compressed=True) if resources[0] == "xref" else resources[1]
            for name in names:
                assert f"/{name}" in text, (page.number, category, name)
        assert len(page.get_drawings()) >= drawings, (page.number, len(page.get_drawings()))
    return f"{document.page_count} pages, drawings {[len(page.get_drawings()) for page in document]}"


def check_acro_form(path: Path, spec: dict):
    document = pymupdf.open(path)
    assert document.page_count == spec["pages"] and document.is_form_pdf
    per_page = [len(list(page.widgets())) for page in document]
    assert per_page == spec["widgetsPerPage"], per_page
    names = {widget.field_name for page in document for widget in page.widgets()}
    for field in spec["fields"]:
        top = field["name"]
        assert top in names or any(name.startswith(top + ".") for name in names), (top, sorted(names))
    values = {widget.field_name: widget.field_value for page in document for widget in page.widgets()}
    assert values.get("address.name") == "Ada" and values.get("address.email") == "ada@example.org", values
    return f"{document.page_count} pages, widgets {per_page}, fields {sorted(names)}"


CHECKS = {"form-images": check_form_images, "page-resources": check_page_resources, "acro-form": check_acro_form}


def main():
    directory = Path(sys.argv[1])
    if len(sys.argv) == 4:
        kind, saved = sys.argv[2], Path(sys.argv[3])
        spec = json.loads((directory / KINDS[kind] / "spec.json").read_text())
        print(kind, "saved:", CHECKS[kind](saved, spec, True) if kind == "page-resources" else CHECKS[kind](saved, spec))
        return
    for kind, folder in KINDS.items():
        spec = json.loads((directory / folder / "spec.json").read_text())
        print(kind, CHECKS[kind](directory / folder / f"{kind}.pdf", spec))


if __name__ == "__main__":
    main()
