#!/usr/bin/env python3
from __future__ import annotations
import json, re, subprocess
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
TICKET = next((ROOT / ".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))
BATCH = json.loads((TICKET / "generators" / "w6-heavy.json").read_text(encoding="utf-8"))
OWNER = json.loads((TICKET / "🧪owner-table.json").read_text(encoding="utf-8"))

def owner_row(plugin: str, artifact: str) -> dict:
    for row in OWNER["owners"]:
        if row["plugin"] == plugin and row["artifact"] == artifact:
            return row
    raise KeyError((plugin, artifact))

def mut_mod(dirname: str) -> str:
    slug = "".join(c if c.isascii() and (c.isalnum() or c == "-") else "" for c in dirname)
    return slug.replace("-", "_") or "mod"

def nest_engine_children(glue_text: str, artifact: str, rust_mod: str, art: Path) -> str:
    eng = art / "⚙️engine"
    if not eng.exists():
        return glue_text
    a = f"../../🗿️artifacts/{artifact}"
    for child in sorted(eng.iterdir()):
        if not child.is_dir() or not (child / "🦀️component.rs").exists():
            continue
        grandchildren = [g for g in sorted(child.iterdir()) if g.is_dir() and (g / "🦀️component.rs").exists()]
        if not grandchildren:
            continue
        child_mod = mut_mod(child.name)
        flat = f'            #[path = "{a}/⚙️engine/{child.name}/🦀️component.rs"]\n            pub mod {child_mod};'
        if flat not in glue_text:
            continue
        nested = [
            '            #[path = "."]',
            f"            pub mod {child_mod} {{",
            f'                #[path = "{a}/⚙️engine/{child.name}/🦀️component.rs"]',
            "                mod component;",
            "                pub use component::*;",
        ]
        for g in grandchildren:
            gmod = mut_mod(g.name)
            nested += [
                f'                #[path = "{a}/⚙️engine/{child.name}/{g.name}/🦀️component.rs"]',
                f"                pub mod {gmod};",
            ]
        nested.append("            }")
        glue_text = glue_text.replace(flat, "\n".join(nested))
        print("nested", rust_mod, child_mod, [mut_mod(g.name) for g in grandchildren])
    return glue_text

def fix_trinity_dups() -> None:
    for entry in BATCH:
        if "trinity" not in entry["plugin"]:
            continue
        art = ROOT / owner_row(entry["plugin"], entry["artifact"])["path"]
        p = art / "🦀️component.rs"
        t = p.read_text(encoding="utf-8")
        if "snapshot::schema::" not in t and "super::snapshot::schema::" not in t:
            continue
        lines = []
        for l in t.splitlines(True):
            if l.strip().startswith("pub use") and "schema::snapshot::" in l:
                print("drop dup", entry["rust_mod"], l.strip())
                continue
            lines.append(l)
        p.write_text("".join(lines), encoding="utf-8")

def fix_en1990_qk() -> None:
    for entry in BATCH:
        if entry["rust_mod"] != "en1990":
            continue
        art = ROOT / owner_row(entry["plugin"], entry["artifact"])["path"]
        p = art / "🦀️component.rs"
        t = p.read_text(encoding="utf-8")
        export = "pub use crate::artifacts::en1990::schema::snapshot::En1990QkEntry;\n"
        if "schema::snapshot::En1990QkEntry" in t:
            print("en1990 qk already ok")
            return
        lines = t.splitlines(True)
        i = 0
        while i < len(lines) and lines[i].startswith("//!"):
            i += 1
        while i < len(lines) and lines[i].strip() == "":
            i += 1
        lines = lines[:i] + [export] + lines[i:]
        p.write_text("".join(lines), encoding="utf-8")
        print("added En1990QkEntry export")

def main() -> int:
    fix_trinity_dups()
    fix_en1990_qk()
    for plugin in sorted({e["plugin"] for e in BATCH}):
        entries = [e for e in BATCH if e["plugin"] == plugin]
        glue = ROOT / "✏️s/🔌️plugins" / plugin / "📦️packages/🦀️rust/📦️glue.rs"
        text = glue.read_text(encoding="utf-8")
        orig = text
        for entry in entries:
            art = ROOT / owner_row(entry["plugin"], entry["artifact"])["path"]
            text = nest_engine_children(text, entry["artifact"], entry["rust_mod"], art)
        if text != orig:
            glue.write_text(text, encoding="utf-8")
            print("wrote glue", plugin)
    crates = []
    for e in BATCH:
        if e["crate"] not in crates:
            crates.append(e["crate"])
    checks = {}
    for crate in crates:
        print("cargo", crate, flush=True)
        r = subprocess.run(["cargo", "check", "-p", crate], cwd=ROOT, capture_output=True, text=True)
        tail = (r.stdout or "") + (r.stderr or "")
        ok = r.returncode == 0
        checks[crate] = ok
        (TICKET / f"🧪w6-heavy-{crate}.log").write_text(tail, encoding="utf-8")
        print(" ->", "OK" if ok else "FAIL", flush=True)
    (TICKET / "generators" / "w6-heavy-cargo.json").write_text(json.dumps(checks, indent=2), encoding="utf-8")
    print(json.dumps(checks, indent=2))
    return 0 if all(checks.values()) else 1

if __name__ == "__main__":
    raise SystemExit(main())
