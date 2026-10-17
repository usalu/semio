import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const E = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers";
const target = process.argv[2] === "barrel" ? `${E}/🟦️.tsx` : `${E}/🔩️plumbing/👥️presence-identity/🟦️.ts`;
const helpers = readModuleTreeSource(target);
console.log("hub presence check needle:", helpers.includes("PRESENCE_HEARTBEAT_INTERVAL_MS = 5000"));
const tw = process.argv[2] === "barrel" ? `${E}/🟦️.tsx` : `${E}/🌳️tree-windows/🟦️.tsx`;
const react = readModuleTreeSource(tw);
console.log("wgpu reconcile needle 1:", react.includes("uiIntentToActionDescriptor(intent: UiIntent)"));
console.log("wgpu reconcile needle 2:", react.includes("actionBindingToActionDescriptor({ action: intent.action, args: null })"));
