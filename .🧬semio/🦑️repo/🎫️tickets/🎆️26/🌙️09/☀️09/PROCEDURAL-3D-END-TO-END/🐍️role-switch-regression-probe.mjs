/** 🔁️ Reproduces the 2026-09-14 23:32 role-switch regression: the navbar `Viewer` control is inert on
 * a converged React procedural 3d playground (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane
 * role-switch-regression).
 *
 * Boots the example, waits for convergence, then drives, one at a time, with a full console capture and
 * a per-second snapshot of the role/mode groups, the mounted windows and the instance ids the console
 * has named: navbar click → Viewer, chord → Editor, chord → Viewer, mode step Edit ⇄ Generate.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-role/repro-1 bun 🐍️role-switch-regression-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6028/?plugin=generation3d&example=sphere-cut-with-torus";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-role/repro");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 240);
const settleWait = Number(process.env.SEMIO_PROBE_SETTLE_WAIT ?? 45);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const [viewportWidth, viewportHeight] = (process.env.SEMIO_PROBE_VIEWPORT ?? "1600x1000").split("x").map(Number);
const page = await browser.newPage({ viewport: { width: viewportWidth, height: viewportHeight } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1600)}`));

const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    const st = parse(el.getAttribute("data-status-json"));
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, phase: st?.phase ?? null, ratio: st?.progress?.ratio ?? null, nodeStatuses: st && !st.phase ? Object.values(st).map((v) => v?.status ?? "?") : undefined };
  });
  const button = (id) => {
    const el = document.getElementById(id);
    if (!el) return null;
    const box = el.getBoundingClientRect();
    return { pressed: el.getAttribute("aria-pressed"), disabled: el.hasAttribute("disabled") || el.getAttribute("aria-disabled") === "true", text: (el.textContent ?? "").trim().slice(0, 40), chord: el.getAttribute("aria-keyshortcuts"), box: { x: Math.round(box.x), y: Math.round(box.y), w: Math.round(box.width), h: Math.round(box.height) }, hit: (() => { const top = document.elementFromPoint(box.x + box.width / 2, box.y + box.height / 2); return top === null ? null : (top.closest("button")?.id ?? top.tagName + "." + (top.className || "").toString().slice(0, 40)); })() };
  };
  const group = document.getElementById("playground.navbar.roles");
  return {
    hosts,
    meshes: hosts.reduce((n, h) => n + h.meshes, 0),
    windows: [...document.querySelectorAll("[data-window-instance-id]")].map((e) => e.getAttribute("data-window-instance-id")),
    surfaces: [...document.querySelectorAll("[data-surface-id]")].map((e) => e.getAttribute("data-surface-id")),
    roleGroup: group === null ? null : { busy: group.getAttribute("aria-busy"), label: group.getAttribute("aria-label") },
    editor: button("playground.navbar.roles.editor"),
    viewer: button("playground.navbar.roles.viewer"),
    modeEdit: button("playground.navbar.modes.edit"),
    modeGenerate: button("playground.navbar.modes.generate"),
    modeButtons: [...document.querySelectorAll('[id^="playground.navbar.modes."]')].map((e) => ({ id: e.id, pressed: e.getAttribute("aria-pressed") })),
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
  await page.screenshot({ path: join(outDir, `${label}.png`) });
  steps.push({ label, t: Date.now() - t0, ok: predicate(last), ...last });
  console.log(`[DEBUG] ${label} t=${((Date.now() - t0) / 1000).toFixed(0)}s ok=${predicate(last)} roles=${JSON.stringify({ e: last.editor?.pressed, v: last.viewer?.pressed, busy: last.roleGroup?.busy, vdis: last.viewer?.disabled, vhit: last.viewer?.hit })} modes=${JSON.stringify(last.modeButtons)} windows=${JSON.stringify(last.windows)} meshes=${last.meshes} notices=${JSON.stringify(last.notices)}`);
  return last;
};

const instanceIds = () => {
  const found = new Set();
  for (const line of lines) for (const m of line.matchAll(/instance[ #:=]+(\d+)/gi)) found.add(m[1]);
  for (const line of lines) for (const m of line.matchAll(/\bprocedural#(\d+)/g)) found.add(m[1]);
  return [...found];
};

await page.goto(url, { waitUntil: "domcontentloaded" });
const boot = await waitFor("1-boot", bootWait, converged);
const dwell = Number(process.env.SEMIO_PROBE_DWELL ?? 0);
if (dwell > 0) {
  const before = lines.length;
  await page.waitForTimeout(dwell * 1000);
  const during = lines.slice(before);
  console.log(`[DEBUG] dwell ${dwell}s lines=${during.length} toolRunPace=${during.filter((l) => l.includes("toolRunPace")).length} flowEvalTick=${during.filter((l) => l.includes("flowEvalTick")).length} sample=${JSON.stringify(during.slice(-3))}`);
}
if (process.env.SEMIO_PROBE_INTERACT === "1") {
  const canvas = page.locator("canvas").first();
  if (await canvas.count()) {
    const box = await canvas.boundingBox();
    if (box) {
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.waitForTimeout(600);
      await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
      await page.waitForTimeout(1500);
      await page.keyboard.press("f");
      await page.waitForTimeout(1500);
    }
  }
  const combo = page.locator('[role="combobox"]').first();
  if (await combo.count()) {
    await combo.click({ timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(500);
    const option = page.locator('[role="option"]').nth(3);
    if (await option.count()) await option.click({ timeout: 5000 }).catch(() => {});
  }
  await waitFor("1b-after-interact", bootWait, converged);
}
const mark = lines.length;

console.log(`[DEBUG] viewer control: ${JSON.stringify(boot.viewer)}`);
console.log(`[DEBUG] editor control: ${JSON.stringify(boot.editor)}`);

await page.mouse.click(boot.viewer.box.x + boot.viewer.box.w / 2, boot.viewer.box.y + boot.viewer.box.h / 2);
const immediate = await snap();
steps.push({ label: "2-click-immediate", t: Date.now() - t0, ...immediate });
console.log(`[DEBUG] immediately after click: busy=${immediate.roleGroup?.busy} e=${immediate.editor?.pressed} v=${immediate.viewer?.pressed}`);
await waitFor("2-click-viewer", settleWait, inRole("viewer"));
const clickLines = lines.slice(mark);
writeFileSync(join(outDir, "click-console.txt"), clickLines.join("\n"));

await page.waitForTimeout(2000);
await page.keyboard.press("Meta+Alt+E");
await waitFor("3-chord-editor", settleWait, inRole("editor"));
await page.waitForTimeout(2000);
await page.keyboard.press("Meta+Alt+V");
await waitFor("4-chord-viewer", settleWait, inRole("viewer"));
await page.waitForTimeout(2000);
await page.keyboard.press("Meta+Alt+E");
await waitFor("5-chord-editor-back", settleWait, inRole("editor"));

const modeBefore = (await snap()).modeButtons;
await page.keyboard.press("Meta+Alt+ArrowRight");
await waitFor("6-mode-step", settleWait, (s) => JSON.stringify(s.modeButtons) !== JSON.stringify(modeBefore) && s.modeGenerate?.pressed === "true");
await page.keyboard.press("Meta+Alt+ArrowLeft");
await waitFor("7-mode-back", settleWait, (s) => JSON.stringify(s.modeButtons) === JSON.stringify(modeBefore) && converged(s));

const grep = (needle) => lines.filter((l) => l.includes(needle));
const summary = {
  instanceIds: instanceIds(),
  pageErrors: grep("pageerror"),
  draining: grep("surface-switch draining"),
  sealed: grep("shell.surface-switch.sealed-instance").length,
  noActor: grep("no actor for instance").length,
  revoked: grep("actor-activation.revoked").length,
  roleSwitchFailed: grep("role switch to"),
  switchTrace: grep("[DEBUG] surface-switch"),
  steps: steps.map((s) => ({ label: s.label, t: s.t, ok: s.ok ?? null, e: s.editor?.pressed, v: s.viewer?.pressed, busy: s.roleGroup?.busy, windows: s.windows, meshes: s.meshes, modes: s.modeButtons, notices: s.notices })),
};
writeFileSync(join(outDir, "steps.json"), JSON.stringify(steps, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "summary.json"), JSON.stringify(summary, null, 2));
console.log("[DEBUG] summary " + JSON.stringify({ ...summary, steps: undefined }, null, 2));
await browser.close();
