const { R, dir, doc, g, kit, write } = require("./lib_ks.cjs");
const file = R + "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx";
write({
  file, function: "registerTests1",
  kit: kit("🧰", "repl-test-kit", "createReplTestKit", "🏭", "The shared cad-js renderer test kit: the destructured vitest, the cad renderer runtime and kernel and the precise spatial math.", "Creates the shared repl test kit of one registration from the injected dependencies.", "ReplTestKit"),
  groups: [
    g("🔤", "repl-text-and-filters", "registerReplTextAndFilterTests", "🧹", "The repl suggestion detail, action text, filter, space and typing target tests.", "Registers the suggestion detail, action text normalization, suggestion filter, space interaction and query typing target tests.", 23, 88),
    g("🔎", "repl-engagement-and-search", "registerReplEngagementAndSearchTests", "🧭", "The interaction repl engagement and search tests.", "Registers the interaction repl engagement and search builder tests.", 104, 104),
    {
      dir: dir("🖼", "adapter-suite"), doc: doc("🖼", "The cad-js renderer interaction adapter describe block: its wrapper and parts."), name: "registerRendererInteractionAdapterSuite", emoji: "🪟", fdoc: "Registers the cad-js renderer interaction adapter describe block from its parts.",
      describeLine: 228,
      parts: [
        g("🎯", "ray-projection-and-picking", "registerRayProjectionAndPickingTests", "📍", "The CAD axes, ray projection, ground pick and geometry pick target tests.", "Registers the CAD axes, host picking states, ray projections, ground pick planes and geometry pick target tests.", 229, 343),
        g("〰", "nurbs-and-visibility", "registerNurbsPolesAndSceneVisibilityTests", "👁", "The nurbs poles, scene visibility and gumball transform tests.", "Registers the wire segments, nurbs poles, scene visibility toggles and gumball translate, rotate and scale tests.", 354, 509),
        g("🎚", "toggles-and-filters", "registerChromeTogglesAndPickFilterTests", "🔀", "The repl chrome toggles and the pick target filter tests.", "Registers the chrome toggle seeds, footprint preview, primitive and typology toggle filters and selection scope tests.", 521, 637),
        g("🖱", "hover-materials-and-isolation", "registerHoverMaterialsAndIsolationTests", "🔦", "The hover key, committed mesh material, selection isolation and toggle group tests.", "Registers the hover key aliases, committed mesh materials, per state selection isolation and toggle group state tests.", 643, 720),
        g("🎨", "pick-render-and-styles", "registerPickRenderingAndTypologyStyleTests", "🖌", "The pick rendering, reveal, palette and typology style tests.", "Registers the hidden and locked entity flags, pick rendering, reveal, palette, planar face shading and typology style tests.", 729, 937),
      ],
    },
    g("📊", "model-stats-pane", "registerModelStatsPaneTests", "📈", "The model stats pane tests.", "Registers the ModelStatsPane tests.", 944, 944),
  ],
  kitInit: [18],
  rootKeep: [],
}, process.argv[2]);
