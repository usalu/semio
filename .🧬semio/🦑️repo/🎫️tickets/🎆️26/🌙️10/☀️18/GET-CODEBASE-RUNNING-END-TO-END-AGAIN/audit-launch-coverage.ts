import { readFileSync } from "node:fs";
import { dirname, basename, join } from "node:path";
import { spawnSync } from "node:child_process";
const ROOT = process.cwd();
const tracked = spawnSync("git", ["ls-files", "-z", "--", "*Cargo.toml"], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\0").filter(Boolean);
const untracked = spawnSync("git", ["ls-files", "-z", "--others", "--exclude-standard", "--", "*Cargo.toml"], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\0").filter(Boolean);
const launchText = readFileSync(join(ROOT, ".vscode/launch.json"), "utf8") + readFileSync(join(ROOT, ".vscode/🧩️launch.seed.jsonc"), "utf8");
const cargoFiles = [...new Set([...tracked, ...untracked])].filter((f) => !f.startsWith(".🧬semio/") && !f.includes("node_modules/") && basename(f) === "Cargo.toml");
const rows: any[] = [];
for (const f of cargoFiles) {
  let t: any;
  try { t = Bun.TOML.parse(readFileSync(join(ROOT, f), "utf8")); } catch { continue; }
  if (!t.package) continue;
  const name = t.package.name as string;
  const dir = dirname(f);
  const proj = join(ROOT, dir, "📋️project.json");
  let projName: string | null = null;
  try { projName = JSON.parse(readFileSync(proj, "utf8")).name ?? null; } catch { }
  const ids = [name, projName].filter(Boolean) as string[];
  const hit = ids.some((n) => launchText.includes(n + ":") || launchText.includes("--projects=" + n) || launchText.includes(n + " "));
  const isTest = /🧪️tests|laws|unit|companion|-tests?-/.test(f) || /-(laws|unit-tests?|tests?)$/.test(name);
  rows.push({ f, name, projName, hasRoute: hit, isTest, untracked: untracked.includes(f) });
}
const noRoute = rows.filter((r) => !r.hasRoute);
const summary = {
  cargoPackages: rows.length,
  withLaunchRoute: rows.length - noRoute.length,
  withoutLaunchRoute: noRoute.length,
  withoutLaunchRouteButTestLike: noRoute.filter((r) => r.isTest).length,
  untrackedPackages: rows.filter((r) => r.untracked).length,
  untrackedWithoutRoute: noRoute.filter((r) => r.untracked).map((r) => ({ name: r.name, projName: r.projName, f: r.f })),
  sampleWithoutRoute: noRoute.filter((r) => !r.isTest).slice(0, 25).map((r) => r.name),
};
console.log(JSON.stringify(summary, null, 1));
