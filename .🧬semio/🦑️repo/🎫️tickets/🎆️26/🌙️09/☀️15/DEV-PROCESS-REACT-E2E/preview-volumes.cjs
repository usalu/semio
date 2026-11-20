const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { Box3, Vector3 } = require("three");

const fixture = JSON.parse(readFileSync(process.argv[2], "utf8"));

function box({ width, depth, height, x = 0 }) {
    const half = new Vector3(width / 2, depth / 2, height / 2);
    const center = new Vector3(x, 0, 0);
    return new Box3(center.clone().sub(half), center.clone().add(half));
}

function volume(solid) {
    const size = solid.getSize(new Vector3());
    return size.x * size.y * size.z;
}

function subtractRightSlab(solid, tool) {
    if (!solid.intersectsBox(tool)) return solid.clone();
    if (tool.containsBox(solid)) return solid.clone().makeEmpty();
    assert(tool.max.x >= solid.max.x);
    for (const axis of ["y", "z"]) {
        assert(tool.min[axis] <= solid.min[axis] && tool.max[axis] >= solid.max[axis]);
    }
    const kept = solid.clone();
    kept.max.x = tool.min.x;
    return kept;
}

const stock = box(fixture.stock);
const cut = subtractRightSlab(stock, box(fixture.cut));
const edited = subtractRightSlab(stock, box({ ...fixture.cut, width: fixture.cut.editedWidth }));
const empty = subtractRightSlab(edited, box(fixture.completeCut));
const emptyCut = subtractRightSlab(empty, box(fixture.cut));
const restored = emptyCut.clone().union(box(fixture.restore));
const actual = {
    stock: volume(stock),
    tinyStock: volume(box(fixture.tinyStock)),
    cut: volume(cut),
    editedCut: volume(edited),
    completeCut: volume(empty),
    emptyCut: volume(emptyCut),
    restored: volume(restored),
};
assert.deepEqual(Object.fromEntries(Object.entries(actual).map(([key, value]) => [key, key === "tinyStock" ? value.toExponential(4) : value.toFixed(4)])), fixture.expected);
console.log("[DEBUG] Three.js fixture volumes verified", actual);
