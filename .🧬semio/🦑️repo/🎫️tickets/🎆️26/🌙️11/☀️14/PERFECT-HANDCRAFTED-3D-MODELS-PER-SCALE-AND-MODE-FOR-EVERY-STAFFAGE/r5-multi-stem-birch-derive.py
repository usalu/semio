"""🌿 Writes the round-5 birch scripts from the round-3/4 ones: the 1:50 tresses become flat curtains of pendulous twigs (20 % wider, half thickness half the half width, finely pointed ends), the model writes to r5; coarse scales keep their capsule tresses."""

def swap(src, old, new):
    assert old in src, old
    return src.replace(old, new)

tree = open("r3-multi-stem-birch-tree.ts", encoding="utf-8").read()
tree = swap(tree, "const TRESS: [number, number][] = [[0, 0.18], [0.1, 0.7], [0.22, 0.95], [0.45, 1], [0.72, 0.84], [0.9, 0.5], [1, 0.14]];", "const TRESS: [number, number][] = [[0, 0.18], [0.1, 0.7], [0.22, 0.95], [0.45, 1], [0.72, 0.88], [0.88, 0.58], [1, 0.1]];")
tree = swap(tree, "    const loft = tress(`leaf-${key}-${i + 1}`, bone, along(owner, f), heading(owner, f) + daz, len, w, thick(w), n, owner.id);", "    const ww = scale === 50 ? CURTAIN * w : w;\n    const loft = tress(`leaf-${key}-${i + 1}`, bone, along(owner, f), heading(owner, f) + daz, len, ww, scale === 50 ? FLAT * ww : thick(w), n, owner.id);")
tree = swap(tree, "const TRESS_N = 5;", "const TRESS_N = 9;\n/** 🪟️ At 1:50 a tress is a curtain of pendulous twigs: 1.2 times as wide as its capsule at the coarser scales and half of its half width thick. */\nconst CURTAIN = 1.2, FLAT = 0.5;")
tree = swap(tree, "const SLEEVE_N: Record<number, number> = { 50: 7,", "const SLEEVE_N: Record<number, number> = { 50: 10,")
tree = tree.replace("/** 🌳️ The round-3 multi-stem birch", "/** 🌳️ The round-5 multi-stem birch", 1)
open("r5-multi-stem-birch-tree.ts", "w", encoding="utf-8").write(tree)

model = open("r3-multi-stem-birch-model.ts", encoding="utf-8").read()
model = swap(model, 'from "./r3-multi-stem-birch-tree.ts";', 'from "./r5-multi-stem-birch-tree.ts";')
model = swap(model, 'join(GENERATED, "r3-multi-stem-birch")', 'join(GENERATED, "r5-multi-stem-birch")')
model = model.replace("/** 🌳️ Round-3 multi-stem birch", "/** 🌳️ Round-5 multi-stem birch", 1)
open("r5-multi-stem-birch-model.ts", "w", encoding="utf-8").write(model)
