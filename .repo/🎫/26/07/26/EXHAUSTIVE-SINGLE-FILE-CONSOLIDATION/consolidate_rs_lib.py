#!/usr/bin/env python3
"""Inline #[path = "src/*.rs"] modules into lib.rs as region-wrapped inline mod blocks."""
from __future__ import annotations

import re
import sys
from pathlib import Path


PATH_RE = re.compile(
    r"^(?P<indent>\s*)#\[path\s*=\s*\"(?P<path>[^\"]+)\"\]\s*\n"
    r"(?P<indent2>\s*)(?P<vis>pub(?:\s*\(crate\))?\s+)?mod\s+(?P<name>\w+)\s*;\s*$",
    re.MULTILINE,
)


def region_name(mod_name: str) -> str:
    return mod_name.replace("_", "").title() if mod_name.islower() else mod_name


def consolidate(crate_dir: Path) -> list[str]:
    lib = crate_dir / "lib.rs"
    if not lib.exists():
        raise SystemExit(f"no lib.rs in {crate_dir}")

    text = lib.read_text()
    deleted: list[str] = []
    replacements: list[tuple[str, str]] = []

    for m in PATH_RE.finditer(text):
        rel = m.group("path")
        src = crate_dir / rel
        if not src.exists():
            raise SystemExit(f"missing {src}")
        body = src.read_text()
        if not body.endswith("\n"):
            body += "\n"
        vis = m.group("vis") or ""
        name = m.group("name")
        rn = region_name(name)
        block = (
            f"// #region 🔖{rn}\n"
            f"{vis}mod {name} {{\n"
            f"{body}"
            f"}}\n"
            f"// #endregion 🔖{rn}\n"
        )
        replacements.append((m.group(0), block))
        deleted.append(str(src))

    for old, new in replacements:
        text = text.replace(old, new, 1)

    if "// #region 🔖Modules" in text:
        text = text.replace("// #region 🔖Modules\n", "", 1)
        text = text.replace("// #endregion 🔖Modules\n", "", 1)

    if "// #region 🔖NativeModules" in text:
        text = text.replace("// #region 🔖NativeModules\n", "", 1)
        text = text.replace("// #endregion 🔖NativeModules\n", "", 1)

    lib.write_text(text)

    for p in deleted:
        Path(p).unlink()

    src_dir = crate_dir / "src"
    if src_dir.exists() and not any(src_dir.iterdir()):
        src_dir.rmdir()

    return deleted


if __name__ == "__main__":
    for arg in sys.argv[1:]:
        removed = consolidate(Path(arg).resolve())
        print(f"consolidated {arg}: removed {len(removed)} files")
