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

      // Extract scale (ensure never 0, default to 1)
      const scaleX = Math.sign(a) * Math.sqrt(a * a + b * b);
      const scaleY = Math.sign(d) * Math.sqrt(c * c + d * d);
      result.scale.x = scaleX === 0 ? 1 : scaleX;
      result.scale.y = scaleY === 0 ? 1 : scaleY;

      // Extract rotation (in degrees)
      const rotation = Math.atan2(b, a) * (180 / Math.PI);
      result.rotate.angle = rotation;
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
      const groupData: GroupData = {
        id,
        transform: parseTransform(transformStr),
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
 * Generate keyframes sequence (1 to n, then n-1 to 1)
 */
function generateKeyframeSequence(keyframes: KeyframeData[]): KeyframeData[] {
  const sequence: KeyframeData[] = [];

  // Add keyframes 1 to n
  for (let i = 0; i < keyframes.length; i++) {
    sequence.push(keyframes[i]);
  }

  // Add keyframes n-1 to 1 (excluding first and last to avoid duplication)
  for (let i = keyframes.length - 2; i > 0; i--) {
    sequence.push(keyframes[i]);
  }

  return sequence;
}

/**
 * Create animated SVG from keyframes
 */
function createAnimatedSVG(keyframes: KeyframeData[], outputPath: string): void {
  const sequence = generateKeyframeSequence(keyframes);
  const totalFrames = sequence.length;

  // Calculate duration: 0.5 seconds per keyframe
  const keyframeDuration = 2;
  const totalDuration = totalFrames * keyframeDuration;

  // Create keyTimes array - each keyframe gets equal time
  const keyTimes = sequence.map((_, i) => (i / (totalFrames - 1)).toFixed(3)).join(';');

  // For smooth in/out easing, we'll use calcMode="spline" with keySplines
  // Each transition gets a smooth ease-in-out curve
  const keySplines: string[] = [];
  for (let i = 0; i < totalFrames - 1; i++) {
    keySplines.push('0.42 0 0.58 1'); // ease-in-out cubic-bezier
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
            keyTimes="${keyTimes}" values="${translateValues}" calcMode="spline" keySplines="${keySplinesStr}" />
`;

    // Generate rotation animation
    const rotateValues = groupFrames.map(gf => {
      if (gf) {
        return `${gf.transform.rotate.angle} ${gf.transform.rotate.cx} ${gf.transform.rotate.cy}`;
      }
      return `${firstGroup.transform.rotate.angle} ${firstGroup.transform.rotate.cx} ${firstGroup.transform.rotate.cy}`;
    }).join(';');

    svgContent += `        <animateTransform attributeName="transform" type="rotate" additive="sum" dur="${totalDuration}s" repeatCount="indefinite"
            keyTimes="${keyTimes}" values="${rotateValues}" calcMode="spline" keySplines="${keySplinesStr}" />
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
            keyTimes="${keyTimes}" values="${scaleValues}" calcMode="spline" keySplines="${keySplinesStr}" />
`;

    // Add path with fill and stroke animations
    const fillValues = groupFrames.map(gf => {
      return gf ? gf.path.fill : firstGroup.path.fill;
    }).join(';');

    const strokeValues = groupFrames.map(gf => {
      return gf ? gf.path.stroke : firstGroup.path.stroke;
    }).join(';');

    svgContent += `        <path d="${firstGroup.path.d}">
            <animate attributeName="fill" dur="${totalDuration}s" repeatCount="indefinite" keyTimes="${keyTimes}"
                values="${fillValues}" calcMode="spline" keySplines="${keySplinesStr}" />
            <animate attributeName="stroke" dur="${totalDuration}s" repeatCount="indefinite" keyTimes="${keyTimes}"
                values="${strokeValues}" calcMode="spline" keySplines="${keySplinesStr}" />
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
    const filePath = path.join(logoDir, `logo-${i}.svg`);
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
  const outputPath = path.join(logoDir, 'logo.svg');
  createAnimatedSVG(keyframes, outputPath);
}

// Run if called directly
if (require.main === module) {
  main();
}

export { createAnimatedSVG, generateKeyframeSequence, parseSVGFile };

