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


import {intrinsicFromJson,intrinsicToJson,type ImportedMap} from "../../🗺️imported-map/🟦️.ts";
function foreignMap(value:unknown):unknown{const map=value as {positions:unknown[];routes:unknown[];regions:unknown[];properties:{name:string;value:unknown}[]};return{positions:map.positions.map(intrinsicFromJson),routes:map.routes.map(intrinsicFromJson),regions:map.regions.map(intrinsicFromJson),properties:map.properties.map(m=>({name:m.name,value:intrinsicFromJson(m.value)}))};}
function foreignDocument(value:unknown):unknown{if(value===null||typeof value!=="object")return value;const row=value as Record<string,unknown>;return{...row,...(typeof row.exaggeration==="number"?{exaggeration:binary64(row.exaggeration)}:{}),...(row.importedMap===undefined?{}:{importedMap:foreignMap(row.importedMap)})};}
function foreignDiff(value:unknown):unknown{if(value===null||typeof value!=="object")return value;const row=value as Record<string,unknown>,change=row.importedMap as {value?:unknown}|null;return{...row,...(row.artifact==null?{}:{artifact:foreignDocument(row.artifact)}),...(typeof row.exaggeration==="number"?{exaggeration:binary64(row.exaggeration)}:{}),...(change?.value===undefined?{}:{importedMap:{value:foreignMap(change.value)}})};}
function nativeMap(map:ImportedMap):unknown{return{positions:map.positions.map(intrinsicToJson),routes:map.routes.map(intrinsicToJson),regions:map.regions.map(intrinsicToJson),properties:map.properties.map(m=>({name:m.name,value:intrinsicToJson(m.value)}))};}
function nativeJson(value:unknown):unknown{const row=value as {artifact?:unknown;exaggeration?:{bits:bigint}|null;importedMap?:ImportedMap|{value?:ImportedMap}|null};let importedMap=row.importedMap;if(importedMap){importedMap="positions"in importedMap?nativeMap(importedMap) as ImportedMap:importedMap.value===undefined?{}:{value:nativeMap(importedMap.value)as ImportedMap};}return{...row,...(row.artifact==null?{}:{artifact:nativeJson(row.artifact)}),...(row.exaggeration==null?{}:{exaggeration:binary64Value(row.exaggeration)}),...(row.importedMap===undefined?{}:{importedMap})};}

/** 🪆️ Compares first-party parsing with Ajv and all committed Terrain mutation documents. */
export function testTerrainDocumentContractOracle(): void {
  for (const row of vectors.diffCases) {
    const expected = applyPatch(structuredClone(row.before), row.patch as Operation[], true, false).newDocument;
    assert.deepEqual(expected, row.after, row.name);
    assert.deepEqual(nativeJson(applyGisTerrainDiff(parseGisTerrainSnapshot(foreignDocument(row.before)), parseGisTerrainDiff(foreignDiff(row.diff)))), expected, row.name);
  }
  const empty = { exaggeration: 0 };
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
  assert.deepEqual(parseGisTerrainDiff({}), { artifact: null, exaggeration: null, importedMap: null });
}

test("GIS terrain foreign JSON contract retains finite native meaning and handles",testTerrainDocumentContractOracle);
