import re
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
SKIP = {".venv", "site-packages", "__pycache__", ".ruff_cache", "🔊️speech"}
NUM = r"(?:\d[\d{},.]*|\{?\{?(?:label_number|ru|fmt|_fmt|num)\([^)]*\)\}?\}?)"
BEFORE = re.compile(NUM + r"\s*\\cdot")
AFTER = re.compile(r"\\cdot\s*" + NUM)
UNIT = re.compile(r"\\mathrm|\\frac\{\{?\\mathrm|\\%|\\,\s*\\text|°")


def main():
    for p in ROOT.rglob("*"):
        if not p.is_file() or p.suffix not in {".py", ".json", ".md"} or set(p.parts) & SKIP or "🧪️tests" in p.parts:
            continue
        for i, line in enumerate(p.read_text(encoding="utf-8").splitlines(), 1):
            if "\\cdot" not in line and "·" not in line:
                continue
            for m in list(BEFORE.finditer(line)) + list(AFTER.finditer(line)):
                tail = line[m.end() : m.end() + 14] if m.re is BEFORE else ""
                head = line[max(0, m.start() - 0) : m.end()]
                after_num = line[m.end() : m.end() + 12]
                if m.re is AFTER and re.match(r"\s*\\,?\s*(\\mathrm|\\frac|\\text|\\%)", after_num):
                    continue
                if m.re is BEFORE:
                    num_end = m.end() - len("\\cdot")
                    if re.search(r"(\\mathrm|\\%|\\text\{|°)", line[num_end - 6 : m.end()]):
                        continue
                print(f"{p.relative_to(ROOT)}:{i}: {line.strip()[:230]}")
                break


main()
