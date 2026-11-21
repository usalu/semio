"""🔍 Derives six neutral Original wire frames from independent pypdf observations."""

from argparse import ArgumentParser
from hashlib import sha256
from io import BytesIO
import json
from pathlib import Path

from google_crc32c import value as crc32c
from pypdf import PdfReader, PdfWriter
from pypdf.generic import NameObject, TextStringObject

TICKET = Path(__file__).resolve().parent
PDF = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base") / "🧪️tests/🧫️fixtures"


def varint(size):
    if size < 128:
        return bytes([size])
    return bytes([(size & 127) | 128]) + varint(size >> 7)


def frame(key, tag, body):
    data = json.dumps({"mutation": key, **body}, ensure_ascii=False, separators=(",", ":"))
    encoded = data.encode()
    wire = bytes([1, tag]) + varint(len(encoded)) + encoded
    return {"json": data, "frameBytes": len(wire), "crc32c": f"{crc32c(wire):08x}", "sha256": sha256(wire).hexdigest()}


def observe(key, pdf):
    if key == "setPageRotation":
        return {"index": 0, "rotation": pdf.pages[0].rotation}
    if key == "setInfo":
        return {"info": {field: pdf.metadata[source] for field, source in (
            ("title", "/Title"), ("author", "/Author"),
        ) if source in pdf.metadata}}
    return {"language": pdf.root_object.get("/Lang")}


def apply(pdf, key, body):
    output = PdfWriter(clone_from=pdf)
    if key == "setPageRotation":
        output.pages[body["index"]].rotation = body["rotation"]
    elif key == "setInfo":
        output.add_metadata({"/Title" if field == "title" else "/Author": value for field, value in body["info"].items()})
    elif body["language"] is None:
        output.root_object.pop(NameObject("/Lang"), None)
    else:
        output.root_object[NameObject("/Lang")] = TextStringObject(body["language"])
    stream = BytesIO()
    output.write(stream)
    return PdfReader(stream)


def read(workspace):
    pairs = []
    for directory, key, tag in (
        ("🔄️set-page-rotation", "setPageRotation", 15),
        ("ℹ️set-info", "setInfo", 5),
        ("🗣️set-language", "setLanguage", 53),
    ):
        base, edit = (PdfReader(workspace / PDF / directory / f"{name}.pdf") for name in ("⬅️before", "➡️after"))
        inverse, forward = observe(key, base), observe(key, edit)
        assert inverse != forward
        applied = apply(base, key, forward)
        restored = apply(applied, key, inverse)
        for field in ("setPageRotation", "setInfo", "setLanguage"):
            assert observe(field, applied) == observe(field, edit), (key, field)
            assert observe(field, restored) == observe(field, base), (key, field)
        pairs.append({"mutation": key, "tag": tag, "forward": frame(key, tag, forward), "inverse": frame(key, tag, inverse)})
    return pairs


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--verify", action="store_true")
    args = parser.parse_args()
    observed = read(args.workspace)
    if args.verify:
        assert observed == json.loads((TICKET / "archive-leaves-laws.json").read_bytes())["pairs"]
        print("[DEBUG] pypdf applied and inverted three PDF edits; Google CRC32C reproduced six canonical format-1 frames")
    else:
        print(json.dumps(observed, ensure_ascii=False, indent=2))
