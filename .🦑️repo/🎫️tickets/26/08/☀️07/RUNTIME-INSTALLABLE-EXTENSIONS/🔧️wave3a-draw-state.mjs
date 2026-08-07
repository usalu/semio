import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const fw = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const drawCargo = path.join(root,sDir,"🔌️plugins","🌊️flow",
  fs.readdirSync(path.join(root,sDir,"🔌️plugins","🌊️flow")).find(n=>n.includes("extensions")),
  fs.readdirSync(path.join(root,sDir,"🔌️plugins","🌊️flow", fs.readdirSync(path.join(root,sDir,"🔌️plugins","🌊️flow")).find(n=>n.includes("extensions")))).find(n=>n.includes("draw")),
  "📦️packages","🦀️rust","Cargo.toml");
console.log("=== draw cargo ===\n"+fs.readFileSync(drawCargo,"utf8"));
const flowCargo = path.join(root,fw,"🛍️products","💻️os","🔨️modules","🌊️flow","📦️packages","🦀️rust","Cargo.toml");
console.log("=== flow cargo ===\n"+fs.readFileSync(flowCargo,"utf8"));
const glue = path.join(root,fw,"🛍️products","💚�os","🔨️modules","🌊️flow","📦️packages","🦀️rust","📦️glue.rs");
const glue2 = path.join(root,fw,"🛍️products","💻️os","🔨️modules","🌊️flow","📦️packages","🦀️rust","📦️glue.rs");
console.log("=== glue ===\n"+fs.readFileSync(glue2,"utf8"));
// check root cargo for draw member
const cargo = fs.readFileSync(path.join(root,"Cargo.toml"),"utf8");
console.log("draw members:", cargo.split("\n").filter(l=>l.includes("draw")&&l.includes("flow")));
