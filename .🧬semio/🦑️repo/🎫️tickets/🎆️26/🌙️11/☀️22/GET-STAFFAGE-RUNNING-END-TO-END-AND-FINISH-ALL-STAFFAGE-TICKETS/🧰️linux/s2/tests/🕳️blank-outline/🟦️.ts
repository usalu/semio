/** 🕳️ Blank outline: a shape drawn as an unfilled outline counts as drawn in the `blank-shapes` test of the matrix, a shape without ink does not.
 *
 * THE REFERENCE is sharp (libvips with librsvg): the ink of the path items of the shapes in the written SVG.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🕳️blank-outline/🔣️.json — the vectors
 * @see ../🔭️multiview-coverage/🧰️.ts — `drawnIds`, `blankReason`
 */
import sharp from "sharp";
import { describe, expect, it } from "vitest";
import type { PlanarViewpoint } from "../../🧬️schema/🟦️.ts";
import type { DrawingScene, View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { resolveViews } from "../../🔨️modules/🔭️multiview/🟦️.ts";
import { blankReason, drawOptions, drawnIds, loadAsset, paperIds, sceneBox } from "../🔭️multiview-coverage/🧰️.ts";
import { fixture } from "../🧰️support/🧰️.ts";

type Vectors = { asset: string; symbol: string; penMm: number; pixelsPerMm: number; cases: { scale: number; view: PlanarViewpoint; mode: View }[] };
const vectors = fixture<Vectors>("🕳️blank-outline");
const asset = loadAsset(vectors.asset)!;

/** 🖨️ The pixels librsvg puts ink on for the path items `ids` of a scene alone. */
async function inkPixels(scene: DrawingScene, ids: ReadonlySet<string>): Promise<number> {
  const only: DrawingScene = { ...scene, background: null, legends: [], layers: scene.layers.map((layer) => ({ ...layer, items: layer.items.filter((item) => item.kind === "path" && ids.has(item.id)) })) } as DrawingScene;
  const { data } = await sharp(Buffer.from(writeSvg(only)), { density: vectors.pixelsPerMm * 25.4 }).ensureAlpha().extractChannel("alpha").raw().toBuffer({ resolveWithObject: true });
  return data.reduce((count, alpha) => count + (alpha > 0 ? 1 : 0), 0);
}

const inkless = (scene: DrawingScene, ids: ReadonlySet<string>): DrawingScene => ({ ...scene, layers: scene.layers.map((layer) => ({ ...layer, items: layer.items.map((item) => (item.kind === "path" && ids.has(item.id) && item.stroke !== undefined ? { ...item, stroke: { ...item.stroke, widthMm: 0 } } : item)) })) }) as DrawingScene;

describe("🕳️ a symbol drawn as an unfilled outline", () => {
  vectors.cases.forEach(({ scale, view, mode }) =>
    it(`is drawn where librsvg puts ink: 1:${scale} ${view} ${mode}`, { timeout: 300_000 }, async () => {
      const multi = resolveViews(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "symbol", symbol: vectors.symbol, denominator: scale }, mode, drawOptions(mode, scale));
      const drawing = multi.views[view];
      if (drawing.kind === "unavailable") throw new Error(drawing.reason.en);
      const shapes = drawing.state.shapes[view];
      const ids = paperIds(drawing.scene, shapes);
      expect(ids.size, "path items of the shapes").toBeGreaterThan(0);
      expect([...ids].every((id) => id.startsWith("outline:")), "the symbol has no fill").toBe(true);
      expect(await inkPixels(drawing.scene, ids), "librsvg ink").toBeGreaterThan(0);
      expect(sceneBox(drawing.scene, drawnIds(drawing.scene, shapes)), "drawn").toBeDefined();
      const without = inkless(drawing.scene, ids);
      expect(await inkPixels(without, ids), "librsvg ink without pen").toBe(0);
      expect(sceneBox(without, drawnIds(without, shapes)), "drawn without pen").toBeUndefined();
      expect(blankReason(drawing.state, view, mode)).toContain(`pen ${vectors.penMm} mm`);
    }),
  );
});
