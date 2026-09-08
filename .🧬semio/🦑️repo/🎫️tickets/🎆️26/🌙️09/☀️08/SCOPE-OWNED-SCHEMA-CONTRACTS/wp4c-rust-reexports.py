#!/usr/bin/env python3
"""🦀️ Turns a schema module's private `use` of its own exports into per-name `pub use` re-exports.

Execution contract §A: a Rust export is `pub struct|enum|type <Export>` **or** a per-name
`pub use <path>::<Export>;` — "a private `use` declares nothing", and a grouped or glob re-export
declares nothing either. Every artifact module in this partition imports the entity types it projects
from elsewhere in its crate (`use crate::{…}`), so the repair is mechanical and loses nothing: the
name stays in scope exactly as before, and it is now also part of the module's declared surface.

A name the module does not import at all is REPORTED, never invented: fabricating a Rust type that
the crate does not have would make the module lie about what it carries.

Usage: python3 wp4c-rust-reexports.py report|apply [--under ✏️s/🔌️plugins/✒️writer]
"""
from __future__ import annotations
import json, os, re, sys, collections

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
JSON_LEAF, RUST_LEAF = "🔣️.json", "🦀️.rs"
FACET_DIRS = ("🔺️diff", "📸️snapshot", "💡️inferences", "🧬️mutations", "📝️text", "💾️binary")
EXPORT_RE = re.compile(r"^[A-Z][A-Za-z0-9]*$")
USE_RE = re.compile(r"^use\s+([A-Za-z_][A-Za-z0-9_:]*)::\{([^}]*)\};\s*$", re.M)
SINGLE_USE_RE = re.compile(r"^use\s+([A-Za-z_][A-Za-z0-9_:]*)::([A-Z][A-Za-z0-9_]*);\s*$", re.M)
REGION = "//#region 🔁️Re-exports"


def declares(source: str, name: str) -> bool:
    return re.search(rf"^\s*pub\s+(?:struct|enum|type)\s+{re.escape(name)}\b", source, re.M) is not None or re.search(rf"^\s*pub use\s+[A-Za-z_][A-Za-z0-9_:]*::{re.escape(name)};", source, re.M) is not None


def exports_of(document: dict) -> dict[str, dict]:
    found: dict[str, dict] = {}
    root = document.get("title")
    root_object = document.get("type") == "object" or ("type" not in document and isinstance(document.get("properties"), dict))
    if isinstance(root, str) and root_object and EXPORT_RE.match(root):
        found[root] = document
    for name, definition in (document.get("$defs") or {}).items():
        if EXPORT_RE.match(name) and name not in found:
            found[name] = definition if isinstance(definition, dict) else {}
    return found


def documents(module: str) -> list[tuple[str, str]]:
    """(json document path, facet directory relative to the module)"""
    found = [(f"{module}/{JSON_LEAF}", "")]
    for base, dirs, files in os.walk(os.path.join(REPO, module)):
        dirs[:] = [name for name in dirs if name in FACET_DIRS]
        relative = os.path.relpath(base, os.path.join(REPO, module))
        if relative != "." and JSON_LEAF in files:
            found.append((f"{module}/{relative}/{JSON_LEAF}", relative))
    return found


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "report"
    under = sys.argv[sys.argv.index("--under") + 1] if "--under" in sys.argv else "✏️s"
    reexported = collections.Counter()
    absent: list[str] = []
    touched: set[str] = set()
    for base, dirs, _ in os.walk(os.path.join(REPO, under)):
        dirs[:] = [name for name in dirs if name not in ("node_modules", ".venv")]
        if os.path.basename(base) != "🧬️schema":
            continue
        module = os.path.relpath(base, REPO)
        if "/🧬️mutations/" in f"{module}/" or not os.path.exists(os.path.join(base, JSON_LEAF)):
            continue
        for path, facet in documents(module):
            rust = os.path.join(REPO, module, facet, RUST_LEAF) if facet else os.path.join(REPO, module, RUST_LEAF)
            if not os.path.exists(rust) or not os.path.exists(os.path.join(REPO, module, RUST_LEAF)):
                continue
            try:
                with open(os.path.join(REPO, path), encoding="utf-8") as handle:
                    document = json.load(handle)
            except Exception:  # noqa: BLE001
                continue
            with open(rust, encoding="utf-8") as handle:
                source = handle.read()
            relative = os.path.relpath(rust, REPO)
            imported: dict[str, str] = {}
            for match in USE_RE.finditer(source):
                for member in (entry.strip() for entry in match.group(2).split(",")):
                    if EXPORT_RE.match(member):
                        imported[member] = match.group(1)
            for match in SINGLE_USE_RE.finditer(source):
                imported[match.group(2)] = match.group(1)
            added: list[str] = []
            for name, definition in exports_of(document).items():
                restricted = definition.get("x-semio-formats")
                if isinstance(restricted, list) and "🦀️rust" not in restricted:
                    continue
                if declares(source, name):
                    continue
                if name not in imported:
                    absent.append(f"{name} {relative}")
                    continue
                added.append(f"pub use {imported[name]}::{name};")
                reexported["re-exported"] += 1
            if not added or mode != "apply":
                continue
            # 🧷️The private import has to go: `use p::X;` beside `pub use p::X;` is a duplicate binding.
            for line in added:
                name = line.rsplit("::", 1)[1].rstrip(";")
                source = SINGLE_USE_RE.sub(lambda match, target=name: "" if match.group(2) == target else match.group(0), source)
                def prune(match, target=name):
                    members = [entry.strip() for entry in match.group(2).split(",") if entry.strip() and entry.strip() != target]
                    return f"use {match.group(1)}::{{{', '.join(members)}}};" if members else ""
                source = USE_RE.sub(prune, source)
            block = "\n".join([REGION, "/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.", *added, "//#endregion 🔁️Re-exports"])
            with open(rust, "w", encoding="utf-8") as handle:
                handle.write(f"{source.rstrip()}\n\n{block}\n")
            touched.add(relative)
    print(json.dumps({"mode": mode, "under": under, **reexported, "filesTouched": len(touched), "absent": len(absent)}, ensure_ascii=False))
    for row in absent[:40]:
        print("ABSENT", row)
    if len(absent) > 40:
        print(f"… and {len(absent) - 40} more absent")


if __name__ == "__main__":
    main()
