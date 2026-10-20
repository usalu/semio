const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "✏️s/🧑‍💻dev/📐️cad/🧪️tests/🔮️spatial-kernel/📐️geometry/🧪️tests/🧪️semio-tech-cad-js-core-vec/🟦️.ts";
write({
  file, function: "registerTests1",
  kit: kit("🧰", "vec-test-kit", "createVecTestKit", "🏭", "The shared cad-js core test kit: the destructured vitest, the core dependencies, the Brepjs kernel and the precise spatial math.", "Creates the shared vec test kit of one registration from the injected dependencies.", "VecTestKit"),
  groups: [
    g("📏", "vec-and-model-definition-catalogs", "registerVecAndModelDefinitionCatalogTests", "📚", "The vec and model definition catalog tests.", "Registers the vec arithmetic tests and the model definition catalog tests.", 48, 53),
    g("🔁", "model-space-and-transformations", "registerModelSpaceAndTransformationTests", "🧮", "The model space, hashing and transformation tests.", "Registers the model space and hashing tests and the transformation tests.", 235, 267),
    g("🧊", "validation-geometry-and-metadata", "registerValidationGeometryAndMetadataTests", "🏷", "The attribute validation, edge and solid geometry, expr, model json, metadata and selection filter tests.", "Registers the attribute validation, edge and solid geometry, expr, model json, metadata and selection filter tests.", 465, 609),
  ],
  kitInit: [42],
  rootKeep: [],
}, process.argv[2]);
