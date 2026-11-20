"""🪺 Check nested form and axial shading fixtures against PyMuPDF 1.24.14."""

import json
from pathlib import Path
import sys

import fitz


def words(values):
    return " ".join(str(value).lower() for value in values)


def add_object(document, dictionary, content=None):
    xref = document.get_new_xref()
    document.update_object(xref, dictionary)
    if content is not None:
        document.update_stream(xref, content.encode("ascii"))
    return xref


def validate_pixels(document, data, resource_key, resource_dict, content, tolerance=0):
    page = document.new_page(width=data["page"][0], height=data["page"][1])
    document.xref_set_key(page.xref, "Resources", f"<< /{resource_key} << {resource_dict} >> >>")
    page.set_contents(add_object(document, "<<>>", content))
    raster = page.get_pixmap(colorspace=fitz.csRGB)
    for probe in data["probes"] if "probes" in data else data["pixels"]:
        actual = list(raster.pixel(*probe["at"]))
        assert all(abs(a - b) <= tolerance for a, b in zip(actual, probe["rgb"])), (probe, actual)


def verify_forms(data):
    references = {}
    with fitz.open() as doc:
        for form in reversed(data["forms"]):
            name = form["name"]
            bbox = words(form["bbox"])
            matrix = words(form["matrix"])
            resource = ""
            if child := form.get("use"):
                resource = f"/Resources << /XObject << /{child} {references[child]} 0 R >> >>"
            references[name] = add_object(doc, f"<< /Type /XObject /Subtype /Form /BBox [{bbox}] /Matrix [{matrix}] {resource} >>", form["content"])
        validate_pixels(doc, data, "XObject", f"/Outer {references['Outer']} 0 R", data["pageContent"])


def verify_shading(data):
    with fitz.open() as doc:
        bbox, coords, extend = (words(data[key]) for key in ("bbox", "coords", "extend"))
        c0, c1 = (words(values) for values in data["colors"])
        function = f"/Function << /FunctionType 2 /Domain [0 1] /C0 [{c0}] /C1 [{c1}] /N 1 >>"
        xref = add_object(doc, f"<< /ShadingType 2 /ColorSpace /DeviceRGB /BBox [{bbox}] /Coords [{coords}] /Extend [{extend}] {function} >>")
        validate_pixels(doc, data, "Shading", f"/Sh1 {xref} 0 R", "/Sh1 sh", tolerance=4)


if __name__ == "__main__":
    verify_forms(json.loads(Path(sys.argv[1]).read_text(encoding="utf-8")))
    verify_shading(json.loads(Path(sys.argv[2]).read_text(encoding="utf-8")))
    print("Four nested form probes and four axial shading probes match")
