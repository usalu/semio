/**
 * 🔎️ Lists every `exports` target (all conditions) of every workspace package.json and reports the ones that do not exist.
 *
 *   bun glue-tools/check-exports.ts [--json] [--all]
 */
import { Glob } from "bun";
import { existsSync, readdirSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";

export const ROOT = resolve(import.meta.dir, "../../../../../../../..");

export type Target = { manifest: string; name: string; subpath: string; conditions: string; target: string; file: string; exists: boolean; problem?: string; buildOutput?: boolean };

const collect = (value: unknown, conditions: string[], out: { conditions: string; target: string }[]): void => {
  if (typeof value === "string") out.push({ conditions: conditions.join("."), target: value });
  else if (Array.isArray(value)) value.forEach(v => collect(v, conditions, out));
  else if (value && typeof value === "object") for (const [k, v] of Object.entries(value)) collect(v, [...conditions, k], out);
};

export const workspaceRoots = (): string[] => [ROOT, ...readdirSync(ROOT, { withFileTypes: true }).filter(d => d.isDirectory() && !d.name.startsWith(".") && existsSync(resolve(ROOT, d.name, "package.json")) && JSON.parse(readFileSync(resolve(ROOT, d.name, "package.json"), "utf8")).workspaces).map(d => resolve(ROOT, d.name))];

export function workspaceManifests(): string[] {
  const manifests = new Set<string>();
  for (const root of workspaceRoots()) {
    const json = JSON.parse(readFileSync(resolve(root, "package.json"), "utf8"));
    for (const w of json.workspaces?.packages ?? json.workspaces ?? []) {
      const dirs = /[*?{]/u.test(w) ? [...new Glob(w + "/package.json").scanSync({ cwd: root, dot: true })].map(m => dirname(m)) : [w];
      for (const d of dirs) { const m = resolve(root, d, "package.json"); if (existsSync(m)) manifests.add(m); }
    }
  }
  return [...manifests];
}

export function listTargets(): Target[] {
  const result: Target[] = [];
  for (const manifest of workspaceManifests()) {
    const json = JSON.parse(readFileSync(manifest, "utf8"));
    const dir = dirname(manifest);
    const exp = json.exports;
    if (exp === undefined || exp === null) continue;
    const entries: [string, unknown][] = typeof exp === "string" || Array.isArray(exp) || Object.keys(exp).every(k => !k.startsWith(".")) ? [[".", exp]] : Object.entries(exp);
    for (const [subpath, value] of entries) {
      const flat: { conditions: string; target: string }[] = [];
      collect(value, [], flat);
      for (const { conditions, target } of flat) {
        const base = { manifest: manifest.slice(ROOT.length + 1), name: json.name ?? "", subpath, conditions, target };
        if (!target.startsWith("./") || target.includes("//") || target.split("/").includes("..")) { result.push({ ...base, file: "", exists: false, problem: "invalid target path" }); continue; }
        if (target.includes("*")) {
          const pattern = target.slice(2);
          const matches = [...new Glob(pattern).scanSync({ cwd: dir, dot: true, onlyFiles: true })];
          result.push({ ...base, file: pattern, exists: matches.length > 0, problem: matches.length ? undefined : "pattern matches no file" });
          continue;
        }
        const file = resolve(dir, target);
        result.push({ ...base, file: file.slice(ROOT.length + 1), exists: existsSync(file) });
      }
    }
  }
  return result;
}

export function markBuildOutputs(targets: Target[]): void {
  const absent = targets.filter(t => !t.exists && t.file && !t.target.includes("*"));
  if (!absent.length) return;
  const ignored = spawnSync("git", ["check-ignore", "--no-index", "-z", "--stdin"], { cwd: ROOT, input: absent.map(t => t.file).join("\0"), encoding: "utf8", maxBuffer: 1 << 26 }).stdout.split("\0");
  const set = new Set(ignored);
  for (const t of absent) if (set.has(t.file)) t.buildOutput = true;
}

if (import.meta.main) {
  const targets = listTargets();
  markBuildOutputs(targets);
  const buildOutputs = targets.filter(t => !t.exists && t.buildOutput);
  const missing = targets.filter(t => !t.exists && !t.buildOutput);
  const manifests = new Set(targets.map(t => t.manifest));
  console.log(`manifests with exports ${manifests.size}, targets ${targets.length}, missing ${missing.length}, gitignored build outputs absent ${buildOutputs.length}`);
  const byTarget = new Map<string, number>();
  for (const t of buildOutputs) byTarget.set(t.target, (byTarget.get(t.target) ?? 0) + 1);
  for (const [target, n] of byTarget) console.log(`build-output ${target} (${n} packages)`);
  if (process.argv.includes("--all")) for (const t of targets) console.log(`${t.exists ? "ok     " : "MISSING"} ${t.name}${t.subpath.slice(1)} [${t.conditions}] -> ${t.target}`);
  else for (const t of missing) console.log(`MISSING ${t.name}${t.subpath.slice(1)} [${t.conditions}] -> ${t.target}${t.problem ? ` (${t.problem})` : ""}  in ${t.manifest}`);
  if (process.argv.includes("--json")) { const out = resolve(import.meta.dir, "../🗑️generated/glue/missing.json"); mkdirSync(dirname(out), { recursive: true }); writeFileSync(out, JSON.stringify(missing, null, 1)); }
  process.exit(missing.length ? 1 : 0);
}
