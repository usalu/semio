#!/usr/bin/env python3
"""🧭️ Build the gltf mutation -> domain-subset mapping for shard A6, from the on-disk mutation
directory names (verb-emoji + entity-emoji + kebab-name), matching the taxonomy the glTF 2.0 core
schema itself uses (scenes, nodes, meshes+primitives+morph-targets+accessors, materials+textures+
samplers+images, animations, skins, cameras, buffers+bufferViews, asset/document/extensions)."""
import re, json, sys

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"

with open(f"{TICKET}/🗑️generated/a6-gltf-mutation-dirs.txt", encoding="utf-8") as f:
    dirs = [l.strip() for l in f if l.strip()]

dirs = [d for d in dirs if d not in ("💾️binary", "📝️text")]
assert len(dirs) == 120, len(dirs)

ENTITY_TO_SUBSET = {
    "🎬️": "scene", "🔘️": "scene",
    "🕸️": "mesh", "🔺️": "mesh", "🧬️": "mesh", "📐️": "mesh",
    "💎️": "material", "🎨️": "material", "🎛️": "material", "🖼️": "material",
    "🎞️": "animation",
    "🧥️": "skin",
    "🎥️": "camera",
    "💾️": "buffer", "👁️": "buffer",
    "📦️": "asset", "📄️": "asset", "🧩️": "asset",
}

pat = re.compile(r'^(\S+?)([a-z][a-z0-9-]*)$')
mapping = {}
for d in dirs:
    m = pat.match(d)
    if not m:
        print("NOMATCH", repr(d)); sys.exit(1)
    emojis, name = m.groups()
    clusters = [c for c in re.findall(r'.️?', emojis) if c.strip()]
    if len(clusters) < 2:
        print("SHORT", repr(d), clusters); sys.exit(1)
    entity = clusters[1]
    subset = ENTITY_TO_SUBSET.get(entity)
    if subset is None:
        print("UNKNOWN ENTITY", repr(d), entity); sys.exit(1)
    mapping[d] = {"ascii": name, "subset": subset}

counts = {}
for v in mapping.values():
    counts[v["subset"]] = counts.get(v["subset"], 0) + 1
print(json.dumps(counts, indent=2))
assert sum(counts.values()) == 120

with open(f"{TICKET}/🗑️generated/a6-gltf-subset-mapping.json", "w", encoding="utf-8") as f:
    json.dump(mapping, f, ensure_ascii=False, indent=2)
print("wrote mapping for", len(mapping), "mutations")
