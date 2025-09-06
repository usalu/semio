#!/usr/bin/env node

import * as fs from 'fs';
import { JSDOM } from 'jsdom';
import * as path from 'path';

interface TransformData {
  translate: { x: number; y: number };
  rotate: { angle: number; cx: number; cy: number };
  scale: { x: number; y: number };
}

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

interface KeyframeData {
  groups: GroupData[];
}

/**
 * Convert transform values to a 2D matrix
 */
function transformToMatrix(translate: { x: number, y: number }, rotate: { angle: number, cx: number, cy: number }, scale: { x: number, y: number }): string {
  const tx = translate.x;
  const ty = translate.y;
  const angle = rotate.angle * Math.PI / 180; // Convert to radians
  const cx = rotate.cx;
  const cy = rotate.cy;
  const sx = scale.x === 0 ? 1 : scale.x;
  const sy = scale.y === 0 ? 1 : scale.y;

  // Start with identity matrix
  let a = 1, b = 0, c = 0, d = 1, e = 0, f = 0;

  // Apply translation
  e += tx;
  f += ty;

  // Apply rotation around point (cx, cy)
  if (angle !== 0) {
    // Translate to rotation center
    e -= cx;
    f -= cy;

    // Apply rotation
    const cos_a = Math.cos(angle);
    const sin_a = Math.sin(angle);
    const new_a = a * cos_a - b * sin_a;
    const new_b = a * sin_a + b * cos_a;
    const new_c = c * cos_a - d * sin_a;
    const new_d = c * sin_a + d * cos_a;
    const new_e = e * cos_a - f * sin_a;
    const new_f = e * sin_a + f * cos_a;

    a = new_a; b = new_b; c = new_c; d = new_d; e = new_e; f = new_f;

    // Translate back from rotation center
    e += cx;
    f += cy;
  }

  // Apply scale
  a *= sx;
  b *= sx;
  c *= sy;
  d *= sy;

  return `${a} ${b} ${c} ${d} ${e} ${f}`;
}

/**
 * Parse transform string (matrix or individual transforms) into structured data
 */
function parseTransform(transformStr: string): TransformData {
  const result: TransformData = {
    translate: { x: 0, y: 0 },
    rotate: { angle: 0, cx: 0, cy: 0 },
    scale: { x: 1, y: 1 }
  };

  if (!transformStr) return result;

  // Handle matrix transform
  const matrixMatch = transformStr.match(/matrix\(([^)]+)\)/);
  if (matrixMatch) {
    const values = matrixMatch[1].split(/[,\s]+/).map(Number);
    if (values.length === 6) {
      const [a, b, c, d, e, f] = values;

      // Extract translation
      result.translate.x = e;
      result.translate.y = f;

      // Calculate determinant to detect reflection
      const det = a * d - b * c;
      const hasReflection = det < 0;

      // Calculate scale magnitudes
      const scaleXMag = Math.sqrt(a * a + b * b);
      const scaleYMag = Math.sqrt(c * c + d * d);

      // Calculate rotation
      let rotation = Math.atan2(b, a) * (180 / Math.PI);

      // Handle the different matrix cases more carefully
      if (Math.abs(a) === 1 && b === 0 && c === 0 && Math.abs(d) === 1) {
        // Simple scale/reflection case: matrix(±1,0,0,±1,tx,ty)
        result.scale.x = a;
        result.scale.y = d;
        result.rotate.angle = 0;
      } else if (a === 0 && Math.abs(b) >= 1 && Math.abs(c) >= 1 && d === 0) {
        // 90° rotation cases: matrix(0,±scale,±scale,0,tx,ty)
        const bSign = Math.sign(b);
        const cSign = Math.sign(c);
        const scaleValueB = Math.abs(b);
        const scaleValueC = Math.abs(c);

        // matrix(0,b,c,0) where b and c can be positive or negative
        if (bSign === -1 && cSign === -1) {
          // matrix(0,-scale,-scale,0,tx,ty) = 90° rotation + reflection
          result.scale.x = -scaleValueB;  // Include the reflection
          result.scale.y = scaleValueC;
          result.rotate.angle = 90;
        } else if (bSign === 1 && cSign === -1) {
          // matrix(0,scale,-scale,0,tx,ty) = -90° rotation
          result.scale.x = scaleValueB;
          result.scale.y = scaleValueC;
          result.rotate.angle = -90;
        } else if (bSign === -1 && cSign === 1) {
          // matrix(0,-scale,scale,0,tx,ty) = 90° rotation
          result.scale.x = scaleValueB;
          result.scale.y = scaleValueC;
          result.rotate.angle = 90;
        } else {
          // matrix(0,scale,scale,0,tx,ty) = -90° rotation + reflection
          result.scale.x = scaleValueB;
          result.scale.y = -scaleValueC; // Include the reflection
          result.rotate.angle = -90;
        }
      } else {
        // General case - decompose more carefully
        if (hasReflection) {
          // For reflections, preserve the sign information in scale
          result.scale.x = Math.sign(a) * scaleXMag;
          result.scale.y = Math.sign(d) * scaleYMag;
          result.rotate.angle = rotation;
        } else {
          result.scale.x = scaleXMag;
          result.scale.y = scaleYMag;
          result.rotate.angle = rotation;
        }
      }

      // Ensure scale values are never 0
      if (result.scale.x === 0) result.scale.x = 1;
      if (result.scale.y === 0) result.scale.y = 1;
    }
    return result;
  }

  // Handle individual transforms
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
    result.rotate.cy = values[2] || 0;

    // If rotation center is specified and is not at origin, we need to preserve this info
    // SVG animation can handle rotation centers directly, so don't convert to translate
    // The animation engine will handle this properly
  }

  const scaleMatch = transformStr.match(/scale\(([^)]+)\)/);
  if (scaleMatch) {
    const values = scaleMatch[1].split(/[,\s]+/).map(Number);
    const scaleX = values[0] || 1;
    const scaleY = values[1] || values[0] || 1;
    // Ensure scale values are never 0
    result.scale.x = scaleX === 0 ? 1 : scaleX;
    result.scale.y = scaleY === 0 ? 1 : scaleY;
  }

  return result;
}

/**
 * Parse SVG file and extract groups with transforms and paths
 */
function parseSVGFile(filePath: string): KeyframeData {
  const svgContent = fs.readFileSync(filePath, 'utf-8');
  const dom = new JSDOM(svgContent, { contentType: 'text/xml' });
  const document = dom.window.document;

  const groups: GroupData[] = [];
  const gElements = document.querySelectorAll('g[id]');

  gElements.forEach(g => {
    const id = g.getAttribute('id')!;
    const transformStr = g.getAttribute('transform') || '';
    const pathElement = g.querySelector('path');

    if (pathElement) {
      const transform = parseTransform(transformStr);
      const groupData: GroupData = {
        id,
        transform,
        path: {
          d: pathElement.getAttribute('d') || '',
          fill: pathElement.getAttribute('fill') || '#000000',
          stroke: pathElement.getAttribute('stroke') || 'none',
          strokeWidth: pathElement.getAttribute('stroke-width') || '0'
        }
      };

      groups.push(groupData);
    }
  });

  return { groups };
}

/**
 * Generate keyframes sequence with hold times (1 to n, then n-1 to 1, then back to 1 for loop)
 */
function generateKeyframeSequence(keyframes: KeyframeData[]): KeyframeData[] {
  const sequence: KeyframeData[] = [];

  // Add keyframes 1 to n with hold frames
  for (let i = 0; i < keyframes.length; i++) {
    sequence.push(keyframes[i]);
    sequence.push(keyframes[i]); // Hold frame 1
    sequence.push(keyframes[i]); // Hold frame 2
  }

  // Add keyframes n-1 to 2 (excluding first and last to avoid duplication) with hold frames
  for (let i = keyframes.length - 2; i > 0; i--) {
    sequence.push(keyframes[i]);
    sequence.push(keyframes[i]); // Hold frame 1
    sequence.push(keyframes[i]); // Hold frame 2
  }

  // Add the first keyframe at the end to complete the loop smoothly
  sequence.push(keyframes[0]);

  return sequence;
}

/**
 * Create animated SVG from keyframes
 */
function createAnimatedSVG(keyframes: KeyframeData[], outputPath: string): void {
  const sequence = generateKeyframeSequence(keyframes);
  const totalFrames = sequence.length;

  // Calculate duration: Keep fast transitions but longer holds
  const transitionDuration = 0.5; // Fast transition between keyframes
  const holdDuration = 1.5; // Longer hold on each keyframe
  const totalDuration = (keyframes.length * (transitionDuration + holdDuration) * 2); // *2 for back and forth

  // Create keyTimes array with proper timing for transitions and holds
  const keyTimes: string[] = [];
  let currentTime = 0;
  const timeStep = 1 / (totalFrames - 1);

  for (let i = 0; i < totalFrames; i++) {
    keyTimes.push((i * timeStep).toFixed(3));
  }
  const keyTimesStr = keyTimes.join(';');

  // For smooth in/out easing with longer holds, we'll use calcMode="spline" with keySplines
  // Each transition gets different easing depending on whether it's a transition or hold
  const keySplines: string[] = [];
  for (let i = 0; i < totalFrames - 1; i++) {
    // Check if this is a transition or hold based on sequence pattern
    const currentFrame = sequence[i];
    const nextFrame = sequence[i + 1];
    const isSameFrame = JSON.stringify(currentFrame) === JSON.stringify(nextFrame);

    if (isSameFrame) {
      // Hold frame - linear (no easing)
      keySplines.push('0 0 1 1');
    } else {
      // Transition frame - smooth ease-in-out
      keySplines.push('0.25 0.1 0.75 0.9');
    }
  }
  const keySplinesStr = keySplines.join(';');

  // Group all unique group IDs
  const allGroupIds = new Set<string>();
  keyframes.forEach(kf => kf.groups.forEach(g => allGroupIds.add(g.id)));

  let svgContent = `<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg viewBox="0 0 410 140" style="background: #001117;" version="1.1" xmlns="http://www.w3.org/2000/svg">
    <title>semio</title>
    <rect id="background" width="100%" height="100%" fill="#001117" />
`;

  // Create animated groups
  Array.from(allGroupIds).forEach(groupId => {
    // Find this group in all keyframes
    const groupFrames = sequence.map(kf => {
      const group = kf.groups.find(g => g.id === groupId);
      return group || null;
    });

    // Skip if group doesn't exist in any frame
    if (groupFrames.every(gf => gf === null)) return;

    // Use first non-null group for path data
    const firstGroup = groupFrames.find(gf => gf !== null);
    if (!firstGroup) return;

    svgContent += `    <g id="${groupId}">
`;

    // Generate translate animation
    const translateValues = groupFrames.map(gf => {
      if (gf) {
        return `${gf.transform.translate.x} ${gf.transform.translate.y}`;
      }
      return `${firstGroup.transform.translate.x} ${firstGroup.transform.translate.y}`;
    }).join(';');

    svgContent += `        <animateTransform attributeName="transform" type="translate" dur="${totalDuration}s" repeatCount="indefinite"
            keyTimes="${keyTimesStr}" values="${translateValues}" calcMode="spline" keySplines="${keySplinesStr}" />
`;

    // Generate rotation animation
    const rotateValues = groupFrames.map(gf => {
      if (gf) {
        return `${gf.transform.rotate.angle} ${gf.transform.rotate.cx} ${gf.transform.rotate.cy}`;
      }
      return `${firstGroup.transform.rotate.angle} ${firstGroup.transform.rotate.cx} ${firstGroup.transform.rotate.cy}`;
    }).join(';');

    svgContent += `        <animateTransform attributeName="transform" type="rotate" additive="sum" dur="${totalDuration}s" repeatCount="indefinite"
            keyTimes="${keyTimesStr}" values="${rotateValues}" calcMode="spline" keySplines="${keySplinesStr}" />
`;

    // Generate scale animation (ensure scale never goes to 0 0, default to 1 1)
    const scaleValues = groupFrames.map(gf => {
      if (gf) {
        // Ensure scale values are never 0, default to 1
        const scaleX = gf.transform.scale.x === 0 ? 1 : gf.transform.scale.x;
        const scaleY = gf.transform.scale.y === 0 ? 1 : gf.transform.scale.y;
        return `${scaleX} ${scaleY}`;
      }
      // Use 1 1 as default scale when group doesn't exist in this frame
      return `1 1`;
    }).join(';');

    svgContent += `        <animateTransform attributeName="transform" type="scale" additive="sum" dur="${totalDuration}s" repeatCount="indefinite"
            keyTimes="${keyTimesStr}" values="${scaleValues}" calcMode="spline" keySplines="${keySplinesStr}" />
`;

    // Add path with fill and stroke animations
    const fillValues = groupFrames.map(gf => {
      return gf ? gf.path.fill : firstGroup.path.fill;
    }).join(';');

    const strokeValues = groupFrames.map(gf => {
      return gf ? gf.path.stroke : firstGroup.path.stroke;
    }).join(';');

    const strokeWidthValues = groupFrames.map(gf => {
      return gf ? gf.path.strokeWidth : firstGroup.path.strokeWidth;
    }).join(';');

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

/**
 * Main function
 */
function main(): void {
  const logoDir = path.dirname(__filename);

  // Parse input keyframes
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
    console.error('No keyframe files found!');
    process.exit(1);
  }

  console.log(`Found ${keyframes.length} keyframes`);
  console.log(`Will generate ${generateKeyframeSequence(keyframes).length} animation frames`);

  // Create animated SVG
  const outputPath = path.join(logoDir, 'logo_generated.svg');
  createAnimatedSVG(keyframes, outputPath);
}

// Run if called directly
if (require.main === module) {
  main();
}

export { createAnimatedSVG, generateKeyframeSequence, parseSVGFile };

