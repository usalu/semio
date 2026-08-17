#!/usr/bin/env python3
"""Map rustc crate paths to Cargo.toml dependency keys (not [package] names)."""
from __future__ import annotations

import os
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
SKIP_DIRS = {"node_modules", "target", ".git", "dist", ".venv", ".nx"}


def rust_ident(name: str) -> str:
    return name.replace("-", "_")


def pkg_name(cargo: Path) -> str | None:
    text = cargo.read_text(encoding="utf-8", errors="ignore")
    m = re.search(r'(?m)^name\s*=\s*"([^"]+)"', text)
    return m.group(1) if m else None


def build_alias_map() -> dict[str, str]:
    aliases: dict[str, str] = {}
    for dp, dns, fns in os.walk(ROOT):
        dns[:] = [d for d in dns if d not in SKIP_DIRS and not (d.startswith(".") and d != ".vscode")]
        if "Cargo.toml" not in fns:
            continue
        rel = Path(dp).relative_to(ROOT).as_posix()
        if ".🦑️repo" in rel:
            continue
        text = (Path(dp) / "Cargo.toml").read_text(encoding="utf-8", errors="ignore")
        for m in re.finditer(
            r'^([a-zA-Z0-9_-]+)\s*=\s*\{([^}]+)\}',
            text,
            re.M,
        ):
            key, body = m.group(1), m.group(2)
            pm = re.search(r'package\s*=\s*"([^"]+)"', body)
            if not pm:
                continue
            pkg = pm.group(1)
            pkg_id = rust_ident(pkg)
            key_id = rust_ident(key)
            if pkg_id != key_id:
                aliases[pkg_id] = key_id
    return aliases


def main() -> None:
    aliases = build_alias_map()
    out = ROOT / ".🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/RENAME-LEGACY-PACKAGE-NAMES-END-TO-END/dep-key-aliases.json"
    import json

    out.write_text(json.dumps(aliases, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    changed = 0
    for dp, dns, fns in os.walk(ROOT):
        dns[:] = [d for d in dns if d not in SKIP_DIRS and not (d.startswith(".") and d != ".vscode")]
        for fn in fns:
            if not fn.endswith(".rs"):
                continue
            path = Path(dp) / fn
            rel = path.relative_to(ROOT).as_posix()
            if ".🦑️repo" in rel:
                continue
            text = path.read_text(encoding="utf-8", errors="ignore")
            orig = text
            for pkg_id, key_id in sorted(aliases.items(), key=lambda x: -len(x[0])):
                text = re.sub(rf"\b{re.escape(pkg_id)}\b", key_id, text)
            if text != orig:
                path.write_text(text, encoding="utf-8")
                changed += 1
    print(f"[fix_rust_dep_crate_ids] {len(aliases)} aliases, {changed} rs files updated")


if __name__ == "__main__":
    main()
