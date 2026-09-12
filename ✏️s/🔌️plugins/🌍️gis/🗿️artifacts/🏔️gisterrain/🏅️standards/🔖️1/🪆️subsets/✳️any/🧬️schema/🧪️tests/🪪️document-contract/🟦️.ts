/** 🧪️ Terrain facets preserve durable child handles independently of window preferences. */
import assert from "node:assert/strict";
import { applyPatch, type Operation } from "fast-json-patch";
import { join } from "node:path";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseGisTerrainArtifact } from "../../🟦️.ts";
import { parseGisTerrainSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseGisTerrainDiff, applyGisTerrainDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🪆️ Compares first-party parsing with Ajv and all committed Terrain mutation documents. */
export function testTerrainDocumentContractOracle(): void {
  for (const row of vectors.diffCases) {
    const expected = applyPatch(structuredClone(row.before), row.patch as Operation[], true, false).newDocument;
    assert.deepEqual(expected, row.after, row.name);
    assert.deepEqual(applyGisTerrainDiff(parseGisTerrainSnapshot(row.before), parseGisTerrainDiff(row.diff)), expected, row.name);
  }
  const empty = { exaggeration: 0, importedFeaturesJson: "" };
  assertDocumentContractOracle({
    name: "GIS Terrain", dependencies: [ioSchema, childSchema],
    artifact: { schema: artifactSchema, parse: parseGisTerrainArtifact },
    snapshot: { schema: snapshotSchema, parse: parseGisTerrainSnapshot },
    diff: { schema: diffSchema, parse: parseGisTerrainDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }, { input: empty, output: empty }],
    invalidDocuments: vectors.invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 4, diffs: 2 },
  });
  assert.deepEqual(parseGisTerrainDiff(vectors.diff), vectors.diff);
  assert.deepEqual(parseGisTerrainDiff({}), { artifact: null, exaggeration: null, importedFeaturesJson: null });
}
