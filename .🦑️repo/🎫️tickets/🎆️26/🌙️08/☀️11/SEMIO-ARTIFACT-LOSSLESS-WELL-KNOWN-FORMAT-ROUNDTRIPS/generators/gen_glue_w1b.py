#!/usr/bin/env python3
# 📎 W1b closer: generates the stdio glue.rs #[path] mount blocks for the 8 new artifact
# directories (semio + 7 formats) by walking the scaffold's real files on disk, mirroring
# gif/step's exact nested-mod convention. See w1b-scaffold-manifest.md §3.
import os, sys

REPO = "/Users/ueli/Documents/semio"
ART_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
RS = "🦀️component.rs"

# Identifier map for directory basenames that need a non-mechanical alias.
STANDARD_ALIASES = {
    "🔖️v1": "v1",
    "🔖️isobmff": "isobmff",
    "🔖️1.0": "v1_0",
    "🔖️mpeg1-layer3": "mpeg1_layer3",
    "🔖️riff-pcm": "riff_pcm",
    "🔖️energyplus": "energyplus",
    "🔖️iana": "iana",
    "🔖️5": "v5",
}

NAME_MAP = {
    "🏗️builder": "builder",
    "🧐️analyzer": "analyzer",
    "🎹️composer": "composer",
    "🏅️standards": "standards",
    "🪆️subsets": "subsets",
    "🧬️schema": "schema",
    "📸️snapshot": "snapshot",
    "🔺️diff": "diff",
    "🧬️mutations": "mutations",
    "📝️text": "text",
    "💾️binary": "binary",
    "📄set-snapshot": "set_snapshot",
    "🦠️mutation": "mutation",
    "↩️inverse": "inverse",
    "🚪️io": "io",
    "⚙️engine": "engine",
    "🧮️geometry": "geometry",
    "🧰️triples": "triples",
    "📚️examples": "examples",
    "🎬️demo": "demo",
}

def ident_for(basename):
    if basename in STANDARD_ALIASES:
        return STANDARD_ALIASES[basename]
    if basename in NAME_MAP:
        return NAME_MAP[basename]
    if basename.startswith("✳️"):
        return basename[len("✳️"):]
    raise SystemExit(f"unmapped dir name: {basename!r}")

def has_rs_below(path):
    for root, dirs, files in os.walk(path):
        if RS in files:
            return True
    return False

def child_dirs_with_rs(path):
    out = []
    for name in sorted(os.listdir(path)):
        full = os.path.join(path, name)
        if os.path.isdir(full) and has_rs_below(full):
            out.append(name)
    return out

def relpath_for_mount(abs_path):
    # 📎 all #[path] attrs in this glue.rs are relative to
    # ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/ — i.e. "../../🗿️artifacts/<rest>"
    rest = os.path.relpath(abs_path, ART_ROOT)
    return "../../🗿️artifacts/" + rest

def emit(path, indent):
    """Yields lines mounting the *contents* of `path` (caller already opened the enclosing block)."""
    own_rs = os.path.join(path, RS)
    has_own = os.path.isfile(own_rs)
    children = child_dirs_with_rs(path)
    pad = "    " * indent
    lines = []
    if has_own:
        lines.append(f'{pad}#[path = "{relpath_for_mount(own_rs)}"]')
        lines.append(f"{pad}mod component;")
        lines.append(f"{pad}pub use component::*;")
    for child in children:
        child_path = os.path.join(path, child)
        ident = ident_for(child)
        child_has_own = os.path.isfile(os.path.join(child_path, RS))
        child_children = child_dirs_with_rs(child_path)
        if child_has_own and not child_children:
            # leaf: single-line mount
            lines.append(f'{pad}#[path = "{relpath_for_mount(os.path.join(child_path, RS))}"]')
            lines.append(f"{pad}pub mod {ident};")
        else:
            lines.append(f'{pad}#[path = "."]')
            lines.append(f"{pad}pub mod {ident} {{")
            lines.extend(emit(child_path, indent + 1))
            lines.append(f"{pad}}}")
    return lines

def emit_artifact(artifact_dir_name, mod_ident):
    path = os.path.join(ART_ROOT, artifact_dir_name)
    pad = "    "
    lines = [f'{pad}#[path = "."]', f"{pad}pub mod {mod_ident} {{"]
    lines.extend(emit(path, 2))
    lines.append(f"{pad}}}")
    return "\n".join(lines)

if __name__ == "__main__":
    targets = [
        ("🧿️semio", "semio"),
        ("🎥️mp4", "mp4"),
        ("📼️avi", "avi"),
        ("🎵️mp3", "mp3"),
        ("🔊️wav", "wav"),
        ("🌦️epw", "epw"),
        ("📑️tsv", "tsv"),
        ("🌐️html", "html"),
    ]
    out = []
    for dirname, ident in targets:
        out.append(emit_artifact(dirname, ident))
    text = "\n\n".join(out) + "\n"
    outpath = sys.argv[1] if len(sys.argv) > 1 else "/dev/stdout"
    with open(outpath, "w", encoding="utf-8") as f:
        f.write(text)
    print(f"wrote {len(text)} bytes to {outpath}", file=sys.stderr)
