/** 🚪️ Declared Process3d physical I/O uses a closed contract and independent SQLite bytes. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🚦️public/🔣️.json";

import corpus from "../../🧫️fixtures/🔣️.json";
import { independentProcess3dSqliteFile, process3dSqliteFixture } from "../🧰️support/🟦️.ts";
import { process3dSnapshotFromSqliteDatabase } from "../../🟦️.ts";
import { importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("Process3d declared public I/O has one closed 3d parent contract", () => {
  
  expect(fixture).toEqual({"dialect":{"artifactKind":"s.process.process3d","standard":"1","subset":"*"},"encodings":["binary","text"],"applicationId":1397576526,"userVersion":1,"domainTables":32,"domainRows":173,"metadataRows":1,"routeHops":1,"fidelity":"exact","exportPhases":["DecodeNative","ProjectSnapshot","WritePages"],"importPhases":["ReadPages","ReconstructSnapshot","EncodeNative"],"independentEdit":{"table":"process3_stock_payload","column":"label","value":"independent public payload 文🌠"},"metadataColumns":["id","artifact_kind","standard","subset","schema_version","native_encoding"],"retirement":"explicit"});
  
  
  expect(fixture.dialect).toEqual(corpus.dialect);
  expect(fixture.metadataColumns).toEqual(["id", "artifact_kind", "standard", "subset", "schema_version", "native_encoding"]);
  expect(fixture.retirement).toBe("explicit");
});

test("independent SQLite supplies every Process3d field, literal and binary64 word for both public encodings", async () => {
  for (const word of corpus.binary64Bits) for (const encoding of fixture.encodings) {
    const bytes = await independentProcess3dSqliteFile(word, encoding as "binary" | "text");
    const independent = Database.deserialize(bytes, { safeIntegers: true });
    try {
      expect(independent.query("SELECT name FROM sqlite_schema WHERE type='table'").all().length).toBe(fixture.domainTables + fixture.metadataRows);
      const metadata = independent.query("SELECT * FROM semio_snapshot").get()!;
      expect(Object.keys(metadata)).toEqual(fixture.metadataColumns);
      expect(metadata).toEqual({ id: 1n, artifact_kind: fixture.dialect.artifactKind, standard: fixture.dialect.standard, subset: fixture.dialect.subset, schema_version: 1n, native_encoding: encoding });
      expect((independent.query("SELECT angle_ieee754_bits FROM process3_measure_pose LIMIT 1").get() as { angle_ieee754_bits: bigint }).angle_ieee754_bits).toBe(BigInt.asIntN(64, BigInt("0x" + word)));
      expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
      const file = await importSqliteDatabase(bytes);
      const tables = file.tables.filter(table => table.name !== "semio_snapshot");
      expect(tables.flatMap(table => table.rows).length).toBe(fixture.domainRows);
      expect(await process3dSnapshotFromSqliteDatabase({ tables })).toEqual(process3dSqliteFixture(word));
    } finally { independent.close(); }
  }
});

test("independent public payload edit leaves root stock presentation and every other Process3d field intact", async () => {
  for (const word of corpus.binary64Bits) for (const encoding of fixture.encodings) {
    const bytes = await independentProcess3dSqliteFile(word, encoding as "binary" | "text", true);
    const file = await importSqliteDatabase(bytes);
    const expected = process3dSqliteFixture(word);
    expected.stockPayload.label = fixture.independentEdit.value;
    expect(await process3dSnapshotFromSqliteDatabase({ tables: file.tables.filter(table => table.name !== "semio_snapshot") })).toEqual(expected);
    expect(expected.stockLabel).toBe(corpus.rootStock.label);
  }
});
