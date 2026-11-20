import assert from "node:assert/strict";
import { Box3, Vector3 } from "three";

const law = await Bun.file(process.argv[2]!).json() as {
  stockSizeMetres: [number, number, number];
  capabilities: { id: string; maxDepthMetres: number; available: boolean }[];
};
const measured = new Box3(new Vector3(), new Vector3(...law.stockSizeMetres)).getSize(new Vector3());
for (const capability of law.capabilities) {
  assert.equal(measured.z <= capability.maxDepthMetres, capability.available, capability.id);
}
console.log("[DEBUG] Three.js stock extents match the capability availability fixture");
