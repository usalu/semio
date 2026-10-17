// ♻️ Reverts an accidentally applied carve of one subset package: created files are deleted, modified files return to their index version, the shared artifact manifest and lock lose only the added members and packages, owner rows are removed.
import {execFileSync} from "node:child_process";
import {existsSync, readFileSync, readdirSync, rmSync, rmdirSync, writeFileSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
const root = "/Users/ueli/Documents/semio";
const [writesFile, facadeName, ...newNames] = process.argv.slice(2) as [string, string, ...string[]];
const artifact = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio";
const rows = readFileSync(writesFile!, "utf8").split("\n").filter(Boolean).map(line => line.split("\t") as [string, string]);
const shared = new Set([`${artifact}/Cargo.toml`, `${artifact}/Cargo.lock`]);
const under = (path: string) => { const absolute = resolve(root, path); if (!absolute.startsWith(`${root}/`)) throw new Error(`outside the repository: ${path}`); return absolute; };
const index = (path: string): string => execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "show", `:${path}`], {cwd: root, encoding: "utf8", maxBuffer: 1 << 28});
const touchedDirs = new Set<string>();
for (const [action, path] of rows) {
  if (shared.has(path)) continue;
  const absolute = under(path);
  if (action === "+") { rmSync(absolute, {force: true}); touchedDirs.add(dirname(absolute)); }
  else writeFileSync(absolute, index(path));
}
for (const start of touchedDirs) for (let dir = start; dir.startsWith(`${root}/`) && existsSync(dir) && readdirSync(dir).length === 0; dir = dirname(dir)) rmdirSync(dir);
const members = rows.filter(([action, path]) => action === "+" && path.endsWith("/📦️packages/🦀️rust/Cargo.toml")).map(([, path]) => dirname(path).slice(artifact.length + 1));
const manifestPath = under(`${artifact}/Cargo.toml`);
let manifest = readFileSync(manifestPath, "utf8");
for (const member of members) { const lines = manifest.split("\n").filter(line => line.trim() !== `"${member}",`); if (lines.length !== manifest.split("\n").length - 1) throw new Error(`member not found once: ${member}`); manifest = lines.join("\n"); }
writeFileSync(manifestPath, manifest);
const lockPath = under(`${artifact}/Cargo.lock`);
const lock = readFileSync(lockPath, "utf8");
const blocks = lock.split(/\n(?=\[\[package\]\]\n)/u);
const original = index(`${artifact}/Cargo.lock`).split(/\n(?=\[\[package\]\]\n)/u);
const nameOf = (block: string) => /^name = "([^"]+)"/mu.exec(block)?.[1] ?? "";
const facadeBlock = original.find(block => nameOf(block) === facadeName);
if (!facadeBlock) throw new Error("facade block not in the index lock");
const dropped = new Set(newNames);
const kept = blocks.filter(block => !dropped.has(nameOf(block))).map(block => nameOf(block) === facadeName ? facadeBlock : block);
const dependents = kept.filter(block => [...dropped].some(name => block.includes(`"${name}"`)));
if (dependents.length) console.log("lock blocks still naming a dropped crate:", dependents.map(nameOf).join(", "));
writeFileSync(lockPath, kept.join("\n"));
const ownersPath = under("📋️build-owners.json");
const ownersText = readFileSync(ownersPath, "utf8");
const owners = JSON.parse(ownersText) as {owners: {name: string}[]};
if (JSON.stringify(owners, null, 2) + "\n" !== ownersText) throw new Error("owner file does not round-trip; not rewriting it");
const before = owners.owners.length;
owners.owners = owners.owners.filter(row => !dropped.has(row.name));
writeFileSync(ownersPath, JSON.stringify(owners, null, 2) + "\n");
console.log("owner rows removed:", before - owners.owners.length);
console.log(JSON.stringify({members, restored: rows.filter(r => r[0] === "~").length, deleted: rows.filter(r => r[0] === "+").length}));
