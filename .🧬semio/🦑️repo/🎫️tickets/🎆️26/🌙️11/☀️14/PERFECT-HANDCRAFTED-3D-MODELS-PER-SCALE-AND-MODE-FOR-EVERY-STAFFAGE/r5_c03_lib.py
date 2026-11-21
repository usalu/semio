"""🧰 Round 5 C03 helpers: load a catalog component, write a candidate document, and splice changed asset members back in the file's own layout.

@see ./volumes-insert.py
@see ./r5-brief.md"""
import json
import os
import re
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components"


def scalar(value):
    """🔹 Leaf value (layout of volumes-insert.py)."""
    return not isinstance(value, (dict, list))


def inline(value):
    """📏 Arrays written on one line."""
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
    return False


def dump(value, depth):
    """🖨️ The catalog layout of volumes-insert.py."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def member_end(text, start):
    """🔚 End of a JSON member value."""
    depth, index, quoted = 0, start, False
    while index < len(text):
        char = text[index]
        if quoted:
            if char == "\\":
                index += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in "{[":
            depth += 1
        elif char in "}]":
            depth -= 1
            if depth == 0:
                return index + 1
        index += 1
    raise SystemExit("unbalanced asset file")


def member_range(text, indent, key):
    """📐 Text range of an asset member."""
    found = re.search(r'\n' + indent + r'"' + key + r'"\s*:\s*', text)
    if not found:
        return None
    return found.start() + 1, member_end(text, found.end())


def path_of(slug):
    """📍 Catalog file of a component."""
    return os.path.join(CATALOG, slug, "🔣️.json")


def load(slug):
    """📖 The whole catalog document."""
    return json.load(open(path_of(slug), encoding="utf-8"))


def candidate(slug, document):
    """🧪 Write the document with the volumes source merged to 🗑️generated/r5-<slug>/🔣️.json and validate it with r3-validate --file; returns (ok, last line)."""
    out = os.path.join(HERE, "🗑️generated", f"r5-{slug}", "🔣️.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    doc = json.loads(json.dumps(document))
    src = json.load(open(os.path.join(HERE, "volumes", "components", f"{slug}.json"), encoding="utf-8"))
    doc["asset"]["volumes"] = src["volumes"]
    json.dump(doc, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    bun = sorted(__import__("glob").glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
    run = subprocess.run([bun, "r3-validate.ts", "--file", out], cwd=HERE, capture_output=True, text=True)
    lines = (run.stdout + run.stderr).strip().splitlines()
    for line in lines[-12:]:
        print(line)
    return run.returncode == 0, out


def splice(slug, members):
    """✂️ Replace the given asset members (key -> value) in the catalog file, re-read right before writing; every other byte stays."""
    path = path_of(slug)
    text = open(path, encoding="utf-8").read()
    indent = re.search(r'\n( +)"id"\s*:', text).group(1)
    depth = len(indent) // 2
    for key, value in members.items():
        found = member_range(text, indent, key)
        if not found:
            raise SystemExit(f"member {key} not found")
        start, end = found
        text = text[:start] + indent + json.dumps(key) + ": " + json.dumps(value, indent=2, ensure_ascii=False).replace("\n", "\n" + indent) + text[end:]
    json.loads(text)
    open(path, "w", encoding="utf-8").write(text)
    print("spliced", list(members), "into", path)
