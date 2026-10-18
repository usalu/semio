// 🧪️ C5B probe: evaluates the callers-corpus assertion and the flow retirement struct extraction over the module trees (read-only).
import {readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import {readModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
const root = "/Users/ueli/Documents/semio";
const callers = JSON.parse(readFileSync(join(root, "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📥️admission/🧪️tests/📋️callers.json"), "utf8"));
let bad = 0, n = 0;
for (const row of callers.callers) {
  if (!/plugin\/📜️declaration|plugin\/🏗️builder/.test(row.source)) continue;
  n += 1;
  const src = readModuleTreeSource(join(root, row.source));
  const okc = src.includes("semio_framework_composition_admission::") && !/(?:os_store|store)::ChildRestoreProjection/u.test(src);
  console.log(okc ? "ok  " : "FAIL", row.source.slice(-60));
  if (!okc) bad += 1;
}
const owner = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
const mounted = ["📜️declaration/👷️job/🦀️.rs", "📜️declaration/📨️emission/🦀️.rs", "📜️declaration/📥️natural-file/🦀️.rs", "📜️declaration/🧩️app/🦀️.rs", "📜️declaration/🪟️surface/🦀️.rs"];
const plugin = [readFileSync(resolve(owner, "🦀️.rs"), "utf8"), ...mounted.map(p => readModuleTreeSource(resolve(owner, p)))].join("\n");
for (const name of ["ArtifactToolCompletionRejection", "Emit", "EphemeralEmit", "ExtensionInvocation", "AsyncTask"]) {
  const m = plugin.match(new RegExp("pub struct " + name + "(?:<[^\\n]+>)? \\{([^}]+)\\}"));
  console.log(m ? "ok  " : "FAIL", "struct", name);
  if (!m) bad += 1;
}
console.log("callers rows", n, "bad", bad);
