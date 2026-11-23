"""🩹️ Splices hand-written values into a catalog asset file without touching any other byte: `Doc(path)` parses the document, records the text span of every value by path, and `set`, `many` and `save` replace single values in place (scalars as they are, point lists inline, objects in the two-space style of the files)."""
import glob
import json
import os

STAFFAGE = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../../../../../../..", "🎓️teaching/🏛️architecture/🧍️staffage"))


def catalog_file(slug):
    """📂️ The catalog file of an asset id."""
    hits = glob.glob(os.path.join(STAFFAGE, "🗂️catalog", "*", slug, "🔣️.json"))
    assert len(hits) == 1, (slug, hits)
    return hits[0]


def _inline(value):
    return json.dumps(value, ensure_ascii=False)


def render(value, depth):
    """🖋️ One value as file text at a nesting depth: flat lists and lists of flat lists inline, objects and lists of objects one member per line."""
    pad = "  " * depth
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(f"{pad}  {json.dumps(k, ensure_ascii=False)}: {render(v, depth + 1)}" for k, v in value.items()) + f"\n{pad}}}"
    if isinstance(value, list) and any(isinstance(v, dict) for v in value):
        return "[\n" + ",\n".join(f"{pad}  {render(v, depth + 1)}" for v in value) + f"\n{pad}]"
    return _inline(value)


class Doc:
    """📄️ One asset document with the text span of every value."""

    def __init__(self, path):
        self.path = path
        self.text = open(path, encoding="utf8").read()
        self.spans = {}
        self.edits = []
        self.data, end = self._value(self._skip(0), ())
        assert self.text[end:].strip() == ""
        assert self.data == json.loads(self.text)

    def _skip(self, i):
        while i < len(self.text) and self.text[i] in " \t\r\n":
            i += 1
        return i

    def _value(self, i, path):
        t = self.text
        start = i
        if t[i] == "{":
            out = {}
            i = self._skip(i + 1)
            while t[i] != "}":
                key, i = json.decoder.scanstring(t, i + 1)
                i = self._skip(i)
                assert t[i] == ":"
                out[key], i = self._value(self._skip(i + 1), path + (key,))
                i = self._skip(i)
                if t[i] == ",":
                    i = self._skip(i + 1)
            i += 1
        elif t[i] == "[":
            out = []
            i = self._skip(i + 1)
            while t[i] != "]":
                item, i = self._value(i, path + (len(out),))
                out.append(item)
                i = self._skip(i)
                if t[i] == ",":
                    i = self._skip(i + 1)
            i += 1
        else:
            out, i = json.JSONDecoder().raw_decode(t, i)
        self.spans[path] = (start, i)
        return out, i

    def depth(self, path):
        """📏️ The indentation depth of the line a value starts on."""
        start = self.spans[path][0]
        line = self.text.rfind("\n", 0, start) + 1
        return (len(self.text[line:start]) - len(self.text[line:start].lstrip(" "))) // 2

    def set(self, path, value):
        """✏️ Replaces the value at a path."""
        path = tuple(path)
        self.edits.append((self.spans[path], render(value, self.depth(path))))

    def many(self, path, values):
        """➕️ Replaces one list element by several."""
        path = tuple(path)
        depth = self.depth(path)
        self.edits.append((self.spans[path], (",\n" + "  " * depth).join(render(v, depth) for v in values)))

    def shapes(self, scale, view):
        """🔎️ The shapes of a view of a scale with their paths."""
        base = ("asset", "representations", str(scale), "geometry", view)
        node = self.data["asset"]["representations"][str(scale)]["geometry"][view]
        return [(base + (i,), shape) for i, shape in enumerate(node)]

    def shape(self, scale, view, shape_id, bone=None):
        """🎯️ The one shape of a view with an id (and bone)."""
        hits = [(p, s) for p, s in self.shapes(scale, view) if s["id"] == shape_id and (bone is None or s["bone"] == bone)]
        assert len(hits) == 1, (scale, view, shape_id, bone, len(hits))
        return hits[0]

    def save(self, target=None):
        """💾️ Writes the document with every edit applied; refuses when the file changed on disk since it was read."""
        assert open(self.path, encoding="utf8").read() == self.text, "file changed under the edit: run again"
        text = self.text
        ordered = sorted(self.edits, key=lambda e: e[0][0], reverse=True)
        for a, b in zip(ordered, ordered[1:]):
            assert b[0][1] <= a[0][0], "overlapping edits"
        for (start, end), new in ordered:
            text = text[:start] + new + text[end:]
        json.loads(text)
        target = target or self.path
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(target, "w", encoding="utf8") as handle:
            handle.write(text)
        return len(self.edits)
