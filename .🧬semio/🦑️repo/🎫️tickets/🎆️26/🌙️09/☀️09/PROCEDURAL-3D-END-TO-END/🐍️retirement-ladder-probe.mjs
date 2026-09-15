/** 🪜️ Reads the retained-UI close ladder of a converged React procedural 3d playground: how many
 * TURNS one whole-instance retirement spends, how many ladder STEPS those turns cover, and whether
 * `plugin-ui.owner-close-budget-exhausted` ever fires (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane
 * retirement-ladder-credit).
 *
 * Boots the hexagonal mushroom column, waits for convergence, then drives EIGHT role/example switches —
 * every one of which retires a live instance — and reports the host's own
 * `[DEBUG] plugin-ui close ladder` census plus every `owner-close-*` fault the console carries.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6022/?plugin=generation3d \
 *   SEMIO_PROBE_OUT=react-retire/ladder bun 🐍️retirement-ladder-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6022/?plugin=generation3d&example=hexagonal-mushroom-column";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-retire/ladder");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 240);
const settleWait = Number(process.env.SEMIO_PROBE_SETTLE_WAIT ?? 60);
const switches = Number(process.env.SEMIO_PROBE_SWITCHES ?? 8);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1600)}`));

const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    const st = parse(el.getAttribute("data-status-json"));
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, phase: st?.phase ?? null, nodeStatuses: st && !st.phase ? Object.values(st).map((v) => v?.status ?? "?") : undefined };
  });
  const button = (id) => {
    const el = document.getElementById(id);
    if (!el) return null;
    const box = el.getBoundingClientRect();
    return { pressed: el.getAttribute("aria-pressed"), text: (el.textContent ?? "").trim().slice(0, 40), box: { x: Math.round(box.x), y: Math.round(box.y), w: Math.round(box.width), h: Math.round(box.height) } };
  };
  return {
    hosts,
    meshes: hosts.reduce((n, h) => n + h.meshes, 0),
    windows: [...document.querySelectorAll("[data-window-instance-id]")].map((e) => e.getAttribute("data-window-instance-id")),
    editor: button("playground.navbar.roles.editor"),
    viewer: button("playground.navbar.roles.viewer"),
    notices: [...document.querySelectorAll('[role="status"], [role="alert"]')].map((e) => (e.textContent ?? "").trim().slice(0, 120)).filter(Boolean),
  };
});

const converged = (s) => {
  const main = s.hosts.find((h) => h.nodeStatuses);
  const preview = s.hosts.find((h) => h.surfaceId && h.surfaceId.includes("preview"));
  const nodes = main?.nodeStatuses ?? [];
  return Boolean(preview) && preview.phase === "idle" && s.meshes > 0 && (nodes.length === 0 || nodes.every((st) => st === "ok" || st === "error"));
};
const inRole = (role) => (s) => s[role]?.pressed === "true" && converged(s);

const steps = [];
const waitFor = async (label, seconds, predicate) => {
  let last = null; let stable = 0;
  for (let i = 0; i < seconds; i++) {
    await page.waitForTimeout(1000);
    last = await snap();
    if (predicate(last)) { stable += 1; if (stable >= 2) break; } else stable = 0;
  }
  steps.push({ label, t: Date.now() - t0, ok: predicate(last), windows: last?.windows, meshes: last?.meshes, notices: last?.notices });
  console.log(`[DEBUG] ${label} t=${((Date.now() - t0) / 1000).toFixed(0)}s ok=${predicate(last)} windows=${JSON.stringify(last?.windows)} meshes=${last?.meshes}`);
  return last;
};

const ladderLines = () => lines.filter((l) => l.includes("plugin-ui close ladder"));
const ladderCensus = () => ladderLines().map((line) => {
  const read = (key) => Number((line.match(new RegExp(`${key}=(\\d+)`)) ?? [])[1] ?? -1);
  return { instance: read("instance"), surfaces: read("surfaces"), turns: read("turns"), steps: read("steps"), ceiling: read("ceiling") };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
await waitFor("1-boot", bootWait, converged);

for (let round = 1; round <= switches; round += 1) {
  const toViewer = round % 2 === 1;
  await page.keyboard.press(toViewer ? "Meta+Alt+V" : "Meta+Alt+E");
  await waitFor(`${round + 1}-switch-${toViewer ? "viewer" : "editor"}`, settleWait, inRole(toViewer ? "viewer" : "editor"));
  await page.waitForTimeout(1500);
}

const census = ladderCensus();
const grep = (needle) => lines.filter((l) => l.includes(needle));
const summary = {
  switches,
  closes: census.length,
  census,
  maxTurns: census.reduce((n, c) => Math.max(n, c.turns), 0),
  maxSteps: census.reduce((n, c) => Math.max(n, c.steps), 0),
  turnsOverSurfaces: census.map((c) => ({ surfaces: c.surfaces, turns: c.turns, ceiling: c.ceiling })),
  budgetExhausted: grep("owner-close-budget-exhausted"),
  closeStalled: grep("owner-close-stalled"),
  closeBlocked: grep("owner-close-blocked").concat(grep("owner-close-rejected")),
  lifecycleExhausted: grep("lifecycle-close-budget-exhausted"),
  pageErrors: grep("pageerror"),
  roleSwitchFailed: grep("role switch to"),
  steps: steps.map((s) => ({ label: s.label, t: s.t, ok: s.ok })),
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "summary.json"), JSON.stringify(summary, null, 2));
console.log("[DEBUG] summary " + JSON.stringify({ ...summary, census: summary.census.slice(0, 16) }, null, 2));
await browser.close();

const red = summary.budgetExhausted.length + summary.closeStalled.length + summary.closeBlocked.length + summary.lifecycleExhausted.length + summary.pageErrors.length + summary.roleSwitchFailed.length;
const unconverged = steps.filter((s) => !s.ok).map((s) => s.label);
console.log(`[DEBUG] verdict red=${red} unconverged=${JSON.stringify(unconverged)} closes=${summary.closes} maxTurns=${summary.maxTurns}`);
if (process.env.SEMIO_PROBE_GATE === "1" && (red > 0 || unconverged.length > 0 || summary.closes === 0)) process.exit(1);
