/** ♻️ Hot-swap board-remount probe — boots the generation3d React playground, waits for the Flow
 * window's node-graph board to carry its 7 nodes, then triggers a REAL dev hot-swap by re-publishing
 * the activation receipt the dev server's `semioActivationVitePlugin` watch turns into a `built`
 * availability event, and asserts the Flow window comes BACK: a second `node-graph host mount`, a live
 * session and the same 7 nodes on the NEW app instance — within a bounded budget, 0 page errors.
 *
 * 🪪️ The receipt is what `activate-generation3d-react-dev` publishes last. The dev server SENDS a
 * `built` event for a changed `artifactSha256`; the shell ROUTES it to a hot swap only when its
 * `rebuiltAt` is newer than the load it already has — so a republish moves both, which is exactly what a
 * peer's restage does, for a full swap and no build. The receipt is shared with every other
 * generation3d serve, so each write is a read-modify-write of whatever is on disk at that instant: a
 * peer's restage landing mid-run is preserved except for the plugin being swapped, whose digest returns
 * to its original value after an even number of swaps.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6023/?plugin=generation3d SEMIO_PROBE_OUT=hot-swap bun 🐍️hot-swap-remount-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, readFileSync, renameSync, rmSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "hot-swap");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 120);
const recoverWait = Number(process.env.SEMIO_PROBE_RECOVER_WAIT ?? 150);
const swaps = Number(process.env.SEMIO_PROBE_SWAPS ?? 2);
const receiptPath = process.env.SEMIO_PROBE_RECEIPT ?? "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/generation3d/activation/🔣️receipt.json";
const swapPluginId = process.env.SEMIO_PROBE_SWAP_PLUGIN ?? "procedural";
const swapAll = process.env.SEMIO_PROBE_SWAP_ALL === "1";
const settleMs = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 20000);
const surfaceId = "window:procedural-main";
mkdirSync(outDir, { recursive: true });

/** 🔁️ Flips one plugin's activation digest on disk and answers what it wrote, atomically and over
 * whatever a peer's restage has left there. `null` means the digest was already what was asked for. */
const republishDigest = (pluginId) => {
  const receipt = JSON.parse(readFileSync(receiptPath, "utf8"));
  const rows_ = swapAll ? receipt.plugins : receipt.plugins.filter((entry) => entry.pluginId === pluginId);
  if (rows_.length === 0) throw new Error(`activation receipt names no plugin ${pluginId}`);
  // 🕰️ The digest is what makes the dev server SEND a `built` event; `rebuiltAt` is what makes the shell
  // ROUTE it to a hot swap (`routePluginAvailability` drops an event that is not newer than the load).
  // Both, therefore, on every republish.
  const at = Date.now();
  for (const entry of rows_) {
    entry.artifactSha256 = flipDigest(entry.artifactSha256);
    entry.rebuiltAt = Math.max(at, Number(entry.rebuiltAt) + 1);
  }
  const staging = `${receiptPath}.probe-${process.pid}.stage`;
  try {
    writeFileSync(staging, `${JSON.stringify(receipt)}\n`, { flag: "wx" });
    renameSync(staging, receiptPath);
  } finally {
    rmSync(staging, { force: true });
  }
  return rows_.map((entry) => entry.pluginId);
};
/** 🎲️ A different, still-valid 64-hex digest, and its OWN inverse — rotating the leading nibble by 8
 * means an even number of swaps leaves the shared receipt byte-identical to what the last real restage
 * published, whatever a peer landed in between. */
const flipDigest = (digest) => `${"0123456789abcdef"[("0123456789abcdef".indexOf(digest[0]) + 8) % 16]}${digest.slice(1)}`;

const lines = [];
const pageErrors = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", (e) => { const text = `${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`; lines.push(text); pageErrors.push(text); });

/** 🔎️ What the Flow board is worth right now: the surface host in the DOM, its probe-registry node
 * ids, and the shell's own instance id for the window that owns it. */
const boardState = () => page.evaluate((id) => {
  const host = document.querySelector(`[data-surface-id="${id}"]`);
  const probe = window.__semioFlowGraphProbe?.[id];
  let nodeIds = null;
  try { nodeIds = probe?.nodeIds ? probe.nodeIds() : null; } catch { nodeIds = null; }
  const window_ = host?.closest("[data-window-instance-id]");
  const canvases = host ? [...host.querySelectorAll("canvas")].map((c) => ({ w: c.width, h: c.height })) : [];
  return {
    hostPresent: !!host,
    probePresent: !!probe,
    nodeIds: Array.isArray(nodeIds) ? nodeIds : null,
    nodeCount: Array.isArray(nodeIds) ? nodeIds.length : 0,
    windowInstanceId: window_?.getAttribute("data-window-instance-id") ?? null,
    canvases,
    windows: [...document.querySelectorAll("[data-window-instance-id]")].map((e) => e.getAttribute("data-window-instance-id")),
    surfaces: [...document.querySelectorAll("[data-surface-id]")].map((e) => e.getAttribute("data-surface-id")),
  };
}, surfaceId);

/** 🔢️ How many `node-graph host mount` lines this console has recorded so far — the only honest way to
 * tell "the board is still the one from before the swap" from "the board came BACK". */
const mountCount = () => lines.filter((l) => l.includes("node-graph host mount")).length;
const swapSeen = () => lines.some((l) => l.includes("[DEBUG] hot-swap procedural"));

const waitForBoard = async (label, seconds, mountsBefore = -1) => {
  let last = null;
  const start = Date.now();
  for (let i = 0; i < seconds; i++) {
    await page.waitForTimeout(1000);
    last = await boardState();
    if (last.hostPresent && last.nodeCount >= 7 && mountCount() > mountsBefore) break;
  }
  const row = { label, seconds: (Date.now() - start) / 1000, mounts: mountCount(), swapSeen: swapSeen(), ...last };
  console.log(`[DEBUG] ${label}: host=${row.hostPresent} probe=${row.probePresent} nodes=${row.nodeCount} mounts=${row.mounts} swapSeen=${row.swapSeen} in ${row.seconds.toFixed(0)}s surfaces=${JSON.stringify(row.surfaces)}`);
  await page.screenshot({ path: join(outDir, `${label}.png`) });
  return row;
};

await page.goto(url, { waitUntil: "domcontentloaded" });
const boot = await waitForBoard("1-boot", bootWait);

const rows = [boot];
for (let swap = 1; swap <= swaps; swap += 1) {
  const mountsBefore = mountCount();
  const swapsSeenBefore = lines.filter((l) => l.includes(`[DEBUG] hot-swap ${swapPluginId}`)).length;
  const republished = republishDigest(swapPluginId);
  console.log(`[DEBUG] republished ${republished.length} activation digest(s)`);
  lines.push(`${Date.now() - t0} probe activation-digest-republished count=${republished.length}`);
  const swapArrived = () => lines.filter((l) => l.includes(`[DEBUG] hot-swap ${swapPluginId}`)).length > swapsSeenBefore;
  for (let i = 0; i < 120 && !swapArrived(); i += 1) await page.waitForTimeout(500);
  lines.push(`${Date.now() - t0} probe hot-swap-observed=${swapArrived()}`);
  rows.push(await waitForBoard(`${swap + 1}-after-swap-${swap}`, recoverWait, mountsBefore));
  await page.waitForTimeout(settleMs);
}

const swapLines = lines.filter((l) => /hot-swap|node-graph host (mount|unmount)|node-graph session|actor-activation|no channel|no actor|SET_SESSION|establishPrimary|surface ready/.test(l));
const verdict = {
  url,
  boot: { host: boot.hostPresent, nodes: boot.nodeCount },
  swaps: rows.slice(1).map((r) => ({ label: r.label, host: r.hostPresent, probe: r.probePresent, nodes: r.nodeCount, mounts: r.mounts, swapSeen: r.swapSeen, seconds: r.seconds, surfaces: r.surfaces })),
  recovered: rows.slice(1).every((r, index) => r.swapSeen && r.hostPresent && r.nodeCount >= 7 && r.mounts > rows[index].mounts),
  pageErrors: pageErrors.length,
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "swap-lines.txt"), swapLines.join("\n"));
writeFileSync(join(outDir, "rows.json"), JSON.stringify(rows, null, 2));
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log(`[DEBUG] VERDICT ${JSON.stringify(verdict)}`);
await browser.close();
process.exit(verdict.recovered && verdict.pageErrors === 0 ? 0 : 1);
