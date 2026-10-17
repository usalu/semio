// 🔎 OSD: parses the touched manifests with Bun's TOML and @iarna-free structural checks: workspace path, lib path, dependency paths, workspace membership, owners rows, project names, longest repo-relative path.
import {readFileSync, existsSync} from "node:fs";
import {resolve} from "node:path";
const R = "/Users/ueli/Documents/semio";
const M = "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory";
const pkgs = ["📦️packages", "🧬️schema/📦️packages", "🧬️schema/🌐️browser-actor/📦️packages", "🧬️schema/📌️document-check-in-v1/📦️packages", "🚪️io/🧱️binary/📦️packages", "🛡️access-policy/📦️packages", "🧪️tests/🍯️companion-1/📦️packages", "🧪️tests/🍰️companion-2/📦️packages"].map(d => `${M}/${d}/🦀️rust`);
const ws = Bun.TOML.parse(readFileSync(`${R}/Cargo.toml`, "utf8")) as any;
const owners = (JSON.parse(readFileSync(`${R}/📋️build-owners.json`, "utf8")) as any).owners as {name: string; root: string; manifest: string}[];
const members: string[] = ws.workspace.members;
let bad = 0, longest = 0;
for (const d of pkgs) {
  const m = resolve(R, d, "Cargo.toml");
  const t = Bun.TOML.parse(readFileSync(m, "utf8")) as any;
  const wsRoot = resolve(R, d, t.package.workspace);
  const okWs = wsRoot === R && existsSync(resolve(wsRoot, "Cargo.toml"));
  const lib = resolve(R, d, t.lib.path);
  const miss: string[] = [];
  for (const sect of ["dependencies", "dev-dependencies"]) for (const [k, v] of Object.entries(t[sect] ?? {}) as [string, any][]) {
    if (v && v.path && !existsSync(resolve(R, d, v.path, "Cargo.toml"))) miss.push(`${k}:${v.path}`);
  }
  const project = JSON.parse(readFileSync(resolve(R, d, "📋️project.json"), "utf8"));
  const ownerOk = owners.some(o => o.name === project.name && o.root === d && o.manifest === `${d}/📋️project.json`);
  const memberOk = members.includes(d);
  const bytes = Buffer.byteLength(`${d}/Cargo.toml`);
  longest = Math.max(longest, bytes);
  const boundary = JSON.stringify(t.package.metadata.semio["rust-source-boundary"]);
  const okBoundary = boundary === JSON.stringify({maxSourceBytes: 262144, maxSourceFiles: 32, maxActiveLines: 4000}) || boundary.includes("262144");
  if (!okWs || !existsSync(lib) || miss.length || !ownerOk || !memberOk || bytes > 240 || !okBoundary) bad++;
  console.log(t.package.name, "ws", okWs, "lib", existsSync(lib), "missing", miss, "owner", ownerOk, "member", memberOk, "pathBytes", bytes, "project", project.name);
}
const dirMembers = members.filter(m => m.includes("📇️directory"));
console.log("directory members", dirMembers.length, "byte-sorted", JSON.stringify(dirMembers) === JSON.stringify([...dirMembers].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)))));
for (const m of dirMembers) if (!existsSync(`${R}/${m}/Cargo.toml`)) { console.log("MISSING member", m); bad++; }
console.log("longest manifest path bytes", longest, "bad", bad);
