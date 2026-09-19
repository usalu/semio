/** 🪦️ Retires every `frozenCoordinateEvidenceContracts` row whose frozen document no longer exists,
 * by RECORDING the retirement on the row rather than deleting it.
 *
 * The 28 rows point at ticket-generated evidence that was removed when its ticket closed, which
 * AGENTS.md requires ("You MUST delete all tool generated output files … inside the ticket folder
 * after you are done"). Deleting the rows would make the count law pass again while destroying the
 * only record that those bytes were ever frozen. The retirement is digest-neutral by construction:
 * `frozenCoordinateEvidenceSeal` projects the evidence and excludes `retired`, so this edit cannot
 * move the seal — the script asserts exactly that before and after.
 *
 * The ticket is derived from the frozen path itself (`.🧬semio/🦑️repo/🎫️tickets/🎆️YY/🌙️MM/☀️DD/SLUG`),
 * never guessed. Re-reads the taxonomy immediately before writing. Idempotent.
 *
 * Usage: `bun 🐍️v3b-retire-frozen-contracts.ts <repoRoot> [--apply]`
 */
import { createHash } from "node:crypto";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = process.argv[2]!;
const apply = process.argv.includes("--apply");
const taxonomyPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");

const { frozenCoordinateEvidenceSeal, validateFrozenCoordinateEvidenceContracts } = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"));
const { canonicalJson } = await import(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts"));

const TICKET_PATTERN = /^\.🧬semio\/🦑️repo\/🎫️tickets\/🎆️(\d{2})\/🌙️(\d{2})\/☀️(\d{2})\/([^/]+)\//u;

const raw = readFileSync(taxonomyPath, "utf8");
const taxonomy = JSON.parse(raw) as Record<string, unknown>;
const contracts = taxonomy.frozenCoordinateEvidenceContracts as Record<string, Record<string, unknown>>;
if (!contracts || Object.keys(contracts).length === 0) throw new Error("taxonomy carries no frozenCoordinateEvidenceContracts — refusing to act on an empty set");

const sealBefore = createHash("sha256").update(canonicalJson(frozenCoordinateEvidenceSeal(contracts))).digest("hex");
console.log(`contracts=${Object.keys(contracts).length} sealBefore=${sealBefore}`);

const retire: { id: string; ticket: string }[] = [];
const alive: string[] = [];
const unretirable: string[] = [];
for (const [id, contract] of Object.entries(contracts)) {
  const path = String(contract.path);
  if (existsSync(join(repoRoot, path))) {
    alive.push(id);
    if (Object.hasOwn(contract, "retired")) unretirable.push(`${id}: marked retired but the document EXISTS on disk`);
    continue;
  }
  if (Object.hasOwn(contract, "retired")) continue;
  const match = TICKET_PATTERN.exec(path);
  if (!match) {
    unretirable.push(`${id}: absent document is not under a ticket folder (${path})`);
    continue;
  }
  retire.push({ id, ticket: `20${match[1]}/${match[2]}/${match[3]}/${match[4]}` });
}

console.log(`alive=${alive.length} toRetire=${retire.length} unretirable=${unretirable.length}`);
for (const problem of unretirable) console.log("  !! " + problem);
for (const row of retire) console.log(`${apply ? "retire" : "would retire"} ${row.id}\t${row.ticket}`);
if (unretirable.length > 0) throw new Error("refusing to write while any contract is in an unexplained state");
if (retire.length === 0 || !apply) process.exit(0);

// 🔁️ Re-read immediately before writing so a peer's concurrent taxonomy edit survives, and splice the
// `retired` block in as TEXT: the taxonomy is a hand-maintained document and must not be reformatted.
let text = readFileSync(taxonomyPath, "utf8");
for (const row of retire) {
  const anchor = new RegExp(`(\\n(\\s*)"${row.id}": \\{\\n)(\\s*)"path":`, "u");
  const match = anchor.exec(text);
  if (!match) throw new Error(`${row.id}: could not locate its contract object in the taxonomy document`);
  const indent = match[3]!;
  const block = `${indent}"retired": { "ticket": ${JSON.stringify(row.ticket)}, "reason": "ticket-close-generated-output-removed" },\n${indent}"path":`;
  text = text.slice(0, match.index) + match[1] + block + text.slice(match.index + match[0].length);
}
writeFileSync(taxonomyPath, text);

const written = JSON.parse(readFileSync(taxonomyPath, "utf8")) as Record<string, unknown>;
const writtenContracts = written.frozenCoordinateEvidenceContracts as Record<string, Record<string, unknown>>;
const problems = validateFrozenCoordinateEvidenceContracts(writtenContracts);
if (problems.length > 0) throw new Error("validator rejected the retired contracts:\n" + problems.join("\n"));
const sealAfter = createHash("sha256").update(canonicalJson(frozenCoordinateEvidenceSeal(writtenContracts))).digest("hex");
const retiredCount = Object.values(writtenContracts).filter((contract) => Object.hasOwn(contract, "retired")).length;
console.log(`retired=${retiredCount} sealAfter=${sealAfter} sealUnchanged=${sealBefore === sealAfter}`);
if (sealBefore !== sealAfter) throw new Error("retirement moved the evidence seal — the projection is wrong");
