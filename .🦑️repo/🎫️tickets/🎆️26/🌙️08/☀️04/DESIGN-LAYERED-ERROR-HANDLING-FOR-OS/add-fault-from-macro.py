#!/usr/bin/env python3
from pathlib import Path
import re

ROOT = Path("/Users/ueli/Documents/semio")
SCAN = [ROOT / "🧰️framework", ROOT / "✏️s"]
SKIP = {"dsl_core", "semio-framework-os-kernel-dsl-core"}

ORIGIN_MODULE = "dsl_core::FaultOrigin::Module"
ORIGIN_OS = "dsl_core::FaultOrigin::Os"
ORIGIN_PLUGIN = "dsl_core::FaultOrigin::Plugin"
ORIGIN_APP = "dsl_core::FaultOrigin::App"

ENUM_RE = re.compile(r"pub enum (\w+Error)\b")


def prefix_for(path: Path, name: str) -> tuple[str, str]:
    parts = path.parts
    if "✏️s" in parts:
        if "🔌️plugin" in parts:
            i = parts.index("🔌️plugin")
            slug = parts[i + 1] if i + 1 < len(parts) else "plugin"
            return ORIGIN_PLUGIN, f"plugin.{slug}.{name.removesuffix('Error').lower()}"
        return ORIGIN_APP, f"app.{name.removesuffix('Error').lower()}"
    if "💻️os" in parts and "🔨️module" in parts:
        i = parts.index("🔨️module")
        slug = parts[i + 1] if i + 1 < len(parts) else "os"
        return ORIGIN_MODULE, f"module.{slug}.{name.removesuffix('Error').lower()}"
    if "🔨️module" in parts:
        i = parts.index("🔨️module")
        slug = parts[i + 1] if i + 1 < len(parts) else "framework"
        return ORIGIN_MODULE, f"module.{slug}.{name.removesuffix('Error').lower()}"
    return ORIGIN_MODULE, f"module.{name.removesuffix('Error').lower()}"


def main() -> None:
    touched = 0
    for base in SCAN:
        for path in base.rglob("📦️lib.rs"):
            if any(s in path.read_text(errors="ignore") for s in ("fault_from_thiserror!",)):
                continue
            text = path.read_text(encoding="utf-8")
            if "thiserror" not in text and "Error" not in text:
                continue
            m = ENUM_RE.search(text)
            if not m:
                continue
            enum = m.group(1)
            if enum in {"ClipboardError", "MediaError"}:
                continue
            origin, code = prefix_for(path, enum)
            line = f"\ndsl_core::fault_from_thiserror!({enum}, {origin}, \"{code}\");\n"
            if line.strip() in text:
                continue
            path.write_text(text.rstrip() + line, encoding="utf-8")
            touched += 1
            print(path, enum, code)
    print("touched", touched)


if __name__ == "__main__":
    main()
