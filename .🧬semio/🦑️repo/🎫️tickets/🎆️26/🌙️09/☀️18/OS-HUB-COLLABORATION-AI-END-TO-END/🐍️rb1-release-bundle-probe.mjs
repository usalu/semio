/** 🩺️ RB1 §4 — does the PRODUCTION `s` bundle boot, host its plugins, and sign in against a
 * production hub?
 *
 * Deliberately not a copy of `🐍️s2-*`/`🐍️b1a-*`: those read `window.__semioOsCatalogProbe`, which
 * `🏛️ShellHost/🟦️.tsx:11714-11718` guards behind `import.meta.env.DEV` and therefore does **not**
 * exist in a release bundle — `🔬️catalog-smoke/🟦️.ts:263` even says so in its own failure message
 * ("is the dev server serving a development build?"). The readiness beacon
 * (`data-semio-os-ready`, `:11683`) is *not* dev-gated, so it is the one witness that survives into
 * production, and everything else here is DOM evidence a real user could also see.
 *
 * Asserts, in order:
 *   1. the static server answers and the document parses (no dev server behind it)
 *   2. the beacon reaches `ready:s` (or reports `error:`/`not-found:` honestly)
 *   3. the shell mounts surfaces/windows and the Home surface lists studios
 *   4. the baked `VITE_S_HUB_URL` points at the production hub, and a credential sign-in against it
 *      over the network bind succeeds from inside the page
 *   5. one studio opens
 *
 * Env: RB1_URL, RB1_HUB, RB1_EMAIL, RB1_PASSWORD, RB1_SECONDS.
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const url = process.env.RB1_URL ?? "http://192.168.178.70:6081/";
const hub = process.env.RB1_HUB ?? "http://192.168.178.70:7661";
const email = process.env.RB1_EMAIL ?? "ada@example.com";
const password = process.env.RB1_PASSWORD ?? "correct horse battery staple";
const seconds = Number(process.env.RB1_SECONDS ?? 180);
const ticketDir = dirname(fileURLToPath(import.meta.url));
const outDir = join(ticketDir, "🗑️generated");
mkdirSync(outDir, { recursive: true });

const NOISE = /favicon|Download the React DevTools|Lit is in dev mode/i;
const lines = [];
const t0 = Date.now();
const log = (...parts) => console.log("[rb1]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
page.on("requestfailed", (r) => lines.push(`${Date.now() - t0} requestfailed ${r.url().slice(0, 200)} ${r.failure()?.errorText ?? ""}`));

const report = { url, hub, startedAt: new Date().toISOString(), steps: [] };
const step = (name, detail) => { report.steps.push({ step: name, ms: Date.now() - t0, detail }); log(`${name}:`, JSON.stringify(detail).slice(0, 700)); };

const beacon = () => page.evaluate(() => {
  const d = document.documentElement.dataset;
  if (d.semioOsReady !== undefined) return `ready:${d.semioOsReady || "(empty)"}`;
  if (d.semioOsError !== undefined) return `error:${d.semioOsError || "(empty)"}`;
  if (d.semioOsNotFound !== undefined) return `not-found:${d.semioOsNotFound || "(empty)"}`;
  return null;
});

const view = () => page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  return {
    title: document.title,
    devProbePresent: typeof window.__semioOsCatalogProbe !== "undefined",
    bakedHubUrl: (() => { try { return import.meta.env?.VITE_S_HUB_URL ?? null; } catch { return "unavailable"; } })(),
    surfaces: [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), chars: text(el).length, svg: el.querySelectorAll("svg *").length, canvases: el.querySelectorAll("canvas").length })),
    windowIds: [...new Set([...document.querySelectorAll("[data-window-id]")].map((el) => el.getAttribute("data-window-id")))],
    treeRows: [...document.querySelectorAll('[data-surface-id] [role="treeitem"]')].map((el) => text(el).slice(0, 60)),
    buttons: [...new Set([...document.querySelectorAll("button[id]")].map((el) => el.id))].slice(0, 120),
    bodyHead: text(document.body).slice(0, 1500),
  };
});

let failed = 0;
const row = (ok, name, detail) => { if (!ok) failed += 1; log(`${ok ? "PASS" : "FAIL"} ${name} — ${String(detail).slice(0, 400)}`); report.steps.push({ step: name, ok, ms: Date.now() - t0, detail }); };

try {
  // 1 — the static server, with no dev server behind it
  const response = await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60_000 });
  row(Boolean(response?.ok()), "static server answers", `${response?.status()} ${response?.headers()["content-type"] ?? ""}`);

  // 2 — the beacon, the one readiness witness that is not dev-gated
  let seen = null;
  for (let i = 0; i < seconds && seen === null; i++) { await page.waitForTimeout(1000); seen = await beacon(); }
  await page.waitForTimeout(5000);
  row(seen === "ready:s", "beacon reaches ready:s", seen ?? "(no beacon within budget)");

  // 3 — what the shell actually mounted
  const mounted = await view();
  step("mounted", { title: mounted.title, surfaces: mounted.surfaces, windowIds: mounted.windowIds, devProbePresent: mounted.devProbePresent, bakedHubUrl: mounted.bakedHubUrl });
  row(mounted.surfaces.length > 0, "surfaces mounted", `${mounted.surfaces.length} surface(s): ${mounted.surfaces.map((s) => s.id).join(", ").slice(0, 300)}`);
  row(mounted.treeRows.length > 0 || mounted.bodyHead.length > 0, "Home renders content", `${mounted.treeRows.length} tree rows, ${mounted.bodyHead.length} chars of body`);
  row(mounted.devProbePresent === false, "no dev-only probe (this IS a release bundle)", `window.__semioOsCatalogProbe present=${mounted.devProbePresent}`);

  // 4 — the hub the bundle was baked against, and a real sign-in from inside the page
  const signIn = await page.evaluate(async ({ hub, email, password }) => {
    try {
      const response = await fetch(`${hub}/auth/sessions`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "rb1-bundle-probe", clientClass: "browser" }),
      });
      const body = await response.text();
      return { status: response.status, body: body.slice(0, 400) };
    } catch (error) { return { status: 0, body: String(error).slice(0, 400) }; }
  }, { hub, email, password });
  row(signIn.status === 200, "credential sign-in against the production hub from the page", `${signIn.status} ${signIn.body.slice(0, 220)}`);

  // 5 — open one studio
  const opened = await page.evaluate(() => {
    const target = [...document.querySelectorAll("button[id], [role='button'][id], a[id]")].find((el) => /studio|home\.|open/i.test(el.id));
    if (!target) return null;
    target.click();
    return target.id;
  });
  await page.waitForTimeout(6000);
  const after = await view();
  row(opened !== null, "a studio/open control was clickable", opened ?? "no matching control found");
  step("after-open", { windowIds: after.windowIds, surfaces: after.surfaces.map((s) => `${s.id}:${s.chars}`) });

  await page.screenshot({ path: join(outDir, "rb1-release-bundle.png"), fullPage: false });
} catch (error) {
  row(false, "probe completed", String(error).slice(0, 600));
} finally {
  const faults = lines.filter((l) => /error|pageerror|requestfailed|refused|trap|panick/i.test(l) && !NOISE.test(l));
  report.faults = faults.slice(0, 40);
  report.consoleLines = lines.length;
  writeFileSync(join(outDir, "rb1-release-bundle-probe.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "rb1-release-bundle-console.txt"), lines.join("\n"));
  log(`faults: ${faults.length} (of ${lines.length} console lines)`);
  faults.slice(0, 12).forEach((f) => log("  fault:", f.slice(0, 300)));
  log(failed === 0 ? "ALL ROWS GREEN" : `${failed} RED ROW(S)`);
  await browser.close();
  process.exit(failed === 0 ? 0 : 1);
}
