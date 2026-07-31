#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const path = join(import.meta.dir, "../../../../../..", "puzzle/5d/react/index.tsx");
let s = readFileSync(path, "utf8");

s = s.replace(/\."2d"/g, '["2d"]');
s = s.replace(/\."3d"/g, '["3d"]');
s = s.replace(/: a\["3d"\], gripKind/g, ': a["3d"], gripKind');

s = s.replace(/if \(!Array\.isArray\(r\.parts\) \|\| !Array\.isArray\(r\.ties\)\)/, "if (!Array.isArray(r.parts) || !Array.isArray(r.fasteners))");
s = s.replace(/fasteners: r\.ties as FastenerV1\[\]/, "fasteners: r.fasteners as FastenerV1[]");

s = s.replace(/a\.kind === "anchor"/g, 'a.kind === "grip"');
s = s.replace(/b\.kind === "anchor"/g, 'b.kind === "grip"');

s = s.replace(/anchorId/g, "gripId");
s = s.replace(/anchorLocalId/g, "gripLocalId");
s = s.replace(/slugAnchorLocalId/g, "slugGripLocalId");
s = s.replace(/buildAnchorsUnified/g, "buildGripsUnified");
s = s.replace(/volumeAnchorIndexOnPart/g, "volumeGripIndexOnPart");
s = s.replace(/targetAnchor/g, "targetGrip");

s = s.replace(/const anchors:/g, "const grips:");
s = s.replace(/anchors\.push/g, "grips.push");
s = s.replace(/return anchors;/g, "return grips;");
s = s.replace(/\banchors,/g, "grips,");
s = s.replace(/\banchors:/g, "grips:");

s = s.replace(
  /export interface FiveDProps \{[\s\S]*?readonly "2d"\?: Omit<Puzzle2dCanvasProps, "children">;[\s\S]*?readonly "3d"\?: Omit<Puzzle3dCanvasProps, "children">;/,
  `export interface FiveDProps {
  readonly mode: PresentationMode;
  readonly instanceId: string;
  readonly className?: string;
  readonly lockedPartIds?: ReadonlySet<string>;
  readonly gumballConfig?: GumballConfig;
  /** @emoji 🕸️ When true, runs a continuous WASM force-graph layout on the flat surface (e.g. kit WIRES). */
  readonly liveForceGraph?: boolean;
  /** @emoji 🔗 Flat graph port model; WIRES surfaces use \`normal\` (node-id edges, no handles). */
  readonly graphPortMode?: Puzzle2dCanvasProps["graphPortMode"];
  /** @emoji 🖌️ Shared authoring tool for both surfaces (\`select\` | \`brush\` | \`fill\`). */
  readonly activeTool?: Puzzle5dActiveTool;
  readonly brushFlushDistance?: number;
  readonly brushOverlapBudget?: number;
  /** 2d surface overrides; LOD uses discrete tiers unless \`automaticLod\` is set on the canvas. */
  readonly puzzle2d?: Omit<Puzzle2dCanvasProps, "children">;
  /** 3d surface overrides; LOD is continuous/camera-driven — not the flat six-tier scale. */
  readonly puzzle3d?: Omit<Puzzle3dCanvasProps, "children">;`,
);

s = s.replace(/props\["2d"\]/g, "props.puzzle2d");
s = s.replace(/props\["3d"\]/g, "props.puzzle3d");

s = s.replace(/\/\*\* @emoji 🔗 Builds a full anchor id/g, "/** @emoji 🔗 Builds a full grip id");
s = s.replace(/\/\*\* @emoji 🔍 Splits a full anchor id/g, "/** @emoji 🔍 Splits a full grip id");

s = s.replace(/\(anchor\) => anchor\["3d"\]/g, '(grip) => grip["3d"]');
s = s.replace(/\(anchor\) => anchor\.id/g, "(grip) => grip.id");
s = s.replace(/\.find\(\(anchor\) =>/g, ".find((grip) =>");
s = s.replace(/\.filter\(\(anchor\) =>/g, ".filter((grip) =>");
s = s.replace(/\.map\(\(anchor\) =>/g, ".map((grip) =>");
s = s.replace(/\.findIndex\(\(anchor\) =>/g, ".findIndex((grip) =>");

writeFileSync(path, s);
console.log("[DEBUG] fixed puzzle/5d/react/index.tsx syntax");
