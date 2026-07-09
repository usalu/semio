#!/usr/bin/env bun
const path = "c:/git/compose/spatial/js/core/index.ts";
let s = await Bun.file(path).text();

s = s.replace(
  `const builtinTypologyJsons = [
  typologyPrimitiveBoxJson,
  typologyCurveArcJson,
  typologyCurveCircleJson,
  typologyCurveControlPointCurveJson,
  typologyCurveInterpolateCurveJson,
  typologyCurveLineJson,
  typologyCurvePolylineJson,
  typologyEditChamferJson,
  typologyEditExplodeJson,
  typologyEditFilletJson,
  typologyEditJoinJson,
  typologyEditSplitJson,
  typologyEditTrimJson,
  typologyFeatureExtrudeWireJson,
  typologyFeatureOffsetSurfaceJson,
  typologyMeasureAreaJson,
  typologyMeasureLengthJson,
  typologySolidBooleanDifferenceJson,
  typologySolidBooleanIntersectionJson,
  typologySolidBooleanUnionJson,
  typologySolidCylinderJson,
  typologySolidSphereJson,
  typologySurfaceExtrudeCrvJson,
  typologySurfaceLoftJson,
  typologySurfaceNetworkSrfJson,
  typologySurfacePlaneJson,
  typologySurfaceSweep1Json,
  typologySurfaceSweep2Json,
  typologyTransformCopyJson,
  typologyTransformMirrorJson,
  typologyTransformMoveJson,
  typologyTransformRotateJson,
  typologyTransformScale1dJson,
  typologyTransformScale3dJson,
] as const;

function builtinTypologyCatalog(): readonly TypologySpec[] {
  return builtinTypologyJsons.map((raw) => parseTypologySpec(raw)).filter((spec): spec is TypologySpec => spec !== null);
}

const extensionTypologyJsons = [buildingTypologyMushroomColumnJson, buildingTypologyWallJson] as const;

function extensionTypologyCatalog(): readonly TypologySpec[] {
  return extensionTypologyJsons.map((raw) => parseTypologySpec(raw)).filter((spec): spec is TypologySpec => spec !== null);
}

function shippedTypologyCatalog(): readonly TypologySpec[] {
  return [...builtinTypologyCatalog(), ...extensionTypologyCatalog(), ...extensionViewTypologyCatalog()];
}`,
  `function shippedTypologyCatalog(): readonly TypologySpec[] {
  return modelDefinitionTypologyCatalog()
    .map((raw) => parseTypologySpec(raw))
    .filter((spec): spec is TypologySpec => spec !== null);
}`,
);

s = s.replace(
  `/** @emoji 📚 Built-in extension manifest (\`spatial/assets/extension/builtin/extension.json\`). */
export function builtinExtensionManifest(): ExtensionManifest | null {
  return parseExtensionManifest(builtinExtensionManifestJson);
}

/** @emoji 📚 Lists typology assets shipped under \`spatial/assets/extension/builtin/typology/**\`. */
export function listBuiltinTypologies(): readonly TypologySpec[] {
  return builtinTypologyCatalog();
}`,
  `/** @emoji 📚 Geometry model-definition manifest (\`spatial/assets/modelDefinition/geometry/extension.json\`). */
export function geometryModelDefinitionManifest(): ModelDefinitionManifest | null {
  const raw = Object.values(geometryModelDefinitionManifestModule)[0];
  return raw ? parseModelDefinitionManifest(raw) : null;
}

/** @emoji 📚 Lists typologies from shipped \`spatial/assets/modelDefinition/**\` assets. */
export function listModelDefinitionTypologies(): readonly TypologySpec[] {
  return shippedTypologyCatalog();
}

/** @emoji 📚 Lists typology assets shipped under geometry model definition (alias). */
export function listBuiltinTypologies(): readonly TypologySpec[] {
  return listModelDefinitionTypologies();
}`,
);

s = s.replace(
  `export function listBuiltinActionSpecs(): readonly ActionSpec[] {
  return (builtinActionsJson as unknown[]).map((raw) => parseActionSpec(raw)).filter((spec): spec is ActionSpec => spec !== null);
}

const extensionActionJsons = [
  buildingActionConstructExtrudedMushroomColumnJson,
  buildingActionConstructFullyQuadraticMushroomColumnJson,
  buildingActionConstructMushroomColumnJson,
  buildingActionConstructRectangularMushroomColumnWithQuadraticSlabJson,
  buildingActionConstructVerticalWallJson,
  buildingActionConstructWallFromBottomAndTopJson,
  buildingActionConstructWallFromHorizontalPathAndProfileJson,
  buildingActionConstructWallFromHorizontalPathAndProfilesJson,
] as const;

function shippedActionCatalog(): readonly ActionSpec[] {
  return [...listBuiltinActionSpecs(), ...extensionActionJsons.map((raw) => parseActionSpec(raw)).filter((spec): spec is ActionSpec => spec !== null)];
}`,
  `export function listModelDefinitionActionSpecs(): readonly ActionSpec[] {
  return modelDefinitionActionCatalog()
    .map((raw) => parseActionSpec(raw))
    .filter((spec): spec is ActionSpec => spec !== null);
}

/** @emoji 📚 Lists declarative actions from model-definition assets (alias). */
export function listBuiltinActionSpecs(): readonly ActionSpec[] {
  return listModelDefinitionActionSpecs();
}

function shippedActionCatalog(): readonly ActionSpec[] {
  return listModelDefinitionActionSpecs();
}`,
);

s = s.replace(
  `const builtinInteractionJsons = [
  createAnchorInteractionJson,
  ...selectionOperationInteractionFixtures,
  boxInteractionJson,
  extrudeWireInteractionJson,
  offsetSurfaceInteractionJson,
  distanceInteractionJson,
  areaInteractionJson,
  curveArcInteractionJson,
  curveCircleInteractionJson,
  curveControlPointCurveInteractionJson,
  curveInterpolateCurveInteractionJson,
  curveLineInteractionJson,
  curvePolylineInteractionJson,
  editChamferInteractionJson,
  editExplodeInteractionJson,
  editFilletInteractionJson,
  editJoinInteractionJson,
  editSplitInteractionJson,
  editTrimInteractionJson,
  solidBooleanDifferenceInteractionJson,
  solidBooleanIntersectionInteractionJson,
  solidBooleanUnionInteractionJson,
  solidCylinderInteractionJson,
  solidSphereInteractionJson,
  surfaceExtrudeCrvInteractionJson,
  surfaceLoftInteractionJson,
  surfaceNetworkSrfInteractionJson,
  surfacePlaneInteractionJson,
  surfaceSweep1InteractionJson,
  surfaceSweep2InteractionJson,
  transformCopyInteractionJson,
  transformMirrorInteractionJson,
  transformMoveInteractionJson,
  transformRotateInteractionJson,
  transformScale1dInteractionJson,
  transformScale3dInteractionJson,
] as readonly BuiltinInteractionFixture[];`,
  `const shippedInteractionJsons = modelDefinitionInteractionCatalog() as readonly BuiltinInteractionFixture[];`,
);

s = s.replaceAll("builtinInteractionJsons", "shippedInteractionJsons");

s = s.replace(`/** @emoji 📚 Host-facing built-in interaction row (\`spatial/assets/extension/builtin/interaction/<group>/*.json\`). */`, `/** @emoji 📚 Host-facing interaction row (\`spatial/assets/modelDefinition/**/interaction/*.json\`). */`);

s = s.replace(`/** @emoji 📚 Built-in interaction ids for host interaction surfaces (\`spatial/assets/extension/builtin/interaction/<group>/*.json\`). */`, `/** @emoji 📚 Interaction ids from shipped model-definition assets. */`);

await Bun.write(path, s);
console.log("patched core catalogs");
