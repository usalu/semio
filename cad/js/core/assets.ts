// #region 🧲Header
/** @emoji 📥 Loads `cad/assets/modelDefinition` into `@cad/js/core` via `registerModelDefinitionAssets`. */
// #endregion 🧲Header

// #region 📥ModelDefinitionAssets
import { registerModelDefinitionAssets } from "./model-definition-registry.ts";

const typologies = import.meta.glob(["../../assets/modelDefinition/**/typology.json", "../../assets/modelDefinition/**/typology/*.json"], {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const actions = import.meta.glob("../../assets/modelDefinition/**/action/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const interactions = import.meta.glob("../../assets/modelDefinition/**/interaction/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const manifests = import.meta.glob("../../assets/modelDefinition/**/modelDefinition.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const attributes = import.meta.glob("../../assets/modelDefinition/**/attributeDefinition/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const propertyDefinitions = import.meta.glob(
  ["../../assets/modelDefinition/**/propertyDefinition/*.json", "../../assets/modelDefinition/**/propertyKind/*.json"],
  {
    eager: true,
    import: "default",
  },
) as Record<string, unknown>;

const properties = import.meta.glob("../../assets/modelDefinition/**/property/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const transformations = import.meta.glob("../../assets/modelDefinition/**/transformation/**/*.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

const extensions = import.meta.glob("../../assets/modelDefinition/**/extension.json", {
  eager: true,
  import: "default",
}) as Record<string, unknown>;

registerModelDefinitionAssets({
  typologies,
  actions,
  interactions,
  manifests,
  extensions,
  attributes,
  propertyDefinitions,
  properties,
  transformations,
});
// #endregion 📥ModelDefinitionAssets
