import { dependencyTruthReport } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts";
const t = Date.now();
const r = dependencyTruthReport(process.cwd());
console.error("ms", Date.now() - t);
console.log(JSON.stringify({ totals: r.totals, oracleConflicts: r.oracleConflicts, toolchainConflicts: r.toolchainConflicts, audited: r.auditedToolchain, prod: r.entries.raw.filter((e) => e.productionReachable) }, null, 1));
