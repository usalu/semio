/** 📕️ norm interaction probe (variant `din4108`, port 6091).
 * Bar: the seeded DIN 4108 document renders 19 evaluated checks, `setSnapshot` — the app's ONE
 * document-mutating verb (`evaluate` recomputes, `setSelectedCheckIndex` writes window config) —
 * commits a real replacement, undo retires it, redo reapplies it.
 *
 * The staged document is the crate's own committed `change-t-int-c` AFTER fixture (interior design
 * temperature 22.5 °C), read off disk rather than re-declared here, so the probe cannot drift from
 * the snapshot codec it is exercising.
 */
import { readFileSync } from "node:fs";
import { runInteractionProbe } from "./🐍️b2b-interaction-probe.mjs";

const FIXTURE =
  "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌡️change-t-int-c/🌡️raises-the-indoor-design-temperature-to-22-point-5-c/📸️snapshot/⬅️before/🔣️.json";

// 🌡️ The staged document is the loaded Demo document with EXACTLY ONE field moved (20 °C → 22.5 °C).
// `set_snapshot` decomposes its payload through `Din4108Mutation::from_snapshot`, and the norm artifact
// lane's authority is a ONE-ITEM preparation (`NormOneItemPreparationFactory`), so a payload differing
// in two or more fields is refused mid-flight — see §3.3 of the slice report. The two values patched
// below are where the committed fixture differs from `🖼️assets/🎬️demo/🗣️.dsl.semio`, i.e. this
// reconstructs the document the app actually has open.
const demo = JSON.parse(readFileSync(FIXTURE, "utf8"));
demo.layers[1].lambdaWMk = 0.035;
demo.moistureMuInterior = 1.3;
const snapshot = JSON.stringify({ ...demo, tIntC: 22.5 });

await runInteractionProbe({ plugin: "norm", variant: "din4108", port: 6091, action: "setSnapshot", args: { snapshot } });
