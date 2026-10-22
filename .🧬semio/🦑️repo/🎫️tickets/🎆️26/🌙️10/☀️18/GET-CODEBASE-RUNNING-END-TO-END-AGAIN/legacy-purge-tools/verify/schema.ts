#!/usr/bin/env bun
/** 🧾️ Validates every staged converted `🎫️ticket.json` against `$defs/TicketDocument` of the staged ticket schema (Ajv 2020, closed required members, interactions checked one by one). usage: bun schema.ts */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020";
const TOOLS = resolve(import.meta.dir, "..");
const manifest = JSON.parse(readFileSync(join(TOOLS, "staged", "manifest.json"), "utf8"));
const after = (row: { after: string }): string => readFileSync(join(TOOLS, "staged", "after", row.after), "utf8");
const schemaRow = manifest.files.find((row: { path: string }) => row.path.endsWith("🎫️tickets/🧬️schema/🔣️.json"));
const schema = JSON.parse(after(schemaRow));
const validate = new Ajv2020({ strict: false, allErrors: true }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/TicketDocument" });
const forbidden = ["prompt", "agents", "started", "finished", "dates", "files", "client", "llm"];
const ROOT = resolve(TOOLS, ...Array(8).fill(".."));
let kept = 0;
let checked = 0;
let interactions = 0;
const problems: string[] = [];
for (const row of manifest.files.filter((row: { path: string }) => row.path.endsWith("🎫️ticket.json"))) {
  const document = JSON.parse(after(row));
  checked += 1;
  if (!validate(document)) problems.push(`${row.path}: ${JSON.stringify(validate.errors)}`);
  const original = JSON.parse(readFileSync(join(ROOT, row.path), "utf8"));
  const opens = (document.interactions ?? []).filter((interaction: { kind: string }) => interaction.kind === "ticket.open");
  const closes = (document.interactions ?? []).filter((interaction: { kind: string }) => interaction.kind === "ticket.close");
  if (original.started && opens.length > 0) { kept += 1; if (opens[0].date !== original.started) problems.push(`${row.path}: started ${original.started} is not the first open ${opens[0].date}`); }
  if (original.finished && closes.length > 0) { kept += 1; if (closes[closes.length - 1].date !== original.finished) problems.push(`${row.path}: finished ${original.finished} is not the last close ${closes[closes.length - 1].date}`); }
  for (const name of forbidden) if (name in document) problems.push(`${row.path}: carries ${name}`);
  for (const interaction of document.interactions ?? []) {
    interactions += 1;
    if (!/^\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}:\d{2}(\.\d+)?Z?$/.test(interaction.date)) problems.push(`${row.path}: date ${interaction.date}`);
    if (!["ticket.open", "ticket.close", "ticket.reopen"].includes(interaction.kind)) problems.push(`${row.path}: kind ${interaction.kind}`);
  }
}
console.log(`${checked} converted tickets, ${interactions} interactions, ${kept} started/finished timestamps kept, ${problems.length} problems`);
for (const problem of problems.slice(0, 20)) console.log(problem);
process.exit(problems.length ? 1 : 0);
