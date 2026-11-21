import { existsSync, readdirSync, readFileSync, statfsSync, statSync, unlinkSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir, "🗑️generated/native-guests/wasm32-wasip2/debug/build");
const candidates: readonly [string, string, number][] = [
  ["semio-s-artifact-stdio-binary-io-registry", "42c880aa235d2418", 218908550],
  ["semio-s-artifact-stdio-binary-io-registry", "bbe40abe5edf2c44", 219256096],
  ["semio-s-artifact-stdio-txt-tests", "7574007c966b8eda", 183205690],
  ["semio-s-artifact-stdio-txt-tests", "20989bd3ecfaabb4", 182999624],
  ["semio-s-artifact-stdio-pdf-model", "2f3b1d857c058187", 173917660],
];
const required = Number(process.argv[2]);
if (process.platform !== "darwin" || !Number.isSafeInteger(required) || required <= 0 || required > 3 * 1024 ** 3) {
  throw Error("This ticket's macOS reclaim requires a positive byte limit up to 3 GiB");
}
function available() {
  const disk = statfsSync(root);
  return disk.bavail * disk.bsize;
}

type BuildRecord = { pkg: string; variant: string; dependencies: string[] };
const graph = new Map<string, BuildRecord[]>();
const roots: string[] = [];
for (const pkg of readdirSync(root)) {
  for (const variant of readdirSync(join(root, pkg))) {
    const folder = join(root, pkg, variant, "fingerprint");
    if (!existsSync(folder)) continue;
    for (const name of readdirSync(folder).filter(name => name.endsWith(".json"))) {
      const keyFile = join(folder, name.slice(0, -5));
      if (!existsSync(keyFile)) continue;
      const hash = readFileSync(keyFile, "utf8").trim();
      if (!/^[0-9a-f]{16}$/u.test(hash)) continue;
      const key = Buffer.from(hash, "hex").readBigUInt64LE().toString();
      const dependencies = Array.from(readFileSync(join(folder, name), "utf8").matchAll(/\[\d+,"[^"\n]+",(?:true|false),(\d+)\]/gu), match => match[1]);
      graph.set(key, [...(graph.get(key) ?? []), { pkg, variant, dependencies }]);
      if (pkg.startsWith("semio-hub-") && pkg.endsWith("-component")) roots.push(key);
    }
  }
}
if (roots.length < 3) throw Error("Missing expected current component fingerprints");
const visited = new Set<string>();
const referenced = new Set<string>();
const queue = [...roots];
while (queue.length) {
  const key = queue.pop()!;
  if (visited.has(key)) continue;
  visited.add(key);
  for (const record of graph.get(key) ?? []) {
    referenced.add(join(record.pkg, record.variant));
    queue.push(...record.dependencies);
  }
}
for (const [pkg, variant, size] of candidates) {
  const before = available();
  if (before >= required) break;
  const location = join(pkg, variant);
  const stem = join(root, location, "out", `lib${pkg.replaceAll("-", "_")}-${variant}`);
  const archive = `${stem}.rlib`;
  if (referenced.has(location) || !existsSync(archive)) continue;
  const stat = statSync(archive);
  if (stat.size !== size || stat.nlink !== 1 || !existsSync(`${stem}.rmeta`)) continue;
  const opened = Bun.spawnSync(["/usr/sbin/lsof", "--", archive]);
  if (opened.exitCode !== 1 || opened.stdout.length || opened.stderr.length) continue;
  unlinkSync(archive);
  console.log(`[DEBUG] Retired ${pkg} ${variant} (${size} bytes). Available ${before} to ${available()}`);
}
console.log(`[DEBUG] Checked ${roots.length} component roots and ${visited.size} fingerprints. Available ${available()}`);
