/** 🌉️ Writes the component bridge the current sources generate for a staged guest, for probes that route it in place of the staged one. */
import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
const { pluginComponentBridgeSource } = await import(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts"));
writeFileSync(process.argv[2]!, pluginComponentBridgeSource("semio_hub_draw_component", "semio_hub_draw_component.core.wasm"));
