import os
import sys

roots = {
    "1990": "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990",
    "1993": "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993",
    "1997": "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997",
    "1999": "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999",
}
need = {
    "1990": "traffic_groups",
    "1993": "reliability_class",
    "1997": "characteristic_bases",
    "1999": "transverse_stiffeners",
}
for code, root in roots.items():
    field = need[code]
    found = None
    for dirpath, _, files in os.walk(root):
        if "sqlite" not in dirpath or "snapshot" not in dirpath:
            continue
        for name in files:
            if name.endswith(".rs") and "test" not in dirpath:
                path = os.path.join(dirpath, name)
                text = open(path, encoding="utf-8").read()
                if "let result = Self" in text or "Ok(Self" in text:
                    found = path
                    break
        if found:
            break
    print("CODE", code, "FILE", "yes" if found else "no")
    if not found:
        continue
    text = open(found, encoding="utf-8").read()
    i = text.rfind("Self {")
    chunk = text[i:i + 1800]
    print(field, field in chunk)
    print(chunk[-400:].replace("\n", " "))
    print("---")
