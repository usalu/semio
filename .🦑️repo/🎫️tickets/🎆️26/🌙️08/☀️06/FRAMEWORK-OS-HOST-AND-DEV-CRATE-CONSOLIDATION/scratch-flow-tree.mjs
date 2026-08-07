import { existsSync, readdirSync, statSync, readFileSync } from "fs";
import { join } from "path";
const flow = "/Users/ueli/Documents/semio/𝒯framework/🛍️products/💻️os/🔨️modules/🌊️flow";
const flow2 = [...Bun.Glob("𝒯framework/**/🌊️flow").scanSync({cwd:"/Users/ueli/Documents/semio"})];
console.log("glob", flow2);
const real = "/Users/ueli/Documents/semio/" + (flow2[0] || "");
console.log("real", real, existsSync(real));
function tree(dir, depth=0, max=4) {
  if (!existsSync(dir) || depth>max) return;
  for (const name of readdirSync(dir)) {
    if (["target","node_modules"].includes(name)) continue;
    const p=join(dir,name); const st=statSync(p);
    console.log("  ".repeat(depth)+(st.isDirectory()?"D ":"F ")+name+(st.isFile()?` ${st.size}`:""));
    if (st.isDirectory()) tree(p, depth+1, max);
  }
}
tree(real);
const cargo = join(real, "📦️packages/🦀️rust/Cargo.toml");
console.log("cargo exists", existsSync(cargo));
if (existsSync(cargo)) console.log(readFileSync(cargo,"utf8").slice(0,800));
const core = join(real, " lobbies");
