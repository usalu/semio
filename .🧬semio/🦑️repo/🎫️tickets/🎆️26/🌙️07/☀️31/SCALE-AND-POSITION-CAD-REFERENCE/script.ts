#!/usr/bin/env bun
/** 🧪️ Verifies the authored CAD forest-reference scale, placement, and coordinate-system contract. */
import { readFileSync } from "node:fs";

//#region 🔖️ReferenceScale
const source = readFileSync("✏️s/🔌️plugin/📐️cad/🎛️app/📐️cad/🔨️module/⚙️engine/⚡️implementation/🦀️rust/📦️lib.rs", "utf8");
const number = (name: string): number => {
  const match = source.match(new RegExp(`${name}: f64 = ([0-9.]+)`));
  if (!match) throw new Error(`missing ${name}`);
  return Number(match[1]);
};
const width = number("CAD_FOREST_REFERENCE_WIDTH_WORLD");
const imageWidth = number("CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX");
const imageHeight = number("CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX");
const planeZ = number("CAD_FOREST_REFERENCE_PLANE_Z");
const yOffsetRatio = number("CAD_FOREST_REFERENCE_Y_OFFSET_RATIO");
const origin = [-24 + width * 0.5, -18 + (width * imageHeight) / imageWidth * (0.5 + yOffsetRatio)];
if (width !== 28.6 || planeZ !== 0.01 || yOffsetRatio !== 0.2 || Math.abs(origin[0] - -9.7) > Number.EPSILON) throw new Error(`unexpected reference transform ${JSON.stringify({ width, planeZ, yOffsetRatio, origin })}`);
const renderer = readFileSync("✏️s/🔌️plugin/📐️cad/🔨️module/📺️renderer/⚡️implementation/🟦️typescript/📦️index.tsx", "utf8");
if (!renderer.includes("CAD_WORLD_FORWARD: Vec3 = [0, 1, 0]") || !renderer.includes("CAD_WORLD_UP: Vec3 = [0, 0, 1]")) throw new Error("CAD world axes must remain Y-forward and Z-up");
if (!renderer.includes("cameraUp={CAD_WORLD_UP}") || !renderer.includes("<InteractionCadWorldCoordinateSystem />")) throw new Error("CAD canvas must apply its coordinate-system boundary");
console.log(`[DEBUG] CAD reference width=${width}, anchored XY center=(${origin[0]}, ${origin[1]}), ground Z=${planeZ}, Y-forward/Z-up enforced`);
//#endregion 🔖️ReferenceScale
