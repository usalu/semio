import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createRustSourceAnalysisV1, readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const [rustc, graphDir, ...fallbackRoots] = process.argv.slice(2);
const root = process.cwd();
const policy = JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json", "utf8"));
const findBoundary = value => value && typeof value === "object" &&
  (value.sourceBoundary ?? Object.values(value).map(findBoundary).find(Boolean));
const defaultBoundary = findBoundary(policy);
const work = new Map(), measured = [], failures = [], stale = new Set();
const seenManifests = new Set(), manifestCache = new Map(), cfgCache = new Map();
const observer = createRustSourceAnalysisV1();
const read = async path => existsSync(path) && statSync(path).isFile() ? readFileSync(path) : null;
const manifests = path => {
  if (!manifestCache.has(path)) manifestCache.set(path, Bun.TOML.parse(readFileSync(path, "utf8")));
  return manifestCache.get(path);
};
function queue(unit) {
  const key = JSON.stringify([unit.manifest, unit.entry, unit.platform, unit.debug, unit.kind, unit.features]);
  work.set(key, unit);
}
const graphNames = graphDir === "-" ? [] : readdirSync(graphDir).filter(name => /^unit-graph-release-\d+\.json$/.test(name));
for (const file of graphNames) {
  const graph = JSON.parse(readFileSync(join(graphDir, file), "utf8"));
  for (const unit of graph.units) {
    if (!unit.pkg_id.startsWith("path+file:") || unit.mode !== "build") continue;
    const manifest = join(fileURLToPath(unit.pkg_id.slice("path+".length)), "Cargo.toml");
    if (relative(root, manifest).startsWith("..")) continue;
    if (!existsSync(manifest)) { stale.add(relative(root, manifest)); continue; }
    const data = manifests(manifest);
    const kind = unit.target.kind[0];
    let entry = resolve(unit.target.src_path);
    if (!existsSync(entry) && !["bin", "custom-build"].includes(kind)) {
      entry = resolve(dirname(manifest), data.lib?.path ?? "src/lib.rs");
    }
    if (!existsSync(entry)) { stale.add(relative(root, entry)); continue; }
    seenManifests.add(manifest);
    queue({manifest, entry, kind, platform:unit.platform, debug:unit.profile.debug_assertions, features:[...unit.features].sort(), mode:"retained"});
  }
}
if (process.env.SEMIO_ACTIVE_LINES_DESCRIBE_RECEIPT) {
  const receipt = JSON.parse(readFileSync(process.env.SEMIO_ACTIVE_LINES_DESCRIBE_RECEIPT,"utf8"));
  for (const {message} of receipt.units) {
    if (!message.package_id.startsWith("path+file:") || message.profile.test) continue;
    const manifest = message.manifest_path;
    if (relative(root,manifest).startsWith("..")) continue;
    if (!existsSync(manifest)) { stale.add(relative(root,manifest)); continue; }
    const data = manifests(manifest);
    const kind = message.target.kind[0];
    let entry = resolve(message.target.src_path);
    if (!existsSync(entry) && !["bin", "custom-build"].includes(kind)) {
      entry = resolve(dirname(manifest),data.lib?.path ?? "src/lib.rs");
    }
    if (!existsSync(entry)) { stale.add(relative(root,entry)); continue; }
    queue({manifest,entry,kind,platform:null,debug:message.profile.debug_assertions,features:[...message.features].sort(),mode:"descriptor-receipt"});
  }
}
function walk(dir) {
  for (const item of readdirSync(dir,{withFileTypes:true})) {
    const path = join(dir,item.name);
    if (item.isDirectory() && !item.name.startsWith(".") && !["🧪️tests","node_modules","target","dist","🤖️generated"].includes(item.name)) walk(path);
    if (item.name !== "Cargo.toml") continue;
    const data = manifests(path);
    if (!data.package) continue;
    const entry = resolve(dirname(path),data.lib?.path ?? "src/lib.rs");
    if (!existsSync(entry)) continue;
    const defaults = new Set();
    const addFeature = feature => {
      if (!Object.hasOwn(data.features ?? {},feature) || defaults.has(feature)) return;
      defaults.add(feature);
      for (const part of data.features[feature]) addFeature(part);
    };
    addFeature("default");
    const features = [...defaults].sort();
    const platforms = seenManifests.has(path) ? ["wasm32-unknown-unknown"] : ["wasm32-wasip2", "wasm32-unknown-unknown"];
    const kind = data.lib?.["proc-macro"] ? "proc-macro" : "lib";
    for (const platform of kind === "proc-macro" ? [null] : platforms) queue({manifest:path,entry,kind,platform,debug:false,features,mode:"fallback"});
  }
}
for (const path of fallbackRoots) walk(resolve(path));
function cfgFor(unit) {
  const isProc = unit.kind === "proc-macro";
  const key = JSON.stringify([unit.platform,unit.debug,isProc]);
  if (!cfgCache.has(key)) {
    const args = ["--print","cfg","-C",`debug-assertions=${unit.debug?"yes":"no"}`];
    if (unit.platform) args.push("--target",unit.platform);
    if (unit.platform === "wasm32-unknown-unknown") args.push("--cfg",'getrandom_backend="wasm_js"');
    if (isProc) args.push("--crate-type=proc-macro");
    cfgCache.set(key,execFileSync(rustc,args,{encoding:"utf8"}).trim().split("\n"));
  }
  return [...cfgCache.get(key), ...unit.features.map(feature => `feature="${feature}"`)];
}
for (const unit of work.values()) {
  const data = manifests(unit.manifest);
  const boundary = data.package?.metadata?.semio?.["rust-source-boundary"];
  const limit = Math.min(defaultBoundary.maxActiveLines,boundary?.maxActiveLines ?? defaultBoundary.maxActiveLines);
  const packageName = data.package?.name;
  try {
    const rows = await readRustSourceClosureV1([unit.entry],root,cfgFor(unit),read,{manifestDirectory:dirname(unit.manifest),generated:()=>undefined},observer);
    const lines = rows.reduce((sum,row)=>sum+row.activeLines,0);
    const bytes = rows.reduce((sum,row)=>sum+row.bytes,0);
    measured.push({package:packageName, kind:unit.kind, lines, limit, bytes, files:rows.length, platform:unit.platform, features:unit.features, mode:unit.mode, manifest:relative(root,unit.manifest)});
  } catch (error) {
    failures.push({package:packageName, platform:unit.platform, mode:unit.mode, manifest:relative(root,unit.manifest), error:String(error.message)});
  }
}
const ordered = measured.sort((a,b)=>b.lines/b.limit-a.lines/a.limit);
const over = ordered.filter(row=>row.lines>row.limit);
const highest = ordered.slice(0,20);
const priority = ordered.filter(row=>/layout|spatial-kernel|bim|flow-brep|^semio-framework-3d-brep-engine$/.test(row.package));
console.log(JSON.stringify({graphs:graphNames.length, units:work.size, measured:measured.length, omitted:[...stale], failures, over, highest, priority},null,2));
