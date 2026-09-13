/**
 * @emoji 🗣️ The TypeScript half of the generation3d i18n laws, answering the very same
 * `🧫️fixtures/🗣️terminology.json` roster the Rust `🔬️unit/🦀️.rs` laws answer against
 * `Generation3dLabels`' own consts.
 *
 * The two halves are deliberately different questions over one fixture. Rust asks "does the compiled
 * label set say exactly this?" — it can see the struct but it IS the struct, so it cannot notice a
 * rule the struct itself breaks. This half asks the structural question instead: every row bilingual,
 * nothing empty, nothing whitespace-only, German distinct from English unless the row is declared
 * identical by design, and no row declared identical that is not. A label added in English only
 * fails both halves, for two independent reasons.
 *
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";

type LabelRow = { readonly nativeEn: string; readonly nativeDe: string; readonly reuseEn: string; readonly reuseDe: string };
type TerminologyFixture = { readonly identicalByDesign: readonly string[]; readonly labels: Readonly<Record<string, LabelRow>> };

const LOCALE_KEYS = ["nativeEn", "nativeDe", "reuseEn", "reuseDe"] as const;

/** 🗣️ Answers every row of the shared roster, returning how many assertions it carried. */
export function generation3dTerminologySelfTests(): number {
  const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("../../../../🧫️fixtures/🗣️terminology.json", import.meta.url)), "utf8")) as TerminologyFixture;
  const identical = new Set(fixture.identicalByDesign);
  const rows = Object.entries(fixture.labels);
  assert(rows.length > 0, "the roster is not empty");
  let checks = 1;

  for (const [field, row] of rows) {
    for (const key of LOCALE_KEYS) {
      const text = row[key];
      assert.equal(typeof text, "string", `${field}.${key}: declared`);
      assert.notEqual(text.trim(), "", `${field}.${key}: never blank — a blank slot is a missing translation that still compiles`);
      checks += 2;
    }
    if (identical.has(field)) {
      assert.equal(row.nativeEn, row.nativeDe, `${field}: declared identical by design, so the two locales must actually match`);
      assert.equal(row.reuseEn, row.reuseDe, `${field}: identical by design holds for the reuse terminology too`);
    } else {
      assert.notEqual(row.nativeEn, row.nativeDe, `${field}: the German slot still holds the English text`);
      assert.notEqual(row.reuseEn, row.reuseDe, `${field}: the reuse terminology's German slot still holds the English text`);
    }
    checks += 2;
  }

  for (const field of identical) {
    assert(field in fixture.labels, `${field}: declared identical by design but absent from the roster`);
    checks += 1;
  }

  const untranslated = rows.filter(([field, row]) => !identical.has(field) && row.nativeEn === row.nativeDe);
  assert.deepEqual(untranslated.map(([field]) => field), [], "no label ships English in its German slot");
  checks += 1;
  return checks;
}
