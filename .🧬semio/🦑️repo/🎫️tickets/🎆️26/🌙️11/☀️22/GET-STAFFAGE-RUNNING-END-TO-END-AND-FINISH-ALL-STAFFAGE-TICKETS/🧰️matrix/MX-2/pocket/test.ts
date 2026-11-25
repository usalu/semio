/** 🕳️ The failing-first cases of the repaired pocket rule on synthetic masks: `bun test ./test.ts` reads the trial copy `./silhouette.ts` (all pass), `MODULE=real bun test ./test.ts` reads the module of the catalog as it is (the cases marked "repaired" fail there, the cases marked "kept" pass in both). Distances are in cells (cell 1).
 * @see ./patch.py — the patch */
import { describe, expect, it } from "bun:test";

const real = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const { boundarySpread, Mask } = await import(process.env.MODULE === "real" ? real : "./silhouette.ts");

const [W, H] = [80, 80];
const POCKET = 256;
const block = (): Uint8Array => {
  const mask = new Uint8Array(W * H);
  for (let j = 10; j < 70; j++) for (let i = 10; i < 70; i++) mask[j * W + i] = 1;
  return mask;
};
const carve = (mask: Uint8Array, i0: number, i1: number, j0: number, j1: number, value = 0): Uint8Array => {
  for (let j = j0; j < j1; j++) for (let i = i0; i < i1; i++) mask[j * W + i] = value;
  return mask;
};
const farthest = (drawn: Uint8Array, body: Uint8Array): number => boundarySpread(drawn, body, undefined, W, H, 1, { i0: 0, i1: W, j0: 0, j1: H }, 200, POCKET).maxM;

describe("🕳️ one pocket decision for the drawn mask and the body mask", () => {
  it("repaired: a slit the body leaves open and the drawing closes at its mouth is outline in both (person-with-white-cane, 1:500 front)", () => {
    const body = carve(block(), 38, 42, 10, 50);
    const drawn = carve(carve(block(), 38, 42, 10, 50), 38, 42, 10, 15, 1);
    expect(farthest(drawn, body)).toBeLessThanOrEqual(5);
  });

  it("repaired: a hole at the limit in the drawing and one row above it in the body is the same hole (japanese-cherry, miscanthus-grass)", () => {
    const drawn = carve(block(), 32, 48, 32, 48);
    const body = carve(block(), 32, 48, 32, 49);
    expect(farthest(drawn, body)).toBeLessThanOrEqual(1);
    expect(farthest(body, drawn)).toBeLessThanOrEqual(1);
  });

  it("kept: a seam between overlapping drawn shapes over a covered body is not outline", () => {
    expect(farthest(carve(block(), 39, 41, 20, 50), block())).toBe(0);
  });

  it("kept: a small hollow of the body under a covered drawing is not outline", () => {
    expect(farthest(block(), carve(block(), 30, 40, 30, 40))).toBe(0);
  });

  it("kept: a pocket both masks enclose is not outline, wherever each draws its rim", () => {
    expect(farthest(carve(block(), 30, 40, 30, 40), carve(block(), 33, 44, 31, 41))).toBe(0);
  });

  it("kept: a notch of the body that the drawing fills is a difference", () => {
    expect(farthest(block(), carve(block(), 35, 45, 10, 40))).toBeGreaterThanOrEqual(20);
  });

  it("kept: a notch of the drawing that the body fills is a difference", () => {
    expect(farthest(carve(block(), 35, 45, 10, 40), block())).toBeGreaterThanOrEqual(20);
  });
});

describe("🧽️ one hairline closing for the drawn mask and the body mask", () => {
  const box = { minX: 0, minY: 0, maxX: 24, maxY: 16 };
  const rect = (x0: number, x1: number): { x: number; y: number }[] => [{ x: x0, y: 2 }, { x: x1, y: 2 }, { x: x1, y: 12 }, { x: x0, y: 12 }];
  const triangles = (ring: { x: number; y: number }[]): { x: number; y: number }[][] => [[ring[0]!, ring[1]!, ring[2]!], [ring[0]!, ring[2]!, ring[3]!]];
  const masks = (gap: number): { body: Uint8Array; drawn: Uint8Array; open: Uint8Array } => {
    const [left, right] = [rect(2, 10.5 - gap / 2), rect(10.5 + gap / 2, 20)];
    const body = new Mask(box, 1);
    for (const ring of [left, right]) for (const triangle of triangles(ring)) body.fill([triangle], true);
    body.seal();
    const drawn = new Mask(box, 1);
    const open = new Mask(box, 1);
    for (const ring of [left, right]) {
      drawn.fill([ring]);
      open.fill([ring]);
    }
    drawn.sealRings?.([[left], [right]]);
    return { body: body.cells, drawn: drawn.cells, open: open.cells };
  };

  it("repaired: a hairline of a fifth of a cell between two drawn shapes is closed as the hairline between two solids is", () => {
    const { body, drawn, open } = masks(0.2);
    expect(open[7 * 24 + 10]).toBe(0);
    expect(body[7 * 24 + 10]).toBe(1);
    expect([...drawn]).toEqual([...body]);
  });

  it("kept: a gap of more than a quarter of a cell stays open in both", () => {
    const { body, drawn, open } = masks(0.6);
    expect(body[7 * 24 + 10]).toBe(0);
    expect([...drawn]).toEqual([...open]);
  });
});
