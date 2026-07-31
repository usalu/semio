/** @emoji 🧪️ Pure-function proof that ortho zoom retunes LOD grid step the way perspective dolly does. */
const WORLD_LOD_REFERENCE_FOV_DEG = 50;
const halfFovTan = (fovDeg) => Math.tan((fovDeg * Math.PI) / 360);
const matchedDistance = (zoom, viewportHeight) =>
  Math.max(viewportHeight, 1) / (2 * Math.max(zoom, 1e-6) * halfFovTan(WORLD_LOD_REFERENCE_FOV_DEG));
const lodFromDistance = (distance, reference) => Math.max(distance, 1e-6) / Math.max(reference, 1e-6);
const lodGridStepWorld = (lod, gridFactor) => {
  if (!Number.isFinite(lod) || lod <= 0 || !Number.isFinite(gridFactor) || gridFactor <= 0) return null;
  const targetMultiplier = Math.max(1, lod / 2);
  const magnitude = 10 ** Math.floor(Math.log10(targetMultiplier));
  const normalized = targetMultiplier / magnitude;
  const quantum = normalized <= 1 ? 1 : normalized <= 2.5 ? 2.5 : normalized <= 5 ? 5 : 10;
  return gridFactor * quantum * magnitude;
};

const viewportHeight = 720;
const reference = 100;
const gridFactor = 10;
const rows = [50, 25, 10, 5, 2, 1].map((zoom) => {
  const dist = matchedDistance(zoom, viewportHeight);
  const lod = lodFromDistance(dist, reference);
  const step = lodGridStepWorld(lod, gridFactor);
  return { zoom, dist: Number(dist.toFixed(2)), lod: Number(lod.toFixed(3)), step };
});

console.log("[DEBUG] orthographic zoom → LOD grid step (viewportHeight=720, reference=100, gridFactor=10)");
console.table(rows);
const zoomedIn = rows[0];
const zoomedOut = rows[rows.length - 1];
if (!(zoomedOut.step > zoomedIn.step)) {
  console.error("[DEBUG] FAIL: zooming out must coarsen the grid");
  process.exit(1);
}
console.log("[DEBUG] ok: zooming out from", zoomedIn.zoom, "to", zoomedOut.zoom, "coarsens grid", zoomedIn.step, "→", zoomedOut.step);
