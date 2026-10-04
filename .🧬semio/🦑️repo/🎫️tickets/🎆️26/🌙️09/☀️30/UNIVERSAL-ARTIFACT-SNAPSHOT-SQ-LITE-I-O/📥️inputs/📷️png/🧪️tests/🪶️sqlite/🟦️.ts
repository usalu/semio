/** 🧫️ Complete logical PNG owner and interpreted native SQLite laws. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { PNG as IndependentPng } from "pngjs";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { parsePngSnapshot, type PngSnapshot } from "../../🟦️.ts";
import { PNG_SQLITE_SCHEMA, pngSnapshotFromSqliteDatabase, pngSnapshotToSqliteDatabase } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input = parsePngSnapshot(fixture) as PngSnapshot;

async function serialized(snapshot: PngSnapshot = input): Promise<Uint8Array> {
  return exportSqliteDatabase(await pngSnapshotToSqliteDatabase(snapshot));
}

test("PNG SQLite interprets the exact native owner and reconstructs it byte-for-byte", async () => {
  expect(PNG_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  const bytes = await serialized();
  const database = Database.deserialize(bytes);
  try {
    expect(database.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(database.query("SELECT id,schema,role FROM png_document").get()).toEqual({id:1,schema:"stdio.png",role:"native"});
    expect(database.query("SELECT COUNT(*) AS count FROM sqlite_schema WHERE type='table'").get()).toEqual({count:24});
    expect(database.query("SELECT COUNT(*) AS count FROM sqlite_schema WHERE type='table' AND sql LIKE '%BLOB%'").get()).toEqual({count:0});
    expect(database.query("SELECT COUNT(*) AS count FROM png_literal_octet").get()).toEqual({count:0});
  } finally {
    database.close();
  }
  expect(await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(input);
});

test("PNG SQLite round-trip preserves independent decoder pixels and source profile", async () => {
  const before = IndependentPng.sync.read(Buffer.from(input.bytes));
  const restored = await pngSnapshotFromSqliteDatabase(await pngSnapshotToSqliteDatabase(input));
  const after = IndependentPng.sync.read(Buffer.from(restored.bytes));
  expect(restored.bytes).toEqual(input.bytes);
  expect({ width: after.width, height: after.height, depth: after.depth, colorType: after.colorType }).toEqual({ width: before.width, height: before.height, depth: before.depth, colorType: before.colorType });
  expect([...after.data]).toEqual([...before.data]);
});

test("PNG SQLite rejects non-byte input and malformed singleton storage", async () => {
  for (const bytes of [[-1], [256], [1.5]]) {
    await expect(pngSnapshotToSqliteDatabase({ schema: "stdio.png", bytes })).rejects.toThrow("outside u8");
  }
  for (const edit of ["DELETE FROM png_document", "UPDATE png_document SET role='literal'"]) {
    const database = Database.deserialize(await serialized());
    try {
      database.run(edit);
      await expect(pngSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(database.serialize())))).rejects.toThrow();
    } finally {
      database.close();
    }
  }
});

test("PNG SQLite projection and reconstruction honor cancellation during bounded copies", async () => {
  const large: PngSnapshot = { schema: "stdio.png", bytes: Array.from({ length: 200_000 }, (_, index) => index & 255) };
  const projection = new AbortController();
  let projected = false;
  await expect(pngSnapshotToSqliteDatabase(large, { signal: projection.signal, onProgress: event => { if (event.phase === "projectSnapshot" && event.completed < event.total) { projected = true; projection.abort(); } } })).rejects.toMatchObject({ kind: "canceled" });
  expect(projected).toBe(true);

  const database = await pngSnapshotToSqliteDatabase(large);
  const reconstruction = new AbortController();
  let reconstructed = false;
  await expect(pngSnapshotFromSqliteDatabase(database, { signal: reconstruction.signal, onProgress: event => { if (event.phase === "reconstructSnapshot" && event.completed < event.total) { reconstructed = true; reconstruction.abort(); } } })).rejects.toMatchObject({ kind: "canceled" });
  expect(reconstructed).toBe(true);
});

import "./🚦️audit/🟦️.ts";
import "./🪆️owner/🟦️.ts";


test("PNG exact native owner exposes independently editable IHDR and ordered chunk entities",async()=>{
  const neutral=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🧩️semantic/🔣️.json",import.meta.url)).json();
  const schema=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/🧩️semantic/🧬️schema/🔣️.json",import.meta.url)).json();
  const {default:Ajv}=await import("ajv");const validate=new Ajv({strict:true}).compile(schema);
  expect(validate(neutral)).toBe(true);expect(validate({...neutral,extra:1})).toBe(false);
  const independent=Database.deserialize(await serialized());
  try{
    expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
    const header=independent.query("SELECT width,height,bit_depth AS bitDepth,color_type AS colorType,compression,filter,interlace FROM png_ihdr").get();
    expect(header).toEqual(neutral.ihdr);
    expect(independent.query("SELECT kind FROM png_chunk ORDER BY ordinal").all().map((row:any)=>row.kind)).toEqual(neutral.chunkKinds);
    expect(await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()))).toEqual(input);
    independent.query("UPDATE png_ihdr SET width=?").run(neutral.edit.value);
    const edited=await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()));
    const decoded=IndependentPng.sync.read(Buffer.from(edited.bytes));
    expect(decoded.width).toBe(3);expect(decoded.height).toBe(1);
    expect([...decoded.data]).toEqual(neutral.editedRgba);
    expect(edited.schema).toBe(input.schema);
    expect(edited.bytes).not.toEqual(input.bytes);
  }finally{independent.close();}
});

import "./🌈️scanline/🟦️.ts";
