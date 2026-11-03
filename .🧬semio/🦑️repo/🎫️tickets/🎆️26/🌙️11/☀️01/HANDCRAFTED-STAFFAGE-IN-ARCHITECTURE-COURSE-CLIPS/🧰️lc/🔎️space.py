import re, pathlib, collections
c = collections.Counter(); ex = {}
for p in pathlib.Path(".").rglob("*.py"):
    s = str(p)
    if ".venv" in s or "cache" in s or "tests" in s or "site-packages" in s: continue
    for i, l in enumerate(p.read_text(encoding="utf-8", errors="ignore").split("\n"), 1):
        for m in re.finditer(r"\d([    ])(°C|W|kWh|Wh|kW)\b", l):
            k = hex(ord(m.group(1))); c[k] += 1; ex.setdefault(k, f"{s}:{i}: {l.strip()[:100]}")
print(c); print(ex)
