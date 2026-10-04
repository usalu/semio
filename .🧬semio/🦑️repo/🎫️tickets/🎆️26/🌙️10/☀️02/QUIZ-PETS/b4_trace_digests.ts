/** 🔁️ Ticket tool of work package B4: lists, per script of the stage-trace vectors, the digest of the committed trace before and after the regeneration and whether its laws all hold.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/b4_trace_digests.ts" <before.json> <after.json>
 */
import { readFileSync } from "node:fs";

type Vectors = { readonly scripts: readonly { readonly id: string; readonly expected: { readonly digest: number; readonly laws: Readonly<Record<string, boolean>> } }[] };

const [beforePath, afterPath] = process.argv.slice(2);
if (beforePath === undefined || afterPath === undefined) throw new Error("usage: b4_trace_digests.ts <before.json> <after.json>");
const before = JSON.parse(readFileSync(beforePath, "utf8").replace(/^﻿/u, "")) as Vectors;
const after = JSON.parse(readFileSync(afterPath, "utf8")) as Vectors;
const lines = ["| script | digest before | digest after | moved | laws |", "|---|---|---|---|---|"];
for (const script of after.scripts) {
  const old = before.scripts.find((entry) => entry.id === script.id);
  const lawful = Object.values(script.expected.laws).every((law) => law);
  lines.push(`| ${script.id} | ${old === undefined ? "new" : old.expected.digest} | ${script.expected.digest} | ${old === undefined ? "new" : old.expected.digest === script.expected.digest ? "no" : "yes"} | ${lawful ? "all hold" : JSON.stringify(script.expected.laws)} |`);
}
process.stdout.write(`${lines.join("\n")}\n`);
