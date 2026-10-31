#!/usr/bin/env python3
"""Ports one mutate-semio-* Python adapter with an inline `# region 🔖️Pack` to the shared record-document codec.

usage: port_inline.py <suite 🐍️.py>   (prints the names the removed region defined that the rest of the file still uses)
"""
import ast
import pathlib
import re
import sys

BLOCK = '''# region 🔖️Pack
_HERE = os.path.dirname(os.path.abspath(__file__))


def _load_shared(alias, *parts):
    """🧬️ Loads a shared module from the `stdio.semio` artifact root, six folders above this case, and registers it under an importable name."""
    if alias in sys.modules:
        return sys.modules[alias]
    path = os.path.join(_HERE, *[os.pardir] * 6, *parts)
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


record_document = _load_shared("record_document", "🚪️io", "💾️binary", "🎒️record-document", "🐍️.py")
RECORD_SPEC = record_document.load_record_spec(os.path.join(_HERE, os.pardir, os.pardir, "🧬️schema", "📸️snapshot", "🗺️record-spec", "🔣️.json"))


def parse_pack(data: bytes) -> dict:
    """📦️ A `.pack.semio` file as the JSON snapshot its record document carries."""
    return record_document.decode_pack(RECORD_SPEC, data)


def pack_bytes(document: dict) -> bytes:
    """📦️ The writing direction of `parse_pack`, envelope included."""
    return record_document.encode_pack(RECORD_SPEC, document)


# endregion 🔖️Pack
'''

path = pathlib.Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
start = text.index("# region 🔖️Pack")
end_marker = "# endregion 🔖️Pack"
end = text.index(end_marker, start) + len(end_marker) + 1
removed = text[start:end]
defined = set()
for node in ast.parse(removed).body:
    if isinstance(node, (ast.FunctionDef, ast.ClassDef)):
        defined.add(node.name)
    elif isinstance(node, ast.Assign):
        for target in node.targets:
            for sub in ast.walk(target):
                if isinstance(sub, ast.Name):
                    defined.add(sub.id)
rest = text[:start] + BLOCK + text[end:]
need = [name for name in ("os", "sys", "importlib") if not re.search(r"^import (%s)\b|^import .*\b%s\b|^from importlib" % (name, name), text, re.M)]
tree = ast.parse(rest)
used = {n.id for n in ast.walk(tree) if isinstance(n, ast.Name)} | {n.value.id for n in ast.walk(tree) if isinstance(n, ast.Attribute) and isinstance(n.value, ast.Name)}
still = sorted(name for name in defined if name in used and name not in ("parse_pack", "pack_bytes"))
region_start = rest.index("# region 🔖️Imports")
region_end = rest.index("# endregion 🔖️Imports")
lines = rest[region_start:region_end].split("\n")
imports = [line for line in lines if line.startswith("import ")]
wanted = {"os": "import os", "sys": "import sys", "importlib": "import importlib.util"}
for name in need:
    if wanted[name] not in imports:
        imports.append(wanted[name])
imports = sorted(set(imports))
first = next(i for i, line in enumerate(lines) if line.startswith("import "))
body = [line for line in lines if not line.startswith("import ")]
insert_at = first if first < len(body) else len(body)
lines = body[:insert_at] + imports + body[insert_at:]
rest = rest[:region_start] + "\n".join(lines) + rest[region_end:]
path.write_text(rest, encoding="utf-8")
print("imports to add:", need)
print("removed names still referenced elsewhere:", still)
