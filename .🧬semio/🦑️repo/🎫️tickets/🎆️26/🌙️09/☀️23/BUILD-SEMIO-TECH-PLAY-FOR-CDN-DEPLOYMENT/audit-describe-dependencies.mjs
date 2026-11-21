import {existsSync, readFileSync, realpathSync} from "node:fs";
import {dirname, join, resolve} from "node:path";

const os = "🧰️framework/🛍️products/💻️os/🔨️modules";
const packageAt = (path) => resolve(os, path, "📦️packages/🦀️rust/Cargo.toml");
const start = packageAt("🔌️plugin/🖨️describe");
const packs = {
  "semio-os-pack": packageAt("🎒️pack"),
  "semio-os-store-codec-registry": packageAt("🏪️store/📦️codec"),
  "semio-os-vcs-entity-identity": packageAt("🌿️vcs/🚪️io/💾️binary/🪪️entity-identity"),
  "semio-os-identity": packageAt("🪪️identity"),
  "semio-os-channel": packageAt("📡️spr/🧵️channel"),
  "semio-os-store-text-io": packageAt("🏪️store/🚪️io"),
  "semio-os-io": packageAt("🚪️io"),
  "semio-framework-replication": resolve("🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml"),
  "semio-framework-io-control": resolve("🧰️framework/🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust/Cargo.toml"),
};
const alternatives = {
  "semio-plugin-describe": ["semio-os-pack"],
  "semio-plugin-guest-contract": ["semio-os-store-codec-registry"],
  "semio-plugin-interpreter-runtime": ["semio-os-vcs-entity-identity", "semio-os-identity", "semio-os-channel"],
  "semio-plugin-interpreter-imports": ["semio-os-vcs-entity-identity", "semio-os-identity"],
  "semio-plugin-abi": [
    "semio-os-io", "semio-os-store-text-io", "semio-os-pack", "semio-framework-replication",
    "semio-os-vcs-entity-identity", "semio-framework-io-control", "semio-os-channel",
  ],
};
const cache = new Map();
function manifest(path) {
  if (!cache.has(path)) cache.set(path, Bun.TOML.parse(readFileSync(path, "utf8")));
  return cache.get(path);
}
function workspace(path) {
  const selected = manifest(path).package?.workspace;
  if (selected) return realpathSync(resolve(dirname(path), selected, "Cargo.toml"));
  let dir = dirname(path);
  for (; dirname(dir) !== dir; dir = dirname(dir)) {
    const candidate = join(dir, "Cargo.toml");
    if (existsSync(candidate) && manifest(candidate).workspace) return candidate;
  }
  throw Error("Workspace missing for " + path);
}
function dependencies(path, expanded) {
  const data = manifest(path);
  const tables = [data.dependencies];
  if (expanded) {
    tables.push(data["build-dependencies"]);
    for (const target of Object.values(data.target ?? {})) tables.push(target.dependencies, target["build-dependencies"]);
  }
  const result = new Map();
  for (const [key, dependency] of tables.flatMap((table) => Object.entries(table ?? {}))) {
    if (!expanded && dependency?.optional) continue;
    const owner = dependency?.workspace ? workspace(path) : path;
    const spec = dependency?.workspace ? manifest(owner).workspace.dependencies[key] : dependency;
    if (spec?.path) {
      const target = realpathSync(resolve(dirname(owner), spec.path, "Cargo.toml"));
      result.set(manifest(target).package.name, target);
    }
  }
  return result;
}
const tracked = new Set(["semio-os-kernel", "semio-os-store-document"]);
const retain = new Set([
  "semio-os-store-member-open-store", "semio-framework-dsl", "semio-framework-value-original-slot",
  "semio-framework-pack-json",
  "semio-os-store-engine", "semio-os-store-engine-core", "semio-os-store-engine-live", "semio-os-store-engine-bundle",
]);
function scan(expanded, replace) {
  const stack = [{path:start, trail:[]}], visited = new Set(), matchingEdges = new Map(), watched = new Map();
  while (stack.length) {
    const {path, trail} = stack.shift(), name = manifest(path).package.name, route = [...trail,name];
    if (tracked.has(name)) {
      matchingEdges.set(route.slice(-2).join(" -> "),route.join(" -> "));
    }
    if (visited.has(path)) continue;
    visited.add(path);
    if (retain.has(name)) watched.set(name,route.join(" -> "));
    const edges = dependencies(path,expanded);
    if (replace && Object.hasOwn(alternatives,name) && edges.delete("semio-os-kernel")) {
      for (const alternate of alternatives[name]) {
        const target = packs[alternate];
        if (!target || manifest(target).package.name !== alternate) throw Error("Unknown alternate " + alternate);
        edges.set(alternate,target);
      }
    }
    for (const target of edges.values()) stack.push({path:target,trail:route});
  }
  console.log(JSON.stringify({expanded,replace,visited:visited.size,matchingEdges:[...matchingEdges.values()],watched:[...watched.values()]},null,2));
}
for (const expanded of [false,true]) for (const replace of [false,true]) scan(expanded,replace);
