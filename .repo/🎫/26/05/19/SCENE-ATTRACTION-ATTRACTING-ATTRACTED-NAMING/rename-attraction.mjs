import { readFileSync, writeFileSync } from "node:fs";

const path = "elements/client/lib/scene/index.tsx";
let s = readFileSync(path, "utf8");

const pairs = [
  ["SceneTieLinkPayload", "SceneAttractionPayload"],
  ["SceneLinkCompatibleNodesPayload", "SceneAttractionCompatibleObjectsPayload"],
  ["SceneLinkTargetRingPayload", "SceneAttractionTargetRingPayload"],
  ["SceneLinkIndirectPickAwait", "SceneAttractionIndirectPickAwait"],
  ["SceneLinkHandleContext", "SceneAttractionHandleContext"],
  ["sceneHandlesLinkCompatibleForDrag", "sceneHandlesAttractionCompatibleForDrag"],
  ["sceneLinkGestureRuleApplies", "sceneAttractionGestureRuleApplies"],
  ["sceneLinkSnapDragTolerancePx", "sceneAttractionSnapDragTolerancePx"],
  ["sceneLinkSnapCommitTolerancePx", "sceneAttractionSnapCommitTolerancePx"],
  ["sceneLinkSnapCommitProximityOk", "sceneAttractionSnapCommitProximityOk"],
  ["sceneNearestLinkSnapFullId", "sceneNearestAttractionSnapFullId"],
  ["SCENE_LINK_HANDLE_SNAP_EXTRA_PX", "SCENE_ATTRACTION_HANDLE_SNAP_EXTRA_PX"],
  ["SCENE_LINK_COMMIT_SNAP_TIGHT_PX", "SCENE_ATTRACTION_COMMIT_SNAP_TIGHT_PX"],
  ["SceneLinkThreeBinder", "SceneAttractionThreeBinder"],
  ["SceneLinkWindowBridge", "SceneAttractionWindowBridge"],
  ["SceneLinkRubberBand", "SceneAttractionRubberBand"],
  ["linkDragActive", "attractionDragActive"],
  ["linkDragSourceFullId", "attractionDragAttractingFullId"],
  ["linkCompatibleTargetFullIds", "attractionCompatibleAttractedFullIds"],
  ["linkHoverRingFullId", "attractionHoverRingFullId"],
  ["linkIndirectPickAwait", "attractionIndirectPickAwait"],
  ["linkEndWorldRef", "attractionEndWorldRef"],
  ["linkThreeRef", "attractionThreeRef"],
  ["linkSessionRef", "attractionSessionRef"],
  ["beginLinkDragFromVortex", "beginAttractionDragFromVortex"],
  ["cancelLinkDrag", "cancelAttractionDrag"],
  ["attachLinkThreeEnv", "attachAttractionThreeEnv"],
  ["updateLinkPointer", "updateAttractionPointer"],
  ["commitLinkPointer", "commitAttractionPointer"],
  ["onLinkCompatibleNodes", "onAttractionCompatibleObjects"],
  ["onLinkTargetRing", "onAttractionTargetRing"],
  ["sourceFullId", "attractingFullId"],
  ["sourceObjectId", "attractingObjectId"],
  ["sourceCtx", "attractingCtx"],
  ["snapTargetFullId", "snapAttractedFullId"],
  ["targetObjectId", "attractedObjectId"],
  ["//#region 🔗LinkGesture", "//#region 🧲AttractionGesture"],
  ["//#endregion 🔗LinkGesture", "//#endregion 🧲AttractionGesture"],
  ['describe("sceneHandlesLinkCompatibleForDrag"', 'describe("sceneHandlesAttractionCompatibleForDrag"'],
];

for (const [a, b] of pairs) {
  if (!s.includes(a)) console.warn("missing:", a);
  s = s.split(a).join(b);
}

s = s.replace(
  /readonly source: string;\n\treadonly target: string;\n\treadonly tieId/,
  "readonly attracting: string;\n\treadonly attracted: string;\n\treadonly tieId",
);
s = s.replace(/readonly source: string;\n\treadonly objectIds/, "readonly attracting: string;\n\treadonly objectIds");
s = s.replace(
  /readonly source: string;\n\treadonly objectId: string \| null/,
  "readonly attracting: string;\n\treadonly objectId: string | null",
);

s = s.replace(/\{ source: "", objectId/g, '{ attracting: "", objectId');
s = s.replace(/\{ source: session\.attractingFullId, objectId/g, "{ attracting: session.attractingFullId, objectId");
s = s.replace(/\{ source: fullId, objectIds/g, "{ attracting: fullId, objectIds");
s = s.replace(
  /\{ source: session\.attractingFullId, target: snapId \}/g,
  "{ attracting: session.attractingFullId, attracted: snapId }",
);
s = s.replace(
  /return \{ source: `\$\{movingObjectId\}:link`, target: best\.id \}/g,
  "return { attracting: `${movingObjectId}:link`, attracted: best.id }",
);

s = s.replace("props: { from: Vec3; to: Vec3 }", "props: { attracting: Vec3; attracted: Vec3 }");
s = s.replaceAll("props.from", "props.attracting");
s = s.replaceAll("props.to", "props.attracted");
s = s.replace("[props.attracting, props.attracted]", "[props.attracting, props.attracted]");

s = s.replaceAll('| "source" | "indirectRing"', '| "attracting" | "indirectRing"');
s = s.replace('case "source":', 'case "attracting":');
s = s.replace('? "source"', '? "attracting"');

s = s.replace("Semantic kinds at one end of a link drag", "Semantic kinds at one end of an attraction drag");
s = s.replace("filtered link compatibility", "filtered attraction compatibility");
s = s.replaceAll("linkBusy", "attractionBusy");
s = s.replace("const wire =", "const attractionLine =");
s = s.replace("if (!wire)", "if (!attractionLine)");

s = s.replace("const sourceFull = session.attractingFullId", "const attractingFull = session.attractingFullId");
s = s.replaceAll("vf !== sourceFull", "vf !== attractingFull");
s = s.replaceAll("{ source: sourceFull, target:", "{ attracting: attractingFull, attracted:");
s = s.replaceAll("{ source: attractingFull,", "{ attracting: attractingFull,");

s = s.replace(
  /function sceneAttractionGestureRuleApplies\(\n\trule: SceneKindCompatEntry,\n\tsource: SceneAttractionHandleContext,\n\ttarget: SceneAttractionHandleContext,/,
  "function sceneAttractionGestureRuleApplies(\n\trule: SceneKindCompatEntry,\n\tattracting: SceneAttractionHandleContext,\n\tattracted: SceneAttractionHandleContext,",
);
s = s.replace(
  /export function sceneHandlesAttractionCompatibleForDrag\(\n\tsource: SceneAttractionHandleContext,\n\ttarget: SceneAttractionHandleContext,/,
  "export function sceneHandlesAttractionCompatibleForDrag(\n\tattracting: SceneAttractionHandleContext,\n\tattracted: SceneAttractionHandleContext,",
);

// Fix function bodies that still reference source/target handle contexts
const fnStart = s.indexOf("function sceneAttractionGestureRuleApplies");
const fnEnd = s.indexOf("/** @emoji 🤝 WASM-style filtered attraction compatibility");
const fnBlock = s.slice(fnStart, fnEnd);
const fnFixed = fnBlock
  .replaceAll("source.vortexKind", "attracting.vortexKind")
  .replaceAll("target.vortexKind", "attracted.vortexKind")
  .replaceAll("source.objectKind", "attracting.objectKind")
  .replaceAll("target.objectKind", "attracted.objectKind");
s = s.slice(0, fnStart) + fnFixed + s.slice(fnEnd);

const dragStart = s.indexOf("export function sceneHandlesAttractionCompatibleForDrag");
const dragEnd = s.indexOf("//#endregion 🧲Helpers", dragStart);
const dragBlock = s.slice(dragStart, dragEnd);
const dragFixed = dragBlock
  .replace("sceneAttractionGestureRuleApplies(r, source, target, catalogs)", "sceneAttractionGestureRuleApplies(r, attracting, attracted, catalogs)");
s = s.slice(0, dragStart) + dragFixed + s.slice(dragEnd);

// beginAttractionDrag local vars
s = s.replaceAll("const attractingCtx: SceneAttractionHandleContext", "const attractingCtx: SceneAttractionHandleContext");
s = s.replace(
  "const targetCtx: SceneAttractionHandleContext",
  "const attractedCtx: SceneAttractionHandleContext",
);
s = s.replace(
  "if (!sceneHandlesAttractionCompatibleForDrag(attractingCtx, targetCtx, kindCompatibility, kindCatalogs)) continue;",
  "if (!sceneHandlesAttractionCompatibleForDrag(attractingCtx, attractedCtx, kindCompatibility, kindCatalogs)) continue;",
);

s = s.replace(
  /\{ source: awaitPick\.attractingFullId, target: vf \}/g,
  "{ attracting: awaitPick.attractingFullId, attracted: vf }",
);

s = s.replace(
  "function sceneAttractionSnapCommitProximityOk(\n\ttargetFullId: string,",
  "function sceneAttractionSnapCommitProximityOk(\n\tattractedFullId: string,",
);
s = s.replaceAll("getVortexWorld(targetFullId)", "getVortexWorld(attractedFullId)");
s = s.replaceAll("metaRadius(targetFullId)", "metaRadius(attractedFullId)");

writeFileSync(path, s);
console.log("wrote", path);
