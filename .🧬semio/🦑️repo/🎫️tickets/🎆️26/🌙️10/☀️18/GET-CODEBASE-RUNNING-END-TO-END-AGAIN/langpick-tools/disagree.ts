/** ⚖️ Evidence law (red on live): runs every language picker copy of a root on the shared vectors and lists each row where it disagrees with the protocol (the Python oracle's answer). usage: bun disagree.ts <root> [vectors.json] */
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.argv[2]!, here = import.meta.dirname;
const vectors = JSON.parse(readFileSync(process.argv[3] ?? join(here, "src/vectors.json"), "utf8")) as { languages: string[]; resolve: { name: string; args: string[]; environment: Record<string, string>; os: string[] }[] };
const oracle = spawnSync("python3", ["-I", join(here, "src/oracle.py"), join(here, "src/protocol.json"), process.argv[3] ?? join(here, "src/vectors.json")], { encoding: "utf8" });
const truth = oracle.stdout.trim().split("\n").map((line) => JSON.parse(line) as { name: string; notices: string[] });
const M = (rel: string): string => join(root, "🧰️framework", rel);
const load = async (rel: string): Promise<Record<string, any> | undefined> => existsSync(M(rel)) ? await import(M(rel)) : undefined;
const withEnv = <T>(environment: Record<string, string>, run: () => T): T => {
  const saved = { ...process.env };
  for (const name of Object.keys(process.env)) if (/^(SEMIO_LANGUAGE|SEMIO_LOCALE|SEMIO_DASHBOARD_LOCALE|LANGUAGE|LC_ALL|LC_MESSAGES|LANG)$/u.test(name)) delete process.env[name];
  Object.assign(process.env, environment);
  try { return run(); } finally { for (const name of Object.keys(process.env)) if (!(name in saved)) delete process.env[name]; Object.assign(process.env, saved); }
};
const fromText = (text: string, en: string, de: string): string[] => [...(text.includes(en) ? ["en"] : []), ...(text.includes(de) ? ["de"] : [])];
type Copy = Readonly<{ name: string; where: string; run: (environment: Record<string, string>) => string[] }>;
const copies: Copy[] = [];
const removed: string[] = [];
const add = async (name: string, rel: string, make: (module: Record<string, any>) => Copy["run"], symbol?: string): Promise<void> => { const module = await load(rel); if (module && (symbol === undefined || symbol in module)) copies.push({ name, where: rel, run: make(module) }); else removed.push(name); };

await add("leases yield leaseLanguagesV1", "🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts", (m) => (e) => [...m.leaseLanguagesV1(e)]);
await add("leases order leaseLanguageV1", "🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts", (m) => (e) => [m.leaseLanguageV1(e)], "leaseLanguageV1");
await add("filesystem churn churnLanguagesV1", "🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts", (m) => (e) => [...m.churnLanguagesV1(e)]);
await add("sealed sealedLanguagesV1", "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts", (m) => (e) => [...m.sealedLanguagesV1(e)]);
await add("format-stamps formatStampLanguagesV1", "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs", (m) => (e) => [...m.formatStampLanguagesV1(e)]);
await add("playground native-host nativeHostLanguagesV1", "🛍️products/🦑️repo/🔨️modules/📚️library/🎮️playground/🖥️native-host/🟨️.mjs", (m) => (e) => [...m.nativeHostLanguagesV1(e)]);
await add("graph farm selectGraphFarmLanguagesV1", "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs", (m) => (e) => [...m.selectGraphFarmLanguagesV1(e)]);
await add("graph memory selectGraphMemoryLanguagesV1", "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs", (m) => (e) => [...m.selectGraphMemoryLanguagesV1(e)]);
await add("remote-cache remoteCacheLanguagesV1", "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/⌨️command/🟦️.ts", (m) => (e) => [...m.remoteCacheLanguagesV1(undefined, e)]);
await add("doctor resolveDoctorLanguagesV1", "🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🟦️.ts", (m) => (e) => [...m.resolveDoctorLanguagesV1(undefined, e)]);
await add("authorizers closureRefusalV1", "🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts", (m) => { const code = Object.keys(m.closureRefusalMessagesV1)[0]!, row = m.closureRefusalMessagesV1[code]; return (e) => fromText(m.closureRefusalV1(code, {}, e).message, row.en.refusal.split("{")[0], row.de.refusal.split("{")[0]); });
await add("testing toolchain describeMissingTestToolV1", "🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts", (m) => (e) => { const text: string = m.describeMissingTestToolV1({ tool: "T", tried: ["a"] }, e); return [...(/Test tool/u.test(text) ? ["en"] : []), ...(/Test-Werkzeug/u.test(text) ? ["de"] : [])]; });
await add("runtime messages runtimeMessageV1", "🔨️modules/⚙️runtime/🗣️messages/🟦️.ts", (m) => (e) => withEnv(e, () => { const text: string = m.runtimeMessageV1("capabilityMissing", { runtime: "R", family: "F" }); return text.includes(" / ") ? ["en", "de"] : /[äöüß]|nicht|stellt/u.test(text) ? ["de"] : ["en"]; }));
await add("developer-tools developerMessageV1", "🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🏃️process/🟦️.ts", (m) => (e) => withEnv(e, () => { const text: string = m.developerMessageV1("setupPrivilege"); return text.includes(" / ") ? ["en", "de"] : text.startsWith("Die ") ? ["de"] : ["en"]; }));
const dashboard = (e: Record<string, string>): string[] => {
  const tag = (value: string | undefined): string | undefined => { const head = value?.split(/[.@]/u)[0]?.split(/[_-]/u)[0]?.toLowerCase(); return head === "en" || head === "de" ? head : undefined; };
  const explicit = tag(e.SEMIO_LOCALE), system = ["LC_ALL", "LC_MESSAGES", "LANG"].map((name) => e[name] ? tag(e[name]) : undefined).find(Boolean);
  return explicit ? [explicit] : system ? [system] : ["en", "de"];
};
if (readFileSync(M("🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🌐️language/🦀️.rs"), "utf8").includes("self.system_variables.iter().find_map")) copies.push({ name: "dashboard Rust Chain::resolve + SEMIO_LOCALE (transcribed)", where: "🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🌐️language/🦀️.rs", run: dashboard });

let red = 0;
const lines: string[] = ["| copy | rows that disagree with the protocol (copy answer vs protocol notice languages) |", "|---|---|"];
for (const copy of copies) {
  const bad: string[] = [];
  for (const [index, row] of vectors.resolve.entries()) {
    if (row.args.length) continue;
    let got: string[];
    try { got = copy.run(row.environment); } catch (error) { got = ["throws: " + String((error as Error).message).slice(0, 40)]; }
    const want = truth[index]!.notices;
    if (JSON.stringify(got) !== JSON.stringify(want)) bad.push(`${row.name} (${got.join("+")} vs ${want.join("+")})`);
  }
  red += bad.length;
  lines.push(`| ${copy.name} | ${bad.length ? bad.length + ": " + bad.join("; ") : "agrees"} |`);
}
console.log(lines.join("\n"));
if (removed.length) console.log(`\nremoved copies (no longer exported): ${removed.join(", ")}`);
console.log(`\n[DEBUG] copies evaluated: ${copies.length}, disagreeing rows: ${red}`);
process.exit(red ? 1 : 0);
