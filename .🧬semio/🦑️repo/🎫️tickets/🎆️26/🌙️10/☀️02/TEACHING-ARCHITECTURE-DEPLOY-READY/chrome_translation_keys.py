import os
import re

ticket = os.path.dirname(os.path.abspath(__file__))
found = next(os.path.join(ticket, name, "module-weights.txt") for name in os.listdir(ticket) if name.endswith("generated") and os.path.isfile(os.path.join(ticket, name, "module-weights.txt")))
print("weights", found.encode("unicode_escape").decode() if found else None)
text = open(found, encoding="utf-8").read().splitlines()
ids = []
seen_chunk = False
for line in text:
    if " chunk assets/" in line:
        if seen_chunk:
            break
        seen_chunk = True
        continue
    if not seen_chunk:
        continue
    marker = line.find("C:/")
    if marker < 0:
        marker = line.find("C:\\")
    if marker >= 0 and "elements" in line:
        ids.append(line[marker:].strip())
print("element files", len(ids))
key_re = re.compile(r"""["'`](ui(?:\.[A-Za-z0-9_]+)+)["'`]""")
for path in ids:
    native = path.replace("/", os.sep)
    if not os.path.exists(native):
        print("missing", native.encode("unicode_escape").decode()[-90:])
        continue
    body = open(native, encoding="utf-8", errors="ignore").read()
    found_keys = sorted(set(key_re.findall(body)))
    if found_keys:
        print(os.path.basename(os.path.dirname(native)).encode("unicode_escape").decode(), len(found_keys))
        for key in found_keys:
            print(" ", key)
