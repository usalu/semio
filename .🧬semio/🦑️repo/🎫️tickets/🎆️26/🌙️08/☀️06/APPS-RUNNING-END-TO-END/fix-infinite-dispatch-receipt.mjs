import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

function find(dir, pred, depth = 0, acc = []) {
  if (depth > 12) return acc;
  let ents;
  try {
    ents = readdirSync(dir);
  } catch {
    return acc;
  }
  for (const name of ents) {
    if (["node_modules", "target", "dist", ".git"].includes(name)) continue;
    const p = join(dir, name);
    let st;
    try {
      st = statSync(p);
    } catch {
      continue;
    }
    if (st.isDirectory()) find(p, pred, depth + 1, acc);
    else if (pred(name, p)) acc.push(p);
  }
  return acc;
}

const fw = readdirSync("/Users/ueli/Documents/semio").find((n) => n.includes("framework") && readdirSync(join("/Users/ueli/Documents/semio", n)).includes("🛍️products"));
const file = find(join("/Users/ueli/Documents/semio", fw), (n, p) => n.endsWith(".rs") && p.includes("infinite") && p.includes("dag") && p.includes("directed") && readFileSync(p, "utf8").includes("pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue>"))[0];
console.log("file", file);
let t = readFileSync(file, "utf8");
const a = `self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))`;
const b = `self.store.borrow_mut().dispatch_text(command_text).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))`;
const c = `self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))`;
const d = `self.store.borrow_mut().dispatch_binary(command_bytes).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))`;
if (!t.includes(a) || !t.includes(c)) {
  console.error("patterns missing");
  process.exit(1);
}
t = t.replace(a, b).replace(c, d);
writeFileSync(file, t);
console.log("patched dispatch receipts");
