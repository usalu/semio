import { Database } from "bun:sqlite";
const db = new Database(":memory:");
const row = db.query("select 1 as v").safeIntegers(true).get() as { v: bigint };
console.log(row.v);
