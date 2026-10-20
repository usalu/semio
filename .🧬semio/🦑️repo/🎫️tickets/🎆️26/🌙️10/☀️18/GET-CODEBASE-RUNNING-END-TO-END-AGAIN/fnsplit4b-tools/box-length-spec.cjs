const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/📦️box-length/🟦️.ts";
write({
  file, function: "registerBoxLengthEntryTests",
  kit: kit("🧰", "box-length-test-kit", "createBoxLengthTestKit", "🏭", "The shared box and length entry test kit: the destructured vitest, the cad-js core API, the Brepjs kernel and the precise spatial math.", "Creates the shared box and length entry test kit of one registration from the injected dependencies.", "BoxLengthTestKit"),
  groups: [
    g("📦", "box-interaction", "registerBoxInteractionTests", "🧊", "The cad-js core box interaction tests.", "Registers the cad-js core box interaction describe block.", 17, 17),
    g("📏", "length-entry-and-state-engine", "registerLengthEntryAndStateEngineTests", "⌨", "The cad-js core length entry and stateEngine option tests.", "Registers the cad-js core length entry and stateEngine option describe blocks.", 266, 478),
  ],
  rootKeep: [],
}, process.argv[2]);
