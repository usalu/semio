const { R, g, dir, doc, write } = require("./lib_ks.cjs");
const file = R + "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🎞️react-deck/🟦️.tsx";
write({
  file, describeLine: 9,
  shared: { dir: dir("🧰", "react-deck-fixtures"), doc: doc("🧰", "The shared react deck test fixtures.") },
  parts: [
    g("🎬", "focus-to-labels-morph", "registerFocusToLabelsMorphTests", "🏷", "The catalogue focus to inline column labels auto animate test.", "Registers the catalogue focus to inline column labels auto animate test.", 10, 10),
    g("👻", "ghosts-and-wrappers", "registerGhostAndWrapperTests", "🪞", "The catalogue rest figure, ghost anchor and tile wrapper tests.", "Registers the catalogue rest figure, target ghost, reveal data id and focus layout tests.", 133, 208),
    g("🧭", "label-frames-and-advance", "registerLabelFramesAndAdvanceTests", "📍", "The catalogue label frame placement and focus advance tests.", "Registers the catalogue label ghost frame and the focus to labels advance tests.", 232, 258),
  ],
}, process.argv[2]);
