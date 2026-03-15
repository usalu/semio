// #region 🔖Header
// [👤semio🏪assets🛅logo💻logo](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#!/usr/bin/env node

// Generates animated SVG logo from static SVG input with keyframe sequences.

// #endregion 🔖Header

//#region 🔖Imports
// [👤semio🏪assets🛅logo💻logo🔖imports](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Imports)
// MUST import Node.js file system, DOM parsing, and path resolution modules.
import * as fs from "fs";
import { JSDOM } from "jsdom";
import * as path from "path";
//#endregion 🔖Imports

//#region 🔖Types
// Types MUST provide the types functionality.
// [👤semio🏪assets🛅logo💻logo🔖types](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Types)
/**
// TransformData holds the data fields for a TransformData record.
 * [👤semio🏪assets🛅logo💻logo🔖types✂️transformdata](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Types/d/i/TransformData)
 **/
interface TransformData {
  translate: { x: number; y: number };
  rotate: { angle: number; cx: number; cy: number };
  scale: { x: number; y: number };
}

/**
 * GroupData holds the data fields for a GroupData record.
 * [👤semio🏪assets🛅logo💻logo🔖types✂️groupdata](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Types/d/i/GroupData)
 **/
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

* KeyframeData holds the data fields for a KeyframeData record.
 * [👤semio🏪assets🛅logo💻logo🔖types✂️keyframedata](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Types/d/i/KeyframeData)
 **/
interface KeyframeData {
  groups: GroupData[];
}
//#endregion 🔖Types

//#region 🔖Logo Generation
// Logo Generation MUST provide the logo generation functionality.
// [👤semio🏪assets🛅logo💻logo🔖logogeneration](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Logo%20Generation)

/**
 * [👤semio🏪assets🛅logo💻logo🔖logogeneration](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Logo%20Generation)
// [👤semio🏪assets🛅logo💻logo🔖logogeneration🛠️transformtomatrix](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Logo%20Generation/d/i/transformToMatrix)
 * Functions for parsing SVG files and generating animated SVG logos.
 **/
function transformToMatrix(translate: { x: number; y: number }, rotate: { angle: number; cx: number; cy: number }, scale: { x: number; y: number }): string {
    const tx = translate.x;
    const ty = translate.y;
    const angle = (rotate.angle * Math.PI) / 180;
    const cx = rotate.cx;
    const cy = rotate.cy;
    const sx = scale.x === 0 ? 1 : scale.x;
    const sy = scale.y === 0 ? 1 : scale.y;

    let a = 1,
      b = 0,
      c = 0,
      d = 1,
      e = 0,
      f = 0;

    e += tx;
    f += ty;

    if (angle !== 0) {
      e -= cx;
      f -= cy;

      const cos_a = Math.cos(angle);
      const sin_a = Math.sin(angle);
      const new_a = a * cos_a - b * sin_a;
      const new_b = a * sin_a + b * cos_a;
      const new_c = c * cos_a - d * sin_a;
      const new_d = c * sin_a + d * cos_a;
      const new_e = e * cos_a - f * sin_a;
      const new_f = e * sin_a + f * cos_a;

      a = new_a;
      b = new_b;
      c = new_c;
      d = new_d;
      e = new_e;
      f = new_f;

      e += cx;
      f += cy;
    }

    a *= sx;
    b *= sx;
    c *= sy;
    d *= sy;

    return `${a} ${b} ${c} ${d} ${e} ${f}`;
  }

  // [👤semio🏪assets🛅logo💻logo🔖logogeneration🛠️parsetransform](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Logo%20Generation/d/i/parseTransform)
  * [👤semio🏪assets🛅logo💻logo🔖logogeneration🪨parsetransform](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Logo%20Generation/d/i/parseTransform)
 * parseTransform holds the data fields for a parseTransform record.
 **/
function parseTransform(transformStr: string): TransformData {
        const result: TransformData = {
          translate: { x: 0, y: 0 },
          rotate: { angle: 0, cx: 0, cy: 0 },
          scale: { x: 1, y: 1 },
        };

        if (!transformStr) return result;

        const matrixMatch = transformStr.match(/matrix\(([^)]+)\)/);
        if (matrixMatch) {
          const values = matrixMatch[1].split(/[,\s]+/).map(Number);
          if (values.length === 6) {
            const [a, b, c, d, e, f] = values;

            result.translate.x = e;
            result.translate.y = f;

            const det = a * d - b * c;
            const hasReflection = det < 0;

            const scaleXMag = Math.sqrt(a * a + b * b);
            const scaleYMag = Math.sqrt(c * c + d * d);

            let rotation = Math.atan2(b, a) * (180 / Math.PI);

            if (Math.abs(a) === 1 && b === 0 && c === 0 && Math.abs(d) === 1) {
              result.scale.x = a;
              result.scale.y = d;
              result.rotate.angle = 0;
            } else if (a === 0 && Math.abs(b) >= 1 && Math.abs(c) >= 1 && d === 0) {
              const bSign = Math.sign(b);
              const cSign = Math.sign(c);
              const scaleValueB = Math.abs(b);
              const scaleValueC = Math.abs(c);

              if (bSign === -1 && cSign === -1) {
                result.scale.x = -scaleValueB;
                result.scale.y = scaleValueC;
                result.rotate.angle = 90;
              } else if (bSign === 1 && cSign === -1) {
                result.scale.x = scaleValueB;
                result.scale.y = scaleValueC;
                result.rotate.angle = -90;
              } else if (bSign === -1 && cSign === 1) {
                result.scale.x = scaleValueB;
                result.scale.y = scaleValueC;
                result.rotate.angle = 90;
              } else {
                result.scale.x = scaleValueB;
                result.scale.y = -scaleValueC;
                result.rotate.angle = -90;
              }
            } else {
              if (hasReflection) {
                result.scale.x = Math.sign(a) * scaleXMag;
                result.scale.y = Math.sign(d) * scaleYMag;
                result.rotate.angle = rotation;
              } else {
                result.scale.x = scaleXMag;
                result.scale.y = scaleYMag;
                result.rotate.angle = rotation;
              }
            }

            if (result.scale.x === 0) result.scale.x = 1;
            if (result.scale.y === 0) result.scale.y = 1;
          }
          return result;
        }

        const translateMatch = transformStr.match(/translate\(([^)]+)\)/);
        if (translateMatch) {
          const values = translateMatch[1].split(/[,\s]+/).map(Number);
          result.translate.x = values[0] || 0;
          result.translate.y = values[1] || 0;
        }

        const rotateMatch = transformStr.match(/rotate\(([^)]+)\)/);
        if (rotateMatch) {
          const values = rotateMatch[1].split(/[,\s]+/).map(Number);
          result.rotate.angle = values[0] || 0;
          result.rotate.cx = values[1] || 0;

          nsformStr.match(/scale\(([^)]+)\)/);
          const values = scaleMatch[1 values[0] || 1;
          const scaleY = values[1] || values[0] || 1;

          result.scale.x = scaleX === 0 ? 1 : scaleX;
          result.scale.y = scaleY === 0 ? 1 : scaleY;
        }

        return result;
      }

//#region 🔖Parse SVG
// [👤semio🏪assets🛅logo💻logo🔖logogeneration🔖parsesvg](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Logo%20Generation/s/Parse%20SVG)
// MUST read SVG content and extract all group transforms and path attributes.
// Parses an SVG file and returns keyframe data with group transforms and paths.
function parseSVGFile(filePath: string): KeyframeData {
  const svgContent = fs.readFileSync(filePath, "utf-8");
  const dom = new JSDOM(svgContent, { contentType: "text/xml" });
  const document = dom.window.document;

  const groups: GroupData[] = [];
  const gElements = document.querySelectorAll("g[id]");

  gElements.forEach((g) => {
    const id = g.getAttribute("id")!;
    const transformStr = g.getAttribute("transform") || "";
    const pathElement = g.querySelector("path");

    if (pathElement) {
      const transform = parseTransform(transformStr);
      const groupData: GroupData = {
        id,
        transform,
        path: {
          d: pathElement.getAttribute("d") || "",
          stroke: pathElement.getAttribute("stroke") || "none",
          strokeWidth: pathElement.getAttribute("stroke-width") || "0",
        };

        groups.push(groupData);
      }
    });

  return { groups };
}
//#endregion 🔖Parse SVG

//#region 🔖Generate Keyframe Sequence
// [👤semio🏪assets🛅logo💻logo🔖logogeneration🔖generatekeyframesequence](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Logo%20Generation/s/Generate%20Keyframe%20Sequence)
// MUST produce forward and reverse sequence for smooth animation looping.
// Generates a palindromic keyframe sequence with triple repetition per frame.
function generateKeyframeSequence(keyframes: KeyframeData[]): KeyframeData[] {
  const sequence: KeyframeData[] = [];

  for (let i = 0; i < keyframes.length; i++) {
    sequence.push(keyframes[i]);
    sequence.push(keyframes[i]);
    sequence.push(keyframes[i]);

    for (let i = keyframes.length - 2; i > 0; i--) {
      sequence.push(keyframes[i]);
      sequence.push(keyframes[i]);
    }

    sequence.push(keyframes[0]);

    return sequence;
  }
  //#endregion 🔖Generate Keyframe Sequence

  //#region 🔖Create Animated SVG
  // [👤semio🏪assets🛅logo💻logo🔖logogeneration🔖createanimatedsvg](semiorepo://p/u/semio/b/a/assets/fd/req/logo/f/logo.ts/s/Logo%20Generation/s/Create%20Animated%20SVG)
  // MUST generate translate, rotate, scale, fill, stroke, and stroke-width animations.
  // Creates an animated SVG file with SMIL animations from keyframe data.
  function createAnimatedSVG(keyframes: KeyframeData[], outputPath: string): void {
    const sequence = generateKeyframeSequence(keyframes);
    const totalFrames = sequence.length;

    const transitionDuration = 0.5;
    const holdDuration = 1.5;
    const totalDuration = keyframes.length * (transitionDuration + holdDuration) * 2;

    const keyTimes: string[] = [];
    let currentTime = 0;
    const timeStep = 1 / (totalFrames - 1);

    for (let i = 0; i < totalFrames; i++) {
      keyTimes.push((i * timeStep).toFixed(3));
    }
    const keyTimesStr = keyTimes.join(";");

    const keySplines: string[] = [];
    for (let i = 0; i < totalFrames - 1; i++) {
      const currentFrame = sequence[i];
      const nextFrame = sequence[i + 1];
      const isSameFrame = JSON.stringify(currentFrame) === JSON.stringify(nextFrame);

      if (isSameFrame) {
        keySplines.push("0 0 1 1");
      } else {
        keySplines.push("0.25 0.1 0.75 0.9");
      }
    }
    const keySplinesStr = keySplines.join(";");

    const allGroupIds = new Set<string>();
    keyframes.forEach((kf) => kf.groups.forEach((g) => allGroupIds.add(g.id)));

    let svgContent = `<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg viewBox="0 0 410 140" style="background: #001117;" version="1.1" xmlns="http://www.w3.org/2000/svg">
    <title>semio</title>
    <rect id="background" width="100%" height="100%" fill="#001117" />
`;

    Array.from(allGroupIds).forEach((groupId) => {
      const groupFrames = sequence.map((kf) => {
        const group = kf.groups.find((g) => g.id === groupId);
        return group || null;
      });

      if (groupFrames.every((gf) => gf === null)) return;

      const firstGroup = groupFrames.find((gf) => gf !== null);
      if (!firstGroup) return;

      svgContent += `    <g id="${groupId}">
`;

      const translateValues = groupFrames
        .map((gf) => {
          if (gf) {
            return `${gf.transform.translate.x} ${gf.transform.translate.y}`;
          }
          return `${firstGroup.transform.translate.x} ${firstGroup.transform.translate.y}`;
        })
        .join(";");

      svgContent += `        <animateTransform attributeName="transform" type="translate" dur="${totalDuration}s" repeatCount="indefinite"
            keyTimes="${keyTimesStr}" values="${translateValues}" calcMode="spline" keySplines="${keySplinesStr}" />
`;

      const rotateValues = groupFrames
        .map((gf) => {
          if (gf) {
            return `${gf.transform.rotate.angle} ${gf.transform.rotate.cx} ${gf.transform.rotate.cy}`;
          }
          return `${firstGroup.transform.rotate.angle} ${firstGroup.transform.rotate.cx} ${firstGroup.transform.rotate.cy}`;
        })
        .join(";");

      svgContent += `        <animateTransform attributeName="transform" type="rotate" additive="sum" dur="${totalDuration}s" repeatCount="indefinite"
            keyTimes="${keyTimesStr}" values="${rotateValues}" calcMode="spline" keySplines="${keySplinesStr}" />
`;

      const scaleValues = groupFrames
        .map((gf) => {
          if (gf) {
            const scaleX = gf.transform.scale.x === 0 ? 1 : gf.transform.scale.x;
            const scaleY = gf.transform.scale.y === 0 ? 1 : gf.transform.scale.y;
            return `${scaleX} ${scaleY}`;
          }

          return `1 1`;
        })
        .join(";");

      svgContent += `        <animateTransform attributeName="transform" type="scale" additive="sum" dur="${totalDuration}s" repeatCount="indefinite"
            keyTimes="${keyTimesStr}" values="${scaleValues}" calcMode="spline" keySplines="${keySplinesStr}" />
`;

      const fillValues = groupFrames
        .map((gf) => {
          return gf ? gf.path.fill : firstGroup.path.fill;
        })
        .join(";");

      const strokeValues = groupFrames
        .map((gf) => {
          return gf ? gf.path.stroke : firstGroup.path.stroke;
        })
        .join(";");

      const strokeWidthValues = groupFrames
        .map((gf) => {
          return gf ? gf.path.strokeWidth : firstGroup.path.strokeWidth;
        })
        .join(";");

      svgContent += `        <path d="${firstGroup.path.d}">
            <animate attributeName="fill" dur="${totalDuration}s" repeatCount="indefinite" keyTimes="${keyTimesStr}"
                values="${fillValues}" calcMode="spline" keySplines="${keySplinesStr}" />
            <animate attributeName="stroke" dur="${totalDuration}s" repeatCount="indefinite" keyTimes="${keyTimesStr}"
                values="${strokeValues}" calcMode="spline" keySplines="${keySplinesStr}" />
            <animate attributeName="stroke-width" dur="${totalDuration}s" repeatCount="indefinite" keyTimes="${keyTimesStr}"
                values="${strokeWidthValues}" calcMode="spline" keySplines="${keySplinesStr}" />
        </path>
    </g>
`;
    });

    svgContent += `</svg>`;

    fs.writeFileSync(outputPath, svgContent);
    console.log(`Animated SVG created: ${outputPath}`);
  }
  //#endregion 🔖Create Animated SVG

  function main(): void {
    const logoDir = path.dirname(__filename);

    const keyframes: KeyframeData[] = [];
    for (let i = 1; i <= 6; i++) {
      const filePath = path.join(logoDir, `logo_${i}.svg`);
      if (fs.existsSync(filePath)) {
        console.log(`Parsing ${filePath}...`);
        keyframes.push(parseSVGFile(filePath));
      } else {
        console.warn(`Warning: ${filePath} not found`);
      }
    }

    if (keyframes.length === 0) {
      console.error("No keyframe files found!");
      process.exit(1);
    }

    console.log(`Found ${keyframes.length} keyframes`);
    console.log(`Will generate ${generateKeyframeSequence(keyframes).length} animation frames`);

    const outputPath = path.join(logoDir, "logo_generated.svg");
    createAnimatedSVG(keyframes, outputPath);
  }

  if (require.main === module) {
    main();
  }

  export { createAnimatedSVG, generateKeyframeSequence, parseSVGFile };
//#endregion 🔖Logo Generation
