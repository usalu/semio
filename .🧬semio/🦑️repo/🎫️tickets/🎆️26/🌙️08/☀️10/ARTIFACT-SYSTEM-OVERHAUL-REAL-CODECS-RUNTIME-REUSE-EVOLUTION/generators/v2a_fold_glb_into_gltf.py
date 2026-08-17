#!/usr/bin/env python3
"""D2 gltf/glb merge step 3: fold every domain-plugin `.glb` stub leaf into the
already-existing `.gltf` leaf (same plugins already have BOTH, generated identically
by generators/w15_add_export_entries.py -- both leaves are dead `default()`/`print_dsl`
stubs, so the fold is a pure deletion of the redundant DEP_GLB half, not an upgrade).

Idempotent: safe to re-run, skips files/blocks already folded.
"""
import re
import shutil
import sys
from pathlib import Path

def find_root(p: Path) -> Path:
    for parent in [p] + list(p.parents):
        if (parent / "✏️s").is_dir() and (parent / ".git").exists():
            return parent
    raise RuntimeError(f"could not find repo root from {p}")

ROOT = find_root(Path(__file__).resolve())

PLUGINS = ["🌀️procedural", "🌍️gis", "🏗️fem", "🏭️process", "💠️lowpoly", "📐️cad", "📸️remodel", "🧩️puzzle", "🧱️block", "🪵️sourcing"]

def find_artifact_dirs():
    dirs = []
    for plugin in PLUGINS:
        base = ROOT / "✏️s" / "🔌️plugins" / plugin / "🗿️artifacts"
        if not base.is_dir():
            continue
        for artifact_dir in sorted(base.iterdir()):
            if not artifact_dir.is_dir():
                continue
            io_dir = artifact_dir / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any" / "🚪️io"
            glb_import = io_dir / "📥️import" / "🧩️deserializers" / "🗿️artifacts" / "🧊️glb"
            glb_export = io_dir / "📤️export" / "🧵️serializers" / "🗿️artifacts" / "🧊️glb"
            if glb_import.is_dir() or glb_export.is_dir():
                dirs.append((plugin, artifact_dir))
    return dirs

def strip_export_glb_block(text: str) -> str:
    # remove `const EXPORT_GLB_DIALECT ...;\nfn compose_export_glb(...) { ... }\n`
    m = re.search(r'const EXPORT_GLB_DIALECT: Dialect = Dialect \{[^\n]*\};\n', text)
    if not m:
        return text
    start = m.start()
    fn_m = re.search(r'fn compose_export_glb\([^\n]*\{', text)
    assert fn_m, "compose_export_glb fn not found even though EXPORT_GLB_DIALECT const was"
    brace_start = fn_m.end() - 1
    depth = 0
    i = brace_start
    while i < len(text):
        if text[i] == '{':
            depth += 1
        elif text[i] == '}':
            depth -= 1
            if depth == 0:
                break
        i += 1
    end = i + 1
    if end < len(text) and text[end] == '\n':
        end += 1
    return text[:start] + text[end:]

def strip_entries_line(text: str) -> str:
    return re.sub(r'[ \t]*ComposerEntry \{ writes: EXPORT_GLB_DIALECT,[^\n]*compose_export_glb[^\n]*\},\n', '', text)

def strip_dep_glb_const(text: str) -> str:
    return re.sub(r'const DEP_GLB: Dialect = Dialect \{[^\n]*\};\n', '', text)

def strip_dep_glb_from_reads(text: str) -> str:
    text = text.replace('DEP_GLB, ', '')
    text = text.replace(', DEP_GLB', '')
    text = text.replace('DEP_GLB,', '')
    text = text.replace('DEP_GLB', '')
    return text

def strip_dep_glb_branch(text: str) -> str:
    m = re.search(r'[ \t]*if source\.dialect == DEP_GLB \{', text)
    if not m:
        return text
    start = m.start()
    brace_start = text.index('{', m.start())
    depth = 0
    i = brace_start
    while i < len(text):
        if text[i] == '{':
            depth += 1
        elif text[i] == '}':
            depth -= 1
            if depth == 0:
                break
        i += 1
    end = i + 1
    if end < len(text) and text[end] == '\n':
        end += 1
    return text[:start] + text[end:]

def strip_stdio_glb_literal(text: str) -> str:
    text = text.replace('"stdio.glb", ', '')
    text = text.replace('"stdio.glb",', '')
    text = text.replace('"stdio.glb"', '')
    return text

def strip_glue_glb_mod(text: str) -> str:
    out = text
    while True:
        m = re.search(r'[ \t]*#\[path = "\."\]\n[ \t]*pub mod glb \{', out)
        if not m:
            break
        start = m.start()
        brace_start = out.index('{', m.start())
        depth = 0
        i = brace_start
        while i < len(out):
            if out[i] == '{':
                depth += 1
            elif out[i] == '}':
                depth -= 1
                if depth == 0:
                    break
            i += 1
        end = i + 1
        if end < len(out) and out[end] == '\n':
            end += 1
        out = out[:start] + out[end:]
    return out

def process_file(path: Path, fn):
    if not path.is_file():
        return False
    text = path.read_text(encoding="utf-8")
    new_text = fn(text)
    if new_text != text:
        path.write_text(new_text, encoding="utf-8")
        return True
    return False

def main():
    touched = []
    artifact_dirs = find_artifact_dirs()
    print(f"Found {len(artifact_dirs)} artifact dirs with glb leaves:")
    for plugin, artifact_dir in artifact_dirs:
        print(f"  {plugin} :: {artifact_dir.name}")

    plugins_touched = set()

    for plugin, artifact_dir in artifact_dirs:
        plugins_touched.add(plugin)
        std_composer = artifact_dir / "🏅️standards" / "🔖️1" / "🎹️composer" / "🦀️component.rs"
        if process_file(std_composer, lambda t: strip_entries_line(strip_export_glb_block(t))):
            touched.append(std_composer)

        subset_composer = artifact_dir / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any" / "🎹️composer" / "🦀️component.rs"
        def fold_subset_composer(t):
            t = strip_dep_glb_branch(t)
            t = strip_dep_glb_const(t)
            t = strip_dep_glb_from_reads(t)
            return t
        if process_file(subset_composer, fold_subset_composer):
            touched.append(subset_composer)

        subset_io = artifact_dir / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any" / "🚪️io" / "🦀️component.rs"
        if process_file(subset_io, strip_stdio_glb_literal):
            touched.append(subset_io)

        root_component = artifact_dir / "🦀️component.rs"
        if process_file(root_component, strip_stdio_glb_literal):
            touched.append(root_component)

        io_dir = artifact_dir / "🏅️standards" / "🔖️1" / "🪆️subsets" / "✳️any" / "🚪️io"
        for side in ["📥️import" , "📤️export"]:
            leaf_parent = io_dir / side / ("🧩️deserializers" if side == "📥️import" else "🧵️serializers") / "🗿️artifacts" / "🧊️glb"
            if leaf_parent.is_dir():
                shutil.rmtree(leaf_parent)
                touched.append(leaf_parent)
                print(f"  deleted dir: {leaf_parent}")

    for plugin in sorted(plugins_touched):
        glue = ROOT / "✏️s" / "🔌️plugins" / plugin / "📦️packages" / "🦀️rust" / "📦️glue.rs"
        if process_file(glue, strip_glue_glb_mod):
            touched.append(glue)

    print(f"\nTouched {len(touched)} paths.")
    for t in touched:
        print(f"  {t.relative_to(ROOT)}")

if __name__ == "__main__":
    main()
