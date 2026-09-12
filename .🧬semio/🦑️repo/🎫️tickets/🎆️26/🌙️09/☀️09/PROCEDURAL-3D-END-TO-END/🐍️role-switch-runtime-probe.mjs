/** 🔀️ Focused runtime probe for the TRANSACTIONAL surface switch (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * Two readings the long journey probe cannot isolate:
 *   1. MID-CHAIN — an example pick is dispatched and `Meta+Alt+V` is pressed while its typed operation
 *      and the `flow-extension-brep` round trip are still in flight. This is the shape that retired
 *      editor instance 1 underneath its own work (`🗑️generated/journey-3/console.txt`).
 *   2. POST-CONVERGENCE — the same chord on a quiet shell.
 * Both report the roles group, the mounted windows, the `surface-switch` trace and every
 * `no actor for instance` / pageerror the run produced.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=role-switch-runtime/switch-1 bun 🐍️role-switch-runtime-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "role-switch-runtime/switch");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 240);
const settleWait = Number(process.env.SEMIO_PROBE_SETTLE_WAIT ?? 60);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`));

const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    const st = parse(el.getAttribute("data-status-json"));
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, phase: st?.phase ?? null, ratio: st?.progress?.ratio ?? null, hint: st?.hint ?? null, nodeStatuses: st && !st.phase ? Object.values(st).map((v) => v?.status ?? "?") : undefined };
  });
  const pressed = (id) => document.getElementById(id)?.getAttribute("aria-pressed") ?? null;
  const group = document.getElementById("playground.navbar.roles");
  return {
    hosts,
    meshes: hosts.reduce((n, h) => n + h.meshes, 0),
    windows: [...document.querySelectorAll("[data-window-instance-id]")].map((e) => e.getAttribute("data-window-instance-id")),
    surfaces: [...document.querySelectorAll("[data-surface-id]")].map((e) => e.getAttribute("data-surface-id")),
    roles: { editor: pressed("playground.navbar.roles.editor"), viewer: pressed("playground.navbar.roles.viewer") },
    rolesBusy: group?.getAttribute("aria-busy") ?? null,
    modes: { edit: pressed("playground.navbar.modes.edit"), generate: pressed("playground.navbar.modes.generate") },
    chip: document.querySelector('[data-surface-role-chip]')?.textContent ?? null,
  };
});

const converged = (s) => {
  const main = s.hosts.find((h) => h.surfaceId === "procedural.play.main" || h.nodeStatuses);
  const preview = s.hosts.find((h) => h.surfaceId && h.surfaceId.includes("preview"));
  const nodes = main?.nodeStatuses ?? [];
  return Boolean(preview) && preview.phase === "idle" && preview.ratio === 1 && nodes.length > 0 && nodes.every((st) => st === "ok" || st === "error");
};

const waitFor = async (label, seconds, predicate) => {
  let last = null; let stable = 0;
  for (let i = 0; i < seconds; i++) {
    await page.waitForTimeout(1000);
    last = await snap();
    if (predicate(last)) { stable += 1; if (stable >= 3) break; } else stable = 0;
  }
  console.log(`[DEBUG] ${label} t=${((Date.now() - t0) / 1000).toFixed(0)}s ok=${predicate(last)} ${JSON.stringify({ roles: last.roles, busy: last.rolesBusy, modes: last.modes, windows: last.windows, hosts: last.hosts })}`);
  await page.screenshot({ path: join(outDir, `${label}.png`) });
  return last;
};

const steps = [];
const record = (label, snapshot) => { steps.push({ label, t: Date.now() - t0, ...snapshot }); };

await page.goto(url, { waitUntil: "domcontentloaded" });
record("boot", await waitFor("1-boot", bootWait, converged));

// 1️⃣ MID-CHAIN: fire an example pick and press the viewer chord while its operation is still running.
const combo = page.locator('[role="combobox"]').first();
if (await combo.count()) {
  await combo.click({ timeout: 5000 });
  await page.waitForTimeout(400);
  const option = page.locator('[role="option"]').nth(2);
  if (await option.count()) await option.click({ timeout: 5000 });
}
await page.waitForTimeout(150);
await page.keyboard.press("Meta+Alt+V");
record("mid-chain-immediate", await snap());
record("mid-chain", await waitFor("2-mid-chain-viewer", settleWait, (s) => s.roles.viewer === "true"));

// 2️⃣ POST-CONVERGENCE: back to editor on a quiet shell, then to viewer again.
await page.waitForTimeout(2000);
await page.keyboard.press("Meta+Alt+E");
record("post-editor", await waitFor("3-post-editor", settleWait, (s) => s.roles.editor === "true"));
await page.waitForTimeout(2000);
await page.keyboard.press("Meta+Alt+V");
record("post-viewer", await waitFor("4-post-viewer", settleWait, (s) => s.roles.viewer === "true"));

// 3️⃣ Generate mode (host side; the guest wasm may be stale — the report says so).
await page.keyboard.press("Meta+Alt+E");
await page.waitForTimeout(3000);
await page.keyboard.press("Meta+Alt+ArrowRight");
record("generate", await waitFor("5-generate", settleWait, (s) => s.hosts.length > 0));

const traces = lines.filter((l) => l.includes("surface-switch"));
const noActor = lines.filter((l) => l.includes("no actor for instance"));
const revoked = lines.filter((l) => l.includes("actor-activation.revoked"));
const pageErrors = lines.filter((l) => l.includes("pageerror"));
const drops = lines.filter((l) => l.includes("shell.surface-switch.sealed-instance"));
writeFileSync(join(outDir, "steps.json"), JSON.stringify(steps, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "summary.json"), JSON.stringify({ traces, drops, noActor: noActor.length, revoked: revoked.length, pageErrors, noActorSample: noActor.slice(0, 4) }, null, 2));
console.log("[DEBUG] traces:\n" + traces.join("\n"));
console.log(`[DEBUG] drops=${drops.length} noActor=${noActor.length} revoked=${revoked.length} pageErrors=${pageErrors.length}`);
await browser.close();
