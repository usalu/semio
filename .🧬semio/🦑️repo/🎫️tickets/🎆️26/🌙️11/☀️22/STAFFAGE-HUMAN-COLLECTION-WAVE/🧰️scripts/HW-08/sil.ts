/** 🌓️ Runs the catalog-silhouette gate (bbox, IoU, Hausdorff per scale and view) on the given slugs. */
import { loadDoc, silhouette } from "./lib.ts";
for (const slug of process.argv.slice(2)) { console.log(slug); for (const l of silhouette(loadDoc(slug).asset)) console.log("  " + l); }
