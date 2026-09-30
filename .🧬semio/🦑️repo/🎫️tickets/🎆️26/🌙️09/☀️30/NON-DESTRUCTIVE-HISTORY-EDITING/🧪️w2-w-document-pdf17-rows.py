"""📨️ W2-W-document: rewrites the `📑️mutate-pdf-1-7` Examples rows to the leaf wire payload (design §11, F10) — the Rust
`payload_value()` of every row the old hand-mapping adapter built: `PdfObject` newtypes under `value` with byte-array
strings, `PdfPage`/`AppendPageContent`/`SetPageContent` carrying the typed `PdfOp` list the old `text` shorthand was
rendered into (`BT /F1 12 Tf 72 720 Td (…) Tj ET`), and `SetInfo` carrying its whole `PdfInfo` record."""
import json, pathlib, re

FEATURE = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/📑️mutate-pdf-1-7/🥒️.feature")

def text_ops(text):
    return [{"op": "beginText"}, {"op": "setFont", "name": "F1", "size": 12}, {"op": "moveText", "tx": 72, "ty": 720}, {"op": "showText", "text": {"kind": "text", "text": text}}, {"op": "endText"}]

def pdf_str(text):
    return {"kind": "str", "value": list(text.encode())}

def pdf_name(name):
    return {"kind": "name", "value": name}

def pdf_dict(entries):
    return {"kind": "dict", "value": [{"key": key, "value": value} for key, value in entries]}

ROWS = {
    "insert-page": {"index": 30, "page": {"mediaBox": [0, 0, 612, 792], "rotate": 0, "content": text_ops("Inserted page for wave 7 mutation testing")}},
    "remove-page": {"index": 7},
    "set-page-media-box": {"index": 15, "mediaBox": [0, 0, 595, 842]},
    "set-page-crop-box": {"index": 16, "cropBox": [10, 10, 580, 820]},
    "append-page-content": {"index": 17, "content": text_ops("Appended content line for wave 7 testing")},
    "set-info": {"info": {"title": "Wave 7 Replaced Title", "author": "Wave 7 Test Author"}},
    "insert-object": {"id": {"num": 900001, "gen": 0}, "value": pdf_dict([("Type", pdf_name("SemioWave7Marker")), ("Note", pdf_str("inserted by wave 7"))])},
    "remove-object": {"id": {"num": 3015, "gen": 0}},
    "set-object-value": {"id": {"num": 145, "gen": 0}, "value": pdf_dict([("S", pdf_name("GoToR")), ("Note", pdf_str("replaced by wave 7"))])},
    "set-dict-entry": {"id": {"num": 3188, "gen": 0}, "path": [], "key": "PageMode", "value": pdf_name("UseNone")},
    "remove-dict-entry": {"id": {"num": 3188, "gen": 0}, "path": [], "key": "Outlines"},
    "set-trailer-entry": {"key": "SemioWave7Marker", "value": {"kind": "int", "value": 42}},
    "remove-trailer-entry": {"key": "ID"},
    "move-page": {"from": 10, "to": 40},
    "set-page-content": {"index": 20, "content": text_ops("Replaced page content for wave 7 mutation testing")},
    "set-page-rotation": {"index": 5, "rotation": 90},
}

def cell(value):
    return json.dumps(value, ensure_ascii=False, separators=(", ", ": "))

def rewrite(text):
    lines = text.split("\n")
    out = []
    for line in lines:
        match = re.match(r"^(\s*)\|\s*([a-z-]+)\s*\|\s*(\{.*\})\s*\|\s*$", line)
        if match and match.group(2) in ROWS:
            out.append((match.group(1), match.group(2), cell(ROWS[match.group(2)])))
        else:
            out.append(line)
    widths = max((len(item[1]) for item in out if isinstance(item, tuple)), default=0)
    rendered = []
    for index, item in enumerate(out):
        if isinstance(item, tuple):
            rendered.append(f"{item[0]}| {item[1]:<{widths}} | {item[2]} |")
        elif re.match(r"^\s*\|\s*id\s*\|\s*params\s*\|\s*$", item):
            indent = re.match(r"^(\s*)", item).group(1)
            rendered.append(f"{indent}| {'id':<{widths}} | params |")
        else:
            rendered.append(item)
    return "\n".join(rendered)

if __name__ == "__main__":
    FEATURE.write_text(rewrite(FEATURE.read_text()))
    print(FEATURE.read_text().count('"content"'), "content rows")
