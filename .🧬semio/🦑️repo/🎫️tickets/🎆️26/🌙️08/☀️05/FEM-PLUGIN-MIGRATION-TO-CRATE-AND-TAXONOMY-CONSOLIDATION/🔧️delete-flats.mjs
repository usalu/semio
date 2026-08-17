import { readdirSync, statSync, unlinkSync, existsSync } from "fs";
import { join, relative } from "path";
const fem="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem";
function walk(dir, acc=[]) {
  for (const n of readdirSync(dir)) {
    const p=join(dir,n); const st=statSync(p);
    if (st.isDirectory()) {
      if (n==="target"||n==="⚡️implementations"||n==="📦️packages") continue;
      walk(p,acc);
    } else acc.push(p);
  }
  return acc;
}
const deleted=[];
for (const p of walk(fem)) {
  if (!p.endsWith(".rs")) continue;
  if (p.endsWith("component.rs")) continue;
  // delete flat variants + root lib
  unlinkSync(p);
  deleted.push(relative(fem,p));
}
console.log(JSON.stringify(deleted,null,2));
console.log("count", deleted.length);
