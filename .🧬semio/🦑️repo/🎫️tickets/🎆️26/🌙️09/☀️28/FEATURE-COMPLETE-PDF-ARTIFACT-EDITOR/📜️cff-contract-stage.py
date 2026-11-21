"""🧫 Builds and checks the tiny neutral CFF decoder contract."""

from io import BytesIO
import json
from pathlib import Path

import fitz
from fontTools import cffLib
from fontTools.misc.transform import Transform
from fontTools.pens.recordingPen import RecordingPen
from fontTools.ttLib import TTFont

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/simple-cff-stage"

def index(*values):
    if not values:
        return bytes([0, 0])
    offsets = [1]
    for value in values:
        offsets.append(offsets[-1] + len(value))
    return bytes([0, len(values), 1] + offsets) + b"".join(values)

def dictionary(charset, encoding, charstrings, private):
    real = [30, 0xA0, 0x02, 0xFF]
    return bytes([charset + 139, 15, encoding + 139, 16, charstrings + 139, 17] +
                 real + [139, 139] + real + [139, 139, 12, 7, 139, private + 139, 18])

prefix = bytes([1, 0, 4, 1]) + index(b"Contract")
suffix = index(b"specialAlternate") + index()
start = len(prefix) + len(index(dictionary(0, 0, 0, 0))) + len(suffix)
charset = bytes([0, 0, 34, 391 >> 8, 391 & 255])
encoding = bytes([0x81, 1, 65, 1, 1, 70, 391 >> 8, 391 & 255])
program = bytes([149, 159, 21, 169, 139, 114, 159, 5, 14])
charstrings = index(bytes([14]), program, program)
font = prefix + index(dictionary(start, start + len(charset), start + len(charset) + len(encoding),
                                 start + len(charset) + len(encoding) + len(charstrings))) + suffix + charset + encoding + charstrings

oracle = cffLib.CFFFontSet()
ranges_only = bytearray(font)
ranges_only[start + len(charset)] &= 0x7F
oracle.decompile(BytesIO(ranges_only), TTFont())
face = oracle.topDictIndex[0]
pen = RecordingPen()
face.CharStrings["specialAlternate"].draw(pen)
transform = Transform(*face.FontMatrix)
paths = [[verb, [[value * 1000 for value in transform.transformPoint(point)] for point in points]] for verb, points in pen.value]
assert paths == [["moveTo", [[20.0, 40.0]]], ["lineTo", [[80.0, 40.0]]], ["lineTo", [[30.0, 80.0]]], ["closePath", []]]
assert face.charset == [".notdef", "A", "specialAlternate"]
assert [face.Encoding[code] for code in [65, 66]] == ["A", "specialAlternate"]

def stream(body, fields=b""):
    return b"<< /Length " + str(len(body)).encode() + b" " + fields + b">>\nstream\n" + body + b"\nendstream"

objects = [
    b"<< /Type /Catalog /Pages 2 0 R >>",
    b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>",
    b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 160 100] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>",
    stream(b"BT /F1 128 Tf 20 20 Td <4246> Tj ET"),
    b"<< /Type /Font /Subtype /Type1 /BaseFont /Contract /FirstChar 65 /LastChar 70 /Widths [500 500 500 500 500 500] /FontDescriptor 6 0 R >>",
    b"<< /Type /FontDescriptor /FontName /Contract /Flags 4 /FontBBox [0 0 50 50] /ItalicAngle 0 /Ascent 50 /Descent 0 /CapHeight 40 /StemV 10 /FontFile3 7 0 R >>",
    stream(font, b"/Subtype /Type1C "),
]
pdf = bytearray(b"%PDF-1.7\n")
offsets = [b"0000000000 65535 f \n"]
for owner, body in enumerate(objects, 1):
    offsets.append(f"{len(pdf):010d} 00000 n \n".encode())
    pdf.extend(f"{owner} 0 obj\n".encode() + body + b"\nendobj\n")
xref = len(pdf)
pdf.extend(f"xref\n0 {len(offsets)}\n".encode() + b"".join(offsets)
           + f"trailer\n<< /Root 1 0 R /Size {len(offsets)} >>\nstartxref\n{xref}\n%%EOF\n".encode())
with fitz.open(stream=pdf, filetype="pdf") as document:
    page = document[0].get_pixmap(colorspace=fitz.csGRAY)
    pixels = lambda left: [page.pixel(x, y) for y in range(60, 82) for x in range(left, left + 20)]
    assert pixels(20) == pixels(84) and min(pixels(20)) < (128,)

expert_charset = bytes([0, 0, 229, 313 >> 8, 313 & 255])
expert = prefix + index(dictionary(start, 1, start + len(expert_charset), start + len(expert_charset) + len(charstrings))) + suffix + expert_charset + charstrings
reference = cffLib.CFFFontSet()
reference.decompile(BytesIO(expert), TTFont())
face_expert = reference.topDictIndex[0]
assert face_expert.Encoding == "ExpertEncoding"
assert face_expert.charset == [".notdef", "exclamsmall", "Macronsmall"]

fixture = {"schema": "semio.stdio.pdf.cff.outline.v1", "font": font.hex(), "names": face.charset,
           "encoding": [[65, face.Encoding[65]], [66, face.Encoding[66]], [70, face.Encoding[66]]],
           "glyph": 2, "outline": paths, "expertFont": expert.hex(),
           "expertEncoding": [[33, face_expert.charset[1]], [175, face_expert.charset[2]]]}
(output / "🧫️contracts.json").write_text(json.dumps(fixture, ensure_ascii=False, indent=2) + "\n")

source = output / "cff.rs"
text = source.read_text()
anchor = '    #[test]\n    fn computed_operands_are_used_in_order()'
assert text.count(anchor) == 1
text = text.replace(anchor, '''    #[test]
    fn raw_name_keyed_glyph_uses_declared_names_encoding_and_matrix() {
        let case: serde_json::Value = serde_json::from_str(include_str!("🧫️contracts.json")).unwrap();
        let hex = case["font"].as_str().unwrap();
        let bytes = (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap()).collect::<Vec<_>>();
        let cff = CffOutlineFont::new(&bytes).unwrap();
        let mut names = Vec::new();
        cff.visit_glyph_names(|name, _| names.push(name)).unwrap();
        assert_eq!(serde_json::json!(names), case["names"]);
        assert_eq!(serde_json::json!(cff.builtin_encoding().unwrap()), case["encoding"]);
        let path = cff.raw_outline(case["glyph"].as_u64().unwrap() as u16).unwrap();
        let path = path.into_iter().map(|segment| match segment {
            CffOutlineSegment::MoveTo(to) => serde_json::json!(["moveTo", [to]]),
            CffOutlineSegment::LineTo(to) => serde_json::json!(["lineTo", [to]]),
            CffOutlineSegment::Close => serde_json::json!(["closePath", []]),
            _ => panic!("unexpected contract outline segment"),
        }).collect::<Vec<_>>();
        assert_eq!(serde_json::json!(path), case["outline"]);
        let hex = case["expertFont"].as_str().unwrap();
        let bytes = (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap()).collect::<Vec<_>>();
        let font = CffOutlineFont::new(&bytes).unwrap();
        let entries = font.builtin_encoding().unwrap();
        for pair in case["expertEncoding"].as_array().unwrap() {
            assert_eq!(entries.iter().find(|(code, _)| u64::from(*code) == pair[0].as_u64().unwrap()).map(|(_, name)| name.as_str()), pair[1].as_str());
        }
    }

''' + anchor)
source.write_text(text)
print("[DEBUG] FontTools verified CFF names, range encoding and FontMatrix outlines. MuPDF verified the supplement renders its alias.")
