#!/usr/bin/env python3
import re
from pathlib import Path

LIST = Path(__file__).with_name("🧪wave7-engine-paths.txt")
paths = [Path(line.strip()) for line in LIST.read_text().splitlines() if line.strip()]

REGISTER_FN = re.compile(
    r"((?:pub )?fn register_artifact_schema\(\) \{)(.*?)(^\})",
    re.MULTILINE | re.DOTALL,
)
REGISTER_EXPR = re.compile(r"\.register\(([^)]+)\)")
STATIC = re.compile(
    r"\nstatic [A-Z_0-9]*SCHEMA_REGISTRY:[^\n]+\n(?:\s*std::sync::OnceLock::new\(\);\n|\s*OnceLock::new\(\);\n)?",
    re.MULTILINE,
)
STATIC2 = re.compile(
    r"\nuse std::sync::\{Mutex, OnceLock\};\n\nstatic SCHEMA_REGISTRY: OnceLock<Mutex<schema::ArtifactSchemaRegistry>> = OnceLock::new\(\);\n",
    re.MULTILINE,
)

for path in paths:
    text = path.read_text(encoding="utf-8")
    m = REGISTER_FN.search(text)
    if not m:
        continue
    body = m.group(2)
    em = REGISTER_EXPR.search(body)
    if not em:
        print("no expr", path)
        continue
    expr = em.group(1).strip()
    pub = "pub " if m.group(1).startswith("pub") else ""
    replacement = f"{pub}fn register_artifact_schema() {{\n    schema::register_artifact_schema_descriptor({expr});\n}}"
    text = REGISTER_FN.sub(replacement, text, count=1)
    text = STATIC.sub("\n", text)
    text = STATIC2.sub("\n", text)
    path.write_text(text, encoding="utf-8")
    print("ok", path.name)
