"""🧪️ Renders the neutral paint-fidelity cases with MuPDF and writes or checks the expected probe colors."""

import json
import sys
from pathlib import Path

import fitz



def build(case):
    objects = [
        "<< /Type /Catalog /Pages 2 0 R >>",
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
    ]
    width, height = case["page"]
    refs = " ".join(f"{5 + index} 0 R" for index in range(len(case["objects"])))
    annots = f"/Annots [{refs}]" if case["kind"] == "annotation" else ""
    objects.append(f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width} {height}] /Contents 4 0 R /Resources << {case['resources']} >> {annots} >>")
    objects.append(f"<< /Length {len(case['content'])} >>\nstream\n{case['content']}\nendstream")
    for entry in case["objects"]:
        stream = entry.get("stream")
        objects.append(entry["dict"] if stream is None else f"{entry['dict'][:-2]}/Length {len(stream)} >>\nstream\n{stream}\nendstream")
    pdf = "%PDF-1.7\n"
    xref = ""
    for index, body in enumerate(objects):
        xref += f"{len(pdf):010} 00000 n \n"
        pdf += f"{index + 1} 0 obj\n{body}\nendobj\n"
    size = len(objects) + 1
    pdf += f"xref\n0 {size}\n0000000000 65535 f \n{xref}trailer\n<< /Root 1 0 R /Size {size} >>\nstartxref\n{len(pdf)}\n%%EOF\n"
    return pdf.encode("ascii")


def render(case):
    with fitz.open(stream=build(case), filetype="pdf") as document:
        pixmap = document[0].get_pixmap(colorspace=fitz.csRGB, annots=True)
        return [list(pixmap.pixel(*probe["at"])) for probe in case["probes"]]


def main(mode, path):
    data = json.loads(Path(path).read_text(encoding="utf-8"))
    failures = 0
    for case in data["cases"]:
        actual = render(case)
        for probe, rgb in zip(case["probes"], actual):
            if mode == "write":
                probe["rgb"] = rgb
            elif any(abs(a - b) > case["tolerance"] for a, b in zip(rgb, probe["rgb"])):
                failures += 1
                print("[DEBUG] mismatch", case["name"], probe["at"], rgb, probe["rgb"])
        print("[DEBUG]", case["name"], actual)
    if mode == "write":
        compact = lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False)
        Path(path).write_text('{"cases":[\n' + ",\n".join(compact(case) for case in data["cases"]) + "\n]}\n", encoding="utf-8")
    assert failures == 0, f"{failures} probes differ from MuPDF"
    print(f"{len(data['cases'])} paint-fidelity cases match MuPDF {fitz.__doc__.split()[1] if fitz.__doc__ else ''}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
