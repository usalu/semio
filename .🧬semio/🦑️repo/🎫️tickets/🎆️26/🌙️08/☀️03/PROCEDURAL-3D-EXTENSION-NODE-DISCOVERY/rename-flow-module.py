#!/usr/bin/env python3
import os
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SKIP = {"node_modules", "target", ".git", "storybook-static"}

REPLACEMENTS = [
    ("🌊️flow/⚡️implementation/🦀️rust/🔨️module/", "🌊️flow/⚡️implementation/🦀️rust/🧩️extension/"),
    ("semio-s-kernel-flow-module-", "semio-s-kernel-flow-extension-"),
    ("flow_module_wasm", "flow_extension_sdk"),
    ("flow_module_", "flow_extension_"),
    ("FlowModuleManifest", "FlowExtensionManifest"),
    ("FlowModuleContributes", "FlowExtensionContributes"),
    ("FlowModuleWidget", "FlowExtensionWidget"),
    ("FlowModuleCommand", "FlowExtensionCommand"),
    ("FlowModuleSetting", "FlowExtensionSetting"),
    ('"flow.module"', '"flow.extension"'),
    ("flow.module", "flow.extension"),
]

EXTS = {".rs", ".toml", ".ts", ".tsx", ".md", ".json", ".mts", ".mjs"}


def should_skip(p: Path) -> bool:
    parts = set(p.parts)
    return bool(parts & SKIP) or ".🦑️repo" in str(p) and "PROCEDURAL-3D-EXTENSION" in str(p) and p.name == "rename-flow-module.py"


def main():
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in SKIP and not d.startswith(".cursor")]
        for name in filenames:
            p = Path(dirpath) / name
            if p.suffix not in EXTS:
                continue
            if should_skip(p):
                continue
            try:
                text = p.read_text(encoding="utf-8")
            except Exception:
                continue
            orig = text
            for a, b in REPLACEMENTS:
                text = text.replace(a, b)
            if text != orig:
                p.write_text(text, encoding="utf-8")
                print("updated", p.relative_to(ROOT))


if __name__ == "__main__":
    main()
