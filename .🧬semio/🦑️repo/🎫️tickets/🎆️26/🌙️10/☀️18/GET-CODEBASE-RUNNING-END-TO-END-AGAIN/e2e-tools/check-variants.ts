/** 🎮️ Compares the playground variants the product targets need (`framework-os-dev:<op>-<variant>-react-<profile>`, `plugin-registry:session-<variant>`) with the variants declared in Cargo manifests. */
import { readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
const root = "/Users/ueli/Documents/semio";
const closure = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { inferred: string[]; missing_target: string[] };
const manifests = execFileSync("git", ["ls-files", "-co", "--exclude-standard", "-z", "--", "🌎️hub", "✏️s", "🧰️framework"], { cwd: root, maxBuffer: 1 << 28 }).toString("utf8").split("\0").filter((p) => p.endsWith("Cargo.toml") && !p.includes("node_modules"));
const declared = new Map<string, string>();
for (const path of manifests) {
  let manifest: any;
  try { manifest = Bun.TOML.parse(readFileSync(`${root}/${path}`, "utf8")); } catch { continue; }
  const metadata = manifest.package?.metadata, semio = metadata?.semio;
  if (!metadata?.component?.package || !["plugin", "extension"].includes(semio?.["component-kind"])) continue;
  for (const row of semio.playground ?? []) declared.set(row.variant, path);
}
const needed = new Set<string>();
for (const key of [...closure.inferred, ...closure.missing_target]) {
  const m = /^@semio-tech\/(?:framework-os-dev:(?:prepare|activate|serve|dev)-(.+)-react-(?:prototype|release)|plugin-registry:session-(.+))$/.exec(key);
  if (m) needed.add((m[1] ?? m[2])!);
}
console.log(JSON.stringify({ declared: declared.size, needed: needed.size, notDeclared: [...needed].filter((v) => !declared.has(v)).sort(), declaredUnused: [...declared.keys()].filter((v) => !needed.has(v)).sort() }, null, 1));
