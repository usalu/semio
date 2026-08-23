#!/usr/bin/env python3
"""Derives a substantial real DOCX (ECMA-376 WordprocessingML) fixture from the repository's own
README.md, once, per the wave 7 fleet brief for stdio/docx/ecma-376/any. Builds a real OPC package
by hand (zip + hand-written XML parts) so the fixture exercises real headings/styles/tables/bold-
italic runs/multiple parts -- not a synthetic 2-paragraph stub. No new runtime dependency: Python's
stdlib zipfile only, and this script itself is a disposable ticket-folder artifact, never imported by
production or test code.
"""
import re
import zipfile
import io
import html
import os
import sys

REPO_ROOT = "/Users/ueli/Documents/semio"
README = os.path.join(REPO_ROOT, "README.md")
OUT = os.path.join(os.path.dirname(__file__), "example-readme.docx")

W_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main"


def esc(text):
    return html.escape(text, quote=False)


def esc_attr(text):
    return html.escape(text, quote=True)


#region Markdown -> blocks
INLINE_RE = re.compile(r"(\*\*.+?\*\*|\*[^*].*?\*)")


def inline_runs(text):
    """Splits `text` on **bold**/*italic* markdown spans into (text, bold, italic) run tuples."""
    runs = []
    pos = 0
    for m in INLINE_RE.finditer(text):
        if m.start() > pos:
            runs.append((text[pos:m.start()], False, False))
        span = m.group(0)
        if span.startswith("**"):
            runs.append((span[2:-2], True, False))
        else:
            runs.append((span[1:-1], False, True))
        pos = m.end()
    if pos < len(text):
        runs.append((text[pos:], False, False))
    if not runs:
        runs.append(("", False, False))
    return runs


def strip_heading(text):
    return re.sub(r"\s*\[.*?\]\(.*?\)\s*$", "", text).strip()


def parse_readme(path):
    """Returns a list of block dicts: {"kind": "heading", "level": 1..3, "text": ...} |
    {"kind": "paragraph", "runs": [(text,bold,italic), ...]} | {"kind": "code", "text": ...} |
    {"kind": "table", "rows": [[cell, ...], ...]}."""
    with open(path, "r", encoding="utf-8") as f:
        lines = f.read().splitlines()

    blocks = []
    i = 0
    in_code = False
    code_lines = []
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        if stripped.startswith("```"):
            if in_code:
                if code_lines:
                    blocks.append({"kind": "code", "text": "\n".join(code_lines)})
                code_lines = []
                in_code = False
            else:
                in_code = True
            i += 1
            continue
        if in_code:
            code_lines.append(line)
            i += 1
            continue

        if not stripped:
            i += 1
            continue
        if stripped.startswith("<"):
            i += 1
            continue

        heading_m = re.match(r"^(#{1,6})\s+(.*)$", stripped)
        if heading_m:
            level = min(len(heading_m.group(1)), 3)
            text = strip_heading(heading_m.group(2))
            if text:
                blocks.append({"kind": "heading", "level": level, "text": text})
            i += 1
            continue

        if stripped.startswith("|") and stripped.endswith("|"):
            table_lines = []
            while i < len(lines) and lines[i].strip().startswith("|"):
                table_lines.append(lines[i].strip())
                i += 1
            rows = []
            for row_line in table_lines:
                if re.match(r"^\|[\s\-:|]+\|$", row_line):
                    continue
                cells = [c.strip() for c in row_line.strip("|").split("|")]
                rows.append(cells)
            if rows:
                blocks.append({"kind": "table", "rows": rows})
            continue

        text = re.sub(r"<[^>]+>", "", stripped).strip()
        if text:
            blocks.append({"kind": "paragraph", "runs": inline_runs(text)})
        i += 1

    return blocks
#endregion


#region WordprocessingML
def run_xml(text, bold, italic):
    if not text:
        return ""
    props = ""
    if bold or italic:
        inner = ("<w:b/>" if bold else "") + ("<w:i/>" if italic else "")
        props = f"<w:rPr>{inner}</w:rPr>"
    return f'<w:r>{props}<w:t xml:space="preserve">{esc(text)}</w:t></w:r>'


def paragraph_xml(style, runs):
    pPr = f'<w:pPr><w:pStyle w:val="{esc_attr(style)}"/></w:pPr>' if style else ""
    body = "".join(run_xml(t, b, it) for (t, b, it) in runs)
    return f"<w:p>{pPr}{body}</w:p>"


def cell_xml(text):
    return f"<w:tc>{paragraph_xml('TableCell', inline_runs(text))}</w:tc>"


def row_xml(cells):
    return "<w:tr>" + "".join(cell_xml(c) for c in cells) + "</w:tr>"


def table_xml(rows):
    return "<w:tbl>" + "".join(row_xml(r) for r in rows) + "</w:tbl>"


HEADING_STYLE = {1: "Heading1", 2: "Heading2", 3: "Heading3"}


def blocks_to_body_xml(blocks):
    parts = []
    for block in blocks:
        if block["kind"] == "heading":
            parts.append(paragraph_xml(HEADING_STYLE[block["level"]], [(block["text"], False, False)]))
        elif block["kind"] == "paragraph":
            parts.append(paragraph_xml("Normal", block["runs"]))
        elif block["kind"] == "code":
            for line in block["text"].split("\n"):
                parts.append(paragraph_xml("Code", [(line, False, False)]))
        elif block["kind"] == "table":
            parts.append(table_xml(block["rows"]))
    return "".join(parts)


def document_xml(blocks):
    body = blocks_to_body_xml(blocks)
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\r\n'
        f'<w:document xmlns:w="{W_NS}"><w:body>{body}</w:body></w:document>'
    )


def styles_xml():
    def style(style_id, name, based_on=None):
        based = f'<w:basedOn w:val="{esc_attr(based_on)}"/>' if based_on else ""
        return f'<w:style w:styleId="{esc_attr(style_id)}"><w:name w:val="{esc_attr(name)}"/>{based}</w:style>'

    styles = "".join([
        style("Normal", "Normal"),
        style("Title", "Title", "Normal"),
        style("Heading1", "heading 1", "Normal"),
        style("Heading2", "heading 2", "Normal"),
        style("Heading3", "heading 3", "Normal"),
        style("Code", "Code", "Normal"),
        style("TableCell", "Table Cell", "Normal"),
    ])
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\r\n'
        f'<w:styles xmlns:w="{W_NS}">{styles}</w:styles>'
    )


CONTENT_TYPES = (
    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\r\n'
    '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">'
    '<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>'
    '<Default Extension="xml" ContentType="application/xml"/>'
    '<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>'
    '<Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>'
    '<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>'
    '<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>'
    '</Types>'
)

ROOT_RELS = (
    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\r\n'
    '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
    '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>'
    '<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>'
    '<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>'
    '</Relationships>'
)

DOCUMENT_RELS = (
    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\r\n'
    '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
    '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>'
    '</Relationships>'
)

CORE_PROPS = (
    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\r\n'
    '<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" '
    'xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" '
    'xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">'
    '<dc:title>semio README (ECMA-376 wave 7 mutation fixture)</dc:title>'
    '<dc:creator>semio repository README.md</dc:creator>'
    '<cp:lastModifiedBy>semio-e2e-wave7</cp:lastModifiedBy>'
    '<dcterms:created xsi:type="dcterms:W3CDTF">2026-08-23T00:00:00Z</dcterms:created>'
    '<dcterms:modified xsi:type="dcterms:W3CDTF">2026-08-23T00:00:00Z</dcterms:modified>'
    '</cp:coreProperties>'
)

APP_PROPS = (
    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\r\n'
    '<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties">'
    '<Application>semio-e2e-wave7-derivation</Application>'
    '</Properties>'
)
#endregion


def main():
    blocks = parse_readme(README)
    title = [{"kind": "heading", "level": 1, "text": "semio"}]
    subtitle = [{"kind": "paragraph", "runs": [("Derived once from this repository's own real README.md for the ECMA-376 wave 7 exhaustive mutation case (stdio/docx). See provenance.md for the exact derivation.", False, True)]}]
    all_blocks = title + subtitle + blocks

    heading_count = sum(1 for b in all_blocks if b["kind"] == "heading")
    paragraph_count = sum(1 for b in all_blocks if b["kind"] == "paragraph")
    table_count = sum(1 for b in all_blocks if b["kind"] == "table")
    code_count = sum(1 for b in all_blocks if b["kind"] == "code")
    print(f"blocks: {len(all_blocks)} (headings={heading_count} paragraphs={paragraph_count} tables={table_count} code={code_count})", file=sys.stderr)

    doc_xml = document_xml(all_blocks)
    sty_xml = styles_xml()

    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as zf:
        zf.writestr("[Content_Types].xml", CONTENT_TYPES)
        zf.writestr("_rels/.rels", ROOT_RELS)
        zf.writestr("word/document.xml", doc_xml)
        zf.writestr("word/styles.xml", sty_xml)
        zf.writestr("word/_rels/document.xml.rels", DOCUMENT_RELS)
        zf.writestr("docProps/core.xml", CORE_PROPS)
        zf.writestr("docProps/app.xml", APP_PROPS)

    with open(OUT, "wb") as f:
        f.write(buf.getvalue())
    print(f"wrote {OUT} ({os.path.getsize(OUT)} bytes)", file=sys.stderr)


if __name__ == "__main__":
    main()
