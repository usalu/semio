#!/usr/bin/env bun
/**
 * 🧭 Logo workspace router: `bun ./script.ts generate` — builds `logo_generated.svg` from keyframe SVGs.
 */
// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

// Generates an animated SVG logo from static keyframe SVG files.

// #endregion 🧲Header

// #region 🔌Adapters
// Imports MUST use Node.js file APIs, path helpers, and DOM parsing for SVG extraction.

import * as fs from "fs";
import { JSDOM } from "jsdom";
import * as path from "path";

// #endregion 🔌Adapters

// #region ⚙️Kinds
// Kinds MUST describe the parsed SVG transform and path state for each animation frame.

/**
 * Parsed transform state for one SVG group.
 *
 * Specs: Translation, rotation, and scale are normalized from matrix and transform attributes.
 */
interface TransformData {
  translate: { x: number; y: number };
  rotate: { angle: number; cx: number; cy: number };
  scale: { x: number; y: number };
}

/**
 * Parsed SVG group state for one keyframe.
 *
 * Specs: Each group stores the first path child so animation can interpolate transforms and style.
 */
interface GroupData {
  id: string;
  transform: TransformData;
  path: {
    d: string;
    fill: string;
    stroke: string;
    strokeWidth: string;
  };
}

/**
 * Parsed SVG document state for one keyframe.
 *
 * Specs: Keyframes are represented as a flat ordered list of group states.
 */
interface KeyframeData {
  groups: GroupData[];
}

// #endregion ⚙️Kinds

// #region 🧿Logo Generation
// Logo generation MUST parse the checked-in keyframes and emit a deterministic animated SVG.

//#region 🔢Helpers
// Helper functions MUST normalize numeric output and XML attribute content.

function normalizeNumber(value: number): number {
  if (!Number.isFinite(value)) {
    return 0;
  }

  return Number(value.toFixed(6));
}

function escapeXmlAttribute(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll('"', "&quot;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

function createIdentityTransform(): TransformData {
  return {
    translate: { x: 0, y: 0 },
    rotate: { angle: 0, cx: 0, cy: 0 },
    scale: { x: 1, y: 1 },
  };
}

//#endregion 🔢Helpers

//#region 🧮Transform Parsing
// Transform parsing MUST support matrix, translate, rotate, and scale syntax from the SVG keyframes.

function transformToMatrix(
  translate: { x: number; y: number },
  rotate: { angle: number; cx: number; cy: number },
  scale: { x: number; y: number },
): string {
  const angleRadians = (rotate.angle * Math.PI) / 180;
  const cosine = Math.cos(angleRadians);
  const sine = Math.sin(angleRadians);
  const scaleX = scale.x === 0 ? 1 : scale.x;
  const scaleY = scale.y === 0 ? 1 : scale.y;

  const a = normalizeNumber(cosine * scaleX);
  const b = normalizeNumber(sine * scaleX);
  const c = normalizeNumber(-sine * scaleY);
  const d = normalizeNumber(cosine * scaleY);
  const e = normalizeNumber(
    translate.x + rotate.cx - rotate.cx * a - rotate.cy * c,
  );
  const f = normalizeNumber(
    translate.y + rotate.cy - rotate.cx * b - rotate.cy * d,
  );

  return `${a} ${b} ${c} ${d} ${e} ${f}`;
}

function parseMatrixTransform(valuesText: string): TransformData {
  const values = valuesText
    .split(/[,\s]+/)
    .filter(Boolean)
    .map(Number);

  if (values.length !== 6 || values.some((value) => !Number.isFinite(value))) {
    return createIdentityTransform();
  }

  const [a, b, c, d, e, f] = values;
  const scaleX = Math.hypot(a, b) || 1;
  const determinant = a * d - b * c;
  const scaleY = determinant === 0 ? 1 : determinant / scaleX;
  const angle = Math.atan2(b, a) * (180 / Math.PI);

  return {
    translate: { x: normalizeNumber(e), y: normalizeNumber(f) },
    rotate: { angle: normalizeNumber(angle), cx: 0, cy: 0 },
    scale: {
      x: normalizeNumber(scaleX),
      y: normalizeNumber(scaleY),
    },
  };
}

function parseTransform(transformText: string | null): TransformData {
  const result = createIdentityTransform();
  if (!transformText) {
    return result;
  }

  const matrixMatch = transformText.match(/matrix\(([^)]+)\)/);
  if (matrixMatch) {
    return parseMatrixTransform(matrixMatch[1]);
  }

  const translateMatch = transformText.match(/translate\(([^)]+)\)/);
  if (translateMatch) {
    const values = translateMatch[1]
      .split(/[,\s]+/)
      .filter(Boolean)
      .map(Number);
    result.translate.x = normalizeNumber(values[0] ?? 0);
    result.translate.y = normalizeNumber(values[1] ?? 0);
  }

  const rotateMatch = transformText.match(/rotate\(([^)]+)\)/);
  if (rotateMatch) {
    const values = rotateMatch[1]
      .split(/[,\s]+/)
      .filter(Boolean)
      .map(Number);
    result.rotate.angle = normalizeNumber(values[0] ?? 0);
    result.rotate.cx = normalizeNumber(values[1] ?? 0);
    result.rotate.cy = normalizeNumber(values[2] ?? 0);
  }

  const scaleMatch = transformText.match(/scale\(([^)]+)\)/);
  if (scaleMatch) {
    const values = scaleMatch[1]
      .split(/[,\s]+/)
      .filter(Boolean)
      .map(Number);
    result.scale.x = normalizeNumber(values[0] ?? 1) || 1;
    result.scale.y = normalizeNumber(values[1] ?? values[0] ?? 1) || 1;
  }

  return result;
}

//#endregion 🧮Transform Parsing

//#region 🎈SVG Parsing
// SVG parsing MUST read the checked-in logo keyframes and preserve group identity across frames.

function parseSVGFile(filePath: string): KeyframeData {
  const svgContent = fs.readFileSync(filePath, "utf-8");
  const dom = new JSDOM(svgContent, { contentType: "image/svg+xml" });
  const document = dom.window.document;
  const groups: GroupData[] = [];

  for (const groupElement of document.querySelectorAll("g[id]")) {
    const id = groupElement.getAttribute("id");
    const pathElement = groupElement.querySelector("path");

    if (!id || !pathElement) {
      continue;
    }

    groups.push({
      id,
      transform: parseTransform(groupElement.getAttribute("transform")),
      path: {
        d: pathElement.getAttribute("d") ?? "",
        fill: pathElement.getAttribute("fill") ?? "none",
        stroke: pathElement.getAttribute("stroke") ?? "none",
        strokeWidth: pathElement.getAttribute("stroke-width") ?? "0",
      },
    });
  }

  return { groups };
}

//#endregion 🎈SVG Parsing

//#region 📮Sequence Generation
// Sequence generation MUST create a palindromic loop with repeated hold frames.

function generateKeyframeSequence(keyframes: KeyframeData[]): KeyframeData[] {
  if (keyframes.length <= 1) {
    return [...keyframes];
  }

  const sequence: KeyframeData[] = [];

  for (const keyframe of keyframes) {
    sequence.push(keyframe, keyframe, keyframe);
  }

  for (let index = keyframes.length - 2; index > 0; index -= 1) {
    sequence.push(keyframes[index], keyframes[index]);
  }

  sequence.push(keyframes[0]);
  return sequence;
}

//#endregion 📮Sequence Generation

//#region 📻Animated SVG Output
// Animated SVG output MUST emit one animated path per group with stable timing and transforms.

function createAnimatedSVG(keyframes: KeyframeData[], outputPath: string): void {
  const sequence = generateKeyframeSequence(keyframes);
  if (sequence.length === 0) {
    throw new Error("Cannot create animated SVG without keyframes.");
  }

  const totalFrames = sequence.length;
  const totalDurationSeconds = Math.max(keyframes.length * 4, 1);
  const keyTimes = sequence.map((_, index) =>
    normalizeNumber(index / Math.max(totalFrames - 1, 1)).toString(),
  );
  const keyTimesText = keyTimes.join(";");
  const keySplinesText = Array.from({ length: Math.max(totalFrames - 1, 1) }, (_, index) => {
    const currentFrame = JSON.stringify(sequence[index]);
    const nextFrame = JSON.stringify(sequence[Math.min(index + 1, totalFrames - 1)]);
    return currentFrame === nextFrame ? "0 0 1 1" : "0.25 0.1 0.75 0.9";
  }).join(";");

  const allGroupIds = new Set<string>();
  for (const keyframe of keyframes) {
    for (const group of keyframe.groups) {
      allGroupIds.add(group.id);
    }
  }

  const lines: string[] = [
    '<?xml version="1.0" encoding="UTF-8" standalone="no"?>',
    '<svg viewBox="0 0 410 140" style="background: #001117;" version="1.1" xmlns="http://www.w3.org/2000/svg">',
    "  <title>semio</title>",
    '  <rect id="background" width="100%" height="100%" fill="#001117" />',
  ];

  for (const groupId of allGroupIds) {
    const groupFrames = sequence.map((keyframe) => keyframe.groups.find((group) => group.id === groupId) ?? null);
    const firstGroup = groupFrames.find((group): group is GroupData => group !== null);

    if (!firstGroup) {
      continue;
    }

    const matrixValues = groupFrames
      .map((group) => transformToMatrix(
        group?.transform.translate ?? firstGroup.transform.translate,
        group?.transform.rotate ?? firstGroup.transform.rotate,
        group?.transform.scale ?? firstGroup.transform.scale,
      ))
      .join(";");
    const fillValues = groupFrames.map((group) => group?.path.fill ?? firstGroup.path.fill).join(";");
    const strokeValues = groupFrames.map((group) => group?.path.stroke ?? firstGroup.path.stroke).join(";");
    const strokeWidthValues = groupFrames
      .map((group) => group?.path.strokeWidth ?? firstGroup.path.strokeWidth)
      .join(";");

    lines.push(`  <g id="${escapeXmlAttribute(groupId)}">`);
    lines.push(
      `    <path d="${escapeXmlAttribute(firstGroup.path.d)}" fill="${escapeXmlAttribute(firstGroup.path.fill)}" stroke="${escapeXmlAttribute(firstGroup.path.stroke)}" stroke-width="${escapeXmlAttribute(firstGroup.path.strokeWidth)}">`,
    );
    lines.push(
      `      <animateTransform attributeName="transform" type="matrix" dur="${totalDurationSeconds}s" repeatCount="indefinite" keyTimes="${keyTimesText}" values="${matrixValues}" calcMode="spline" keySplines="${keySplinesText}" />`,
    );
    lines.push(
      `      <animate attributeName="fill" dur="${totalDurationSeconds}s" repeatCount="indefinite" keyTimes="${keyTimesText}" values="${fillValues}" calcMode="spline" keySplines="${keySplinesText}" />`,
    );
    lines.push(
      `      <animate attributeName="stroke" dur="${totalDurationSeconds}s" repeatCount="indefinite" keyTimes="${keyTimesText}" values="${strokeValues}" calcMode="spline" keySplines="${keySplinesText}" />`,
    );
    lines.push(
      `      <animate attributeName="stroke-width" dur="${totalDurationSeconds}s" repeatCount="indefinite" keyTimes="${keyTimesText}" values="${strokeWidthValues}" calcMode="spline" keySplines="${keySplinesText}" />`,
    );
    lines.push("    </path>");
    lines.push("  </g>");
  }

  lines.push("</svg>");
  fs.writeFileSync(outputPath, `${lines.join("\n")}\n`);
  console.log(`Animated SVG created: ${outputPath}`);
}

//#endregion 📻Animated SVG Output

//#region 🚀Entrypoint
// Entrypoint MUST scan the local logo keyframe files and regenerate the checked-in animated logo asset.

function runLogoGenerate(): void {
  const logoDir = import.meta.dir;
  const keyframes: KeyframeData[] = [];

  for (let index = 1; index <= 6; index += 1) {
    const filePath = path.join(logoDir, `logo_${index}.svg`);
    if (fs.existsSync(filePath)) {
      console.log(`Parsing ${filePath}...`);
      keyframes.push(parseSVGFile(filePath));
    }
  }

  if (keyframes.length === 0) {
    throw new Error("No logo keyframe SVG files were found.");
  }

  console.log(`Found ${keyframes.length} keyframes`);
  console.log(`Will generate ${generateKeyframeSequence(keyframes).length} animation frames`);
  createAnimatedSVG(keyframes, path.join(logoDir, "logo_generated.svg"));
}

export { createAnimatedSVG, generateKeyframeSequence, parseSVGFile };

const segs = process.argv.slice(2);
if (segs[0] !== "generate") {
  console.error("usage: bun ./script.ts generate");
  process.exit(1);
}
runLogoGenerate();

//#endregion 🚀Entrypoint

// #endregion 🧿Logo Generation
