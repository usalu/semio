#!/usr/bin/env python3
"""Patch per-engine schema registries to use the global catalog."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")

STATIC_RE = re.compile(
    r"\nstatic [A-Z_0-9]*SCHEMA_REGISTRY: std::sync::OnceLock<std::sync::Mutex<schema::ArtifactSchemaRegistry>> =\s*\n\s*std::sync::OnceLock::new\(\);\n",
    re.MULTILINE,
)
STATIC_INLINE_RE = re.compile(
    r"\n\s*static [A-Z_0-9]*SCHEMA_REGISTRY: std::sync::OnceLock<std::sync::Mutex<schema::ArtifactSchemaRegistry>> = std::sync::OnceLock::new\(\);\n",
    re.MULTILINE,
)
REGISTER_BODY_RE = re.compile(
    r"(pub )?fn register_artifact_schema\(\) \{\n(?:.*?\n)*?\s*\.register\(([^)]+)\);\n\}",
    re.MULTILINE | re.DOTALL,
)


def patch_file(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    if "register_artifact_schema_descriptor" in text and "SCHEMA_REGISTRY" not in text:
        return False
    if "register_artifact_schema" not in text:
        return False
    m = REGISTER_BODY_RE.search(text)
    if not m:
        print(f"skip (no match): {path}")
        return False
    pub = m.group(1) or ""
    descriptor_expr = m.group(2).strip()
    new_fn = (
        f"{pub}fn register_artifact_schema() {{\n"
        f"    schema::register_artifact_schema_descriptor({descriptor_expr});\n"
        f"}}"
    )
    text = REGISTER_BODY_RE.sub(new_fn, text, count=1)
    text = STATIC_RE.sub("\n", text)
    text = STATIC_INLINE_RE.sub("\n", text)
    text = text.replace(
        "schema::ArtifactSchemaRegistry::new()",
        "",
    )
    # artifact_schema_registered helpers
    text = re.sub(
        r"LOWPOLY_SCHEMA_REGISTRY\s*\n\s*\.get\(\)\s*\n\s*\.map\(\|registry\| registry\.lock\(\)\.expect\([^)]+\)\.get\(\"([^\"]+)\"\)\.is_some\(\)\)\s*\n\s*\.unwrap_or\(false\)",
        r'schema::artifact_schema_descriptor_registered("\1")',
        text,
    )
    text = re.sub(
        r"[A-Z_]*SCHEMA_REGISTRY\s*\n\s*\.get\(\)\s*\n\s*\.map\(\|registry\| registry\.lock\(\)\.expect\([^)]+\)\.get\(\"([^\"]+)\"\)\.is_some\(\)\)\s*\n\s*\.unwrap_or\(false\)",
        r'schema::artifact_schema_descriptor_registered("\1")',
        text,
    )
    path.write_text(text, encoding="utf-8")
    return True


def main() -> None:
    n = 0
    for path in ROOT.rglob("⚙️engine/🦀️component.rs"):
        if patch_file(path):
            n += 1
            print(path)
    print(f"patched {n} files")


if __name__ == "__main__":
    main()
