#!/usr/bin/env python3
import re
import subprocess
from pathlib import Path

REGISTER_BODY_RE = re.compile(
    r"(pub )?fn register_artifact_schema\(\) \{\n(?:.*?\n)*?\s*\.register\(([^)]+)\);\n\}",
    re.MULTILINE | re.DOTALL,
)
STATIC_RE = re.compile(
    r"\nstatic [A-Z_0-9]*SCHEMA_REGISTRY: std::sync::OnceLock<std::sync::Mutex<schema::ArtifactSchemaRegistry>> =\s*\n\s*std::sync::OnceLock::new\(\);\n",
    re.MULTILINE,
)
STATIC_INLINE_RE = re.compile(
    r"\n\s*static [A-Z_0-9]*SCHEMA_REGISTRY: std::sync::OnceLock<std::sync::Mutex<schema::ArtifactSchemaRegistry>> = std::sync::OnceLock::new\(\);\n",
    re.MULTILINE,
)
REGISTERED_RE = re.compile(
    r"[A-Z_]*SCHEMA_REGISTRY\s*\n\s*\.get\(\)\s*\n\s*\.map\(\|registry\| registry\.lock\(\)\.expect\([^)]+\)\.get\(\"([^\"]+)\"\)\.is_some\(\)\)\s*\n\s*\.unwrap_or\(false\)",
    re.MULTILINE,
)

files = subprocess.check_output(
    [
        "rg",
        "-l",
        "SCHEMA_REGISTRY",
        "/Users/ueli/Documents/semio/✏️s/🔌️plugins",
        "--glob",
        "**/⚙️engine/🦀️component.rs",
    ],
    text=True,
).strip().splitlines()

n = 0
for fp in files:
    path = Path(fp)
    text = path.read_text(encoding="utf-8")
    m = REGISTER_BODY_RE.search(text)
    if not m:
        print("skip", path)
        continue
    pub = m.group(1) or ""
    expr = m.group(2).strip()
    new_fn = (
        f"{pub}fn register_artifact_schema() {{\n"
        f"    schema::register_artifact_schema_descriptor({expr});\n"
        f"}}"
    )
    text = REGISTER_BODY_RE.sub(new_fn, text, count=1)
    text = STATIC_RE.sub("\n", text)
    text = STATIC_INLINE_RE.sub("\n", text)
    text = REGISTERED_RE.sub(r'schema::artifact_schema_descriptor_registered("\1")', text)
    path.write_text(text, encoding="utf-8")
    n += 1
    print(path)
print("patched", n)
