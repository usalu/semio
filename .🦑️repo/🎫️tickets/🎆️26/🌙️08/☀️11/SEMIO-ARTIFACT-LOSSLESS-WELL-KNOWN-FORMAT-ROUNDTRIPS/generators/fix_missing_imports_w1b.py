#!/usr/bin/env python3
# 📎 W1b closer: fixes a scaffold-wide gap — each subset/format's 🧬️schema/🦀️component.rs field
# references a companion type from its own snapshot module (e.g. `Vec<BrepSolid>`) without
# importing it (only the top-level *Snapshot type was imported). Real bug, in-scope per the task
# brief ("fixing any scaffold-file compile errors you find along the way is IN your scope").
import re

REPO = "/Users/ueli/Documents/semio/"

fixes = {
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/🦀️component.rs": ["AnimTimeline"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️audio/🧬️schema/🦀️component.rs": ["SemioAudioChannel"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🦀️component.rs": ["BrepSolid"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🦀️component.rs": ["CadEntity"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/🦀️component.rs": ["DocBlock", "DocStyle"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🦀️component.rs": ["DrawNode"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️image/🧬️schema/🦀️component.rs": ["SemioImageFrame"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🦀️component.rs": ["SemioMesh", "SemioMaterial"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🦀️component.rs": ["SemioModelElement"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🦀️component.rs": ["SemioValue"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️presentation/🧬️schema/🦀️component.rs": ["Slide"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/🧬️schema/🦀️component.rs": ["SemioVideoStream"],
    "🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️workflow/🧬️schema/🦀️component.rs": ["WorkflowNode", "WorkflowEdge"],
    "🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🦀️component.rs": ["Mp4RawBox"],
    "🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️component.rs": ["AviRawChunk"],
    "🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🦀️component.rs": ["Id3v2Header"],
    "🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🦀️component.rs": ["WavFmt", "WavRawChunk"],
    "🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🦀️component.rs": ["EpwLocation"],
    "🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🦀️component.rs": ["TsvRecord"],
}

IMPORT_RE = re.compile(r"use ((?:crate::)?[\w:]*::schema::snapshot::)(\{[^}]*\}|\w+);")

for relpath, missing in fixes.items():
    path = REPO + "✏️s/🔌️plugins/🗄️stdio/" + relpath
    with open(path, encoding="utf-8") as f:
        content = f.read()
    m = IMPORT_RE.search(content)
    if not m:
        print("NO MATCH:", relpath)
        continue
    prefix, names_blob = m.group(1), m.group(2)
    if names_blob.startswith("{"):
        existing = [n.strip() for n in names_blob[1:-1].split(",") if n.strip()]
    else:
        existing = [names_blob]
    for name in missing:
        if name not in existing:
            existing.append(name)
    new_names = "{" + ", ".join(existing) + "}"
    new_line = f"use {prefix}{new_names};"
    new_content = content[: m.start()] + new_line + content[m.end():]
    if new_content == content:
        print("NO CHANGE:", relpath)
        continue
    with open(path, "w", encoding="utf-8") as f:
        f.write(new_content)
    print("fixed:", relpath, "->", existing)
