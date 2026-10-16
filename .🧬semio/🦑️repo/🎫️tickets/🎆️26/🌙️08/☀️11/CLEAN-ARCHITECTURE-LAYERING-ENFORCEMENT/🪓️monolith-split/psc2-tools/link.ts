/** 🔗️ Links every package a plan exports from into the root `node_modules`, as `bun install` does for referenced workspace packages. Usage: bun link.ts <plan.json> [--write] */
import { readFileSync, existsSync, lstatSync, mkdirSync, symlinkSync, readlinkSync, realpathSync } from "node:fs";
import { dirname, join } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const plan = JSON.parse(readFileSync(process.argv[2], "utf8"));
const write = process.argv.includes("--write");
const pkgs = new Map<string, string>();
for (const t of Object.values<any>(plan.targetMap)) pkgs.set(t.pkg, t.pkgDir);
let made = 0, ok = 0;
for (const [name, dir] of pkgs) {
  const link = join(ROOT, "node_modules", name);
  const want = join(ROOT, dir);
  let state = "missing";
  try { lstatSync(link); state = realpathSync(link).normalize("NFC") === want.normalize("NFC") ? "ok" : "WRONG " + realpathSync(link); } catch {}
  if (state === "ok") { ok++; continue; }
  console.log(write ? "LINK" : "WOULD LINK", name, state);
  if (state === "missing" && write) { mkdirSync(dirname(link), { recursive: true }); symlinkSync(want, link, "dir"); made++; }
}
console.log({ packages: pkgs.size, ok, made });
