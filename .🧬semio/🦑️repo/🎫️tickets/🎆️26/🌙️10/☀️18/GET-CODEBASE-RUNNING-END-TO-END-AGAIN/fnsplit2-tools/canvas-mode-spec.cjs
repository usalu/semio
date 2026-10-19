const { R, f, ph, write, dir, doc } = require("./lib_ps.cjs");
const spec = {
  file: R + "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx", function: "Mode", arrowVar: true, keep: [], resolve: true, typesFromProducers: true, allowExportedMoves: true, typeOverrides: { children: "React.ReactNode" },

  files: [
    f("🧭", "mode-layout-state-and-sync", "The layout state, the refs and the prop synchronization effects of the docking mode shell.", [
      ph("useModeLayoutState", "🗺", "Binds the translation, the window indexes, the layout state, the refs and the prop synchronization effects.", 98, 156),
    ]),
    f("🖱", "mode-drag-and-window-callbacks", "The window, drag and axis callbacks and effects of the docking mode shell.", [
      ph("useModeWindowCallbacks", "🪟", "Builds the stack drop registration, the window activation, close and maximize callbacks and the pointer drag callbacks.", 185, 284),
      ph("useModeDragEffectsAndAxisCallbacks", "🧲", "Runs the pointer drag effects and builds the axis layout, group, element and join corner callbacks.", 298, 414),
    ]),
    f("📥", "mode-template-drop-and-contexts", "The template drop, preview, shortcut and context bindings of the docking mode shell.", [
      ph("useModeTemplateDropAndPreview", "🧺", "Builds the template drop callbacks and effects and the drag preview values.", 424, 559),
      ph("useModeShortcutsAndContexts", "⌨", "Builds the new window callback, the keybindings, the dock and render contexts and the rendered body.", 561, 655),
    ]),
  ],
};
write(spec, process.argv[2]);
