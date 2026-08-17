#!/usr/bin/env python3
"""Adds <KIND>_DIALECT const(s) to each stdio artifact root component.rs (12 files, my kinds only)."""
import re, os

ART = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"

# kind_dir, [(const_name, artifact_kind, standard, subset)]
ROOTS = [
    ("📷️png", [("PNG_DIALECT", "s.stdio.png", "1.2", "*")]),
    ("📷️jpg", [("JPG_ANY_DIALECT", "s.stdio.jpg", "jfif-1.01", "*"),
                ("JPG_BASELINE_DIALECT", "s.stdio.jpg", "jfif-1.01", "baseline")]),
    ("🖼️bmp", [("BMP_DIALECT", "s.stdio.bmp", "v3", "*")]),
    ("🖼️tiff", [("TIFF_ANY_DIALECT", "s.stdio.tiff", "6.0", "*"),
                 ("TIFF_BASELINE_DIALECT", "s.stdio.tiff", "6.0", "baseline")]),
    ("🎞️gif", [("GIF_87A_DIALECT", "s.stdio.gif", "87a", "*"),
                ("GIF_89A_DIALECT", "s.stdio.gif", "89a", "*")]),
    ("🎨️svg", [("SVG_ANY_DIALECT", "s.stdio.svg", "1.1", "*"),
                ("SVG_BASIC_DIALECT", "s.stdio.svg", "1.1", "basic"),
                ("SVG_TINY_DIALECT", "s.stdio.svg", "1.1", "tiny")]),
    ("🎥️mp4", [("MP4_DIALECT", "s.stdio.mp4", "isobmff", "*")]),
    ("🎵️mp3", [("MP3_DIALECT", "s.stdio.mp3", "mpeg1-layer3", "*")]),
    ("🔊️wav", [("WAV_DIALECT", "s.stdio.wav", "riff-pcm", "*")]),
    ("📼️avi", [("AVI_DIALECT", "s.stdio.avi", "1.0", "*")]),
    ("🌐️html", [("HTML_DIALECT", "s.stdio.html", "5", "*")]),
    ("📝️md", [("MD_DIALECT", "s.stdio.md", "commonmark", "*")]),
]

OLD_USE = "use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};"
NEW_USE = "use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};"

for kind_dir, consts in ROOTS:
    path = os.path.join(ART, kind_dir, "🦀️component.rs")
    text = open(path, encoding="utf-8").read()
    assert OLD_USE in text, f"use-line not found verbatim in {path}"
    text = text.replace(OLD_USE, NEW_USE, 1)

    m = re.search(r'\npub const \w+_ARTIFACT_SCHEMA_ID: &str = "[^"]+";\n', text)
    assert m, f"ARTIFACT_SCHEMA_ID const not found in {path}"
    insert_at = m.end()

    block_lines = ["\n//#region 🔖️Dialect"]
    block_lines.append(f'/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor\n'
                        f'/// id above verbatim (never guessed); `standard`/`subset` match this file\'s own on-disk\n'
                        f'/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under\n'
                        f'/// `editor`/`viewer`) so a viewer file can read it without ever importing through the\n'
                        f'/// sibling `editor` module.')
    for const_name, artifact_kind, standard, subset in consts:
        block_lines.append(f'pub const {const_name}: Dialect = Dialect {{ artifact_kind: "{artifact_kind}", standard: StandardId("{standard}"), subset: SubsetId("{subset}") }};')
    block_lines.append("//#endregion 🔖️Dialect\n")
    block = "\n".join(block_lines)

    text = text[:insert_at] + block + text[insert_at:]
    open(path, "w", encoding="utf-8").write(text)
    print("patched", path, "with", [c[0] for c in consts])
