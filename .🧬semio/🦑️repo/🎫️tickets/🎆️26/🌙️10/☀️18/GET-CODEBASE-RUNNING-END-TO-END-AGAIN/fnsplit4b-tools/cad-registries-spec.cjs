const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🗂️registries/🟦️.ts";
write({
  file, function: "registerRegistryTests",
  kit: kit("🧰", "kit", "createRegistriesTestKit", "🏭", "The shared cad-js registries test kit: the destructured vitest, the cad-js core API, the Brepjs kernel and the precise spatial math.", "Creates the shared registries test kit of one registration from the injected dependencies.", "RegistriesTestKit"),
  groups: [{
    dir: dir("🗂", "suite"), doc: doc("🗂", "The cad-js core action and interaction registries describe block: its wrapper and parts."), name: "registerActionAndInteractionRegistrySuite", emoji: "🏛", fdoc: "Registers the cad-js core action and interaction registries describe block from its parts.",
    describeLine: 25,
    parts: [
      g("📜", "docs", "registerActionDocumentAndTypologyKitTests", "🧱", "The data-only action documents, typology kits and interpolate curve rollback tests.", "Registers the executable field rejection, model-definition action loading, typology construct kits and the interpolate curve rollback tests.", 26, 116),
      g("➰", "curves", "registerCurveInterpolationStepTests", "🧵", "The interpolate curve step, numeric entry and commit tests.", "Registers the interpolate curve finalize, numeric lock, one point confirm and commit binding tests.", 136, 189),
      g("🔧", "diffs", "registerModelDiffAndTypologyModeTests", "📐", "The nurbs pole model diffs, box commit and typology mode action tests.", "Registers the nurbs pole sync, selection target transforms, box commit binding and typology mode action tests.", 216, 327),
      g("📞", "nested", "registerNestedInteractionCallTests", "🪆", "The nested interaction call rollback, output mapping and arbitrary nesting tests.", "Registers the nested interaction call abort rollback, the child output mapping and the arbitrarily nested session tests.", 333, 441),
      g("📚", "loading", "registerRegistryLoadingAndBoxFootprintTests", "🗃", "The model-definition registry registration, spatial interaction loading and box footprint tests.", "Registers the model-definition action and interaction registration, the spatial interaction loading and the box triplet footprint tests.", 563, 600),
      g("🎯", "select", "registerSelectionCommandTests", "🖱", "The selection command, apply, headless and chain tests.", "Registers the selection modifiers, apply, selectAll, invert, headless history, registered operation and chain tests.", 625, 757),
    ],
  }],
  rootKeep: [],
}, process.argv[2]);
