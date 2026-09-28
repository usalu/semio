"""🧾️ EN2: writes (or refreshes) the seven real-input row manifests of the glTF artifact-root case in a catalog copy —
`--placeholder` before the generator runs (it resolves its output paths from the catalog), then with the measured
sha256/bytes of every committed file. Text-level: appends/replaces only these seven entries, the rest of the catalog
keeps its bytes.

usage: en2-gltf-real-manifests.py <catalog 🔣️.json> [--placeholder]
"""
import hashlib
import json
import re
import sys
from pathlib import Path

REAL_INPUT = "../🧫️fixtures/🧊️mutate-gltf-2-0/🌳️base-with-nested-node/🧊️.glb"
ROWS = [
    ("bind-node-child-metabolism-applied", "bind-node-child", "scene", "🔗️bind-node-child", "structural", "Links node 3 as node 2's first child — bind-node-child{parent:2,child:3,position:0}; node 3 keeps its scene-root entry, which the operation never touches."),
    ("unbind-node-child-metabolism-applied", "unbind-node-child", "scene", "✂️unbind-node-child", "structural", "Removes the real input's one nested link, node 1 from node 0's children — unbind-node-child{parent:0,child:1}."),
    ("bind-scene-root-node-metabolism-applied", "bind-scene-root-node", "scene", "🌲️bind-scene-root-node", "structural", "Adds node 1 (node 0's child) as scene 0's first root — bind-scene-root-node{scene:0,node:1,position:0}."),
    ("unbind-scene-root-node-metabolism-applied", "unbind-scene-root-node", "scene", "🍂️unbind-scene-root-node", "structural", "Removes node 5 from scene 0's root list — unbind-scene-root-node{scene:0,node:5}."),
    ("change-material-alpha-mode-metabolism-applied", "change-material-alpha-mode", "material", "🎭️change-material-alpha-mode", "material", "Material 0's alphaMode: OPAQUE (absent) -> MASK — change-material-alpha-mode{material:0,alphaMode:MASK}."),
    ("change-material-double-sided-metabolism-applied", "change-material-double-sided", "material", "🪞️change-material-double-sided", "material", "Material 0's doubleSided: false (absent) -> true — change-material-double-sided{material:0,doubleSided:true}."),
    ("create-scene-metabolism-applied", "create-scene", "scene", "🎬️create-scene", "structural", "Inserts one empty scene at position 0 and bumps the default scene pointer 0 -> 1 so it still names the real scene — create-scene{position:0}."),
]
GENERATOR = "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🏭️generator/📜️script.ts generate --only "


def file_entry(catalog_dir, role, path, placeholder):
    data = b"" if placeholder else (catalog_dir / path).read_bytes()
    digest = "0" * 64 if placeholder else hashlib.sha256(data).hexdigest()
    return {"role": role, "path": path, "mediaType": "model/gltf-binary", "sha256": "sha256:" + digest, "bytes": len(data)}


def manifest(catalog_dir, row, placeholder):
    ident, mutation, subset, directory, family, notes = row
    return {
        "schema": "semio.repository-test.fixture/v2",
        "id": ident,
        "class": "handcrafted",
        "target": {"artifact": "s.stdio.gltf", "standard": "2.0", "subset": subset},
        "mutation": mutation,
        "outcome": "applied",
        "units": {"length": "unitless", "angle": "radian"},
        "files": [
            file_entry(catalog_dir, "expected-before-glb", REAL_INPUT, placeholder),
            file_entry(catalog_dir, "expected-after-glb", "../🧫️fixtures/🧊️mutate-gltf-2-0/%s/➡️after.glb" % directory, placeholder),
        ],
        "provenance": {
            "source": "authored",
            "license": "n/a (structural JSON-chunk patch of the repository's own metabolism example export)",
            "attribution": "The artifact-root case's committed real GLB export with this row's edit applied to its JSON chunk by the subset generator's generic structural operations, the BIN chunk carried byte for byte; reproduced by `%s%s`" % (GENERATOR, ident),
            "security": "scanned-clean",
            "privacy": "no-personal-data",
        },
        "comparisonProfile": "semantic-gltf-reader-v1",
        "reproducible": True,
        "family": family,
        "notes": notes,
    }


def main(argv):
    path = Path(argv[0])
    placeholder = "--placeholder" in argv
    text = path.read_text(encoding="utf-8")
    ids = {row[0] for row in ROWS}
    catalog = json.loads(text)
    kept = [entry for entry in catalog["fixtureManifests"] if entry["id"] not in ids]
    if len(kept) != len(catalog["fixtureManifests"]):
        start = text.index('\n    {\n      "schema": "semio.repository-test.fixture/v2",\n      "id": "%s"' % ROWS[0][0])
        text = text[:start] + "\n  ]\n}\n"
        text = re.sub(r",\n  \]\n\}\n$", "\n  ]\n}\n", text)
    blocks = [json.dumps(manifest(path.parent, row, placeholder), indent=2, ensure_ascii=False) for row in ROWS]
    body = ",\n".join("\n".join("    " + line for line in block.splitlines()) for block in blocks)
    if not text.endswith("\n    }\n  ]\n}\n"):
        raise SystemExit("unexpected catalog tail")
    text = text[: -len("\n  ]\n}\n")] + ",\n" + body + "\n  ]\n}\n"
    json.loads(text)
    path.write_text(text, encoding="utf-8")
    print("wrote %d real-row manifests (%s)" % (len(ROWS), "placeholder" if placeholder else "measured"))


if __name__ == "__main__":
    main(sys.argv[1:])
