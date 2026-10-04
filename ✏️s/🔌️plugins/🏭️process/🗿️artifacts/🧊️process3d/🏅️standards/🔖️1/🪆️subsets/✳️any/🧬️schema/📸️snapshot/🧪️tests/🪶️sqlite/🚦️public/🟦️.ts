/** 🚪️ Declared Process3d physical I/O uses a closed contract and independent SQLite bytes. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../../../🧫️fixtures/🪶️sqlite/🚦️public/🔣️.json";
import schema from "../../../🧫️fixtures/🪶️sqlite/🚦️public/🧬️schema/🔣️.json";
import corpus from "../../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { independentProcess3dSqliteFile, process3dSqliteFixture } from "../../../🧫️fixtures/🪶️sqlite/🟦️.ts";
import { process3dSnapshotFromSqliteDatabase } from "../../../🟦️.ts";
import { importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("Process3d declared public I/O has one closed 3d parent contract", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect(validate({ ...fixture, extraCarrier: "opaque" })).toBe(false);
  expect(validate({ ...fixture, dialect: { ...fixture.dialect, artifactKind: "s.process.process2d" } })).toBe(false);
  expect(fixture.dialect).toEqual(corpus.dialect);
});

test("independent SQLite supplies every Process3d field, literal and binary64 word for both public encodings", async () => {
  for (const word of corpus.binary64Bits) for (const encoding of fixture.encodings) {
    const bytes = await independentProcess3dSqliteFile(word, encoding as "binary" | "text");
    const independent = Database.deserialize(bytes, { safeIntegers: true });
    try {
      expect(independent.query("SELECT name FROM sqlite_schema WHERE type='table'").all().length).toBe(fixture.domainTables + fixture.metadataRows);
      expect(independent.query("SELECT artifact_kind,standard,subset,native_encoding FROM semio_snapshot").get()).toEqual({ artifact_kind: fixture.dialect.artifactKind, standard: fixture.dialect.standard, subset: fixture.dialect.subset, native_encoding: encoding });
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
