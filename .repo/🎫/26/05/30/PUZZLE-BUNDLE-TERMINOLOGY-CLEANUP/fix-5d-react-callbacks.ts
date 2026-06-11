#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const path = join(import.meta.dir, "../../../../../..", "puzzle/5d/react/index.tsx");
let s = readFileSync(path, "utf8");

s = s.replace(/mergeAnchorsFlatAndVolume/g, "mergeGripsFlatAndVolume");
s = s.replace(/tieTouchesPartOrAnchors/g, "fastenerTouchesPartOrGrips");
s = s.replace(/removeAnchorFromModel/g, "removeGripFromModel");
s = s.replace(/removeTieFromModel/g, "removeFastenerFromModel");
s = s.replace(/remainingAnchorIds/g, "remainingGripIds");
s = s.replace(/fullAnchorId/g, "fullGripId");
s = s.replace(/tieId/g, "fastenerId");
s = s.replace(/tieSource/g, "fastenerSource");
s = s.replace(/tieTarget/g, "fastenerTarget");

s = s.replace(
  /let "2d": Part2dAspect \| undefined;\n  let "3d": Part3dAspect \| undefined;\n  let grips: GripV1\[\];\n  let fastenerSource = "";\n  let fastenerTarget = "";/,
  `let flatAspect: Part2dAspect | undefined;
  let volumeAspect: Part3dAspect | undefined;
  let grips: GripV1[];
  let fastenerSource = "";
  let fastenerTarget = "";`,
);

s = s.replace(/    "2d" =\n      payload\.shape === "rectangle"/, "    flatAspect =\n      payload.shape === \"rectangle\"");
s = s.replace(/    "3d" = volume\.aspect;/g, "    volumeAspect = volume.aspect;");
s = s.replace(/    anchors = mergeGripsFlatAndVolume/g, "    grips = mergeGripsFlatAndVolume");
s = s.replace(/    anchors = volume\.grips;/g, "    grips = volume.grips;");
s = s.replace(/    const matingLocal = anchors\[/g, "    const matingLocal = grips[");
s = s.replace(/    "2d" = flat;/g, "    flatAspect = flat;");
s = s.replace(/model\.fasteners\.some\(\(tie\) => fastener\.source === fastenerSource && fastener\.target === fastenerTarget\)/, "model.fasteners.some((f) => f.source === fastenerSource && f.target === fastenerTarget)");
s = s.replace(/const fastenerId = placement\.fastenerId\?\.trim\(\) \|\| `puzzle5d\.brush\.tie\./, "const fastenerId = placement.fastenerId?.trim() || `puzzle5d.brush.fastener.");
s = s.replace(
  /const part: PartV1 = \{\n    id: partId,\n    partKind,\n    "2d",\n    "3d",\n    grips,\n  \};/,
  `const part: PartV1 = {
    id: partId,
    partKind,
    ...(flatAspect ? { "2d": flatAspect } : {}),
    ...(volumeAspect ? { "3d": volumeAspect } : {}),
    grips,
  };`,
);
s = s.replace(/fasteners: \[\.\.\.model\.fasteners, \{ id: fastenerId, source: fastenerSource, target: fastenerTarget \}\]/, "fasteners: [...model.fasteners, { id: fastenerId, source: fastenerSource, target: fastenerTarget }]");

s = s.replace(/gripFullId\(part\.id, anchor\.id\)/, "gripFullId(part.id, grip.id)");

s = s.replace(
  /function fastenerTouchesPartOrGrips\(tie: FastenerV1, partId: string, gripIds: ReadonlySet<string>\): boolean \{\n  if \(fastener\.source === partId \|\| fastener\.target === partId\) \{/,
  `function fastenerTouchesPartOrGrips(fastener: FastenerV1, partId: string, gripIds: ReadonlySet<string>): boolean {
  if (fastener.source === partId || fastener.target === partId) {`,
);

s = s.replace(/fasteners: model\.fasteners\.filter\(\(tie\) => !fastenerTouchesPartOrGrips\(tie, partId, gripIds\)\)/, "fasteners: model.fasteners.filter((f) => !fastenerTouchesPartOrGrips(f, partId, gripIds))");
s = s.replace(/fasteners: model\.fasteners\.filter\(\(tie\) => fastener\.source !== fullGripId && fastener\.target !== fullGripId\)/, "fasteners: model.fasteners.filter((f) => f.source !== fullGripId && f.target !== fullGripId)");
s = s.replace(/if \(!model\.fasteners\.some\(\(tie\) => fastener\.id === fastenerId\)\)/, "if (!model.fasteners.some((f) => f.id === fastenerId))");
s = s.replace(/fasteners: model\.fasteners\.filter\(\(tie\) => fastener\.id !== fastenerId\)/, "fasteners: model.fasteners.filter((f) => f.id !== fastenerId)");

s = s.replace(
  /applyFastener\(source: string, target: string, fastenerKind\?: string\): void \{\n    const ties = this\.snapshot\.model\.fasteners;\n    if \(ties\.some\(\(tie\) => fastener\.source === source && fastener\.target === target\)\) \{\n      this\.setSnapshot\(\{ \.\.\.this\.snapshot, connectSession: null \}\);\n      return;\n    \}\n    const id = crypto\.randomUUID\(\);\n    const nextFasteners: FastenerV1\[\] = \[\.\.\.fasteners, \{ id, source, target, \.\.\.\(fastenerKind \? \{ fastenerKind \} : \{\}\) \}\];\n    this\.setSnapshot\(\{\n      \.\.\.this\.snapshot,\n      model: \{ \.\.\.this\.snapshot\.model, fasteners: nextTies \},\n      connectSession: null,\n    \}\);\n  \}/,
  `applyFastener(source: string, target: string, fastenerKind?: string): void {
    const fasteners = this.snapshot.model.fasteners;
    if (fasteners.some((f) => f.source === source && f.target === target)) {
      this.setSnapshot({ ...this.snapshot, connectSession: null });
      return;
    }
    const id = crypto.randomUUID();
    const nextFasteners: FastenerV1[] = [...fasteners, { id, source, target, ...(fastenerKind ? { fastenerKind } : {}) }];
    this.setSnapshot({
      ...this.snapshot,
      model: { ...this.snapshot.model, fasteners: nextFasteners },
      connectSession: null,
    });
  }`,
);

s = s.replace(/const anchors = buildGripsUnified/g, "const grips = buildGripsUnified");
s = s.replace(/expect\(store\.read\(\)\.ties\)/g, "expect(store.read().fasteners)");
s = s.replace(/model\.fasteners\.some\(\(tie\) => fastener\./g, "model.fasteners.some((f) => f.");

writeFileSync(path, s);
console.log("[DEBUG] fixed callbacks in puzzle/5d/react/index.tsx");
