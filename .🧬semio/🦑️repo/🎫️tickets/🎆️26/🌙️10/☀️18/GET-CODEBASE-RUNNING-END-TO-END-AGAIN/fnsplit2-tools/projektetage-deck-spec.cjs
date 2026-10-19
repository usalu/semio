const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🧪️projektetage-deck/🟦️.ts";
const spec = {
  file, function: "registerProjektetageDeckTests",
  kit: kit("🧰", "deck-test-kit", "createProjektetageDeckTestKit", "🏭", "The shared projektetage deck test kit: the destructured deck module exports, the vitest functions and the bauteilbörsen thought lookup.", "Creates the shared deck test kit from the injected dependencies and the vitest functions.", "ProjektetageDeckTestKit"),
  groups: [
    {
      dir: dir("🎞", "deck-contract-describe"), doc: doc("🎞", "The projektetage deck describe block: its wrapper and parts."), name: "registerDeckContractTests", emoji: "🪄", fdoc: "Registers the projektetage deck describe block from its parts.",
      describeLine: 18,
      parts: [
        g("🧭", "structure-and-bookends", "registerStructureAndBookendTests", "📑", "The deck structure, German bookmark and bookend slide tests.", "Registers the intro and render slide, German bookmark and Zukunft Bau bookend tests.", 19, 54),
        g("🖼", "catalogue-morphs", "registerCatalogueMorphTests", "🎭", "The catalogue morph and auto animate tests.", "Registers the catalogue assembly, morph target, focus, auto animate and label morph tests.", 93, 154),
        g("🧱", "tool-slides", "registerToolSlideTests", "🛠", "The slide path, puzzle, catalog step, bauteillogik and modell slide tests.", "Registers the slide path loading, puzzle, entwurfswerkzeug catalog, bauteillogik and modell iframe tests.", 171, 280),
        g("⌨", "input-and-check-slides", "registerInputAndCheckSlideTests", "✋", "The connectivity, input type, input surface and manual check slide tests.", "Registers the Generatoren Konnektivität, Eingabearten, Eingabeoberfläche and Manuelles Prüfen slide tests.", 298, 385),
        g("📥", "import-and-typology-slides", "registerImportAndTypologySlideTests", "🌳", "The import, output and typology slide tests.", "Registers the import example, import processing, output json and typology figure tests.", 405, 515),
        g("🔎", "research-and-component-slides", "registerResearchAndComponentSlideTests", "🧩", "The research and component grid slide tests.", "Registers the Schweiz research figure and the Baukomponenten grid tests.", 535, 584),
      ],
    },
  ],
  rootKeep: [],
};
write(spec, process.argv[2]);
