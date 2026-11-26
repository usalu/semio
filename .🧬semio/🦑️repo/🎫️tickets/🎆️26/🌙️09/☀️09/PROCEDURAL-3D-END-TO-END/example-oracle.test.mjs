/** 🧪️ Preview-oracle errors are acceptance failures even when a drawable is present. */
import { expect, test } from "bun:test";
import { Box3, Vector3 } from "three";
import { gradeMeshes, oracleAtHeight, oracleForLabel } from "./🐍️example-oracle.mjs";

const oracle = oracleForLabel("Mesh Workbench");
const bounds = { min: [-0.5, -0.5, -1], max: [0.5, 0.5, 0.5] };
const delivered = {
  meshCount: 2,
  instanceCount: 2,
  meshRoles: { solid: 2 },
  meshIds: ["eval-extrude@meshOut#0", "eval-extrude@mesh#0"],
  allBounds: bounds,
  preview: { triangles: 28, edgeSegments: 0, bounds },
};

test("the authored workbench has a required buffer and shape oracle", () => {
  expect(gradeMeshes(oracle, delivered)).toEqual({ ok: true, reasons: [] });
  expect(gradeMeshes(null, delivered).ok).toBe(false);
});

test("an otherwise drawable publication fails with a missing solid role or wrong delivery extent", () => {
  expect(gradeMeshes(oracle, { ...delivered, meshRoles: { solid: 1, wire: 1 } }).ok).toBe(false);
  expect(gradeMeshes(oracle, { ...delivered, allBounds: { min: [-0.6, -0.5, -1], max: bounds.max } }).ok).toBe(false);
});

test("an edited mushroom height must change the Z extent of the original hexagonal cross-section", () => {
  const stock = oracleForLabel("Hexagonal Mushroom Column");
  const edited = oracleAtHeight(stock, 8);
  const corners = [0, 8].flatMap((z) => Array.from({ length: 6 }, (_, index) => {
    const theta = index * Math.PI / 3;
    return new Vector3(0.5 * Math.cos(theta), 0.5 * Math.sin(theta), z);
  }));
  const box = new Box3().setFromPoints(corners);
  const bounds = { min: box.min.toArray(), max: box.max.toArray() };
  const observation = { meshCount: 3, instanceCount: 3, meshRoles: { solid: 1, vector: 1, wire: 1 }, allBounds: bounds, preview: { triangles: 20, edgeSegments: 18, bounds } };
  expect(gradeMeshes(edited, observation).ok).toBe(true);
  expect(gradeMeshes(stock, observation).ok).toBe(false);
});

test("an edited rectangle height changes its Y edge and leaves the wire in the XY plane", () => {
  const stock = oracleForLabel("Rectangle Wire Preview");
  const edited = oracleAtHeight(stock, 2.5);
  const box = new Box3().setFromPoints([new Vector3(0, 0, 0), new Vector3(2, 0, 0), new Vector3(2, 2.5, 0), new Vector3(0, 2.5, 0)]);
  const bounds = { min: box.min.toArray(), max: box.max.toArray() };
  const observation = { meshCount: 1, instanceCount: 1, meshRoles: { wire: 1 }, allBounds: bounds, preview: { triangles: 0, edgeSegments: 4, bounds } };
  expect(gradeMeshes(edited, observation).ok).toBe(true);
  expect(gradeMeshes(stock, observation).ok).toBe(false);
});
