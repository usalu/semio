import { toasterManifest } from "./toaster.mjs";
import { vacuumManifest } from "./vacuum.mjs";
import { heaterManifest } from "./heater.mjs";
const rows = [toasterManifest, vacuumManifest, heaterManifest].map((r) => JSON.stringify(r, null, 2).split("\n").map((l) => "    " + l).join("\n")).join(",\n");
console.log(rows);
