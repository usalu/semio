"""🧩 Rewrite the PdfImageBody samples variant from logical words to native packed rows in the non-Rust schema twins."""
import pathlib
import sys

P = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base")


def swap(path, old, new):
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        sys.exit(f"{path}: expected one occurrence, found {count}")
    path.write_text(text.replace(old, new), encoding="utf-8")
    print("updated", path)


def json_block(indent):
    pad = " " * indent
    lines = lambda name, maximum: "\n".join(line and pad + line for line in f'''"required": [
  "kind",
  "{name}"
],
"properties": {{
  "kind": {{
    "const": "samples"
  }},
  "{name}": {{
    "type": "array",
    "items": {{
      "type": "integer",
      "minimum": 0,
      "maximum": {maximum}
    }}
  }}
}},'''.split("\n"))
    return lines("values", 65535), lines("packed", 255)


swap(P / "🧬️schema/📸️snapshot/🔣️.json", *json_block(10))
swap(P / "🚪️io/📝️text/📸️snapshot/🪪️native-json/🧬️schema/🖼️resource/🟦️.ts", *json_block(8))
swap(P / "🧬️schema/📸️snapshot/🛰️.proto", "message PdfImageBody {\n  string kind = 1;\n  repeated uint32 values = 2;", "message PdfImageBody {\n  string kind = 1;\n  bytes packed = 2;")
swap(P / "🧬️schema/📸️snapshot/🔗️.graphql", "type PdfImageBody {\n  kind: String!\n  values: [Int!]", "type PdfImageBody {\n  kind: String!\n  packed: [Int!]")
swap(P / "🧬️schema/📸️snapshot/🔗️.graphql", "input PdfImageBodyInput {\n  kind: String!\n  values: [Int!]", "input PdfImageBodyInput {\n  kind: String!\n  packed: [Int!]")
swap(P / "🧬️schema/📸️snapshot/🖼️resource/🟦️.ts", '{ kind: "samples"; values: number[] }', '{ kind: "samples"; packed: number[] }')
