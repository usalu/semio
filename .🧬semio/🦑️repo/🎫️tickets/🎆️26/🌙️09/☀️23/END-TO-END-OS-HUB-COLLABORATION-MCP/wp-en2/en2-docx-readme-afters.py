"""📜️ EN2: third-party generator of the committed afters of `📜️mutate-docx-ecma-376` — python-docx (MIT) applies each
Examples row of the case's `@id-mutate` outline to the real `📜️example-readme.docx` through its own package/part/oxml
object model and saves it. The rows are READ from the case feature, never restated, exactly as the case's own unit test
reads them. Two normalizations keep the output a function of the row alone: `docProps/core.xml` is loaded as a plain
`Part` (python-docx would otherwise re-serialize an untouched part and change its digest), and the saved archive is
re-zipped with a fixed entry date so a rerun is byte-identical.

usage: en2-docx-readme-afters.py [--root <repo or overlay root>] [--only <fixture id>]
       SEMIO_FIXTURE_OUT=<dir> stages <id>/<declared file basename> (both files) for `fixture reproduce`
"""
import io
import json
import os
import shutil
import sys
import zipfile
from pathlib import Path

import docx
from docx.enum.style import WD_STYLE_TYPE
from docx.opc.constants import CONTENT_TYPE as CT
from docx.opc.part import PartFactory
from docx.oxml import OxmlElement
from docx.oxml.ns import qn
from docx.text.run import Run

SUBSET = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"
FEATURE = "🧪️tests/📜️mutate-docx-ecma-376/🥒️.feature"
INPUT = "🧫️fixtures/📜️example-readme.docx"
AFTERS = "🧫️fixtures/🧾️readme-afters"
DIRECTORIES = {
    "set-snapshot": "📸️set-snapshot",
    "insert-block": "➕️insert-block",
    "remove-block": "➖️remove-block",
    "set-block-content": "📝️set-block-content",
    "set-run-text": "✍️set-run-text",
    "set-run-formatting": "🪄️set-run-formatting",
    "insert-style": "💬️insert-style",
    "remove-style": "🗨️remove-style",
    "set-style-name": "🔤️set-style-name",
    "set-style-based-on": "🌳️set-style-based-on",
    "set-part": "🧩️set-part",
    "remove-part": "🧹️remove-part",
}
FIXED_DATE = (2026, 1, 1, 0, 0, 0)
BLOCK_TAGS = (qn("w:p"), qn("w:tbl"))


def fixture_id(kind):
    return "%s-readme-applied" % kind


def rows(feature_text):
    found, inside = [], False
    for line in feature_text.splitlines():
        stripped = line.strip()
        if stripped.startswith("@id-"):
            inside = stripped == "@id-mutate"
        if inside and stripped.startswith("|"):
            cells = [cell.strip() for cell in stripped.strip("|").split("|")]
            if cells[0] != "id" and cells[0] not in {kind for kind, _ in found}:
                found.append((cells[0], json.loads(cells[1])))
    return found


def blocks(container):
    return [child for child in container.iterchildren() if child.tag in BLOCK_TAGS]


def container_at(body, segments):
    container = body
    for segment in segments:
        table = blocks(container)[segment["blockIndex"]]
        assert table.tag == qn("w:tbl"), "path segment does not address a table"
        container = table.findall(qn("w:tr"))[segment["row"]].findall(qn("w:tc"))[segment["cell"]]
    return container


def run_element(spec):
    run = OxmlElement("w:r")
    flags = [(tag, spec.get(key)) for tag, key in (("w:b", "bold"), ("w:i", "italic"), ("w:u", "underline"))]
    if any(value for _, value in flags):
        properties = OxmlElement("w:rPr")
        for tag, value in flags:
            if value:
                flag = OxmlElement(tag)
                if tag == "w:u":
                    flag.set(qn("w:val"), "single")
                properties.append(flag)
        run.append(properties)
    text = OxmlElement("w:t")
    text.set(qn("xml:space"), "preserve")
    text.text = spec["text"]
    run.append(text)
    return run


def block_element(spec):
    if spec["kind"] == "table":
        table = OxmlElement("w:tbl")
        for row_spec in spec["rows"]:
            row = OxmlElement("w:tr")
            for cell_spec in row_spec["cells"]:
                cell = OxmlElement("w:tc")
                for child in cell_spec["blocks"]:
                    cell.append(block_element(child))
                row.append(cell)
            table.append(row)
        return table
    paragraph = OxmlElement("w:p")
    if spec.get("style"):
        properties = OxmlElement("w:pPr")
        style = OxmlElement("w:pStyle")
        style.set(qn("w:val"), spec["style"])
        properties.append(style)
        paragraph.append(properties)
    for run in spec.get("runs", []):
        paragraph.append(run_element(run))
    return paragraph


def style_element(spec):
    style = OxmlElement("w:style")
    style.set(qn("w:type"), "paragraph")
    style.set(qn("w:styleId"), spec["id"])
    name = OxmlElement("w:name")
    name.set(qn("w:val"), spec["name"])
    style.append(name)
    if spec.get("basedOn"):
        based_on = OxmlElement("w:basedOn")
        based_on.set(qn("w:val"), spec["basedOn"])
        style.append(based_on)
    return style


def style_by_id(document, style_id):
    matches = [style for style in document.styles.element.style_lst if style.styleId == style_id]
    assert len(matches) == 1, "the real input declares no style %s" % style_id
    return matches[0]


def run_at(document, params):
    paragraph = blocks(container_at(document.element.body, params["path"]["segments"]))[params["path"]["index"]]
    assert paragraph.tag == qn("w:p"), "the addressed block is not a paragraph"
    return Run(paragraph.findall(qn("w:r"))[params["runIndex"]], None)


def part_named(document, path):
    matches = [part for part in document.part.package.iter_parts() if part.partname == "/" + path]
    assert len(matches) == 1, "the real input carries no part %s" % path
    return matches[0]


def apply(document, kind, params):
    body = document.element.body
    if kind == "set-snapshot":
        for block in blocks(body):
            body.remove(block)
        anchor = body.find(qn("w:sectPr"))
        for spec in params["body"]:
            element = block_element(spec)
            if anchor is None:
                body.append(element)
            else:
                anchor.addprevious(element)
        styles = document.styles.element
        for style in styles.findall(qn("w:style")):
            styles.remove(style)
        for spec in params["styles"]:
            styles.append(style_element(spec))
    elif kind == "insert-block":
        container = container_at(body, params["path"]["segments"])
        existing = blocks(container)
        element = block_element(params["block"])
        if params["path"]["index"] < len(existing):
            existing[params["path"]["index"]].addprevious(element)
        else:
            existing[-1].addnext(element)
    elif kind == "remove-block":
        block = blocks(container_at(body, params["path"]["segments"]))[params["path"]["index"]]
        block.getparent().remove(block)
    elif kind == "set-block-content":
        block = blocks(container_at(body, params["path"]["segments"]))[params["path"]["index"]]
        block.addprevious(block_element(params["block"]))
        block.getparent().remove(block)
    elif kind == "set-run-text":
        run_at(document, params).text = params["text"]
    elif kind == "set-run-formatting":
        run = run_at(document, params)
        run.bold = True if params["bold"] else None
        run.italic = True if params["italic"] else None
        run.underline = True if params["underline"] else None
    elif kind == "insert-style":
        style = document.styles.element.add_style_of_type(params["style"]["name"], WD_STYLE_TYPE.PARAGRAPH, False)
        style.styleId = params["style"]["id"]
        style.basedOn_val = params["style"].get("basedOn") or None
    elif kind == "remove-style":
        style_by_id(document, params["id"]).delete()
    elif kind == "set-style-name":
        style_by_id(document, params["id"]).name_val = params["name"]
    elif kind == "set-style-based-on":
        style_by_id(document, params["id"]).basedOn_val = params.get("basedOn") or None
    elif kind == "set-part":
        part = part_named(document, params["path"])
        assert part.content_type == params["contentType"], "set-part would change the content type"
        part._blob = params["content"].encode("utf-8")
    elif kind == "remove-part":
        package = document.part.package
        targets = [r_id for r_id, rel in package.rels.items() if not rel.is_external and rel.target_part.partname == "/" + params["path"]]
        assert len(targets) == 1, "the real input relates no part %s" % params["path"]
        package.rels.pop(targets[0])
    else:
        raise SystemExit("unknown mutation kind %s" % kind)


def deterministic(archive):
    source = zipfile.ZipFile(io.BytesIO(archive))
    out = io.BytesIO()
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as target:
        for info in source.infolist():
            entry = zipfile.ZipInfo(info.filename, FIXED_DATE)
            entry.compress_type = zipfile.ZIP_DEFLATED
            entry.external_attr = 0o644 << 16
            target.writestr(entry, source.read(info.filename))
    return out.getvalue()


def main(argv):
    root = Path(argv[argv.index("--root") + 1]) if "--root" in argv else Path.cwd()
    only = argv[argv.index("--only") + 1] if "--only" in argv else None
    subset = root / SUBSET
    staging = os.environ.get("SEMIO_FIXTURE_OUT")
    PartFactory.part_type_for.pop(CT.OPC_CORE_PROPERTIES, None)
    selected = [(kind, params) for kind, params in rows((subset / FEATURE).read_text(encoding="utf-8")) if kind in DIRECTORIES and (only is None or fixture_id(kind) == only)]
    if not selected:
        raise SystemExit("no Examples row matches %s" % only)
    for kind, params in selected:
        document = docx.Document(str(subset / INPUT))
        apply(document, kind, params)
        saved = io.BytesIO()
        document.save(saved)
        archive = deterministic(saved.getvalue())
        if staging:
            stage = Path(staging) / fixture_id(kind)
            stage.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(subset / INPUT, stage / Path(INPUT).name)
            (stage / "➡️after.docx").write_bytes(archive)
        else:
            target = subset / AFTERS / DIRECTORIES[kind] / "➡️after.docx"
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(archive)
        print("[docx readme afters] %s (%d bytes)" % (fixture_id(kind), len(archive)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
