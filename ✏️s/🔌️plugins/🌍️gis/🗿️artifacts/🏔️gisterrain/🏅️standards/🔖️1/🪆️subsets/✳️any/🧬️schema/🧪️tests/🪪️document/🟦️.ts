/** 🧪️ Terrain facets preserve durable child handles independently of window preferences. */
import {test} from "bun:test";
import {binary64,binary64Value} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import assert from "node:assert/strict";
import { applyPatch, type Operation } from "fast-json-patch";
import { join } from "node:path";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseGisTerrainArtifact } from "../../🟦️.ts";
import { parseGisTerrainSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseGisTerrainDiff, applyGisTerrainDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document/🔣️.json" with { type: "json" };


function foreignDocument(value:unknown):unknown{if(value===null||typeof value!=="object")return value;const row=value as Record<string,unknown>;return{...row,...(typeof row.exaggeration==="number"?{exaggeration:binary64(row.exaggeration)}:{})};}
function foreignDiff(value:unknown):unknown{if(value===null||typeof value!=="object")return value;const row=value as Record<string,unknown>;return{...row,...(row.artifact==null?{}:{artifact:foreignDocument(row.artifact)}),...(typeof row.exaggeration==="number"?{exaggeration:binary64(row.exaggeration)}:{})};}
function nativeJson(value:unknown):unknown{const row=value as {artifact?:unknown;exaggeration?:{bits:bigint}|null};return{...row,...(row.artifact==null?{}:{artifact:nativeJson(row.artifact)}),...(row.exaggeration==null?{}:{exaggeration:binary64Value(row.exaggeration)})};}

/** 🪆️ Compares first-party parsing with Ajv and all committed Terrain mutation documents. */
export function testTerrainDocumentContractOracle(): void {
  for (const row of vectors.diffCases) {
    const expected = applyPatch(structuredClone(row.before), row.patch as Operation[], true, false).newDocument;
    assert.deepEqual(expected, row.after, row.name);
    assert.deepEqual(nativeJson(applyGisTerrainDiff(parseGisTerrainSnapshot(foreignDocument(row.before)), parseGisTerrainDiff(foreignDiff(row.diff)))), expected, row.name);
  }
  const empty = { exaggeration: 0, importedFeaturesJson: "" };
  assertDocumentContractOracle({
    name: "GIS Terrain", dependencies: [ioSchema, childSchema],
    artifact: { schema: artifactSchema, parse: value=>parseGisTerrainArtifact(foreignDocument(value)), nativeJson },
    snapshot: { schema: snapshotSchema, parse: value=>parseGisTerrainSnapshot(foreignDocument(value)), nativeJson },
    diff: { schema: diffSchema, parse: value=>parseGisTerrainDiff(foreignDiff(value)), nativeJson },
    validDocuments: [{ input: vectors.document, output: vectors.document }, { input: empty, output: empty }],
    invalidDocuments: vectors.invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 4, diffs: 2 },
  });
  assert.deepEqual(nativeJson(parseGisTerrainDiff(foreignDiff(vectors.diff))), vectors.diff);
  assert.deepEqual(parseGisTerrainDiff({}), { artifact: null, exaggeration: null, importedFeaturesJson: null });
}

test("GIS terrain foreign JSON contract retains finite native meaning and handles",testTerrainDocumentContractOracle);
