// usage: bun pipesmoke.ts <label> <module-for-each-export>... : imports the given modules, merges exports and runs the context-free pipeline verifications, printing PASS/FAIL per name
const [label, ...modules] = process.argv.slice(2);
const merged: Record<string, unknown> = {};
for (const m of modules) Object.assign(merged, await import(m));
const names = ["verifyPrintToolchainManifest", "verifyPrintApiFreshness", "verifyPrintMacroStaging", "verifyPrintDocumentCatalog"];
const log = console.log;
for (const name of names) {
  const fn = merged[name] as undefined | (() => unknown);
  if (typeof fn !== "function") { log(`${label} ${name} MISSING`); continue; }
  const quiet = console.log; console.log = () => undefined;
  try { await fn(); console.log = quiet; log(`${label} ${name} PASS`); } catch (e) { console.log = quiet; log(`${label} ${name} FAIL ${String((e as Error).message).slice(0, 160).replace(/\s+/g, " ")}`); }
}
log(`${label} exports=${Object.keys(merged).length}`);
