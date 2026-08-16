from __future__ import annotations

import json
import re
import sys
from pathlib import Path


ROOT = Path(r"E:\semio")
sys.stdout.reconfigure(encoding="utf-8")
TEX = ROOT / "mit-bestand/bericht/zwischenbericht/anhang/projekte.tex"
BIB = ROOT / "mit-bestand/bericht/zwischenbericht/references.bib"


def group(text: str, start: int) -> tuple[str, int]:
    while start < len(text) and text[start].isspace():
        start += 1
    if text[start] != "{":
        raise ValueError((start, text[start : start + 30]))
    depth = 0
    for index in range(start, len(text)):
        char = text[index]
        if char == "{" and (index == 0 or text[index - 1] != "\\"):
            depth += 1
        elif char == "}" and (index == 0 or text[index - 1] != "\\"):
            depth -= 1
            if depth == 0:
                return text[start + 1 : index], index + 1
    raise ValueError("unclosed group")


def bib_entries(text: str) -> dict[str, dict[str, str]]:
    entries: dict[str, dict[str, str]] = {}
    for match in re.finditer(r"@\w+\{([^,]+),", text):
        key = match.group(1).strip()
        body, _ = group(text, text.find("{", match.start()))
        fields = {
            name.lower(): value.strip()
            for name, value in re.findall(r"(?ms)^\s*(\w+)\s*=\s*\{(.*?)\}\s*,?", body)
        }
        entries[key] = fields
    return entries


text = TEX.read_text(encoding="utf-8")
bib = bib_entries(BIB.read_text(encoding="utf-8"))
projects: list[dict[str, object]] = []
cursor = 0
while True:
    start = text.find("\\SemioProject", cursor)
    if start < 0:
        break
    pos = start + len("\\SemioProject")
    args: list[str] = []
    for _ in range(5):
        value, pos = group(text, pos)
        args.append(value)
    title, label, image, metadata, rows_raw = args
    project_id = f"P{int(label.split(':')[1]):02d}"
    meta = [part.strip() for part in metadata.split(" & ")]
    cite_keys: list[str] = []
    for cites in re.findall(r"\\cite\{([^}]+)\}", metadata):
        cite_keys.extend(item.strip() for item in cites.split(","))
    rows = []
    row_cursor = 0
    while True:
        row_start = rows_raw.find("\\SemioTableRow", row_cursor)
        if row_start < 0:
            break
        row, row_cursor = group(rows_raw, row_start + len("\\SemioTableRow"))
        rows.append([part.strip() for part in row.split(" & ")])
    projects.append(
        {
            "id": project_id,
            "title": title,
            "image": image,
            "city": meta[0],
            "country": meta[1],
            "year": meta[2],
            "type": meta[3],
            "status": meta[4],
            "citations": cite_keys,
            "sources": [
                {
                    "key": key,
                    "title": bib.get(key, {}).get("title", ""),
                    "url": bib.get(key, {}).get("url", ""),
                    "urldate": bib.get(key, {}).get("urldate", ""),
                }
                for key in cite_keys
            ],
            "components": rows,
        }
    )
    cursor = pos

projects.sort(key=lambda project: project["id"])
assert len(projects) == 67, len(projects)
out = Path(__file__).with_name("projects_existing.json")
out.write_text(json.dumps(projects, ensure_ascii=False, indent=2), encoding="utf-8")
print(f"projects={len(projects)} components={sum(len(p['components']) for p in projects)} output={out}")
for project in projects:
    urls = ", ".join(source["url"] for source in project["sources"])
    print(f"{project['id']}\t{project['title']}\t{project['city']}\t{project['year']}\t{urls}")
