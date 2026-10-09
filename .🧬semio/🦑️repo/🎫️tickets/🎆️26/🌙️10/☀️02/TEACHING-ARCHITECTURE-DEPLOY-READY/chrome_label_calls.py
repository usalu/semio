import os
import re

ticket = os.path.dirname(os.path.abspath(__file__))
weights = next(os.path.join(ticket, name, "module-weights.txt") for name in os.listdir(ticket) if name.endswith("generated"))
ids = []
seen = False
for line in open(weights, encoding="utf-8"):
    if " chunk assets/" in line:
        if seen:
            break
        seen = True
        continue
    if not seen:
        continue
    marker = line.find("C:/")
    if marker >= 0 and ("elements" in line or "i18n" in line or "appearance" in line or "chrome" in line):
        ids.append(line[marker:].strip())
patterns = ("useLabel", "resolveUiLabel", "uiI18n", "t(", "translation")
for path in ids:
    native = path.replace("/", os.sep)
    if not os.path.exists(native):
        continue
    lines = open(native, encoding="utf-8", errors="ignore").read().splitlines()
    hits = [f"{index}:{line.strip()[:180]}" for index, line in enumerate(lines, 1) if any(pattern in line for pattern in patterns)]
    if not hits:
        continue
    print(os.path.basename(os.path.dirname(native)).encode("unicode_escape").decode(), len(hits))
    for hit in hits[:12]:
        print(" ", hit)
