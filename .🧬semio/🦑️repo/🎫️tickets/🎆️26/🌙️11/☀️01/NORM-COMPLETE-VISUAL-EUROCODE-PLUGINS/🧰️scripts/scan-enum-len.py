import os
import re

root = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
enum_re = re.compile(r"pub enum (\w+)")
field_re = re.compile(r"pub (\w+): (\w+)")
use_re = re.compile(r"\.(\w+)\.len\(")

for name in sorted(os.listdir(root)):
    artifact = os.path.join(root, name)
    if not os.path.isdir(artifact):
        continue
    enums = set()
    enum_fields = set()
    sqlite = []
    for dirpath, _, files in os.walk(artifact):
        for filename in files:
            if not filename.endswith(".rs"):
                continue
            path = os.path.join(dirpath, filename)
            text = open(path, encoding="utf-8", errors="replace").read()
            if "/🧬️schema/" in path.replace("\\", "/"):
                enums.update(enum_re.findall(text))
            if "sqlite" in dirpath and dirpath.rstrip("/").endswith("snapshot") and "/tests/" not in path and "admission" not in path:
                sqlite.append((path, text))
    for dirpath, _, files in os.walk(artifact):
        for filename in files:
            if not filename.endswith(".rs") or "/🧬️schema/" not in dirpath.replace("\\", "/"):
                continue
            text = open(os.path.join(dirpath, filename), encoding="utf-8", errors="replace").read()
            for field, typ in field_re.findall(text):
                if typ in enums:
                    enum_fields.add(field)
    for path, text in sqlite:
        for match in use_re.finditer(text):
            field = match.group(1)
            if field in enum_fields:
                line = text.count("\n", 0, match.start()) + 1
                print(f"{name}:{line}:{field}")
