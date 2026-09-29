/** 🧮️ FH1: runs the overlay fault census and writes family A (framework) + H (11 plugins) findings to the FH1 capture dir. */
import { writeFileSync } from "node:fs";
const overlay = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults";
const out = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-fh1-census";
const { runFaultCensus, faultOwnerOfPath, faultFactsOfText } = await import(`${overlay}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts`);
process.env.GIT_DIR = "/Users/ueli/Documents/semio/.git";
process.env.GIT_WORK_TREE = overlay;
const H = ["✒️writer", "📸️remodel", "🔋️energy", "🎞️animate", "🪵️sourcing", "🧱️block", "🕸️dag", "🎥️shooting", "🎪️demonstrator", "📜️imperative", "📖️playbook"];
const only = process.argv.slice(2);
const census = runFaultCensus(overlay, new AbortController().signal, () => {});
const ownerOf = (path: string): string => { const owner = faultOwnerOfPath(path); return owner.kind === "app" ? owner.plugin : "framework"; };
const families: Record<string, string[]> = { A: ["framework"], H };
for (const [family, owners] of Object.entries(families)) {
  if (only.length > 0 && !only.includes(family)) continue;
  const violations = census.violations.filter((v: { path: string }) => owners.includes(ownerOf(v.path)));
  const raises = census.raises.filter((r: { path: string }) => owners.includes(ownerOf(r.path)));
  const declarations = census.declarations.filter((d: { path: string }) => owners.includes(ownerOf(d.path)));
  writeFileSync(`${out}/family-${family}.json`, JSON.stringify({ violations, raises, declarations }, null, 1));
  const byOwnerRule = new Map<string, number>();
  for (const v of violations) byOwnerRule.set(`${ownerOf(v.path)} ${v.rule}`, (byOwnerRule.get(`${ownerOf(v.path)} ${v.rule}`) ?? 0) + 1);
  console.log(`family ${family}: violations=${violations.length} raises=${raises.length} declarations=${declarations.length}`);
  for (const [key, count] of [...byOwnerRule].sort()) console.log(`  ${key} ${count}`);
}
if (only.length === 0 || only.includes("A")) {
  const { spawnSync } = await import("node:child_process");
  const { readFileSync } = await import("node:fs");
  const listed = spawnSync("git", ["-c", "core.quotePath=false", "ls-files", "-z", "-c", "-o", "--exclude-standard", "--", "*.rs", ":!.🧬semio"], { cwd: overlay, encoding: "utf8", maxBuffer: 1 << 30 }).stdout.split("\0").filter((path: string) => path && ownerOf(path) === "framework" && !/(^|\/)(🧪️tests|tests|🧫️fixtures|benches|examples)\//u.test(path));
  const all = spawnSync("git", ["-c", "core.quotePath=false", "ls-files", "-z", "-c", "-o", "--exclude-standard", "--", "*.rs", ":!.🧬semio"], { cwd: overlay, encoding: "utf8", maxBuffer: 1 << 30 }).stdout.split("\0").filter((path: string) => path && !/(^|\/)(🧪️tests|tests|🧫️fixtures|benches|examples)\//u.test(path));
  const consts = new Map<string, string[]>();
  for (const path of all) {
    let text = "";
    try { text = readFileSync(`${overlay}/${path}`, "utf8"); } catch { continue; }
    for (const match of text.matchAll(/\bconst\s+([A-Z][A-Z0-9_]*)\s*:\s*&\s*(?:'static\s+)?str\s*=\s*"([^"\\]*)"/gu)) consts.set(match[1]!, [...(consts.get(match[1]!) ?? []), match[2]!]);
  }
  const raised: Record<string, string[]> = {};
  for (const path of listed) {
    let text = "";
    try { text = readFileSync(`${overlay}/${path}`, "utf8"); } catch { continue; }
    if (!/Fault|app_fault|fault_from_error/u.test(text)) continue;
    const facts = faultFactsOfText(path, text);
    for (const raise of facts.raises) raised[raise.code] = [...new Set([...(raised[raise.code] ?? []), raise.parameters.slice().sort().join(",")])];
    for (const reference of facts.constRaises) for (const value of consts.get(reference.name) ?? []) raised[value] = [...new Set([...(raised[value] ?? []), reference.parameters.slice().sort().join(",")])];
  }
  writeFileSync(`${out}/framework-raised.json`, JSON.stringify(raised, null, 1));
  console.log(`framework raised codes (literal + const) = ${Object.keys(raised).length}`);
}
console.log(`files=${census.files} raises=${census.raises.length} disagreements=${census.disagreements.length}`);
