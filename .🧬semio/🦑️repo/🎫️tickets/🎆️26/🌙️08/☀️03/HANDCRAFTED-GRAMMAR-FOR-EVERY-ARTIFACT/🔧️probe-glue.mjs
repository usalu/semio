import { readFileSync, readdirSync, statSync, writeFileSync, existsSync } from "fs";
import { join } from "path";
function findFile(root, pred, depth=8) {
  if (depth<0) return null;
  for (const n of readdirSync(root)) {
    const p=join(root,n); let st; try{st=statSync(p);}catch{continue;}
    if (st.isDirectory()) { if(n==="target"||n==="node_modules") continue; const h=findFile(p,pred,depth-1); if(h) return h; }
    else if (pred(n,p)) return p;
  } return null;
}
for (const plug of ["dag","note","writer","fem"]) {
  const base=readdirSync(join("✏️s","🔌️plugins")).find(n=>n.includes(plug));
  const glue=join("✏️s","🔌️plugins",base,"📦️packages","🦀️rust","📦️glue.rs");
  console.log("====", glue, "====");
  console.log(readFileSync(glue,"utf8").split("\n").slice(0,120).join("\n"));
}