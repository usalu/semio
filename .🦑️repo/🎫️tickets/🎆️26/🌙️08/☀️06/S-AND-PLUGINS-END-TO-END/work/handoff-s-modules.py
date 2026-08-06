#!/usr/bin/env python3
"""Temporary ticket work: repoint dependents to consolidated s-modules and update root workspace."""
from __future__ import annotations

from pathlib import Path
import os
import re

ROOT = Path(__file__).resolve().parents[5]  # may be wrong with emoji depth — fix below
# Locate repo root by walking up until Cargo.toml workspace
here = Path(__file__).resolve()
ROOT = here
while ROOT != ROOT.parent and not (ROOT / "Cargo.toml").exists():
    ROOT = ROOT.parent
assert (ROOT / "Cargo.toml").exists(), ROOT
os.chdir(ROOT)

def find_pkg(name: str) -> Path:
    for p in (ROOT / "✏️s" / "🔨️modules").rglob("Cargo.toml"):
        try:
            t = p.read_text(encoding="utf-8")
        except Exception:
            continue
        if f'name = "{name}"' in t:
            return p.parent.resolve()
    raise SystemExit(f"missing package {name}")

s3d_dir = find_pkg("semio-s-3d")
smind_dir = find_pkg("semio-s-mindmap")
slang_dir = find_pkg("semio-s-language-bundle")
print("s3d", s3d_dir.relative_to(ROOT))
print("mind", smind_dir.relative_to(ROOT))
print("lang", slang_dir.relative_to(ROOT))

OLD_3D_KEYS = [
    "kernel_3d_brepkit",
    "kernel_3d_engine",
    "kernel_3d_mesh",
    "kernel_3d_scene",
    "kernel_3d_spatial",
]
OLD_3D_PKGS = [
    "semio-framework-os-kernel-3d-brep",
    "semio-framework-os-kernel-3d-brep-engine",
    "semio-framework-os-kernel-3d-mesh",
    "semio-framework-os-kernel-3d-scene",
    "semio-framework-os-kernel-3d-spatial",
]
OLD_MM_KEYS = ["reasoning_mindmap"]
OLD_MM_PKGS = ["semio-s-kernel-reasoning-mindmap", "semio-framework-os-kernel-reasoning-mindmap"]

def is_old_impl_crate(p: Path) -> bool:
    s = str(p).replace("\\", "/")
    if "/📦️packages/" in s:
        return False
    return any(
        m in s
        for m in (
            "/⚡️implementations/",
        )
    ) and any(
        m in s
        for m in (
            "/📐️brep/",
            "/🥽️mesh/",
            "/🎬️scene/",
            "/🗺️spatial/",
            "/💭️mindmap/",
        )
    )

def rel_dep(from_cargo: Path, target_dir: Path) -> str:
    return os.path.relpath(target_dir, start=from_cargo.parent).replace("\\", "/")

dep_line_re = re.compile(
    r'^(?P<indent>\s*)(?P<key>[A-Za-z0-9_-]+)\s*=\s*\{[^}]*package\s*=\s*"(?P<pkg>[^"]+)"[^}]*\}\s*$'
    r'|^(?P<indent2>\s*)(?P<key2>[A-Za-z0-9_-]+)\s*=\s*\{[^}]*path\s*=\s*"[^"]*"(?P<rest>[^}]*)\}\s*$',
    re.M,
)

# Simpler: drop any dependency line whose key or package is old; insert new dep once.
line_dep_re = re.compile(r'^(\s*)([A-Za-z0-9_-]+)\s*=\s*\{(.*)\}\s*$')

def rewrite_cargo(path: Path) -> bool:
    if path.name != "Cargo.toml":
        return False
    if is_old_impl_crate(path):
        return False
    if path.resolve() in {s3d_dir / "Cargo.toml", smind_dir / "Cargo.toml", slang_dir / "Cargo.toml"}:
        return False
    text = path.read_text(encoding="utf-8")
    if not any(k in text for k in OLD_3D_KEYS + OLD_3D_PKGS + OLD_MM_KEYS + OLD_MM_PKGS):
        return False

    lines = text.splitlines(True)
    out = []
    need_3d = False
    need_mm = False
    removed = 0
    insert_at = None
    for i, line in enumerate(lines):
        m = line_dep_re.match(line.rstrip("\n"))
        if m:
            key = m.group(2)
            body = m.group(3)
            pkg_m = re.search(r'package\s*=\s*"([^"]+)"', body)
            pkg = pkg_m.group(1) if pkg_m else None
            is_3d = key in OLD_3D_KEYS or (pkg in OLD_3D_PKGS if pkg else False) or any(p in body for p in OLD_3D_PKGS)
            is_mm = key in OLD_MM_KEYS or (pkg in OLD_MM_PKGS if pkg else False) or any(p in body for p in OLD_MM_PKGS)
            if is_3d or is_mm:
                if is_3d:
                    need_3d = True
                if is_mm:
                    need_mm = True
                removed += 1
                if insert_at is None:
                    insert_at = len(out)
                continue
        out.append(line)

    # Unused-only deletes: playbook procedural + flow core — still need_3d True from scan;
    # orchestrator says DELETE unused. Detect by path markers.
    s = str(path).replace("\\", "/")
    unused_3d = ("playbook" in s and "procedural" in s and "extensions" in s) or (
        "/🌊️flow/" in s and "/🟀️core/" in s
    )
    if unused_3d:
        need_3d = False

    if need_3d or need_mm:
        indent = "    "
        # find [dependencies] indent from nearby
        for line in out:
            m = line_dep_re.match(line.rstrip("\n"))
            if m:
                indent = m.group(1)
                break
        additions = []
        if need_3d and "semio-s-3d" not in "".join(out):
            additions.append(f'{indent}semio-s-3d = {{ path = "{rel_dep(path, s3d_dir)}" }}\n')
        if need_mm and "semio-s-mindmap" not in "".join(out):
            additions.append(f'{indent}semio-s-mindmap = {{ path = "{rel_dep(path, smind_dir)}" }}\n')
        idx = insert_at if insert_at is not None else len(out)
        out[idx:idx] = additions

    new = "".join(out)
    if new != text:
        path.write_text(new, encoding="utf-8")
        print(f"CARGO {path.relative_to(ROOT)} removed={removed} need_3d={need_3d} need_mm={need_mm}")
        return True
    return False

# Rust use rewrites
RS_REPLACEMENTS = [
    ("kernel_3d_brepkit::", "semio_s_3d::brep::kernel::"),
    ("kernel_3d_engine::", "semio_s_3d::brep::engine::"),
    ("kernel_3d_mesh::", "semio_s_3d::"),
    ("kernel_3d_scene::", "semio_s_3d::"),
    ("kernel_3d_spatial::", "semio_s_3d::"),
    ("reasoning_mindmap::", "semio_s_mindmap::"),
    ("use reasoning_mindmap as", "use semio_s_mindmap as"),
    ("pub use reasoning_mindmap as", "pub use semio_s_mindmap as"),
]

def rewrite_rs(path: Path) -> bool:
    if is_old_impl_crate(path):
        # still rewrite comments? skip entire old impl trees for code that is dead
        # but mesh component under taxonomy is NEW source — not under ⚡️implementations
        pass
    s = str(path).replace("\\", "/")
    if "/⚡️implementations/" in s and any(x in s for x in ("/📐️brep/", "/🥽️mesh/", "/🎬️scene/", "/🗺️spatial/", "/💭️mindmap/")):
        return False
    if "/.🦑️repo/" in s or "/target/" in s:
        return False
    try:
        text = path.read_text(encoding="utf-8")
    except Exception:
        return False
    if not any(k in text for k in ("kernel_3d_", "reasoning_mindmap")):
        return False
    new = text
    for a, b in RS_REPLACEMENTS:
        new = new.replace(a, b)
    # bare extern crate style unlikely; also `use kernel_3d_mesh` without ::
    new = re.sub(r"\buse kernel_3d_mesh\b", "use semio_s_3d", new)
    new = re.sub(r"\buse kernel_3d_scene\b", "use semio_s_3d", new)
    new = re.sub(r"\buse kernel_3d_spatial\b", "use semio_s_3d", new)
    new = re.sub(r"\buse kernel_3d_engine\b", "use semio_s_3d::brep::engine", new)
    new = re.sub(r"\buse kernel_3d_brepkit\b", "use semio_s_3d::brep::kernel", new)
    new = re.sub(r"\buse reasoning_mindmap\b", "use semio_s_mindmap", new)
    if new != text:
        path.write_text(new, encoding="utf-8")
        print(f"RS {path.relative_to(ROOT)}")
        return True
    return False

cargo_n = 0
for p in ROOT.rglob("Cargo.toml"):
    if "/.🦑️repo/" in str(p) or "/target/" in str(p) or "node_modules" in str(p):
        continue
    if p.resolve() == (ROOT / "Cargo.toml").resolve():
        continue
    if rewrite_cargo(p):
        cargo_n += 1

rs_n = 0
for p in ROOT.rglob("*.rs"):
    if "/.🦑️repo/" in str(p) or "/target/" in str(p) or "node_modules" in str(p):
        continue
    if rewrite_rs(p):
        rs_n += 1

# Strip mindmap overlay
mind_cargo = smind_dir / "Cargo.toml"
mt = mind_cargo.read_text(encoding="utf-8")
if "TEMPORARY VERIFICATION OVERLAY" in mt or mt.lstrip().startswith("cargo-features"):
    # Remove everything before [package] that is overlay, keep from first [package]
    idx = mt.find("[package]")
    if idx > 0:
        # drop cargo-features and overlay comments; also drop a duplicated [workspace] table if present
        body = mt[idx:]
        # remove [workspace] ... until next [table] that isn't workspace children? entire workspace section
        body = re.sub(r"\n\[workspace\][\s\S]*?(?=\n\[|\Z)", "\n", body)
        mind_cargo.write_text(body if body.startswith("[package]") else "[package]" + body.split("[package]",1)[-1], encoding="utf-8")
        # ensure no cargo-features left
        mt2 = mind_cargo.read_text(encoding="utf-8")
        if "cargo-features" in mt2 or "[workspace]" in mt2:
            # more aggressive: keep only from [package] and strip workspace tables
            parts = re.split(r"\n(?=\[)", mt2)
            kept = []
            for part in parts:
                title = part.split("\n",1)[0]
                if title.startswith("[workspace"):
                    continue
                if part.startswith("cargo-features"):
                    continue
                kept.append(part if part.startswith("[") else part)
            mind_cargo.write_text("\n".join(kept).lstrip() + ("\n" if not "\n".join(kept).endswith("\n") else ""), encoding="utf-8")
        print("STRIPPED mindmap overlay")

# Root Cargo.toml registrar handoff
root_cargo = ROOT / "Cargo.toml"
rt = root_cargo.read_text(encoding="utf-8")
old_members = [
    '    "✏️s/🔨️modules/✨️3d/🎬️scene/⚡️implementations/🦀️rust",\n',
    '    "✏️s/🔨️modules/✨️3d/🥽️mesh/⚡️implementations/🦀️rust",\n',
    '    "✏️s/🔨️modules/💭️mindmap/⚡️implementations/🦀️rust",\n',
    '    "✏️s/🔨️modules/✨️3d/📐️brep/⚡️implementations/🦀️rust",\n',
    '    "✏️s/🔨️modules/✨️3d/📐️brep/⚙️engine/⚡️implementations/🦀️rust",\n',
    '    "✏️s/🔨️modules/📜️imperative/⚡️implementations/🦀️rust",\n',
    '    "✏️s/🔨️modules/✨️3d/🗺️spatial/⚡️implementations/🦀️rust",\n',
]
# Actual emoji in tree is 🧊️3d not ✨️3d for some — discover from file
member_lines_to_remove = []
for line in rt.splitlines(True):
    if "⚡️implementations/🦀️rust" in line and any(
        x in line for x in ("/🎬️scene/", "/🥽️mesh/", "/💭️mindmap/", "/📐️brep/", "/🗺️spatial/", "/📜️imperative/")
    ):
        if "✏️s/🔨️modules/" in line:
            member_lines_to_remove.append(line)

new_members = [
    f'    "{s3d_dir.relative_to(ROOT).as_posix()}",\n',
    f'    "{smind_dir.relative_to(ROOT).as_posix()}",\n',
    f'    "{slang_dir.relative_to(ROOT).as_posix()}",\n',
]
# imperative package if exists
try:
    simp = find_pkg("semio-s-imperative")
    new_members.append(f'    "{simp.relative_to(ROOT).as_posix()}",\n')
except SystemExit:
    pass
try:
    s2d = find_pkg("semio-s-2d")
    # may already be member
    m = f'    "{s2d.relative_to(ROOT).as_posix()}",\n'
    if m not in rt and m not in new_members:
        new_members.insert(0, m)
except SystemExit:
    pass

rt2 = rt
for line in member_lines_to_remove:
    rt2 = rt2.replace(line, "")
    print("REMOVE MEMBER", line.strip())

# Insert new members near other ✏️s/🔨️modules entries
if "semio-s-3d" not in rt2 and str(s3d_dir.relative_to(ROOT)) not in rt2:
    # find a modules member line to insert after
    lines = rt2.splitlines(True)
    out = []
    inserted = False
    for i, line in enumerate(lines):
        out.append(line)
        if not inserted and '✏️s/🔨️modules/' in line and "📦️packages/🦀️rust" in line:
            # insert after first packages member cluster end — wait until blank or non-module
            pass
    # simpler: after last removed area, find "✏️s/🔨️modules" first occurrence and insert before old ones
    out = []
    inserted = False
    for line in lines:
        if not inserted and "✏️s/🔨️modules/" in line and line.strip().startswith('"'):
            for nm in new_members:
                if nm not in rt2 and nm not in out:
                    out.append(nm)
                    print("ADD MEMBER", nm.strip())
            inserted = True
        out.append(line)
    if not inserted:
        # fallback: before members close
        out = []
        for line in lines:
            if not inserted and line.strip() == "]" and "members" in "".join(out[-30:]):
                for nm in new_members:
                    out.append(nm)
                    print("ADD MEMBER", nm.strip())
                inserted = True
            out.append(line)
    rt2 = "".join(out)

# workspace.dependencies cleanup / add
for pkg in OLD_3D_PKGS:
    rt2 = re.sub(rf'^{re.escape(pkg)}\s*=\s*\{{[^}}]*\}}\s*\n', "", rt2, flags=re.M)
    # with comments
    rt2 = re.sub(rf'^{re.escape(pkg)}\s*=\s*\{{[^}}]*\}}[^\n]*\n', "", rt2, flags=re.M)

ws_deps_add = []
for name, d in (
    ("semio-s-3d", s3d_dir),
    ("semio-s-mindmap", smind_dir),
    ("semio-s-language-bundle", slang_dir),
):
    if f"{name} =" not in rt2:
        ws_deps_add.append(f'{name} = {{ path = "{d.relative_to(ROOT).as_posix()}" }}\n')
if ws_deps_add:
    # insert into [workspace.dependencies]
    marker = "[workspace.dependencies]"
    if marker in rt2:
        idx = rt2.find(marker) + len(marker)
        rt2 = rt2[:idx] + "\n" + "".join(ws_deps_add) + rt2[idx:]
        print("ADDED workspace.dependencies", ws_deps_add)

if rt2 != rt:
    root_cargo.write_text(rt2, encoding="utf-8")
    print("UPDATED root Cargo.toml")

print(f"DONE cargo={cargo_n} rs={rs_n}")
