#!/usr/bin/env bun
/** 🗄️ Language-neutral replication of every assertion the Rust retained-job fixture oracle makes
 * (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🧪️tests/🔬️unit/🦀️.rs`
 * `language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle` +
 * `retained_publication_contracts_are_an_exact_nonempty_tool_bijection`), so `🗄️retained-jobs/🔣️.json`
 * can be checked against the Rust catalogs without a cargo run. Diagnostic only — the gate stays the
 * Rust test. */
import { resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const puzzleRoot = resolve(repoRoot, "✏️s/🔌️plugins/🧩️puzzle");
const retained = await Bun.file(resolve(puzzleRoot, "🎮️commands/🧵️retained/🦀️.rs")).text();

const capacity = (name: string): number => {
  const match = retained.match(new RegExp(`${name}: (?:usize|u32) = ([0-9_]+);`));
  if (!match) throw new Error(`${name} is missing from the retained command module`);
  return Number(match[1]!.replaceAll("_", ""));
};
const capacities = {
  rawBytes: capacity("PUZZLE_COMMAND_RAW_BYTES"),
  decodedItems: capacity("PUZZLE_COMMAND_DECODED_ITEMS"),
  workItems: capacity("PUZZLE_COMMAND_WORK_ITEMS"),
  outputBytes: capacity("PUZZLE_COMMAND_OUTPUT_BYTES"),
  stepMicros: capacity("PUZZLE_COMMAND_STEP_MICROS"),
  semanticUnitsPerGrant: 1,
  checkpointBytes: capacity("PUZZLE_COMMAND_CHECKPOINT_BYTES"),
};

const VECTOR_IDS = ["zero", "max", "maxPlusOne", "malformed", "staleGeneration", "wrongOperation", "abaGeneration", "cancelWirePage", "cancelWireByte", "cancelPreflight", "cancelWork", "cancelPublish", "faultWork", "retry", "close", "replay"];
const CHECKPOINT_VECTOR_IDS = ["checkpointEmpty", "checkpointSingle", "checkpointMax", "checkpointMaxPlusOne", "checkpointCorrupt", "checkpointInterruptedClose"];
const fingerprint = (id: string): string =>
  id === "maxPlusOne" ? `${capacities.rawBytes + 1}:0:0:0:0:0` : id === "malformed" ? "1:0:0:0:0:0" : ["staleGeneration", "wrongOperation", "abaGeneration"].includes(id) ? "1:1:1:0:0:0" : "0:0:0:0:0:0";
const checkpointExpectation: [string, number, string][] = [
  ["checkpointEmpty", 0, "rejectedExactHandback"],
  ["checkpointSingle", 1, "rejectedExactHandback"],
  ["checkpointMax", capacities.checkpointBytes, "sameSemanticDigestTerminalEmpty"],
  ["checkpointMaxPlusOne", capacities.checkpointBytes + 1, "rejectedExactHandback"],
  ["checkpointCorrupt", capacities.checkpointBytes, "rejectedExactHandback"],
  ["checkpointInterruptedClose", capacities.checkpointBytes, "terminalEmptyExactHandback"],
];

type Fixture = {
  $schema: string;
  owner: string;
  documentSchema: string;
  payloadSchema: string;
  toolIds: string[];
  evidenceToolIds: string[];
  capacities: Record<string, number>;
  locales: Record<string, Record<string, string>>;
  vectors: Record<string, unknown>[];
};

const owners: [string, string, string, string][] = [
  ["puzzle2d", "◻️2d", "PUZZLE2D_RETAINED_TOOL_IDS", "puzzle.2d.fixture"],
  ["puzzle3d", "🧊️3d", "PUZZLE3D_RETAINED_TOOL_IDS", "puzzle.3d.fixture"],
  ["puzzle5d", "🖐️5d", "PUZZLE5D_RETAINED_TOOL_IDS", "puzzle.5d"],
];

let failures = 0;
for (const [owner, directory, constant, documentSchema] of owners) {
  const subset = `🗿️artifacts/${directory}/🏅️standards/🔖️1/🪆️subsets/✳️any`;
  const editor = await Bun.file(resolve(puzzleRoot, `${subset}/✏️editor/🦀️.rs`)).text();
  const fixture = await Bun.file(resolve(puzzleRoot, `${subset}/🗄️retained-jobs/🔣️.json`)).json() as Fixture;
  const declaration = editor.match(new RegExp(`${constant}: &\\[&str\\] = &\\[([\\s\\S]*?)\\];`));
  const ids = [...(declaration?.[1] ?? "").matchAll(/"([^"]+)"/g)].map((match) => match[1]!);
  const report = (held: boolean, clause: string): void => {
    if (held) return;
    failures += 1;
    console.log(`  ❌️ ${clause}`);
  };
  console.log(`===== ${owner} (rust ${ids.length} ids, fixture ${fixture.toolIds.length} ids)`);
  report(ids.length > 0, `${constant} did not parse`);
  report(JSON.stringify(fixture.toolIds) === JSON.stringify(ids), `toolIds must equal ${constant} in order — onlyFixture=${fixture.toolIds.filter((id) => !ids.includes(id)).join(",")} onlyRust=${ids.filter((id) => !fixture.toolIds.includes(id)).join(",")}${JSON.stringify(fixture.toolIds.slice().sort()) === JSON.stringify(ids.slice().sort()) ? " (same set, wrong order)" : ""}`);
  report(fixture.owner === owner, `owner is ${fixture.owner}`);
  report(fixture.documentSchema === documentSchema, `documentSchema is ${fixture.documentSchema}`);
  report(fixture.payloadSchema === `${documentSchema}.tool-command.v1`, `payloadSchema is ${fixture.payloadSchema}`);
  for (const [key, value] of Object.entries(capacities)) report(fixture.capacities[key] === value, `capacities.${key} is ${fixture.capacities[key]}, source says ${value}`);
  report(JSON.stringify(Object.keys(fixture.locales).sort()) === JSON.stringify(["de", "en"]), `locales are ${Object.keys(fixture.locales).join(",")}`);
  for (const locale of ["en", "de"]) report(typeof fixture.locales[locale]?.["cancel"] === "string", `locales.${locale}.cancel is missing`);
  const vectorIds = fixture.vectors.map((vector) => String(vector["id"]));
  report(JSON.stringify(vectorIds.slice(0, VECTOR_IDS.length)) === JSON.stringify(VECTOR_IDS), "vectors must start with the base vector ids");
  report(CHECKPOINT_VECTOR_IDS.every((id) => vectorIds.includes(id)), "every checkpoint vector id must be present");
  report(new Set(vectorIds).size === vectorIds.length, "vector ids must be unique");
  report(fixture.vectors.slice(0, VECTOR_IDS.length).every((vector, index) => vector["fingerprint"] === fingerprint(VECTOR_IDS[index]!)), "base vector fingerprints must match the source capacities");
  for (const [id, bytes, expected] of checkpointExpectation) {
    const vector = fixture.vectors.find((candidate) => candidate["id"] === id);
    report(vector?.["checkpointBytes"] === bytes && vector?.["expected"] === expected, `checkpoint vector ${id} must be ${bytes} bytes / ${expected}`);
  }
  const evidence = new Set(fixture.evidenceToolIds);
  const strayVectors = fixture.vectors.filter((vector) => {
    if (vector["control"] !== undefined || vector["authority"] !== undefined || vector["closeGrant"] !== undefined) return false;
    if (typeof vector["toolId"] === "string") return !evidence.has(vector["toolId"]);
    if (Array.isArray(vector["toolIds"])) return !(vector["toolIds"] as unknown[]).every((tool) => typeof tool === "string" && evidence.has(tool));
    return false;
  });
  report(strayVectors.length === 0, `vectors reference tool ids outside evidenceToolIds: ${strayVectors.map((vector) => vector["id"]).join(",")}`);
  report(fixture.vectors.every((vector) => typeof vector["id"] === "string" && typeof vector["expected"] === "string"), "every vector needs an id and an expected outcome");
}
console.log(failures === 0 ? "✅️ every retained-jobs fixture matches its Rust catalog" : `❌️ ${failures} failing clause(s)`);
process.exit(failures === 0 ? 0 : 1);
