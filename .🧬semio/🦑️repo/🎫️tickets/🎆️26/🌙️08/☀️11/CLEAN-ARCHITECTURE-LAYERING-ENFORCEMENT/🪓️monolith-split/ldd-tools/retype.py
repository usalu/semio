"""🔁️ Re-types the drawing fixtures of one layout test file onto the neutral vector scene (sandbox-free, edits one file in place, asserts every anchor)."""
import re, sys

RENAMES = [("SemioDrawingSnapshot", "VectorScene"), ("SemioPoint2", "VectorPoint2"), ("SemioPoint3", "VectorPoint3"), ("SemioQuaternion", "VectorQuaternion"), ("SemioTransform", "VectorTransform"), ("SemioRgba", "VectorRgba"), ("DrawLayer", "VectorLayer"), ("DrawNode", "VectorNode"), ("DrawStyle", "VectorStyle"), ("DrawCanvas", "VectorCanvas"), ("PathSegment", "VectorSegment")]
BASE = "semio_s_artifact_stdio_semio_base::schema::geometry::"
DRAW = "semio_s_artifact_stdio_semio_drawing::schema::snapshot::"


def rename_words(text):
    for old, new in RENAMES:
        text = re.sub(r"\b%s\b" % old, new, text)
    return text


def retype_uses(text):
    out = []
    lines = text.split("\n")
    i = 0
    while i < len(lines):
        line = lines[i]
        m = re.match(r"^(\s*)use " + re.escape(BASE) + r"(\w+);$", line)
        d = re.match(r"^(\s*)use " + re.escape(DRAW) + r"\{([^}]*)\};$", line)
        d1 = re.match(r"^(\s*)use " + re.escape(DRAW) + r"(\w+);$", line)
        if m or d or d1:
            indent = (m or d or d1).group(1)
            names = []
            j = i
            while j < len(lines):
                l = lines[j]
                mm = re.match(r"^\s*use " + re.escape(BASE) + r"(\w+);$", l)
                dd = re.match(r"^\s*use " + re.escape(DRAW) + r"\{([^}]*)\};$", l)
                d11 = re.match(r"^\s*use " + re.escape(DRAW) + r"(\w+);$", l)
                if mm:
                    names.append(mm.group(1))
                elif dd:
                    names += [n.strip() for n in dd.group(1).split(",") if n.strip()]
                elif d11:
                    names.append(d11.group(1))
                else:
                    break
                j += 1
            mapped = sorted({dict(RENAMES).get(n, n) for n in names})
            out.append(indent + "use semio_framework_io_ports::{" + ", ".join(mapped) + "};")
            i = j
            continue
        out.append(line)
        i += 1
    return "\n".join(out)


def retype_qualified(text):
    text = text.replace(DRAW + "SemioDrawingSnapshot", "semio_framework_io_ports::VectorScene")
    text = text.replace(DRAW + "DrawNode", "semio_framework_io_ports::VectorNode")
    text = text.replace(DRAW + "DrawLayer", "semio_framework_io_ports::VectorLayer")
    text = text.replace(DRAW + "PathSegment", "semio_framework_io_ports::VectorSegment")
    text = text.replace(DRAW + "DrawStyle", "semio_framework_io_ports::VectorStyle")
    text = text.replace(DRAW + "DrawCanvas", "semio_framework_io_ports::VectorCanvas")
    text = text.replace(BASE + "SemioPoint2", "semio_framework_io_ports::VectorPoint2")
    text = text.replace(BASE + "SemioTransform", "semio_framework_io_ports::VectorTransform")
    text = text.replace(BASE + "SemioRgba", "semio_framework_io_ports::VectorRgba")
    return text


PNG_BLOCK = re.compile(r'let mut (\w+) = semio_s_artifact_stdio_png::[^\n]*?PngProjection \{[^\n]*?\};\n(\s*)\1\.width = (\d+);\n\s*\1\.height = (\d+);\n\s*\1\.pixels = (vec!\[[^\]]*\]);\n\s*let (\w+) = semio_s_artifact_stdio_png::[^\n]*?author_png_projection\(&\1\)\.expect\("png"\);')


def retype_png(text):
    def sub(m):
        return 'let %s = semio_framework_pixels::RasterImage { width: %s, height: %s, pixels: %s };\n%slet %s = semio_framework_pixels::encode_png(&%s).expect("png");' % (m.group(1), m.group(3), m.group(4), m.group(5), m.group(2), m.group(6), m.group(1))
    text = PNG_BLOCK.sub(sub, text)
    return text.replace("semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::project_png(", "semio_framework_pixels::decode_png(")


def bind_functions(text, marker="background_drawing = ", slot="semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING"):
    fn_re = re.compile(r"(\n(?:async )?fn (\w+)\(\) \{\n)")
    names = []
    for m in fn_re.finditer(text):
        start = m.end()
        nxt = fn_re.search(text, start)
        body = text[start:(nxt.start() if nxt else len(text))]
        if marker in body or "LayoutDrawingChild {" in body:
            names.append((m.group(1), m.group(2)))
    for header, name in names:
        assert text.count(header) == 1, name
        text = text.replace(header, header + "    bind(&" + slot + ");\n")
    return text, [n for _, n in names]


def add_testkit_import(text):
    assert text.startswith("use super::*;\n"), text[:40]
    return text.replace("use super::*;\n", "use super::*;\nuse semio_framework_io_ports::testkit::{bind, scene_content};\n", 1)


if __name__ == "__main__":
    path = sys.argv[1]
    text = open(path, encoding="utf8").read()
    text = retype_qualified(text)
    text = retype_uses(text)
    text = rename_words(text)
    open(path, "w", encoding="utf8").write(text)
