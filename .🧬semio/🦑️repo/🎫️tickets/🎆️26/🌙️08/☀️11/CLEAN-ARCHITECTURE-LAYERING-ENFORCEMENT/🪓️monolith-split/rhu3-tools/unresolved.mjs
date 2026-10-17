import { imports } from "./clos.mjs";
const R = "/Users/ueli/Documents/semio/";
let bad = 0;
for (const f of process.argv.slice(2)) { for (const d of imports(R + f)) if (!d.file && d.spec.startsWith(".") && !d.spec.includes("?")) { console.log("UNRESOLVED", f.slice(-60), d.spec); bad++; } }
console.log("unresolved", bad);
