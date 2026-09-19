/** 🩺️ Slice B3a2 — `gis2d` (gismap, react :6040) interaction probe.
 *
 * B3a measured the rail against the framework clipboard route (`selectAll` → `cut`) because every
 * argument-carrying gis verb was an `ActionKind::View`. B3a2 §9.2 landed four per-feature document
 * verbs instead, each fully defaulted so the rail dispatches them one-click: `addFeature` mints the
 * lowest free `position-N` at the staged `(lon, lat)` and runs through the collection's authored
 * `create-position` leaf, so its inverse is the leaf's own and undo/redo are structural.
 *
 * `SEMIO_PROBE_ACTION` swaps the measured verb (`moveFeature`/`renameFeature`/`deleteFeature` all
 * address the collection's newest entry when no `featureId` is staged, so they run right after an
 * `addFeature` with nothing typed).
 */
import { runInteractionProbe } from "./🐍️b3a-interaction-probe.mjs";

const action = process.env.SEMIO_PROBE_ACTION ?? "addFeature";
const setup = action === "addFeature" ? [] : ["addFeature"];
await runInteractionProbe({ plugin: `gis2d-${action}`, variant: "gis2d", port: 6040, setup, action });
