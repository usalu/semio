import { discoverCatalogPackages, getWorkspaceRoot, registryCatalogInputView } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { TAXONOMY, generateComponentSourceRegistry } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";
import { installationDirectoryEmoji, parseInstallationDirectoryV1 } from "../../../../../../../🧰️framework/🔨️modules/🪪️identity/📁️installation/🟦️.ts";
const repoRoot = getWorkspaceRoot();
const view = registryCatalogInputView(repoRoot, TAXONOMY);
const packages = discoverCatalogPackages(repoRoot, TAXONOMY, view);
const rows = generateComponentSourceRegistry(repoRoot, { packages, view }).flatMap((r: any) => r.directoryName === undefined ? [] : [{ pluginId: r.pluginId, directoryName: r.directoryName }]);
const by = new Map<string, string[]>();
for (const r of rows) { const e = installationDirectoryEmoji(parseInstallationDirectoryV1(r.directoryName)); by.set(e, [...(by.get(e) ?? []), `${r.pluginId}=${r.directoryName}`]); }
const ids = new Map<string, number>(); for (const r of rows) ids.set(r.pluginId, (ids.get(r.pluginId) ?? 0) + 1);
console.log("rows", rows.length);
for (const [e, v] of by) if (v.length > 1) console.log("DUP emoji", e, v);
for (const [i, n] of ids) if (n > 1) console.log("DUP id", i, n);
const cand = ["🏡","🏬","🏰","🏚","🏫","🏤","🏪","🏩","🏨","🏭","🏯","🗼","🏠","🏢"];
for (const c of cand) console.log(c, by.has(c) ? "USED " + by.get(c)!.join(",") : "free");
