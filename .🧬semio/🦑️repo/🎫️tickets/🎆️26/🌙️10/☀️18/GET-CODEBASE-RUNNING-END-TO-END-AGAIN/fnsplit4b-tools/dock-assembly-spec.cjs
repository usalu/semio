const { R, f, ph, write } = require("./lib_ps.cjs");
const file = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🏗️dock-assembly/🟦️.ts";
write({
  file, function: "useDockAssembly", keep: [], resolve: true, typesFromProducers: true,
  files: [
    f("🧱", "dock-model-and-persistence", "The dock model assembly, the persisted dock skeleton and ui state and the tab and tree unit drop handlers.", [
      ph("useDockModel", "🏗", "Assembles the default dock, the host dock and the mobile panel tabs from the shell core.", 67, 131),
      ph("useDockPersistence", "💾", "Persists the dock skeleton and ui state once and on every change and builds the tab and tree unit drop handlers.", 160, 221),
    ]),
    f("🎓", "override-anchors-and-introduction", "The host, studio and details override anchors and the introduction step derived ids with their reveal effects.", [
      ph("useOverrideAnchorsAndIntroductionIds", "🧭", "Derives the override anchors and every introduction element, utility, window and tool id.", 234, 314),
      ph("useIntroductionRevealEffects", "🎬", "Reveals the introduction tool, tool pick, panel tab and expand targets as the steps advance.", 322, 402),
    ]),
    f("🪟", "panel-reveal-and-tool-tabs", "The panel active paths, the history tab and override tab reveals, the time travel reveal and the tool tab selection effects.", [
      ph("usePanelPathsAndOverrideReveals", "🗺", "Derives the panel active paths and reveals the history tab, studio override and tool run panels.", 426, 493),
      ph("useToolTabSelectionEffects", "🛠", "Selects, activates and reveals the tool tabs and the focused panel and details override tabs.", 527, 728),
    ]),
  ],
}, process.argv[2]);
