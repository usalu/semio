import { readFileSync, writeFileSync, existsSync } from "fs";
import { execSync } from "child_process";

const ticket = process.argv[2];

function rgFiles(pattern) {
  try {
    return execSync(`rg -l ${JSON.stringify(pattern)} -g '!target/**' -g '!node_modules/**' -g '!.🦑️repo/**' -g '!storybook-static/**' -g '!.venv/**'`, { encoding: "utf8" })
      .trim().split("\n").filter(Boolean);
  } catch { return []; }
}

function replaceInFiles(files, pairs, label) {
  let n = 0;
  for (const f of files) {
    if (!existsSync(f)) continue;
    let t = readFileSync(f, "utf8");
    const o = t;
    for (const [a, b] of pairs) t = t.split(a).join(b);
    if (t !== o) { writeFileSync(f, t); n++; console.log(label, f); }
  }
  console.log(label, "updated", n);
}

// 1) flow extension crate path/name
{
  const files = rgFiles("flow-extension-core|extensions/� combcore/📦️packages|extensions/� combcore/");
  // also scan Cargo.toml specifically
  const cargo = execSync(`rg -l 'flow-extension-core|imperative-core' --glob 'Cargo.toml' -g '!target/**' || true`, { encoding: "utf8" }).trim().split("\n").filter(Boolean);
  const all = [...new Set([...files, ...cargo, "Cargo.toml"])];
  replaceInFiles(all, [
    ["semio-s-plugin-flow-extension-core", "semio-s-plugin-flow-extension-primitive"],
    ["🌊️flow/� combextensions/� combcore/", "🌊️flow/� combextensions/🔤️primitive/"],
    ["semio-s-plugin-imperative-core", "semio-s-plugin-imperative-effect"],
    ["📜️imperative/� combextensions/� combcore/", "📜️imperative/� combextensions/📣️effect/"],
  ], "ext-rename");
}

// Fix with exact emoji by reading Cargo.toml and replacing known old path strings from deferred JSON
{
  const deferredFlow = JSON.parse(readFileSync(`${ticket}/deferred-flow-ext.json`, "utf8"));
  const deferredImp = JSON.parse(readFileSync(`${ticket}/deferred-imperative-ext.json`, "utf8"));
  for (const entry of [...deferredFlow.workspaceMembers, ...deferredImp.workspaceMembers]) {
    const files = rgFiles(entry.oldPath.split("/").slice(-3).join("/")) ;
    // broader: any file containing oldPackageName or oldPath
    const byName = rgFiles(entry.oldPackageName);
    const byPath = (() => { try { return execSync(`rg -l ${JSON.stringify(entry.oldPath)} -g '!target/**' -g '!node_modules/**' -g '!.🦑️repo/**' || true`, {encoding:"utf8"}).trim().split("\n").filter(Boolean);} catch {return [];}})();
    replaceInFiles([...new Set([...byName, ...byPath, "Cargo.toml"])], [
      [entry.oldPackageName, entry.newPackageName],
      [entry.oldPath, entry.newPath],
    ], "deferred-ext");
  }
  if (deferredImp.externCrateRenames) {
    for (const r of deferredImp.externCrateRenames) {
      replaceInFiles(rgFiles(r.old), [[r.old, r.new]], "extern-crate");
    }
  }
}

// 2) flow_core → flow
{
  const files = rgFiles("flow_core");
  replaceInFiles(files, [["flow_core", "flow"]], "flow_core");
}

// 3) Remaining core aliases
{
  replaceInFiles(rgFiles("dsl_core::"), [["dsl_core::", "os_dsl::"], ["dsl_core", "os_dsl"]], "dsl_core");
  replaceInFiles(rgFiles("db_core::"), [["db_core::", "db::"], ["use db_core", "use db"]], "db_core");
  replaceInFiles(rgFiles("animate_core"), [["animate_core", "animate"]], "animate_core");
  replaceInFiles(rgFiles("os_pack::core"), [["os_pack::core::", "os_pack::"], ["os_pack::core", "os_pack"]], "os_pack_core");
  replaceInFiles(rgFiles("os_spr::core"), [["os_spr::core::", "os_spr::"], ["os_spr::core", "os_spr"]], "os_spr_core");
  replaceInFiles(rgFiles("os_dsl::core"), [["os_dsl::core::", "os_dsl::"], ["os_dsl::core", "os_dsl"]], "os_dsl_core");
}

// 4) Cargo workspace aliases *-core pointing at packages (rename alias keys)
{
  let cargo = readFileSync("Cargo.toml", "utf8");
  const o = cargo;
  cargo = cargo.replaceAll("semio-framework-os-kernel-flow-core", "semio-framework-os-kernel-flow");
  cargo = cargo.replaceAll("semio-framework-os-kernel-db-core", "semio-framework-os-kernel-db");
  cargo = cargo.replaceAll("semio-framework-os-kernel-pack-core", "semio-framework-os-kernel-pack");
  cargo = cargo.replaceAll("semio-framework-os-kernel-protocol-core", "semio-framework-os-kernel-protocol");
  cargo = cargo.replaceAll("semio-framework-os-kernel-dsl-core", "semio-framework-os-kernel-dsl");
  if (cargo !== o) { writeFileSync("Cargo.toml", cargo); console.log("root Cargo aliases renamed"); }
  // update consumers of those alias names
  replaceInFiles(rgFiles("semio-framework-os-kernel-flow-core"), [["semio-framework-os-kernel-flow-core", "semio-framework-os-kernel-flow"]], "alias-flow");
  replaceInFiles(rgFiles("semio-framework-os-kernel-db-core"), [["semio-framework-os-kernel-db-core", "semio-framework-os-kernel-db"]], "alias-db");
  replaceInFiles(rgFiles("semio-framework-os-kernel-pack-core"), [["semio-framework-os-kernel-pack-core", "semio-framework-os-kernel-pack"]], "alias-pack");
  replaceInFiles(rgFiles("semio-framework-os-kernel-protocol-core"), [["semio-framework-os-kernel-protocol-core", "semio-framework-os-kernel-protocol"]], "alias-protocol");
  replaceInFiles(rgFiles("semio-framework-os-kernel-dsl-core"), [["semio-framework-os-kernel-dsl-core", "semio-framework-os-kernel-dsl"]], "alias-dsl");
}

// 5) framework-core leftover path strings
{
  replaceInFiles(rgFiles("🧩core"), [["🧩core/", ""], ["modules/🧩core", "modules"]], "puzzle-core-path");
  replaceInFiles(rgFiles("@semio-tech/framework-core"), [["@semio-tech/framework-core", "@semio-tech/framework"]], "ts-pkg");
  replaceInFiles(rgFiles("@semio-tech/framework-os-core"), [["@semio-tech/framework-os-core", "@semio-tech/framework-os"]], "ts-os-pkg");
}

console.log("wave2 deferred apply done");
