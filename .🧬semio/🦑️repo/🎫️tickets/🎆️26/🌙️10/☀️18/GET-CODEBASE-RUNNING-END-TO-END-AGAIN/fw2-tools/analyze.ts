import { readFileSync } from "node:fs";
import { join } from "node:path";
const workspace = process.cwd();
const taxonomy = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const contract = taxonomy.generatorContracts["wgpu-frame-worker"], profile = contract.packageGeneration.browserProfile;
const observed = new Set<string>();
for (const entry of profile.entries) {
  await Bun.build({ entrypoints: [join(workspace, profile.ownerPath, entry.sourceRelativePath)], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "o", setup(build) { build.onLoad({ filter: /.*/ }, (args) => { observed.add(args.path.slice(workspace.length + 1).replaceAll("\\", "/")); return undefined; }); } }] });
}
const project = JSON.parse(readFileSync(join(workspace, profile.ownerPath, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
const fws: string[] = project.namedInputs.frameWorkerSources.filter((x: unknown) => typeof x === "string");
const globToRe = (g: string) => new RegExp("^" + g.replace("{workspaceRoot}/", "").replace(/[.+^${}()|[\]\\]/g, "\\$&").replace(/\*\*\//g, "(?:.*/)?").replace(/\*/g, "[^/]*") + "$", "u");
const res = fws.map(globToRe);
const inFws = (p: string) => res.some(r => r.test(p));
const I = new Set<string>(contract.inputPatterns);
const o = [...observed];
console.log("observed", o.length, "inInputPatterns", o.filter(p => I.has(p)).length, "inFws", o.filter(inFws).length, "inEither", o.filter(p => I.has(p) || inFws(p)).length);
console.log("neither:", o.filter(p => !I.has(p) && !inFws(p)).filter(p => !p.includes("/🤖️generated/")).length);
for (const p of o.filter(p => !I.has(p) && !inFws(p)).sort()) console.log("  ", p);
console.log("inputPatterns not in closure:");
for (const p of contract.inputPatterns) if (!observed.has(p)) console.log("  ", p);
console.log("fws entries", fws.length); for (const f of fws.slice(0,400)) console.log("   F", f.replace("{workspaceRoot}/","")); 
