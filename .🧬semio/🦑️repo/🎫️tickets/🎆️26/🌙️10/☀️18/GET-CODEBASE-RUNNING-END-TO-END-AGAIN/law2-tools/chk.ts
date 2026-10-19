import { readModuleTreeSource, readSourceFamilySource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
// usage: chk.ts <tree|family> <path> <startMarker|-> <endMarker|-> needle...   (needle prefixed T: must be present, F: must be absent)
const [mode, path, startMarker, endMarker, ...needles] = process.argv.slice(2);
const text = mode === "family" ? readSourceFamilySource(path!) : readModuleTreeSource(path!);
let body = text;
if (startMarker !== "-") { const a = text.indexOf(startMarker!); if (a < 0) { console.log("START NOT FOUND", startMarker); process.exit(0); } const b = endMarker !== "-" ? text.indexOf(endMarker!, a) : -1; body = text.slice(a, b < 0 ? undefined : b); console.log("slice", a, b, body.length); }
for (const n of needles) { const must = n.startsWith("T:"); const v = n.slice(2); const has = body.includes(v); console.log(has === must ? "ok  " : "FAIL", n.slice(0, 110)); }
