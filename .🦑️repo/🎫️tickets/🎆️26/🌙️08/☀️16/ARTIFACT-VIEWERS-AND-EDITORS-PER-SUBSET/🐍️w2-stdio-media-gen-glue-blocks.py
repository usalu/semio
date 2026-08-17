#!/usr/bin/env python3
"""Builds the glue.rs `pub mod editor { ... }` / `pub mod viewer { ... }` mount text for my 17
stdio-media subsets, and the `.editor()/.viewer()` registration lines for the plugin() builder.
Writes results to scratch files for manual, careful insertion (glue.rs and the plugin builder file
are shared with sibling packets — insertion is done by hand via Edit, not by this script)."""
import os

REPO = "/Users/ueli/Documents/semio"
ART = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")

def p(*parts):
    return os.path.join(*parts)

def find_subset_dir(kind_dir, std_dirname, subset_dirname):
    base = p(ART, kind_dir, "🏅️standards", std_dirname, "🪆️subsets", subset_dirname)
    assert os.path.isdir(base), f"MISSING {base}"
    return base

SUBSETS = [
    dict(subset_key="png", kind_dir="📷️png", std="🔖️1.2", subset="✳️any", X="Png"),
    dict(subset_key="jpg_any", kind_dir="📷️jpg", std="🔖️jfif-1.01", subset="✳️any", X="JpgAny"),
    dict(subset_key="jpg_baseline", kind_dir="📷️jpg", std="🔖️jfif-1.01", subset="✳️baseline", X="JpgBaseline"),
    dict(subset_key="bmp", kind_dir="🖼️bmp", std="🔖️v3", subset="✳️any", X="Bmp"),
    dict(subset_key="tiff_any", kind_dir="🖼️tiff", std="🔖️6.0", subset="✳️any", X="TiffAny"),
    dict(subset_key="tiff_baseline", kind_dir="🖼️tiff", std="🔖️6.0", subset="✳️baseline", X="TiffBaseline"),
    dict(subset_key="gif_87a", kind_dir="🎞️gif", std="🔖️87a", subset="✳️any", X="Gif87a"),
    dict(subset_key="gif_89a", kind_dir="🎞️gif", std="🔖️89a", subset="✳️any", X="Gif89a"),
    dict(subset_key="svg_any", kind_dir="🎨️svg", std="🔖️1.1", subset="✳️any", X="SvgAny"),
    dict(subset_key="svg_basic", kind_dir="🎨️svg", std="🔖️1.1", subset="✳️basic", X="SvgBasic"),
    dict(subset_key="svg_tiny", kind_dir="🎨️svg", std="🔖️1.1", subset="✳️tiny", X="SvgTiny"),
    dict(subset_key="mp4", kind_dir="🎥️mp4", std="🔖️isobmff", subset="✳️any", X="Mp4"),
    dict(subset_key="mp3", kind_dir="🎵️mp3", std="🔖️mpeg1-layer3", subset="✳️any", X="Mp3"),
    dict(subset_key="wav", kind_dir="🔊️wav", std="🔖️riff-pcm", subset="✳️any", X="Wav"),
    dict(subset_key="avi", kind_dir="📼️avi", std="🔖️1.0", subset="✳️any", X="Avi"),
    dict(subset_key="html", kind_dir="🌐️html", std="🔖️5", subset="✳️any", X="Html"),
    dict(subset_key="md", kind_dir="📝️md", std="🔖️commonmark", subset="✳️any", X="Md"),
]
for s in SUBSETS:
    s["base"] = find_subset_dir(s["kind_dir"], s["std"], s["subset"])

def rel(base_abs):
    # relative path from glue.rs's own dir: ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/ -> ../../🗿️artifacts/...
    idx = base_abs.index("🗿️artifacts")
    return "../../" + base_abs[idx:]

def surface_block(s, role, indent):
    base = s["base"]
    role_dir = "✏️editor" if role == "editor" else "👁️viewer"
    mode_name = "edit" if role == "editor" else "view"
    mode_dir = "✏️edit" if role == "editor" else "👁️view"
    I = "    " * indent
    lines = []
    lines.append(f'{I}#[path = "."]')
    lines.append(f'{I}pub mod {s["subset_key"]} {{')
    lines.append(f'{I}    #[path = "{rel(p(base, role_dir, "🦀️component.rs"))}"]')
    lines.append(f'{I}    mod component;')
    lines.append(f'{I}    pub use component::*;')
    lines.append(f'{I}    #[path = "."]')
    lines.append(f'{I}    pub mod modes {{')
    lines.append(f'{I}        #[path = "."]')
    lines.append(f'{I}        pub mod {mode_name} {{')
    lines.append(f'{I}            #[path = "{rel(p(base, role_dir, "🎭️modes", mode_dir, "🦀️component.rs"))}"]')
    lines.append(f'{I}            mod component;')
    lines.append(f'{I}            pub use component::*;')
    lines.append(f'{I}            #[path = "."]')
    lines.append(f'{I}            pub mod windows {{')
    lines.append(f'{I}                #[path = "."]')
    lines.append(f'{I}                pub mod main {{')
    lines.append(f'{I}                    #[path = "{rel(p(base, role_dir, "🎭️modes", mode_dir, "🪟️windows", "🪟️main", "🦀️component.rs"))}"]')
    lines.append(f'{I}                    mod component;')
    lines.append(f'{I}                    pub use component::*;')
    lines.append(f'{I}                }}')
    lines.append(f'{I}            }}')
    lines.append(f'{I}        }}')
    lines.append(f'{I}    }}')
    lines.append(f'{I}}}')
    return "\n".join(lines)

editor_lines = ["//#region ✏️Editor", "pub mod editor {"]
for s in SUBSETS:
    editor_lines.append(surface_block(s, "editor", 1))
editor_lines.append("}")
editor_lines.append("//#endregion ✏️Editor")

viewer_lines = ["//#region 👁️Viewer", "pub mod viewer {"]
for s in SUBSETS:
    viewer_lines.append(surface_block(s, "viewer", 1))
viewer_lines.append("}")
viewer_lines.append("//#endregion 👁️Viewer")

out_dir = "/private/tmp/claude-501/-Users-ueli-Documents-semio/674bff55-53f7-4bce-9cda-3d1a0d05ab6e/scratchpad"
open(os.path.join(out_dir, "glue_editor_block.txt"), "w", encoding="utf-8").write("\n".join(editor_lines) + "\n")
open(os.path.join(out_dir, "glue_viewer_block.txt"), "w", encoding="utf-8").write("\n".join(viewer_lines) + "\n")

# Plugin builder registration lines ---------------------------------------------------------------
reg_lines = []
for s in SUBSETS:
    reg_lines.append(f'    builder = builder.editor::<crate::editor::{s["subset_key"]}::{s["X"]}Editor>(crate::editor::{s["subset_key"]}::create_{s["subset_key"]}_editor());')
    reg_lines.append(f'    builder = builder.viewer::<crate::viewer::{s["subset_key"]}::{s["X"]}Viewer>(crate::viewer::{s["subset_key"]}::create_{s["subset_key"]}_viewer());')
open(os.path.join(out_dir, "plugin_registrations.txt"), "w", encoding="utf-8").write("\n".join(reg_lines) + "\n")

print("wrote glue_editor_block.txt, glue_viewer_block.txt, plugin_registrations.txt")
print("editor block lines:", len(editor_lines), "viewer block lines:", len(viewer_lines), "reg lines:", len(reg_lines))
