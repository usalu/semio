/** ⚖️ WG7 (ticket-local) — cross-runtime oracle for the browser component codec: the SERVED note's jco-transpiled component (V8 +
 * jco, the browser shell's runtime) answers `codec.pack-schema-hash` and `codec.genesis` for the hub document, and both must equal
 * what wasmtime (the hub's trusted catalog) computed for the same component: the catalog's pinned `packSchemaHash` and the genesis
 * pair the hub's artifact CAS holds for the artifact. Two independent third-party component runtimes, one answer.
 * Usage: node --experimental-wasm-jspi s12-codec-oracle.mjs <componentJs> <casDir> <pinHex> <documentId> */
import { createHash } from "node:crypto";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const [componentJs, casDir, pinHex, documentId] = process.argv.slice(2);
const sha = (bytes) => createHash("sha256").update(bytes).digest("hex");
const component = await import(pathToFileURL(componentJs).href);
const hash = Buffer.from(await component.codec.packSchemaHash("note.document")).toString("hex");
const pair = await component.codec.genesis("note.document", documentId);
const cas = new Map(readdirSync(join(casDir, "chunk")).map((name) => [name, sha(readFileSync(join(casDir, "chunk", name)))]));
const rows = [
  { check: "codec.pack-schema-hash (jco) == catalog pin (wasmtime)", jco: hash, wasmtime: pinHex, ok: hash === pinHex },
  { check: "codec.genesis pack (jco) is the hub CAS genesis pack", jco: sha(pair.pack), ok: [...cas.values()].includes(sha(pair.pack)) },
  { check: "codec.genesis spr (jco) is the hub CAS genesis spr", jco: sha(pair.spr), ok: [...cas.values()].includes(sha(pair.spr)) },
  { check: "genesis spr names the hub document", ok: Buffer.from(pair.spr).includes(Buffer.from(documentId)) },
];
for (const row of rows) console.log(`${row.ok ? "PASS" : "FAIL"} ${row.check} ${JSON.stringify(row)}`);
console.log(`RESULT ${rows.filter((row) => row.ok).length}/${rows.length}`);
process.exit(rows.every((row) => row.ok) ? 0 : 1);
