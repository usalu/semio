import graphlib from "graphlib";
import assert from "node:assert/strict";
import fs from "node:fs";
const root = "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/🧪️tests/🧫️fixtures/";
const fixture = JSON.parse(fs.readFileSync(root + "fill-budget.json", "utf8"));
const size = fixture.textureSize;
const graph = new graphlib.Graph({directed: false});
for (let pixel = 0; pixel < size * size; pixel++) graph.setNode(String(pixel));
for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) {
    const pixel = y * size + x;
    if (x + 1 < size) graph.setEdge(String(pixel), String(pixel + 1));
    if (y + 1 < size) graph.setEdge(String(pixel), String(pixel + size));
}
const x = Math.round(fixture.startUv[0] * (size - 1));
const y = Math.round((1 - fixture.startUv[1]) * (size - 1));
const paths = graphlib.alg.dijkstra(graph, String(y * size + x), () => 1, node => graph.nodeEdges(node));
const filled = Object.values(paths).filter(path => Number.isFinite(path.distance)).length;
assert.deepEqual({publishedMutations: 1, changedPixels: filled, channelSums: fixture.color.map(channel => channel * filled)}, fixture.expected);
assert.equal(Math.ceil(filled / fixture.turn.fuel), fixture.minimumProgressTurns);
assert.equal(filled, fixture.minimumPaidWork);
console.log("[DEBUG] Independent Graphlib fill contract: " + filled + " four-connected pixels and all RGBA sums passed.");
