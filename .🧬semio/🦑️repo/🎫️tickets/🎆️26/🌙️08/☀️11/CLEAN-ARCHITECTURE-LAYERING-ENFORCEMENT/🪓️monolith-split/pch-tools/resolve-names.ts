// 🎯️ Read-only: resolves package names through the launcher's own repository inventory (cargoRepositoryPackages).
import {readFileSync} from "node:fs";
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const lib = await import(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts"));
const packages = lib.cargoRepositoryPackages(ROOT);
const byManifest = new Map<string, any>(packages.map((p: any) => [p.manifest, p]));
const manifests = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
let known = 0; const unknown: string[] = []; const wrongWs: string[] = [];
for (const m of manifests) {
  const row = byManifest.get(m);
  if (!row) { unknown.push(m); continue; }
  known++;
}
console.log(JSON.stringify({inventory: packages.length, asked: manifests.length, known, unknown: unknown.length, sampleUnknown: unknown.slice(0, 3)}));
