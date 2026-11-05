import graphlib from "graphlib";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import {Matrix4, Quaternion, Euler, Vector3} from "three";

const base = path.resolve("✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧫️fixtures");
const exportFixture = JSON.parse(fs.readFileSync(path.join(base, "managed-export.json"), "utf8"));
const transform = exportFixture.transform;
const rotation = new Quaternion().setFromEuler(new Euler(...transform.rotation.map(value => value * Math.PI / 180)));
const world = new Matrix4().compose(new Vector3(...transform.position), rotation, new Vector3(...transform.scale));
const check = (actual, expected) => actual.forEach((point, index) => point.forEach((value, axis) => assert.ok(Math.abs(value - expected[index][axis]) < 1e-5)));
check(exportFixture.positions.map(point => new Vector3(...point).applyMatrix4(world).toArray()), exportFixture.expectedPositions);
const reflected = new Matrix4().compose(new Vector3(...transform.position), rotation, new Vector3(...exportFixture.reflected.scale));
check(exportFixture.positions.map(point => new Vector3(...point).applyMatrix4(reflected).toArray()), exportFixture.reflected.expectedPositions);
for (const scale of [exportFixture.reflected.scale, exportFixture.tinyReflectedScale]) {
    const matrix = new Matrix4().compose(new Vector3(...transform.position), rotation, new Vector3(...scale));
    const indices = [...exportFixture.expectedIndices];
    if (matrix.determinant() < 0) indices.reverse();
    assert.deepEqual(indices, exportFixture.reflected.expectedIndices);
}
const scene = JSON.parse(fs.readFileSync(path.join(base, "gltf-scene.json"), "utf8"));
const parent = new Matrix4().makeTranslation(...scene.rootTranslation);
const transforms = [
    new Matrix4().compose(new Vector3(...scene.childTranslation), new Quaternion(...scene.childRotation), new Vector3(...scene.childScale)),
    new Matrix4().fromArray(scene.matrix),
    new Matrix4().makeScale(...scene.reflectedScale),
];
transforms.forEach((local, index) => {
    const matrix = parent.clone().multiply(local);
    check(scene.positions.map(point => new Vector3(...point).applyMatrix4(matrix).toArray()), scene.expectedPositions[index]);
    const face = [...scene.faces[0]];
    if (matrix.determinant() < 0) face.reverse();
    assert.deepEqual(face, scene.expectedFaces[index]);
});
console.log("[DEBUG] Three.js independently validated managed export coordinates and three scene instance coordinates/winding.");

const forest = JSON.parse(fs.readFileSync(path.join(base, "gltf-forest.json"), "utf8"));
for (const sample of forest.cases) {
    const graph = new graphlib.Graph({directed: true, multigraph: true});
    sample.nodes.forEach((node, index) => graph.setNode(String(index)));
    let valid = sample.nodes.every(node => node.mesh === undefined || Number.isInteger(node.mesh) && node.mesh >= 0 && node.mesh < (sample.meshCount ?? forest.meshCount));
    sample.nodes.forEach((node, parent) => (node.children ?? []).forEach((child, ordinal) => {
        if (!Number.isInteger(child) || child < 0 || child >= sample.nodes.length) valid = false;
        else graph.setEdge(String(parent), String(child), undefined, String(ordinal));
    }));
    valid &&= graphlib.alg.isAcyclic(graph) && graph.nodes().every(node => graph.inEdges(node).length <= 1);
    valid &&= sample.scenes.every(roots => new Set(roots).size === roots.length && roots.every(root => graph.hasNode(String(root)) && graph.inEdges(String(root)).length === 0));
    valid &&= sample.scene === null || Number.isInteger(sample.scene) && sample.scene >= 0 && sample.scene < sample.scenes.length;
    assert.equal(valid, sample.valid, sample.id);
    if (valid) {
        const roots = sample.scenes.length ? sample.scenes[sample.scene ?? 0].map(String) : graph.sources();
        const nodes = graphlib.alg.preorder(graph, roots);
        const referenced = new Set(sample.nodes.flatMap(node => node.mesh === undefined ? [] : [node.mesh]));
        const instances = nodes.filter(node => sample.nodes[Number(node)].mesh !== undefined).length;
        const definitions = sample.scenes.length ? 0 : (sample.meshCount ?? forest.meshCount) - referenced.size;
        assert.equal(instances + definitions, sample.objects, sample.id + " geometry completeness");
        if (sample.expectedObjectPositions) {
            const matrix = new Matrix4().makeTranslation(...sample.nodes[0].translation);
            check(forest.positions.map(point => new Vector3(...point).applyMatrix4(matrix).toArray()), sample.expectedObjectPositions[0]);
            check(forest.positions, sample.expectedObjectPositions[1]);
        }
    }
}
console.log("[DEBUG] Graphlib independently validated all " + forest.cases.length + " complete node-forest admission cases.");
