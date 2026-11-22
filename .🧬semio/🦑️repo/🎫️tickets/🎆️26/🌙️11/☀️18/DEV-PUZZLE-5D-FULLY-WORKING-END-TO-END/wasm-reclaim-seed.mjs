import { lstatSync, readdirSync, unlinkSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ticket = dirname(fileURLToPath(import.meta.url));
const workspace = resolve(ticket, "../../../../../../..");
const privateBuild = join(ticket, "🗑️generated/w/b");
const originalBuild = join(workspace, ".cache/b/r/m/u/l/pfbba8d");
const cutoff = lstatSync(join(privateBuild, "../profile-selectors"), { bigint: true }).mtimeNs;
const apply = process.argv[2] === "--apply";
const files = [];
const seen = new Map();
for (const name of ["prototype", "wasm32-unknown-unknown"]) {
  const entries = readdirSync(join(privateBuild, name), { recursive: true, withFileTypes: true });
  for (const entry of entries) {
    if (!entry.isFile()) continue;
    const path = join(entry.parentPath, entry.name);
    const current = lstatSync(path, { bigint: true });
    if (current.ctimeNs >= cutoff || current.mtimeNs >= cutoff) continue;
    const original = lstatSync(join(originalBuild, path.slice(privateBuild.length + 1)), { bigint: true, throwIfNoEntry: false });
    if (!original?.isFile() || original.size !== current.size || original.mtimeNs !== current.mtimeNs) continue;
    files.push(path);
    const key = current.ino;
    const item = seen.get(key) ?? { bytes: current.blocks * 512n, links: current.nlink, count: 0n };
    item.count++;
    seen.set(key, item);
  }
}
const reclaimable = [...seen.values()].reduce((sum, item) => sum + (item.links === item.count ? item.bytes : 0n), 0n);
console.log(JSON.stringify({ files: files.length, bytes: Number(reclaimable), apply }));
if (apply) {
  for (const path of files) unlinkSync(path);
}
