import {expect, test} from "bun:test";
import {Database} from "bun:sqlite";
import derivedLaw from '../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🌱️derived/🧪️tests/🧫️fixtures/🔣️.json';
import previewClose from "./preview-close-source.json";
import {verifyMutationSourceCancellation} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🟦️.ts";
import {originalHistoryInputOracle} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/🧪️tests/🟦️.ts";

test("mutation Source cancellation returns all borrowed issuer aliases", verifyMutationSourceCancellation);
test("history Source cancellation restores the original ledger slot", originalHistoryInputOracle);

test("preview retirement only supplies Source to its selected owner and preserves pending Work", () => {
  const db = new Database(":memory:");
  try {
    const close = db.query(`
      SELECT CASE WHEN ? THEN 0 WHEN ? THEN ? ELSE ? AND ? AND NOT ? END AS source
    `);
    for (const row of previewClose.previewClose) {
      expect(close.get(
        Number(row.precedingDiscard), Number(row.closingDerived), Number(row.derivedNeedsControl), Number(row.cancelled),
        Number(row.mutationSource), Number(row.inputProjection),
      )).toEqual({source: Number(row.needsSource)});
    }
    const prefix = db.query(`
      SELECT NOT ? AND ? AND (? OR ?) AS source
    `);
    for (const row of previewClose.prefixClose) {
      expect(prefix.get(Number(row.finishedFirst), Number(row.closingPreview), Number(row.historySource), Number(row.derivedNeedsControl)))
        .toEqual({source: Number(row.needsSource)});
    }
    const recovery = db.query(`SELECT ? OR (NOT ? AND ?) AS admitted`);
    for (const row of previewClose.workRecovery) {
      expect(recovery.get(Number(row.sameWork), Number(row.pending), Number(row.source)))
        .toEqual({admitted: Number(row.admitted)});
    }
  } finally {
    db.close();
  }
});

/** 🪙️ The native failure-storage quote gets its own turn before its child can spend. */
test("failure-storage admission and child retirement use separate original Work quotes", () => {
  const db = new Database(":memory:");
  try {
    const observe = db.query("SELECT CASE WHEN offered >= quote THEN 1 ELSE 0 END AS work, 0 AS childRetires FROM (SELECT ? AS offered, 1 AS quote)");
    for (const row of derivedLaw.failureStorageBeforeClose) {
      expect(observe.get(Number(row.funded))).toEqual({work: row.work, childRetires: Number(row.childRetires)});
    }
  } finally {
    db.close();
  }
});
