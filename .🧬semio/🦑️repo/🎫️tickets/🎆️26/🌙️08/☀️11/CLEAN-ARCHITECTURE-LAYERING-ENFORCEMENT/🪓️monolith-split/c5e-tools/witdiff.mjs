import { generateHostTypes } from "/Users/ueli/Documents/semio-kinan-wip/node_modules/@bytecodealliance/jco-transpile/dist/index.js";
const [, , oldPath, newPath] = process.argv;
const gen = async (p) => { const files = await generateHostTypes(p, { name: "actor", worldName: "actor", instantiation: "async", asyncMode: "jspi" }); const out = {}; for (const [k, v] of Object.entries(files)) out[k] = Buffer.from(v).toString("utf8"); return out; };
const a = await gen(oldPath), b = await gen(newPath);
const ka = Object.keys(a).sort(), kb = Object.keys(b).sort();
console.log("files", ka.length, kb.length, JSON.stringify(ka) === JSON.stringify(kb));
let bad = 0; for (const k of ka) if (a[k] !== b[k]) { bad++; console.log("DIFF", k); }
console.log("diffs", bad);
