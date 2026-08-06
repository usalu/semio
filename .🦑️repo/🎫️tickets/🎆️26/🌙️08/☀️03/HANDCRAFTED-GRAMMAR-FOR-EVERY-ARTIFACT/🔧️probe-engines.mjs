import { readFileSync, readdirSync, existsSync } from "fs";
import { join } from "path";
const engines = [];
for (const plug of ["🕸️dag","🗒️note","✒️writer","🏗️fem"]) {
  const arts = readdirSync(join("✏️s","🔌️plugins",plug,"🗿️artifacts"));
  for (const art of arts) {
    const path = join("✏️s","🔌️plugins",plug,"🗿️artifacts",art,"⚙️engine","🦀️component.rs");
    if (existsSync(path)) engines.push(path);
  }
}
for (const path of engines) {
  const t = readFileSync(path,"utf8");
  console.log("====", path, "====");
  const i = t.indexOf("pub fn register");
  console.log(i < 0 ? t.slice(0,600) : t.slice(i, i+900));
}
const glue = join("✏️s","🔌️plugins","🏗️fem","📦️packages","🦀️rust","📦️glue.rs");
const g = readFileSync(glue,"utf8");
console.log("==== FEM GLUE ====");
console.log(g.slice(g.indexOf("Artifacts"), g.indexOf("Artifacts")+1800));