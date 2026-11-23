"""🩹️ Hand edits of a catalog asset document that keep every untouched byte of the file: a value is addressed by its path, its text span is found by an own JSON scanner and only that span is rewritten (new objects are laid out like the hand-written files: two spaces, short arrays of numbers on one line)."""
import json
import os

ROOT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")


def catalog_file(asset_id):
    """📁️ The catalog file of an asset id."""
    for category in sorted(os.listdir(CATALOG)):
        path = os.path.join(CATALOG, category, asset_id, "🔣️.json")
        if os.path.exists(path):
            return path
    raise SystemExit(f"no catalog asset {asset_id}")


def _scalar(value):
    return not isinstance(value, (dict, list))


def _inline(value):
    if all(_scalar(item) for item in value):
        return len(json.dumps(value, ensure_ascii=False)) <= 160
    return all(isinstance(item, list) and all(_scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160


def layout(value, depth):
    """🧾️ The text of a value at an indentation depth."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        return "{\n" + ",\n".join(inner + json.dumps(key, ensure_ascii=False) + ": " + layout(item, depth + 1) for key, item in value.items()) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if _inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + layout(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


class Doc:
    """📄️ One asset document as text with the spans of its values."""

    def __init__(self, path):
        self.path = path
        self.text = open(path, encoding="utf8").read()
        self.before = self.text
        self._scan()

    def _scan(self):
        self.data = json.loads(self.text)
        self.spans = {}
        decoder = json.JSONDecoder()
        text = self.text

        def skip(i):
            while text[i] in " \t\r\n":
                i += 1
            return i

        def value(i, path):
            i = skip(i)
            start = i
            if text[i] == "{":
                i = skip(i + 1)
                if text[i] == "}":
                    i += 1
                else:
                    while True:
                        key, i = decoder.raw_decode(text, skip(i))
                        i = skip(i)
                        i = value(i + 1, path + (key,))
                        i = skip(i)
                        if text[i] == ",":
                            i += 1
                            continue
                        i += 1
                        break
            elif text[i] == "[":
                i = skip(i + 1)
                if text[i] == "]":
                    i += 1
                else:
                    index = 0
                    while True:
                        i = value(i, path + (index,))
                        index += 1
                        i = skip(i)
                        if text[i] == ",":
                            i += 1
                            continue
                        i += 1
                        break
            else:
                _, i = decoder.raw_decode(text, i)
            self.spans[path] = (start, i)
            return i

        value(0, ())

    def get(self, *path):
        """🔎️ The value at a path."""
        node = self.data
        for key in path:
            node = node[key]
        return node

    def _depth(self, start):
        line = self.text.rfind("\n", 0, start) + 1
        return (len(self.text[line:start]) - len(self.text[line:start].lstrip(" "))) // 2

    def set(self, path, value):
        """✏️ Rewrites the value at a path (the member must exist)."""
        start, end = self.spans[tuple(path)]
        self.text = self.text[:start] + layout(value, self._depth(start)) + self.text[end:]
        self._scan()

    def put(self, path, key, value):
        """➕️ Adds a member to the object at a path, or rewrites it when it exists."""
        if key in self.get(*path):
            return self.set(tuple(path) + (key,), value)
        node = dict(self.get(*path))
        node[key] = value
        self.set(path, node)

    def drop(self, path, key):
        """➖️ Removes a member of the object at a path."""
        node = dict(self.get(*path))
        del node[key]
        self.set(path, node)

    def insert(self, path, index, value):
        """📥️ Inserts an element into the array at a path before `index` (or appends when `index` is the length)."""
        items = self.get(*path)
        if index < len(items):
            start, _ = self.spans[tuple(path) + (index,)]
            self.text = self.text[:start] + layout(value, self._depth(start)) + ",\n" + "  " * self._depth(start) + self.text[start:]
        else:
            start, end = self.spans[tuple(path) + (len(items) - 1,)]
            self.text = self.text[:end] + ",\n" + "  " * self._depth(start) + layout(value, self._depth(start)) + self.text[end:]
        self._scan()

    def remove(self, path, index):
        """🗑️ Removes the element `index` of the array at a path."""
        items = self.get(*path)
        start, end = self.spans[tuple(path) + (index,)]
        if index + 1 < len(items):
            end = self.spans[tuple(path) + (index + 1,)][0]
        else:
            start = self.spans[tuple(path) + (index - 1,)][1]
        self.text = self.text[:start] + self.text[end:]
        self._scan()

    def shape(self, scale, view, shape_id):
        """🧩️ The path of a shape of a view of a scale by its id."""
        shapes = self.get("asset", "representations", str(scale), "geometry", view)
        matches = [i for i, s in enumerate(shapes) if s["id"] == shape_id]
        if len(matches) != 1:
            raise SystemExit(f"{len(matches)} shapes {shape_id} in 1:{scale} {view}")
        return ("asset", "representations", str(scale), "geometry", view, matches[0])

    def save(self):
        """💾️ Writes the file when the document on disk is still the one that was read (the worktree is shared and live)."""
        if open(self.path, encoding="utf8").read() != self.before:
            raise SystemExit(f"{self.path} changed on disk since it was read: nothing written, run again")
        json.loads(self.text)
        with open(self.path, "w", encoding="utf8") as handle:
            handle.write(self.text)
        self.before = self.text
