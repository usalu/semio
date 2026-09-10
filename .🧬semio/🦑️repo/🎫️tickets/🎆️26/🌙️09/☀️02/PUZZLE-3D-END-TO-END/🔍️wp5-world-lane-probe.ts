/** 🚚️ Read-only probe for ticket 26/09/02 wave W-P5: does the world-3d surface's paged lane set
 * actually become instances at the host, and how long after the example click?
 *
 * Reads nothing but the DOM: `World3dHost` mirrors the scene it consumes onto its own root element
 * (`data-instances-json` / `data-meshes-json` / `data-status-json`, `🌐️World3dHost/🟦️.tsx`), so the
 * ASSEMBLED scene — spine plus every reattached lane — is directly observable without instrumenting
 * the build. Drives a SEPARATE headless chromium against an already-running dev target; it never
 * starts a server and never touches an interactive browser tab.
 *
 * `bun 🔍️wp5-world-lane-probe.ts [url] [--round-trip] [--out <dir>]`
 */
import { chromium } from "../../../../../../../node_modules/playwright/index.mjs";

const url = process.argv.find((argument) => argument.startsWith("http")) ?? "http://127.0.0.1:6013/";
const out = process.argv.includes("--out") ? process.argv[process.argv.indexOf("--out") + 1]! : ".";
const roundTrip = process.argv.includes("--round-trip");
const started = Date.now();
const log = (line: string) => console.log(`[${((Date.now() - started) / 1000).toFixed(1)}s] ${line}`);

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });
page.setDefaultTimeout(180_000);
await page.goto(url, { waitUntil: "commit", timeout: 180_000 });

/** 🚚️ One entry per mounted world viewport: how much assembled scene it is holding right now. */
const worlds = async () =>
  page.evaluate(() =>
    Array.from(document.querySelectorAll("[data-instances-json]")).map((element) => {
      const host = element as HTMLElement;
      let instances: { readonly meshId?: string }[] = [];
      let meshes: { readonly id?: string }[] = [];
      try { instances = JSON.parse(host.dataset.instancesJson ?? "[]"); } catch { instances = []; }
      try { meshes = JSON.parse(host.dataset.meshesJson ?? "[]"); } catch { meshes = []; }
      const declared = new Set(meshes.map((mesh) => mesh.id));
      return { bytes: (host.dataset.instancesJson ?? "").length, instances: instances.length, meshes: meshes.length, unresolved: instances.filter((instance) => !declared.has(instance.meshId)).length };
    }),
  );

for (let attempt = 0; attempt < 60; attempt += 1) {
  await page.waitForTimeout(3_000);
  const current = await worlds();
  if (current.length > 0) { log(`booted ${JSON.stringify(current)}`); break; }
}
const skip = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i });
if (await skip.count()) await skip.first().click().catch(() => {});
await page.waitForTimeout(4_000);
log(`start ${JSON.stringify(await worlds())}`);

const pick = async (label: RegExp) => {
  await page.locator('[role="combobox"]').first().click({ timeout: 15_000 });
  await page.locator('[role="option"]').filter({ hasText: label }).first().click({ timeout: 15_000 });
};

/** ⏱️ Polls once a second until every viewport satisfies `settled`, reporting how long it took. */
const settle = async (settled: (world: { readonly bytes: number }) => boolean, name: string, seconds = 90) => {
  const clicked = Date.now();
  for (let second = 0; second < seconds; second += 1) {
    await page.waitForTimeout(1_000);
    const current = await worlds();
    if (current.length > 0 && current.every(settled)) { log(`${name} settled after ${((Date.now() - clicked) / 1000).toFixed(1)}s: ${JSON.stringify(current)}`); return; }
  }
  log(`${name} DID NOT settle in ${seconds}s: ${JSON.stringify(await worlds())}`);
};

await pick(/nakagin/i);
log("→ nakagin");
await settle((world) => world.bytes > 10_000, "nakagin");
await page.screenshot({ path: `${out}/wp5-nakagin.png` });
if (roundTrip) {
  await pick(/concrete|forest/i);
  log("→ concrete forest");
  await settle((world) => world.bytes > 0 && world.bytes < 10_000, "forest-back");
  await page.screenshot({ path: `${out}/wp5-forest-back.png` });
  await pick(/nakagin/i);
  log("→ nakagin again");
  await settle((world) => world.bytes > 10_000, "nakagin-again");
  await page.screenshot({ path: `${out}/wp5-nakagin-again.png` });
}
await browser.close();
