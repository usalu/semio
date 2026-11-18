// R135 real run, cell "dependencies": the real capacity gate (lock directory, grant ledger on disk, sample injected) is asked for `repo dependencies sync --all` on an injected 8 GB / 4 core host with half of the memory free; prints what is admitted and reserved. env: R135_TOTAL (8192), R135_CORES (4), R135_FREE_SHARE (0.5)
import { mkdtempSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio", B = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const { withBuildCapacityV1, BUILD_HOST_CAPACITY_POLICY_V1 } = await import(B + "/🌡️host-capacity/🟦️.ts");
const { readHostCapacityPolicyV1 } = await import(root + "/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts");
const buildPolicy = (await import(B + "/🔣️policy.json", { with: { type: "json" } })).default;
const total = Number(process.env.R135_TOTAL ?? 8192), cores = Number(process.env.R135_CORES ?? 4), share = Number(process.env.R135_FREE_SHARE ?? 0.5);
const fast = readHostCapacityPolicyV1({ ...buildPolicy.hostCapacity, wait: { ...buildPolicy.hostCapacity.wait, pollMilliseconds: 50, progressMilliseconds: 200, sampleMaximumAgeMilliseconds: 0 } });
const host = () => ({ version: 1, atMs: Date.now(), platform: "darwin", cores, totalMegabytes: total, availableMegabytes: Math.floor(total * share), pressure: "normal", swapUsedMegabytes: 0, swapOutMegabytesPerSecond: 0, load: 1, openFiles: 8000, openFilesMaximum: 245760, attachedImages: 2 });
const directory = join(mkdtempSync(join(tmpdir(), "r135-gate-")), "gate"), lines: string[] = [];
const started = Date.now();
const result: any = {};
await withBuildCapacityV1(["dependencies", "sync", "--all"], { LANG: "en_US.UTF-8" }, async (granted: any) => {
  const ledger = readdirSync(directory, { recursive: true }).filter((name: string) => /grant|json/u.test(name)).length;
  result.admitted = granted.handle !== undefined; result.reservedMegabytes = granted.handle?.reservedMegabytes; result.jobs = granted.handle?.jobs; result.ledgerFiles = ledger;
}, { directory, policy: fast, strip: (args: readonly string[]) => args.filter(a => !a.startsWith("--tool") && !a.startsWith("--workers")), sample: async () => ({ host: host(), swappedOutMegabytes: 0 }), write: (line: string) => { lines.push(line); } });
console.log("DEPENDENCIES", JSON.stringify({ host: { total, cores, free: Math.floor(total * share) }, policy: { fixed: BUILD_HOST_CAPACITY_POLICY_V1.operations.find((row: any) => row.name === "dependencies") }, ...result, waitedMilliseconds: Date.now() - started, lines: lines.slice(0, 4) }));
rmSync(directory, { recursive: true, force: true });
process.exit(0);
