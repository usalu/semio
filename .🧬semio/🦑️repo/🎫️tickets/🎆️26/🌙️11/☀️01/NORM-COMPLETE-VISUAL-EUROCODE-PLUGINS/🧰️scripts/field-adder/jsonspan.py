"""🧵 Structure-preserving JSON text editor: parses a JSON text into nodes that carry their character spans and inserts members or items next to existing ones while the file keeps its own layout (pretty, compact or a mix of both). Only the inserted text is new; everything else stays byte for byte. Used by the field-adder profiles to add schema entries and fixture members."""
import json


class Node:
    __slots__ = ("kind", "start", "end", "members", "items", "text")

    def __init__(self, kind, start, end, members=None, items=None, text=None):
        self.kind, self.start, self.end, self.members, self.items, self.text = kind, start, end, members, items, text

    def get(self, key):
        for member in self.members or []:
            if member.key == key:
                return member
        return None


class Member:
    __slots__ = ("key", "key_start", "key_end", "value")

    def __init__(self, key, key_start, key_end, value):
        self.key, self.key_start, self.key_end, self.value = key, key_start, key_end, value


def parse(text):
    position = 0
    size = len(text)

    def skip():
        nonlocal position
        while position < size and text[position] in " \t\r\n":
            position += 1

    def string():
        nonlocal position
        start = position
        position += 1
        while text[position] != '"':
            position += 2 if text[position] == "\\" else 1
        position += 1
        return start, position

    def value():
        nonlocal position
        skip()
        start = position
        char = text[position]
        if char == "{":
            position += 1
            members = []
            skip()
            if text[position] == "}":
                position += 1
                return Node("object", start, position, members=members)
            while True:
                skip()
                key_start, key_end = string()
                skip()
                assert text[position] == ":", position
                position += 1
                members.append(Member(json.loads(text[key_start:key_end]), key_start, key_end, value()))
                skip()
                if text[position] == ",":
                    position += 1
                    continue
                assert text[position] == "}", position
                position += 1
                return Node("object", start, position, members=members)
        if char == "[":
            position += 1
            items = []
            skip()
            if text[position] == "]":
                position += 1
                return Node("array", start, position, items=items)
            while True:
                items.append(value())
                skip()
                if text[position] == ",":
                    position += 1
                    continue
                assert text[position] == "]", position
                position += 1
                return Node("array", start, position, items=items)
        if char == '"':
            _, end = string()
            return Node("scalar", start, end, text=text[start:end])
        while position < size and text[position] not in ",}] \t\r\n":
            position += 1
        return Node("scalar", start, position, text=text[start:position])

    root = value()
    skip()
    assert position == size, "trailing text"
    return root


def render(data, text, level=0):
    """The JSON text of `data` in the style of `text`: one line when the file has no line breaks, otherwise two-space pretty at nesting `level` (members keep `": "`)."""
    if "\n" not in text.strip():
        return json.dumps(data, ensure_ascii=False, separators=(",", ":"))
    pretty = json.dumps(data, indent=2, ensure_ascii=False)
    return pretty.replace("\n", "\n" + "  " * level)


def inline(data):
    return json.dumps(data, ensure_ascii=False)


def _separator(text, node, index):
    """The text between two neighbouring members or items (comma included), taken from the neighbours of position `index` so a new element repeats the local layout."""
    entries = node.members if node.kind == "object" else node.items
    first = (lambda e: e.key_start) if node.kind == "object" else (lambda e: e.start)
    last = (lambda e: e.value.end) if node.kind == "object" else (lambda e: e.end)
    if len(entries) >= 2:
        i = min(index, len(entries) - 2)
        return text[last(entries[i]):first(entries[i + 1])]
    opening = node.start + 1
    gap = text[opening:first(entries[0])]
    return "," + (gap if gap.strip() == "" else "")


def insert_member(text, obj, after_key, key, rendered):
    """Text with `"key": rendered` inserted behind the member `after_key` of the parsed object `obj`; `rendered` is the already formatted value. Idempotent: an existing key is left alone."""
    if obj.get(key):
        return text
    anchor = obj.get(after_key)
    assert anchor, ("member anchor", after_key)
    index = obj.members.index(anchor)
    colon = text[anchor.key_end:anchor.value.start]
    separator = _separator(text, obj, index)
    return text[:anchor.value.end] + separator + json.dumps(key) + colon + rendered + text[anchor.value.end:]


def insert_item(text, array, after_index, rendered):
    """Text with an array item inserted behind item `after_index`."""
    anchor = array.items[after_index]
    separator = _separator(text, array, after_index)
    return text[:anchor.end] + separator + rendered + text[anchor.end:]


def find_objects(node, predicate, path=()):
    """Every object node under `node` (itself included) whose members satisfy `predicate(obj)`, depth first, with its member path."""
    found = []
    if node.kind == "object":
        if predicate(node):
            found.append((path, node))
        for member in node.members:
            found += find_objects(member.value, predicate, path + (member.key,))
    elif node.kind == "array":
        for index, item in enumerate(node.items):
            found += find_objects(item, predicate, path + (index,))
    return found


def apply_edits(text, edits):
    """Apply `(position, insertion)` edits from the back so earlier positions stay valid."""
    for position, insertion in sorted(edits, key=lambda e: -e[0]):
        text = text[:position] + insertion + text[position:]
    return text
