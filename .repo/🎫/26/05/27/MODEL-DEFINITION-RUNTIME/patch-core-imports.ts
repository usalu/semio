#!/usr/bin/env bun
const path = "c:/git/semio/spatial/js/core/index.ts";
const replacement = `// #region 📥ModelDefinitionAssets
import geometryLoomFixtureJson from "../../fixtures/geometry-loom.json";
import geometryRoutesFixtureJson from "../../fixtures/geometry-routes.json";
import smallBuildingModelFixtureJson from "../../fixtures/small-building.model.json";

const modelDefinitionTypologyModules = import.meta.glob("../../assets/modelDefinition/**/typology.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const modelDefinitionActionModules = import.meta.glob("../../assets/modelDefinition/**/action/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const modelDefinitionInteractionModules = import.meta.glob("../../assets/modelDefinition/**/interaction/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const modelDefinitionManifestModules = import.meta.glob("../../assets/modelDefinition/**/modelDefinition.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const geometryModelDefinitionManifestModule = import.meta.glob("../../assets/modelDefinition/geometry/extension.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

function modelDefinitionTypologyCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionTypologyModules);
}

function modelDefinitionActionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionActionModules);
}

function modelDefinitionInteractionCatalog(): readonly unknown[] {
  return Object.values(modelDefinitionInteractionModules);
}

function modelDefinitionManifestCatalog(): readonly unknown[] {
  return [...Object.values(modelDefinitionManifestModules), ...Object.values(geometryModelDefinitionManifestModule)];
}
// #endregion 📥ModelDefinitionAssets`;

const s = await Bun.file(path).text();
const startMarker = "// #region 📥InteractionAssets";
const endMarker = "// #endregion 📥InteractionAssets";
const start = s.indexOf(startMarker);
const end = s.indexOf(endMarker);
if (start < 0 || end < 0) throw new Error("markers not found");
const endPos = end + endMarker.length;
await Bun.write(path, s.slice(0, start) + replacement + s.slice(endPos));
console.log("patched core imports");
