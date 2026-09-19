/** 🩺️ Slice B3a — `gis2d` (gismap, react :6040) interaction probe.
 *
 * Every gis verb the palette can stage with an argument is an `ActionKind::View` (camera, render
 * mode, vector style, LOD, layer visibility/stroke) — by the manifest's own contract those amend
 * window config and never the document, so none of them can clear the interaction bar. The document
 * mutation a user can actually reach from the rail is the framework clipboard verb over a selection:
 * `selectAll` stages the whole demo map, `cut` removes it (a real batched delete with a true
 * inverse), and `undo` restores it. */
import { runInteractionProbe } from "./🐍️b3a-interaction-probe.mjs";
await runInteractionProbe({ plugin: "gis2d", variant: "gis2d", port: 6040, setup: ["selectAll"], action: "cut" });
