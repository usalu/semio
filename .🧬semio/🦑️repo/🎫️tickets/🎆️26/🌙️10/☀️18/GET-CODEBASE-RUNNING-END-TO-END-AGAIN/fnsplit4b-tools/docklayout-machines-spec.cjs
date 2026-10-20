const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "🧰️framework/🧪️tests/🧪️docklayoutstore/🎰️machines/🟦️.ts";
write({
  file, function: "registerMachineTests",
  kit: kit("🧰", "machine-test-kit", "createMachineTestKit", "🏭", "The shared statechart machine test kit: the destructured framework statechart API and the unit toggle, toggle, player, recorder and checkout fixture machines.", "Creates the shared machine test kit of one registration from the injected framework.", "MachineTestKit"),
  groups: [
    g("⚙", "statechart-kernel-and-model-suites", "registerStatechartKernelAndModelSuites", "🧪", "The TestHost, TraceInspector, kernel, persistence, ActorSystem, model testing and BitSet machine suites with their unit toggle, toggle, player and recorder fixture machines.", "Registers the machine TestHost, TraceInspector, kernel, persist and restore, ActorSystem, model testing and BitSet suites.", 234, 445),
    g("🛒", "checkout-dsl-twin", "registerCheckoutDslTwinTests", "🔗", "The checkout DSL twin integration machine tests.", "Registers the machine checkout DSL twin integration tests.", 477, 477),
  ],
  rootKeep: [],
}, process.argv[2]);
