
from pathlib import Path
import json, re
ROOT = Path(".").resolve()
TICKET = next(ROOT.joinpath(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))

def mimes_hits():
    hits = []
    for p in ROOT.rglob("*mimes.csv"):
        s = str(p)
        if any(x in s for x in ["node_modules", "/target/", "/dist/", "fixture", "storybook-static"]):
            continue
        hits.append(p)
    return sorted(hits)

def find_refs():
    out = []
    for p in ROOT.rglob("*"):
        if not p.is_file():
            continue
        s = str(p)
        if any(x in s for x in ["node_modules", "/target/", "/dist/", ".git/", "storybook-static"]):
            continue
        if p.suffix not in {".rs", ".ts", ".tsx", ".js", ".json", ".jsonc", ".md", ".toml"}:
            continue
        try:
            text = p.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue
        if "mimes.csv" in text or "mimes" in text and "ui/🖼️assets" in text:
            for i, line in enumerate(text.splitlines(), 1):
                if "mimes.csv" in line or ("mimes" in line and "assets" in line):
                    out.append(f"{p}:{i}:{line.strip()[:160]}")
    return out

print("MIMES", [str(h) for h in mimes_hits()])
refs = find_refs()
(TICKET/"generators"/"w7-mimes-refs.txt").write_text("\n".join(refs)+"\n", encoding="utf-8")
print("REFS", len(refs))
for r in refs[:50]:
    print(r)
