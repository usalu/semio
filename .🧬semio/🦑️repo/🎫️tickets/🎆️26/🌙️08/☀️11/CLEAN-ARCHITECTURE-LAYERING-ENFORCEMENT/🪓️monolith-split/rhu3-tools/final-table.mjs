import fs from "node:fs";
const rows = fs.readFileSync(process.argv[2], "utf8").trim().split("\n").map(l => JSON.parse(l));
const isTest = p => /\/🧪️tests\//.test(p) || /\/🧫️fixtures\//.test(p) || /\.test\.tsx?$/.test(p) || /📖️stories/.test(p);
const tool = /(^|\/)(?:📜️script\.ts|🏗️builder\/🌐️vite\/🟦️\.ts|🧪️tests\/🎚️config\/🟦️\.ts|(?:⚙️|🧪️)?(?:vite|vitest)\.config\.[cm]?[jt]s)$/;
const OS = "🧰️framework/🛍️products/💻️os/", FW = "🧰️framework/🔨️modules/", ENG = OS + "🔨️modules/📺️renderer/🧑‍🎨engine/";
function group(r) {
  const to = r.to;
  if (to.startsWith(".cache/")) { const m = to.match(/node_modules\/((?:@[^/]+\/)?[^/]+)/); return /^react(-dom)?$/.test(m?.[1] ?? "") ? "A artefact: react resolved through the dependency-image symlink" : "T third-party npm package (" + (m?.[1] ?? "?") + ")"; }
  if (to.includes("🎭️actor/")) return "P parent-owned: Actor";
  if (to.includes("🎬️media") || to.includes("media-export")) return "P parent-owned: Media";
  if (tool.test(r.from)) return "L tooling entry (script, builder, test config)";
  if (to.startsWith("bun:") || /^[a-z]+$/.test(to)) return "T node/bun builtin";
  if (to.includes("bindings/") || to.includes("framework-os-node-graph-rs")) return "W wasm binding modules of WasmSessionLoader";
  if (to === OS + "📦️packages/🟦️typescript/🟦️.ts" || to === OS + "🟦️.ts") return "O os wire stack: `@semio-tech/framework-os` (backbone, directory commands, document archive)";
  if (to.startsWith(OS + "🔨️modules/📇️directory/")) return "O os wire stack: directory (hub sign-in, spaces, delegations, session, access policy)";
  if (to.startsWith(OS + "🔨️modules/💡️inference/") || to.startsWith(OS + "🔨️modules/🔌️plugin/🌐️browser-bundle/")) return "O os wire stack: browser-bundle action and patch handoff, inference opening";
  if (to.startsWith(OS + "🔨️modules/🔌️plugin/📡️backbone/")) return "H actor-document binding (`plugin/📡️backbone/🔗️binding`)";
  if (to.startsWith(OS + "🔨️modules/🔌️plugin/")) return "H plugin registry: hub source, deployment, catalog";
  if (to.startsWith(OS + "🔨️modules/♾️infinite/🌍️world/")) return "R r3f package `@semio-tech/infinite-world-r3f`";
  if (to.startsWith(OS + "🔨️modules/♾️infinite/")) return "R dag-input (NodeGraph selection and hover wire)";
  if (to.startsWith(OS + "🎚️config/")) return "C os config mutations";
  if (to.startsWith(OS + "🔨️modules/🌉️mcp/")) return "M mcp gateway wire codecs (TS twin of the Rust bridge)";
  if (to.startsWith(OS + "🔨️modules/🖥️shell/")) return "M shell state reducer (TS twin of the Rust reducer)";
  if (to.startsWith(OS + "🔨️modules/🏪️store/")) return "O os store worker constants";
  if (to.startsWith(ENG)) return "E renderer engine shared leaf (UiPreferences, resident, work)";
  if (to.startsWith(FW + "🗺️surface/")) return "W wasm binding modules of WasmSessionLoader";
  if (to.includes("🦑️repo/")) return "L tooling entry (script, builder, test config)";
  if (to.startsWith(FW)) return "F framework module (kernel, trace, replication, canvas, pixels, ...): fixable through an SDK entry";
  return "? " + to.slice(0, 80);
}
const cats = new Map();
for (const r of rows) { const k = (isTest(r.from) ? "test|" : "prod|") + group(r); cats.set(k, (cats.get(k) ?? 0) + 1); }
for (const part of ["prod", "test"]) { console.log("### " + part); let t = 0; for (const [k, v] of [...cats].filter(([k]) => k.startsWith(part)).sort((a, b) => a[0].slice(5).localeCompare(b[0].slice(5)))) { console.log(String(v).padStart(4), k.slice(5)); t += v; } console.log(String(t).padStart(4), "total"); }
if (process.argv[3] === "parent") { console.log("\n### parent-owned prod edges"); for (const r of rows) if (!isTest(r.from) && /^P /.test(group(r))) console.log(r.from.replace(ENG, "RT/").replace("🧱️elements/", "E/"), "->", r.to.replace(OS, "OS/").replace("🧰️framework/", "F/")); }
