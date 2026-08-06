import fs from "fs";
import path from "path";
const OS = fs.readFileSync("/tmp/os-path.txt", "utf8").trim();
const modules = path.join(OS, fs.readdirSync(OS).find((x) => x.includes("modules")));
const spr = path.join(modules, fs.readdirSync(modules).find((x) => x.includes("spr") && !x.includes("protocol")));
const sprCoreDir = path.join(spr, fs.readdirSync(spr).find((x) => x.includes("core")));
const sprCore = path.join(sprCoreDir, fs.readdirSync(sprCoreDir).find((x) => x.includes("component")));
const sc = fs.readFileSync(sprCore, "utf8");
const idx = sc.indexOf("OperationMeta");
console.log(sc.slice(idx, idx + 700));
const cmdDir = path.join(spr, fs.readdirSync(spr).find((x) => x.includes("command")));
const cmd = path.join(cmdDir, fs.readdirSync(cmdDir).find((x) => x.includes("component")));
const ct = fs.readFileSync(cmd, "utf8");
for (const pat of ["fn operation_id", "trait Operation", "UndoPolicy", "use crate::os_spr::core", "use semio_framework_core"]) {
  const i = ct.indexOf(pat);
  console.log("\n==", pat, i, "==");
  if (i >= 0) console.log(ct.slice(i, i + 250));
}
const vcs = path.join(modules, fs.readdirSync(modules).find((x) => x.includes("vcs")), fs.readdirSync(path.join(modules, fs.readdirSync(modules).find((x) => x.includes("vcs")))).find((x) => x.includes("component")));
const vt = fs.readFileSync(vcs, "utf8");
const ci = vt.indexOf("create_document_vcs_id");
console.log("\ncreate_document_vcs_id:\n", vt.slice(ci, ci + 200));
const store = path.join(modules, fs.readdirSync(modules).find((x) => x.includes("store")), "component.rs");
// fix: find component
const storeDir = path.join(modules, fs.readdirSync(modules).find((x) => x.includes("store")));
const storeComp = path.join(storeDir, fs.readdirSync(storeDir).find((x) => x.includes("component")));
const st = fs.readFileSync(storeComp, "utf8");
console.log("\nstore core type refs count", (st.match(/semio_framework_core::(ActorId|DocumentId|HybridLogicalTimestamp|OperationId|SchemaId|UndoPolicy)/g) || []).length);
console.log("qualified refs:", [...new Set(st.match(/semio_framework_core::(ActorId|DocumentId|HybridLogicalTimestamp|OperationId|SchemaId|UndoPolicy)/g) || [])]);
// PresencePeer
console.log("PresencePeer in spr?", sc.includes("PresencePeer"));
console.log("PresencePeer in store?", st.includes("PresencePeer"));
