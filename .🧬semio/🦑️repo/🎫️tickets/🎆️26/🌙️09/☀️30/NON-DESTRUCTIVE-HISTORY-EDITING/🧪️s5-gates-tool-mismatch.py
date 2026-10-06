"""🔀️ Converts every raw per-plugin tool-mismatch refusal to the framework code (audit F9, `📓️s4-gates-report.md` § S5.3).

An explicit-file-list codemod: `SITES` names every file, the call form and raw code at each site and how often it occurs
(census 2026-10-05: 84 sites, 33 raw codes, 77 files, 8 plugins). It touches nothing else and never walks a tree.

    python3 🧪️s5-gates-tool-mismatch.py --root "✏️s/🔌️plugins/<plugin>"            # check (default): prints file:line → new code, exit 1 when sites remain
    python3 🧪️s5-gates-tool-mismatch.py --root "✏️s/🔌️plugins/<plugin>" --apply    # rewrites exactly those sites

Run it from the repository root on YOUR plugin tree. It fails closed (exit 2, nothing written) when `--root` is missing, empty,
absolute, not a directory, not a plugin tree (`✏️s/🔌️plugins/<plugin>` or below), holds none of the listed files, or when a
listed file holds neither exactly the listed sites nor their rewritten form (someone edited it: re-run the census first). A
second run after `--apply` reports "none left" and exits 0.

Rewrites: `Fault::from("<raw>")` becomes `Fault::new(semio_framework_plugin::FaultOrigin::App,
semio_framework_plugin::FaultCode::new("<code>"), "<raw>")` (the form the 20 adopted plugins use; the raw text stays as the
fault's message), and a helper call `edit_fault("<raw>", …)` / `fault("<raw>", …)` keeps its helper and swaps the code.
`<code>` is `app.command.tool-mismatch`, or `app.command.unsupported` for a `…-tool-unmapped` refusal; both are labelled en/de
in the kernel's framework notice table.
"""
import os
import re
import sys

PLUGINS_ROOT = "✏️s/🔌️plugins"
QUALIFIER = "semio_framework_plugin::"
SITES = [
    ("✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🦀️.rs", "Fault::from", "norm-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "cad-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "playground-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "block3d-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "block5d-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "block2d-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs", "edit_fault", "bounded-native-edit.tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs", "edit_fault", "snapshot-edit.tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🦀️.rs", "Fault::from", "stdio-bcf-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-wav-audio-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎮️commands/🔊️edit-audio/🦀️.rs", "fault", "stdio.wav.audio-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-bmp-paint-region-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-html-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-deflate-text-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/✏️editor/🦀️.rs", "Fault::from", "stdio-xml-valid-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs", "Fault::from", "stdio-xml-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-md-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🦀️.rs", "Fault::from", "stdio-tiff-paint-region-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-txt-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-binary-text-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-png-pixel-region-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-png-native-region-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs", "Fault::from", "stdio-json-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs", "Fault::from", "stdio-json-i-json-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-csv-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "stdio-tsv-retained-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/✏️editor/🦀️.rs", "Fault::from", "stdio-example-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "animate-presentation-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "sourcing-curation-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "puzzle3d-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "puzzle5d-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "puzzle2d-command-tool-mismatch", 1),
    ("✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs", "Fault::from", "puzzle2d-command-tool-unmapped", 1),
]


def target_code(raw):
    """🎯️ The framework code a raw refusal becomes."""
    return "app.command.unsupported" if raw.endswith("tool-unmapped") else "app.command.tool-mismatch"


def refuse(message):
    """🛑️ Fails closed: prints why and exits 2 before anything is written."""
    print(f"[tool-mismatch] REFUSED: {message}", file=sys.stderr)
    sys.exit(2)


def rewritten(source, form, raw):
    """✍️ `source` with every site of one `(form, raw)` rewritten, the 1-based lines of those sites, and how often the
    rewritten form already stands in `source`."""
    old = f'{form}("{raw}"'
    pattern = re.compile(r"(?<![A-Za-z0-9_])" + re.escape(old))
    lines = [source.count("\n", 0, match.start()) + 1 for match in pattern.finditer(source)]
    new = f'Fault::new({QUALIFIER}FaultOrigin::App, {QUALIFIER}FaultCode::new("{target_code(raw)}"), "{raw}"' if form == "Fault::from" else f'{form}("{target_code(raw)}"'
    return pattern.sub(lambda _: new, source), lines, source.count(new)


def main():
    arguments = sys.argv[1:]
    apply = "--apply" in arguments
    unknown = [argument for index, argument in enumerate(arguments) if argument not in ("--apply", "--check", "--root") and (index == 0 or arguments[index - 1] != "--root")]
    if unknown or ("--apply" in arguments and "--check" in arguments):
        refuse(f"unknown or conflicting arguments {unknown}; usage: --root <plugin tree> [--check | --apply]")
    root = arguments[arguments.index("--root") + 1] if "--root" in arguments and arguments.index("--root") + 1 < len(arguments) else ""
    root = root.rstrip("/")
    if root == "" or root.startswith("-"):
        refuse("--root is required and must not be empty")
    if os.path.isabs(root) or ".." in root.split("/"):
        refuse(f"--root {root!r} must be repository-relative, without ..")
    if not (root + "/").startswith(PLUGINS_ROOT + "/") or len(root.split("/")) < 3:
        refuse(f"--root {root!r} is not a plugin tree ({PLUGINS_ROOT}/<plugin> or below)")
    if not os.path.isdir(PLUGINS_ROOT) or not os.path.isdir(root):
        refuse(f"--root {root!r} is not a directory under the current directory {os.getcwd()!r}: run from the repository root")
    chosen = [site for site in SITES if site[0].startswith(root + "/")]
    if not chosen:
        refuse(f"--root {root!r} holds none of the {len(SITES)} listed site rows")
    plan = {}
    report = []
    for path, form, raw, expected in chosen:
        if not os.path.isfile(path):
            refuse(f"{path} is listed but missing")
        source = plan.get(path)
        if source is None:
            with open(path, encoding="utf-8") as handle:
                source = handle.read()
        if form == "Fault::from" and QUALIFIER not in source:
            refuse(f"{path} never names {QUALIFIER}: its Fault type cannot be rewritten mechanically")
        after, lines, done = rewritten(source, form, raw)
        if not lines and done >= expected:
            continue
        if len(lines) != expected:
            refuse(f"{path} holds {len(lines)} site(s) of {form}(\"{raw}\"), the list expects {expected}: the file changed, re-run the census")
        plan[path] = after
        report.extend((path, line, raw) for line in lines)
    for path, line, raw in report:
        print(f"{path}:{line}\t{raw} → {target_code(raw)}")
    if apply:
        for path, after in plan.items():
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(after)
    print(f"[tool-mismatch] {len(report)} site(s) in {len(plan)} file(s) under {root}: {'REWRITTEN' if apply else 'to rewrite (check only; pass --apply)' if report else 'none left, every listed site already carries the framework code'}")
    sys.exit(0 if apply or not report else 1)


main()
