/** 💥 The two cases the per-edit matrix cannot cover: an edit whose very NEXT request arrives with no
 * settle window at all (so only the request-time stat guard can have caught it, never the watcher), and a
 * burst of twenty host modules saved atomically at once (so a dropped or coalesced filesystem event is
 * guaranteed).
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts */
import { readFileSync, writeFileSync, renameSync, readdirSync } from "node:fs";
import { join } from "node:path";

const port = Number(process.argv[2] ?? 6029);
const repoRoot = "/Users/ueli/Documents/semio";
const MARKER = "[DEBUG] stale-guard";
const served = async (path) => {
  const response = await fetch(`http://127.0.0.1:${port}/@fs${path}`, { headers: { accept: "*/*" } });
  return { status: response.status, text: await response.text() };
};
const strip = (path) => {
  const current = readFileSync(path, "utf8");
  const cleaned = current.split("\n").filter((line) => !line.includes(MARKER)).join("\n");
  if (cleaned !== current) writeFileSync(path, cleaned);
};
const atomicAppend = (path, line) => {
  const temporary = `${path}.stale-guard-tmp`;
  writeFileSync(temporary, `${readFileSync(path, "utf8")}${line}\n`);
  renameSync(temporary, path);
};

const elementsRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements");
const burst = readdirSync(elementsRoot, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => join(elementsRoot, entry.name, "🟦️.tsx")).filter((path) => { try { readFileSync(path); return true; } catch { return false; } }).slice(0, 20);

const zeroDelayTarget = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx");
const baseline = await served(zeroDelayTarget);
atomicAppend(zeroDelayTarget, `console.info(${JSON.stringify(`${MARKER} zero-delay ${Date.now()}`)});`);
const immediate = await served(zeroDelayTarget);
console.log(`zero-delay atomic save -> next request: ${immediate.text !== baseline.text ? "fresh" : "STALE"} (status ${immediate.status})`);
strip(zeroDelayTarget);
await served(zeroDelayTarget);

console.log(`burst modules: ${burst.length}`);
const before = new Map();
for (const path of burst) before.set(path, (await served(path)).text);
for (const path of burst) atomicAppend(path, `console.info(${JSON.stringify(`${MARKER} burst ${Date.now()}`)});`);
const stale = [];
for (const path of burst) if ((await served(path)).text === before.get(path)) stale.push(path);
console.log(`burst of ${burst.length} atomic saves -> stale after the burst: ${stale.length}`);
for (const path of stale) console.log(`  STALE ${path.replace(`${repoRoot}/`, "")}`);
for (const path of burst) strip(path);
for (const path of burst) await served(path);
const residue = burst.filter((path) => readFileSync(path, "utf8").includes(MARKER));
console.log(`marker residue after cleanup: ${residue.length}`);
