import { readFileSync } from "node:fs";
import { dirname, basename, join } from "node:path";
import { spawnSync } from "node:child_process";
const ROOT = process.cwd();
const strip = (s: string) => {
  let out = "", i = 0, inStr = false;
  while (i < s.length) {
    const c = s[i];
    if (inStr) { out += c; if (c === "\\") { out += s[i + 1]; i += 2; continue; } if (c === '"') inStr = false; i++; continue; }
    if (c === '"') { inStr = true; out += c; i++; continue; }
    if (c === "/" && s[i + 1] === "/") { while (i < s.length && s[i] !== "\n") i++; continue; }
    if (c === "/" && s[i + 1] === "*") { i += 2; while (i < s.length && !(s[i] === "*" && s[i + 1] === "/")) i++; i += 2; continue; }
    out += c; i++;
  }
  return out.replace(/,(\s*[}\]])/g, "$1");
};
const isManifest = (f: string) => ["project.json", "📋️project.json", "Cargo.toml", "package.json"].includes(basename(f));
const tracked = spawnSync("git", ["ls-files", "-z"], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\0").filter(Boolean);
const untracked = spawnSync("git", ["ls-files", "-z", "--others", "--exclude-standard"], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\0").filter(Boolean);
const all = [...new Set([...tracked, ...untracked])].filter((f) => !f.startsWith(".🧬semio/") && !f.includes("node_modules/") && isManifest(f));
const names = new Map<string, string[]>();
for (const f of all) {
  let name: string | undefined;
  try {
    if (basename(f).endsWith("project.json")) name = JSON.parse(readFileSync(join(ROOT, f), "utf8")).name;
    else if (basename(f) === "Cargo.toml") name = (Bun.TOML.parse(readFileSync(join(ROOT, f), "utf8")) as any).package?.name;
    else name = JSON.parse(readFileSync(join(ROOT, f), "utf8")).name;
  } catch { continue; }
  if (!name) continue;
  names.set(name, [...(names.get(name) ?? []), f]);
}
const dupProjects = [...names].filter(([, fs]) => fs.length > 1).map(([n, fs]) => ({ name: n, dirs: fs.map((f) => dirname(f)) }));
const compounds: any = {};
for (const f of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) {
  const j = JSON.parse(strip(readFileSync(join(ROOT, f), "utf8")));
  const cfgNames = new Set((j.configurations ?? []).map((c: any) => c?.name));
  let inline = 0, byName = 0, missing = 0;
  for (const c of j.compounds ?? []) for (const x of c.configurations ?? []) { if (typeof x === "string") { byName++; if (!cfgNames.has(x)) missing++; } else inline++; }
  compounds[f] = { compounds: (j.compounds ?? []).length, byName, inlineObjects: inline, missingNames: missing };
}
console.log(JSON.stringify({ manifestNames: names.size, dupProjects, compounds }, null, 1));
