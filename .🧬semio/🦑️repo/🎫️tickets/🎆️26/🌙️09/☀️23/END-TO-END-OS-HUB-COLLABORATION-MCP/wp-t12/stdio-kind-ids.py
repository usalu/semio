#!/usr/bin/env python3
"""🪪️ Post-publish patch set (T12, session 12; coordinator decision 23:3x): every stdio artifact kind id becomes the
canonical `s.stdio.<artifact>` its own dialects, artifact definition and codec ids already use (gltf's precedent), so the
hub's one open-target rule pairs each plugin-level native-codec kind with the editors whose dialect names it.
Only the KIND id moves: document schemas (`stdio.<x>`), format kinds (`export_stdio_kinds`), codec/factory ids stay.
`--inventory` classifies every tracked occurrence of a quoted `stdio.<artifact>` token by its line context.
Usage: stdio-kind-ids.py --inventory | --dry-run | --write"""
import re
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SLUGS = ["avi", "bcf", "binary", "bmp", "csv", "deflate", "docx", "dwg", "dxf", "epw", "gif", "html", "ifc", "jpg", "json", "las", "md", "mp3", "mp4", "obj", "pdf", "ply", "png", "pptx", "semio", "step", "stl", "svg", "tiff", "tsv", "txt", "wav", "xlsx", "xml", "zip"]
TOKEN = re.compile(r'(?<![\w.])"stdio\.(' + "|".join(SLUGS) + r')"')
KIND = [r'\bid: "stdio\.', r'"artifact_kind": "stdio\.', r'"artifactKind": "stdio\.', r'\bartifact_kind: "stdio\.', r'artifactKind: "stdio\.', r'onArtifactKind', r'OnArtifactKind', r'"kindId": "stdio\.', r'kindId: "stdio\.']
SCHEMA = [r'"schema": "stdio\.', r'_SCHEMA: &str = "stdio\.', r'"artifact_schema": "stdio\.', r'"artifactSchema": "stdio\.', r'artifact_schema: "stdio\.', r'artifactSchema: "stdio\.', r'source_format', r'sourceFormat', r'envelope_id', r'ArtifactCodec::of', r'\bschema: "stdio\.', r'DOCUMENT_SCHEMA']
FORMAT = [r'_stdio_kinds', r'StdioKinds', r'stdio_kinds']


def classify(line):
    for label, patterns in (("kind", KIND), ("schema", SCHEMA), ("format", FORMAT)):
        if any(re.search(pattern, line) for pattern in patterns):
            return label
    return "unknown"


files = subprocess.run(["git", "grep", "-lE", r'"stdio\.(' + "|".join(SLUGS) + r')"', "--", ".", ":!.tmp-ticket*", ":!.🧬semio"], cwd=ROOT, capture_output=True, text=True).stdout.splitlines()
rows = defaultdict(list)
for name in files:
    path = ROOT / name
    try:
        text = path.read_text(encoding="utf-8")
    except (UnicodeDecodeError, FileNotFoundError):
        continue
    for number, line in enumerate(text.splitlines(), 1):
        if TOKEN.search(line):
            rows[classify(line)].append((name, number, line.strip()[:220]))
if "--inventory" in sys.argv:
    for label in ("kind", "schema", "format", "unknown"):
        print(f"== {label}: {len(rows[label])} lines in {len({r[0] for r in rows[label]})} files")
    by_dir = Counter()
    for label in ("kind", "unknown"):
        for name, number, line in rows[label]:
            by_dir[(label, "/".join(name.split("/")[:3]))] += 1
    for (label, directory), count in sorted(by_dir.items()):
        print(f"  {label:8} {count:5}  {directory}")
    if "--unknown" in sys.argv:
        for name, number, line in rows["unknown"]:
            print(f"U {name}:{number}: {line}")
    if "--kind" in sys.argv:
        for name, number, line in rows["kind"]:
            print(f"K {name}:{number}: {line}")
