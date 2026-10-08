import { generatePluginRegistryReport } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";
const report = generatePluginRegistryReport(process.cwd(), { staleChannel: "exclude" });
console.log(JSON.stringify(report.entries.filter((entry) => entry.pluginId === "bim"), null, 1));
console.log(`admitted=${report.entries.length} stale=${report.diagnostics.length}`);
