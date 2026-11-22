"""One-shot editor: two levels of puff spheres by paper radius."""
import sys, pathlib
path = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🌊️flow/🟦️.ts")
text = path.read_text(encoding="utf-8")
pairs = [
    ("`polyhedron` icospheres of {@link FLOW_MODEL}`.puffTriangles` triangles, because a soft cloud",
     "`polyhedron` icospheres of {@link FLOW_MODEL}`.puffTriangles` triangles (the finer one from a paper radius of `puffFineMm` on), because a soft cloud"),
    ("the bins of the vapour and fog materials and the triangles of a puff sphere (an icosahedron subdivided twice). */\nexport const FLOW_MODEL = { chordMm: 0.08, tubeShare: 0.012, vapourBins: 4, fogBins: 3, puffTriangles: 320 } as const;",
     "the bins of the vapour and fog materials, the triangles of a small and of a large puff sphere (an icosahedron subdivided once and twice) and the paper radius in millimetres from which a puff is large. */\nexport const FLOW_MODEL = { chordMm: 0.08, tubeShare: 0.012, vapourBins: 4, fogBins: 3, puffTriangles: [80, 320], puffFineMm: 2 } as const;"),
    ("function icosphere(): {", "function icosphere(levels: number): {"),
    ("  for (let level = 0; level < 2; level++) {", "  for (let level = 0; level < levels; level++) {"),
    ("const SPHERE = icosphere();", "const SPHERES = [icosphere(1), icosphere(2)] as const;"),
    ("    const [cx, cy, cz] = neutral(puff.at);\n    return { id, node, material, tags: [FLOW_TAG], kind: \"polyhedron\", vertices: SPHERE.vertices.map(([x, y, z]) => [cx + radius * x, cy + radius * y, cz + radius * z]), triangles: SPHERE.triangles };",
     "    const [cx, cy, cz] = neutral(puff.at);\n    const mesh = SPHERES[radius < FLOW_MODEL.puffFineMm * mm ? 0 : 1];\n    return { id, node, material, tags: [FLOW_TAG], kind: \"polyhedron\", vertices: mesh.vertices.map(([x, y, z]) => [cx + radius * x, cy + radius * y, cz + radius * z]), triangles: mesh.triangles };"),
]
for old, new in pairs:
    if text.count(old) != 1:
        sys.exit(f"expected one match of {old[:80]!r}, found {text.count(old)}")
    text = text.replace(old, new)
path.write_text(text, encoding="utf-8")
print("ok")
