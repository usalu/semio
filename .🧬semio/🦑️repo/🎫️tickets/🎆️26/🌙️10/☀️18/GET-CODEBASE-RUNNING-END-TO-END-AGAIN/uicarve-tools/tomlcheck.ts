import {readFileSync, existsSync} from "node:fs";
const R = "/Users/ueli/Documents/semio/";
const ledger = JSON.parse(readFileSync(import.meta.dir + "/ledger.json", "utf8"));
let bad = 0, n = 0;
for (const rel of [...Object.keys(ledger.backed), ...ledger.created, "🧰️framework/Cargo.toml", "Cargo.toml"]) {
  if (!rel.endsWith("Cargo.toml") || !existsSync(R + rel)) continue;
  n++;
  try { Bun.TOML.parse(readFileSync(R + rel, "utf8")); } catch (e) { bad++; console.log("BAD", rel.slice(-80), String(e).slice(0, 120)); }
}
for (const rel of ["📋️build-owners.json"]) JSON.parse(readFileSync(R + rel, "utf8"));
console.log(n, "manifests parsed; bad", bad);
