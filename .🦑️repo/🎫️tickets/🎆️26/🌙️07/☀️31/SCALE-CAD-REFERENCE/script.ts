#!/usr/bin/env bun
/** 🧪️ Verifies the authored CAD forest-reference scale and its anchored placement contract. */
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
const origin = [-24 + width * 0.5, -18 + (width * imageHeight) / imageWidth * 0.5];
if (width !== 28.6 || Math.abs(origin[0] - -9.7) > Number.EPSILON) throw new Error(`unexpected reference scale ${JSON.stringify({ width, origin })}`);
console.log(`[DEBUG] CAD reference width=${width}, anchored center=(${origin[0]}, ${origin[1]})`);
//#endregion 🔖️ReferenceScale
