import { measureRustCrate } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
const root = process.cwd();
const entry = process.argv[2];
for (const t of [false, true]) {
  const m = await measureRustCrate(root, root + "/" + entry, t);
  console.log("test=" + t, m.files, m.bytes, m.lines, "embedded", m.embedded);
  if (t) { const fs=require("fs"); const rows=(m.paths as string[]).map(f=>[fs.statSync(f).size,f]).sort((a:any,b:any)=>b[0]-a[0]); for (const [n,f] of rows) console.log(n,String(f).slice(-100)); }
}
