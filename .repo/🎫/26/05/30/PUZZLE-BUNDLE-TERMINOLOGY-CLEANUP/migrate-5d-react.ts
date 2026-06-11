#!/usr/bin/env bun
/** One-off: apply 5d terminology renames to puzzle/5d/react/index.tsx */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const path = join(import.meta.dir, "../../../../../..", "puzzle/5d/react/index.tsx");
let s = readFileSync(path, "utf8");

const pairs: [string, string][] = [
  ["Puzzle2dAnchorAspect", "Grip2dAspect"],
  ["Puzzle3dAnchorAspect", "Grip3dAspect"],
  ["NodeAspect", "Part2dAspect"],
  ["Puzzle3dPartAspect", "Part3dAspect"],
  ["AnchorV1", "GripV1"],
  ["TieV1", "FastenerV1"],
  ["PUZZLE_5D_ANCHOR_ID_SEPARATOR", "PUZZLE_5D_GRIP_ID_SEPARATOR"],
  ["anchorFullId", "gripFullId"],
  ["parseAnchorFullId", "parseGripFullId"],
  ["applyFlatNodeCenters", "applyPart2dCenters"],
  ["applyNodeMoves", "applyPart2dMoves"],
  ["applyNodeMove", "applyPart2dMove"],
  ["apply3dRelocate", "applyPart3dRelocate"],
  ["applyTie", "applyFastener"],
  ["sourceAnchor", "sourceGrip"],
  ["ringAnchorIds", "ringGripIds"],
  ["anchorIds", "gripIds"],
  ['domain: "anchor"', 'domain: "grip"'],
  ['case "anchor"', 'case "grip"'],
  ['kind: "anchor"', 'kind: "grip"'],
  ['domain: "tie"', 'domain: "fastener"'],
  ['case "tie"', 'case "fastener"'],
  ['kind: "tie"', 'kind: "fastener"'],
  ["Puzzle5dKindHoverDomain = \"part\" | \"anchor\" | \"tie\"", "Puzzle5dKindHoverDomain = \"part\" | \"grip\" | \"fastener\""],
  ["readonly ties:", "readonly fasteners:"],
  ["model.ties", "model.fasteners"],
  ["snap.model.ties", "snap.model.fasteners"],
  [".ties.length", ".fasteners.length"],
  [".ties.map", ".fasteners.map"],
  [".ties.some", ".fasteners.some"],
  ["const ties:", "const fasteners:"],
  ["const tieIds", "const fastenerIds"],
  ["tieIds.has", "fastenerIds.has"],
  ["tieIds.add", "fastenerIds.add"],
  ["for (const tie of", "for (const fastener of"],
  ["tie.id", "fastener.id"],
  ["tie.source", "fastener.source"],
  ["tie.target", "fastener.target"],
  ["tie.tieKind", "fastener.fastenerKind"],
  ["nextTies:", "nextFasteners:"],
  ["ties,", "fasteners,"],
  ["ties:", "fasteners:"],
  ["tieKind", "fastenerKind"],
  [".anchors", ".grips"],
  ["anchors:", "grips:"],
  ["anchorKind:", "gripKind:"],
  [".anchorKind", ".gripKind"],
  ["readonly anchors:", "readonly grips:"],
  ["volumeAnchors", "volumeGrips"],
  ["anchorById", "gripById"],
  ["Store applyTie", "Store applyFastener"],
];

for (const [from, to] of pairs) {
  s = s.split(from).join(to);
}

s = s.replace(/\bpuzzle2d\b/g, '"2d"');
s = s.replace(/\bpuzzle3d\b/g, '"3d"');

s = s.replace(
  /specificity\?: "edge" \| "general" \| "handle" \| "node" \| "wire" \| "object" \| "attraction" \| "part" \| "grip" \| "fastener" \| "rope" \| "vortex" \| "cable"/,
  'specificity?: "general" | "part" | "grip" | "fastener" | "rope"',
);

s = s.replace(
  /entry\.specificity === "general" \|\| entry\.specificity === "node" \|\| entry\.specificity === "edge" \|\| entry\.specificity === "handle" \|\| entry\.specificity === "wire" \|\| entry\.specificity === "object" \|\| entry\.specificity === "attraction" \|\| entry\.specificity === "part" \|\| entry\.specificity === "grip" \|\| entry\.specificity === "fastener" \|\| entry\.specificity === "rope" \|\| entry\.specificity === "vortex" \|\| entry\.specificity === "cable"/,
  'entry.specificity === "general" || entry.specificity === "part" || entry.specificity === "grip" || entry.specificity === "fastener" || entry.specificity === "rope"',
);

writeFileSync(path, s);
console.log("[DEBUG] migrated puzzle/5d/react/index.tsx");
