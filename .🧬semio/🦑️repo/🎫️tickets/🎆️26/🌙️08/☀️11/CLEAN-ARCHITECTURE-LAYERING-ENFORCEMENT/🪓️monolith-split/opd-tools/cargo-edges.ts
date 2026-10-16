// 🔗️ Lock propagation for the opd slice: new manifest edges across every Cargo.lock (dry run unless --write). Phase "a" = dependencies added with the consumer re-points; phase "b" = the os-pack schema-index removal (after the duplicate is deleted).
import {propagateEdges, propagateRemovals, DRY, log, type Edge} from "./lib.ts";
const phase = process.argv.includes("--phase=b") ? "b" : "a";
if (phase === "a") {
  const edges: Edge[] = [
    {consumer: "semio-framework-os-pack", dep: "semio-framework-pack-record-schema", dev: false},
    {consumer: "semio-framework-os-pack", dep: "semio-framework-trace-memory-testing", dev: true},
    {consumer: "semio-framework-os-mcp", dep: "semio-framework-pack", dev: true},
    {consumer: "semio-framework-plugin-decl-surface", dep: "semio-framework-pack", dev: false},
  ];
  const {changed, skipped} = propagateEdges(edges, []);
  log(`${DRY ? "(dry) " : ""}phase a: ${changed.length} locks changed, ${skipped.length} skipped`);
  for (const s of skipped) log("  skipped " + s);
} else {
  const changed = propagateRemovals([{consumer: "semio-framework-os-pack", dep: "semio-framework-pack-schema-index", dev: false}, {consumer: "semio-framework-os-pack", dep: "semio-framework-pack-json", dev: false}]);
  log(`${DRY ? "(dry) " : ""}phase b: ${changed.length} locks changed`);
}
