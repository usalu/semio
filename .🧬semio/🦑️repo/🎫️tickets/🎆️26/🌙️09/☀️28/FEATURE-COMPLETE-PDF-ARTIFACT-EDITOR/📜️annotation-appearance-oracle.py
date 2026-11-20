"""📌 Verify the selected annotation appearance and its transformed placement with MuPDF."""
from pathlib import Path
import json
import sys

import fitz

data = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))

def words(key):
    return " ".join(str(value) for value in data[key])

with fitz.open() as document:
    width, height = data["page"]
    page = document.new_page(width=width, height=height)
    appearance = {}
    for state, stream in [("On", "selected"), ("Off", "unselected")]:
        owner = document.get_new_xref()
        document.update_object(owner, f"<< /Type /XObject /Subtype /Form /BBox [{words('bbox')}] /Matrix [{words('matrix')}] >>")
        document.update_stream(owner, data[stream].encode("ascii"))
        appearance[state] = owner
    annotation = document.get_new_xref()
    document.update_object(annotation, (
        f"<< /Type /Annot /Subtype /Stamp /Rect [{words('rect')}] "
        f"/AP << /N << /On {appearance['On']} 0 R /Off {appearance['Off']} 0 R >> >> /AS /{data['state']} >>"
    ))
    document.xref_set_key(page.xref, "Annots", f"[{annotation} 0 R]")
    with fitz.open(stream=document.tobytes(), filetype="pdf") as reopened:
        actual = reopened[0].get_pixmap(colorspace=fitz.csRGB)
        for probe in data["probes"]:
            observed = list(actual.pixel(*probe["at"])[:3])
            assert observed == probe["rgb"], (probe, observed)
        actual.save(sys.argv[2])
        print(f"MuPDF verified {len(data['probes'])} selected appearance pixels.")
