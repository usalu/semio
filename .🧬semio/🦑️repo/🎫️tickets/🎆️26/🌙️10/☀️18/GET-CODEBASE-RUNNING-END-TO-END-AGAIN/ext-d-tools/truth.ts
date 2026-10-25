import { dependencyTruthReportFromEntries } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts";
import { dependencyCollectGo, dependencyFreezeCurrentThirdParty, dependencyReadFileSafe, DEPENDENCY_TOOLCHAIN_RECIPE_MANIFESTS } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🟦️.ts";
const root = process.cwd();
const t = Date.now();
const first = new Map<string, any>();
dependencyCollectGo(root, () => {}, (e: any) => { const k = `${e.ecosystem}:${e.name}`; if (!first.has(k)) first.set(k, e); });
const r = dependencyTruthReportFromEntries(dependencyFreezeCurrentThirdParty(root), [...first.values()], dependencyReadFileSafe(root, "package.json"), dependencyReadFileSafe(root, "bun.lock"), Object.fromEntries([...DEPENDENCY_TOOLCHAIN_RECIPE_MANIFESTS].map(([m, l]) => [m, dependencyReadFileSafe(root, l)])));
console.error("ms", Date.now() - t);
console.log(JSON.stringify({ totals: r.totals, oracleConflicts: r.oracleConflicts, toolchainConflicts: r.toolchainConflicts, audited: r.auditedToolchain, literal: r.entries.literalExternal }, null, 1));
