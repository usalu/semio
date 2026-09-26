#!/usr/bin/env bun
/** 🧭️ U5 — C10's hard-load defect: a signed-in shell hard-loading `/spaces/<id>` must mount the Space app ONCE. Signs
 * `U5_EMAIL` in through the hub workspace on `/`, then hard-loads `/spaces/<id>` N times and samples every 200 ms for 30 s
 * whether the Space app is mounted (`s-space-create-artifact`), the route-admission notice and the path; a mount is an
 * absent→present transition, a loss a present→absent one. Every console line naming a closed/failed opening is kept.
 * Usage: bun u5-space-mount-probe.mjs <baseUrl> <spaceId> <tag> [attempts]   (env U5_EMAIL / U5_PASSWORD) */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl, spaceId, tag, attemptsText = "3"] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
const stamp = () => new Date().toISOString().slice(11, 23);
page.on("console", (m) => lines.push(`${stamp()} ${m.type()}: ${m.text()}`.slice(0, 400)));
page.on("pageerror", (e) => lines.push(`${stamp()} pageerror: ${String(e)}`.slice(0, 400)));
page.on("response", (response) => { if (response.status() >= 400) lines.push(`${stamp()} http ${response.status()} ${response.url().replace(/^https?:\/\/[^/]+/u, "").slice(0, 160)}`); });
await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") === "s", undefined, { timeout: 300_000 });
await page.locator("[data-semio-hub-sign-in]").first().click();
const workspace = page.locator("[data-semio-hub-workspace]");
await workspace.waitFor({ state: "visible", timeout: 30_000 });
for (const skip of await page.getByRole("button", { name: /^(Skip|Überspringen)$/u }).all()) await skip.click({ force: true }).catch(() => undefined);
await workspace.locator('input[type="email"]').fill(process.env.U5_EMAIL);
await workspace.locator('input[type="password"]').fill(process.env.U5_PASSWORD);
await workspace.locator('form:has(input[type="password"]) button[type="submit"]').first().click();
await page.waitForFunction(() => /Signed in as|Angemeldet als/u.test(document.querySelector("[data-semio-hub-workspace]")?.textContent ?? ""), undefined, { timeout: 120_000 });
await page.locator('[id="os.hub.signIn.cancel"]').click();
await page.waitForTimeout(3_000);
const report = { baseUrl, spaceId, attempts: [] };
for (let attempt = 1; attempt <= Number(attemptsText); attempt += 1) {
  const from = lines.length;
  const started = Date.now();
  await page.goto(new URL(`spaces/${spaceId}`, baseUrl).toString(), { waitUntil: "domcontentloaded" });
  const samples = [];
  let present = false;
  let mounts = 0;
  let losses = 0;
  let firstMountMs = null;
  const admissions = new Set();
  while (Date.now() - started < 30_000) {
    const now = await page.evaluate(() => ({
      space: document.querySelector('[data-ui-node-key="s-space-create-artifact"]') !== null,
      admission: document.querySelector("[data-semio-route-admission]")?.getAttribute("data-semio-route-admission") ?? null,
      path: location.pathname,
      windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).slice(0, 3).join(","),
    })).catch(() => null);
    if (now !== null) {
      if (now.admission !== null) admissions.add(now.admission);
      if (now.space && !present) { mounts += 1; firstMountMs ??= Date.now() - started; }
      if (!now.space && present) losses += 1;
      present = now.space;
      const key = JSON.stringify(now);
      if (samples.at(-1)?.key !== key) samples.push({ ms: Date.now() - started, key });
    }
    await page.waitForTimeout(200);
  }
  const faults = lines.slice(from).filter((line) => /space index opening failed|document closed|revoked|pageerror| error: /iu.test(line) && !/typed-operation slots/u.test(line));
  const row = { attempt, mounts, losses, firstMountMs, endsInSpace: present, admissions: [...admissions], faults: faults.slice(-8), samples };
  report.attempts.push(row);
  console.log(JSON.stringify({ attempt, mounts, losses, firstMountMs, endsInSpace: present, admissions: [...admissions], faults: faults.length }));
  for (const sample of samples) console.log(`  ${sample.ms} ${sample.key}`.slice(0, 260));
  await page.screenshot({ path: out(`u5-space-mount-${tag}-${attempt}.png`) });
}
report.errors = lines.filter((line) => / error: |pageerror:| http [45]\d\d /u.test(line));
writeFileSync(out(`u5-space-mount-${tag}.json`), JSON.stringify(report, null, 1));
console.log(`console errors: ${report.errors.length}`);
for (const line of report.errors.slice(0, 20)) console.log(line.slice(0, 300));
await browser.close();
