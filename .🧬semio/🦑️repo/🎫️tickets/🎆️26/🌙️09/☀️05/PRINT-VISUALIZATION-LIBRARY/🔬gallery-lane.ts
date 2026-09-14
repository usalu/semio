#!/usr/bin/env bun
/** 🖼️ One lane of the exhaustive gallery sweep.
 *
 * `bun 🔬gallery-lane.ts <out.json> <sections.tsv> <section…>` measures every variant (light/dark x
 * en/de) of each named taxonomy section and writes the evidence into its OWN json, appending the
 * wall time of each section to the TSV. Two lanes may run at once because neither touches the
 * committed fixture; `🔬gallery-merge.ts` assembles it from the lane files afterwards.
 */
import { appendFileSync, existsSync, readFileSync, writeFileSync } from "node:fs";
import { measurePrintGalleryVariant, printGalleryMatrix } from "../../../../../../../🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts";

const [out, tsv, ...sections] = process.argv.slice(2);
if (out === undefined || tsv === undefined || sections.length === 0) throw new Error("usage: bun 🔬gallery-lane.ts <out.json> <sections.tsv> <section…>");

const measured: Record<string, unknown> = existsSync(out) ? JSON.parse(readFileSync(out, "utf8")) : {};
for (const section of sections) {
  const started = Date.now();
  let failure = "";
  for (const variant of printGalleryMatrix().filter((candidate) => candidate.section === section)) {
    if (measured[variant.id] !== undefined) continue;
    try {
      measured[variant.id] = await measurePrintGalleryVariant(variant);
    } catch (error) {
      failure = String(error instanceof Error ? error.message : error).replaceAll(/\s+/gu, " ").slice(0, 300);
      break;
    }
  }
  writeFileSync(out, `${JSON.stringify(measured, null, 2)}\n`, "utf8");
  appendFileSync(tsv, `${section}\t${failure === "" ? "ok" : "FAIL"}\t${((Date.now() - started) / 1000).toFixed(1)}\t${failure}\n`, "utf8");
  console.log(`[DEBUG] gallery lane: section ${section} ${failure === "" ? "ok" : "FAIL"} after ${((Date.now() - started) / 1000).toFixed(1)}s`);
}
