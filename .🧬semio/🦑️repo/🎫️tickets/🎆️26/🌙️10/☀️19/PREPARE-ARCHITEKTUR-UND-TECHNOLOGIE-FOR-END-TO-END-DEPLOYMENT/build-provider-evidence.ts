import { createHash } from "node:crypto";
import { createReadStream, lstatSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const generated = resolve(process.env.SEMIO_TICKET_DIR!, "🗑️generated"), ticket = resolve(process.env.SEMIO_TICKET_DIR!);
const compiler = join(generated, "msvc-stage-2765774819"), sdk = join(generated, "k-EJEv0T"), flat = join(generated, "sdk-flat-1164172136");
const compilerPlan = JSON.parse(readFileSync(join(generated, "msvc-vsix-inventory-5.json"), "utf8")), sdkPlan = JSON.parse(readFileSync(join(generated, "sdk-file-plan.json"), "utf8"));
const digest = (data: Uint8Array) => createHash("sha256").update(data).digest("hex");
const hashFile = async (path: string): Promise<string> => {
  const before = lstatSync(path, { bigint: true });
  if (!before.isFile() || before.isSymbolicLink()) throw Error(`Nonregular file: ${path}`);
  const h = createHash("sha256"); for await (const bytes of createReadStream(path)) h.update(bytes);
  const after = lstatSync(path, { bigint: true });
  if (before.dev !== after.dev || before.ino !== after.ino || before.size !== after.size || before.mtimeNs !== after.mtimeNs) throw Error(`File changed: ${path}`);
  return h.digest("hex");
};
const walk = (root: string): string[] => {
  const pending = [root], result: string[] = [];
  while (pending.length) { const directory = pending.pop()!; for (const item of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, item.name);
    if (item.isDirectory()) pending.push(path); else if (item.isFile() && !item.isSymbolicLink()) result.push(relative(root, path).replaceAll("\\", "/")); else throw Error("Nonregular tree entry");
  } }
  return result.sort();
};
const rows: { path: string; size: number; sha256: string; source: unknown }[] = [], names = new Set<string>();
for (const row of compilerPlan) {
  if (!Number.isInteger(row.archive) || row.archive < 0 || row.archive > 4 || !row.name.startsWith("VC/") || names.has(row.name.toLowerCase())) throw Error("Invalid compiler inventory");
  names.add(row.name.toLowerCase()); const file = join(compiler, ...row.name.split("/"));
  if (lstatSync(file).size !== row.size) throw Error("Compiler size mismatch");
  rows.push({ path: row.name, size: row.size, sha256: await hashFile(file), source: { vsix: row.archive, member: `Contents/${row.name}` } });
}
if (compilerPlan.length !== 494 || walk(compiler).length !== 494) throw Error("Compiler tree count mismatch");
for (const row of sdkPlan) {
  if (!row.path.startsWith("Windows Kits/10/") || !/^[a-z0-9_]+$/iu.test(row.id) || !/^[a-f0-9]{32}\.cab$/u.test(row.cabinet)) throw Error("Invalid SDK inventory");
  const path = `sdk/${row.path.slice("Windows Kits/10/".length)}`, key = path.toLowerCase(); if (names.has(key)) throw Error("Duplicate output path"); names.add(key);
  const current = join(sdk, ...path.slice(4).split("/")), original = join(flat, row.id);
  if (lstatSync(current).size !== row.size || lstatSync(original).size !== row.size) throw Error("SDK size mismatch");
  const sha256 = await hashFile(current); if (sha256 !== await hashFile(original)) throw Error(`SDK parity failed: ${row.id}`);
  rows.push({ path, size: row.size, sha256, source: { msi: row.index, cabinet: row.cabinet, member: row.id } });
  if (rows.length % 250 === 0) console.log(`Hashed ${rows.length} provider files`);
}
if (sdkPlan.length !== 4453 || walk(sdk).length !== 4454 || walk(flat).length !== 4453) throw Error("SDK tree count mismatch");
rows.sort((a, b) => a.path < b.path ? -1 : a.path > b.path ? 1 : 0);
const recipes = Object.fromEntries(["extract-msvc-vsix.go", "inspect-sdk-files.go", "extract-sdk-flat.go", "msi-paths.ts", "plan-sdk-files.ts", "materialize-sdk.ts"].map(name => [name, digest(readFileSync(join(ticket, name)))]));
const evidence = { version: 1, catalogSha256: "a8e854fa9f3fbaeac27d7304e297f1660d7137401389deb57cf7983f33c16fef", compilerVersion: "14.50.35717", sdkVersion: "10.0.26100.0", recipes, files: rows };
const bytes = Buffer.from(JSON.stringify(evidence)); writeFileSync(join(generated, "provider-evidence.json"), bytes, { flag: "wx" });
console.log(JSON.stringify({ files: rows.length, bytes: bytes.length, sha256: digest(bytes) }));
