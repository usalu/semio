import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join } from "path";
function walk(d, pred, acc = []) { for (const n of readdirSync(d)) { if (["node_modules","target",".git"].includes(n)) continue; const p = join(d,n); try { const s = statSync(p); if (s.isDirectory()) walk(p,pred,acc); else if (pred(p)) acc.push(p);} catch {} } return acc; }
const p = walk(".", x => x.endsWith("🔌️plugin/🦀️component.rs") && x.includes("🛍️products"))[0];
let s = readFileSync(p, "utf8");
const old = "Result<Emit<Self::Operation, Self::ConfigOperation>, MediaError>";
const neu = "Result<Emit<Self::Operation, Self::ConfigOperation, Self::DraftOperation>, MediaError>";
if (!s.includes(old)) throw new Error("missing import_media Emit");
s = s.replace(old, neu);
writeFileSync(p, s);
console.log("ok");
