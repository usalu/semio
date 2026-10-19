const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪟️tree-windows/🟦️.tsx";
write({
  file, function: "registerTests1",
  kit: kit("🧰", "tree-windows-test-kit", "createTreeWindowsTestKit", "🏭", "The shared interpreted tree window test kit: the destructured interpreter and ui-react test API, the node and tree item builders, the scroll stubs and the shared window fixtures.", "Creates the shared tree window test kit of one registration from the injected dependencies.", "TreeWindowsTestKit"),
  groups: [
    g("📖", "history-scroll-layout", "registerHistoryScrollLayoutTests", "🗂", "The authored history scroll layout tree viewport test.", "Registers the authored history scroll layout as the Tree viewport test.", 39, 39),
    {
      dir: dir("🪟", "interpreted-tree-windows"), doc: doc("🪟", "The interpreted tree windows describe block: its wrapper and parts."), name: "registerInterpretedTreeWindowsSuite", emoji: "🌲", fdoc: "Registers the interpreted tree windows describe block from its parts.",
      describeLine: 136,
      parts: [
        g("🧬", "window-data-and-granularity-picks", "registerWindowDataAndGranularityPickTests", "🎯", "The authored window data and granularity row pick tests.", "Registers the cleanup hook, the authored window and key data and the granularity pick tests.", 137, 180),
        g("📜", "scroll-container-resolution", "registerScrollContainerResolutionTests", "📏", "The scroll container resolution and viewport measuring tests.", "Registers the scroll container resolution, overflow candidate and client box measuring tests.", 202, 251),
        g("🪜", "window-requests-and-nesting", "registerWindowRequestsAndNestingTests", "📨", "The window request, idle re-measure and nested container tests.", "Registers the scroll anchored window request, the answered window, the idle re-measure and the nested container tests.", 265, 327),
        g("🧱", "fold-seeding-ids-and-duplicates", "registerFoldSeedingIdsAndDuplicateTests", "🗝", "The below the fold seeding, inline control ids, duplicate key and range pick tests.", "Registers the below the fold seeding, inline control ids, same key windows, duplicate key warning, body row cap and range pick tests.", 370, 520),
      ],
    },
    g("📐", "neutral-viewport-vectors", "registerNeutralViewportVectorTests", "🧮", "The neutral viewport vectors both hosts answer.", "Registers the neutral viewport vector tests.", 539, 539),
    g("🧠", "short-answer-window", "registerShortAnswerWindowTests", "✂", "The window the guest answers short tests.", "Registers the short answered window tests.", 554, 554),
  ],
  rootKeep: [],
}, process.argv[2]);
