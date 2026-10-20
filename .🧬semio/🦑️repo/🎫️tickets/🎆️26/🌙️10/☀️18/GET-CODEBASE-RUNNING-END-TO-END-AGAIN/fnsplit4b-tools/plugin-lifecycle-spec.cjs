const { R, f, ph, write, dir, doc } = require("./lib_ps.cjs");
const file = R + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🚀️plugin-lifecycle/🟦️.ts";
write({
  file, function: "usePluginLifecycle", keep: [], resolve: true, typesFromProducers: true,
  shared: { dir: dir("🧰", "lifecycle-types"), doc: doc("🧰", "The shared plugin lifecycle types: the extension ledger entry the lifecycle callbacks exchange.") },
  files: [
    f("🧱", "lifecycle-state-and-install", "The plugin lifecycle state, refs and effects and the plugin and hub document program install callbacks.", [
      ph("usePluginLifecycleState", "🗃", "Binds the shell core, the plugin lifecycle refs and the effects that keep them current.", 77, 237),
      ph("usePluginInstallCallbacks", "📥", "Builds the install plugin callback and the hub document program install callbacks.", 261, 402),
    ]),
    f("🔄", "reload-and-extension-callbacks", "The plugin reload, space extension operation and extension install, uninstall and enable callbacks.", [
      ph("usePluginReloadCallbacks", "🔁", "Builds the reload plugin callback and the space extension operation dispatcher.", 426, 596),
      ph("useExtensionCallbacks", "🧩", "Builds the install, uninstall and enable extension callbacks.", 617, 792),
    ]),
  ],
}, process.argv[2]);
