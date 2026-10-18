import { readdirSync, statSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { programArtifactFromJson } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts";
const { programSnapshotToSqliteDatabase, programSnapshotFromSqliteDatabase } = await import(process.env.ARCH_LEAF!);
const j = (v: unknown) => JSON.stringify(v, (_k, x) => typeof x === "bigint" ? x.toString() + "n" : x && typeof x === "object" && !Array.isArray(x) ? Object.fromEntries(Object.entries(x).sort(([a],[b]) => a < b ? -1 : a > b ? 1 : 0)) : x);
const files: string[] = [];
const walk = (d: string) => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) walk(p); else if (e.endsWith(".json") && p.includes("📸️snapshot") && p.includes("after")) files.push(p); } };
walk("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures");
files.sort();
const hasher = new Bun.CryptoHasher("sha256");
let ok = 0, refused = 0;
for (const f of files) {
  let expected: any;
  try { expected = programArtifactFromJson(JSON.parse(readFileSync(f, "utf8")) as any); } catch { refused++; continue; }
  const db = await programSnapshotToSqliteDatabase(expected);
  const actual = await programSnapshotFromSqliteDatabase(db);
  if (j(actual) !== j(expected)) throw new Error("round trip differs: " + f);
  hasher.update(j(db));
  ok++;
}
console.log(JSON.stringify({ fixtures: files.length, ok, refused, digest: hasher.digest("hex") }));
