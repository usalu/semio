import fs from "fs";
import path from "path";
const OS = fs.readFileSync("/tmp/os-path.txt", "utf8").trim();
const modules = path.join(OS, fs.readdirSync(OS).find((x) => x.includes("modules")));
function findComp(relParts) {
  let p = modules;
  for (const part of relParts) {
    const hit = fs.readdirSync(p).find((x) => x.includes(part) || x === part);
    if (!hit) throw new Error("missing " + part + " in " + p);
    p = path.join(p, hit);
  }
  if (fs.statSync(p).isDirectory()) {
    const c = fs.readdirSync(p).find((x) => x.includes("component"));
    p = path.join(p, c);
  }
  return p;
}
const sprCore = findComp(["spr", "core"]);
const sc = fs.readFileSync(sprCore, "utf8");
for (const name of ["HistoryOpMeta", "OperationMeta", "pub enum UndoPolicy", "pub struct ActorId"]) {
  const i = sc.indexOf(name);
  console.log(name, i);
  if (i >= 0) console.log(sc.slice(i, i + 400), "\n---");
}
const store = findComp(["store"]);
const st = fs.readFileSync(store, "utf8");
const qi = st.indexOf("semio_framework_core::UndoPolicy");
console.log("qualified UndoPolicy context:\n", st.slice(qi - 80, qi + 120));
// sync deps needs
const sync = findComp(["store", "sync"]);
const synct = fs.readFileSync(sync, "utf8");
console.log("\nsync size", synct.length);
console.log("sync imports core:", synct.match(/use semio_framework_core::\{[^}]+\}/g));
console.log("has notify?", synct.includes("notify"));
console.log("has tokio::", /tokio::/.test(synct));
console.log("has wasm_bindgen?", synct.includes("wasm_bindgen"));
console.log("cfg gates:", [...synct.matchAll(/#\[cfg\([^\]]+\]/g)].slice(0, 20).map((m) => m[0]));
