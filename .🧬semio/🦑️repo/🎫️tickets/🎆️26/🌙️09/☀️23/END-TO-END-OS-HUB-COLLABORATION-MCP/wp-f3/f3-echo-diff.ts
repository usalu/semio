#!/usr/bin/env bun
/** 🔬️ F3 — what an editor's scene echo changes while typing: opens a text-editor program, hooks
 * `EditorSession.syncFromScenePack` (every pack the host syncs) and `renderFrame`, types, then decodes consecutive packs with
 * the page's own pack codec and lists the top-level fields that differ. usage: bun f3-echo-diff.ts <baseUrl> <pluginId> <appId> <windowSuffix> [keys] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { awaitBeacon, dismissIntroduction } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const [baseUrl, pluginId, appId, windowSuffix, keysArg] = process.argv.slice(2) as [string, string, string, string, string?];
const keys = Number(keysArg ?? 12);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
try {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
  page.setDefaultNavigationTimeout(300_000);
  await page.addInitScript(() => performance.setResourceTimingBufferSize(100_000));
  await page.goto(baseUrl, { waitUntil: "commit" });
  console.log("beacon", await awaitBeacon(page, Date.now() + 300_000));
  await dismissIntroduction(page);
  await page.keyboard.press("Escape").catch(() => undefined);
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)![1]!);
  await page.waitForTimeout(1_500);
  for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    if ((await item.count()) > 0) { await item.click({ force: true }); break; }
  }
  const target = page.locator(`[id$="${windowSuffix}"] [data-slot="window-body"] .semio-text-editor-host canvas`).first();
  await target.waitFor({ state: "visible", timeout: 90_000 });
  const box = (await target.boundingBox())!;
  await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.4);
  await page.waitForTimeout(800);
  await page.keyboard.press("End");
  await page.evaluate(async () => {
    const state = { armed: false, log: [] as string[], packs: [] as { t: number; bytes: number[] }[] };
    Object.defineProperty(window, "__f3Echo", { value: state });
    addEventListener("keydown", (event) => { if (state.armed) state.log.push(`${Math.round(performance.now())} key ${event.key}`); }, { capture: true });
    const urls = [...new Set(performance.getEntriesByType("resource").map((entry) => entry.name).filter((name) => /\.js(\?|$)/u.test(name) && /\/pkg\/|bindings\//u.test(decodeURIComponent(name))))];
    for (const url of urls) {
      let module: Record<string, unknown>;
      try { module = await import(url); } catch { continue; }
      const session = module.EditorSession as { prototype: Record<string, (...args: unknown[]) => unknown> } | undefined;
      if (!session || (session.prototype as { __f3?: boolean }).__f3) continue;
      (session.prototype as { __f3?: boolean }).__f3 = true;
      const sync = session.prototype.syncFromScenePack!;
      session.prototype.syncFromScenePack = function (this: unknown, ...args: unknown[]) {
        if (state.armed) { const bytes = args[0] as Uint8Array; state.packs.push({ t: Math.round(performance.now()), bytes: [...bytes] }); state.log.push(`${Math.round(performance.now())} sync ${bytes.length}B`); }
        return sync.apply(this, args);
      };
      const render = session.prototype.renderFrame!;
      session.prototype.renderFrame = function (this: unknown, ...args: unknown[]) {
        if (state.armed) state.log.push(`${Math.round(performance.now())} paint`);
        return render.apply(this, args);
      };
    }
  });
  await page.waitForTimeout(1_500);
  await page.evaluate(() => { (window as unknown as { __f3Echo: { armed: boolean } }).__f3Echo.armed = true; });
  for (let index = 0; index < keys; index++) {
    await page.keyboard.press(index % 2 === 0 ? "a" : "Backspace");
    await page.waitForTimeout(180);
  }
  await page.waitForTimeout(1_500);
  const result = await page.evaluate(async () => {
    const state = (window as unknown as { __f3Echo: { armed: boolean; log: string[]; packs: { t: number; bytes: number[] }[] } }).__f3Echo;
    state.armed = false;
    const osUrl = performance.getEntriesByType("resource").map((entry) => entry.name).find((name) => decodeURIComponent(name).includes("/💻️os/🟦️.ts"));
    const os = osUrl ? ((await import(osUrl)) as { decodePackValue: (bytes: Uint8Array) => unknown }) : null;
    const decoded = state.packs.map((pack) => (os ? os.decodePackValue(new Uint8Array(pack.bytes)) : null)) as (Record<string, unknown> | null)[];
    const diffs: string[] = [];
    for (let index = 1; index < decoded.length; index++) {
      const before = decoded[index - 1], after = decoded[index];
      if (!before || !after) continue;
      const keysUnion = [...new Set([...Object.keys(before), ...Object.keys(after)])];
      const changed = keysUnion.filter((key) => JSON.stringify(before[key]) !== JSON.stringify(after[key]));
      diffs.push(`${state.packs[index]!.t}: ${changed.map((key) => `${key}=${JSON.stringify(after[key])?.slice(0, 80)} (was ${JSON.stringify(before[key])?.slice(0, 80)})`).join("; ")}`);
    }
    return { log: state.log, diffs, osUrl: osUrl ?? null, fields: decoded[0] ? Object.keys(decoded[0]) : [] };
  });
  writeFileSync(`/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated/f3-echo-diff-${pluginId}.json`, JSON.stringify(result, null, 1));
  console.log(result.log.join("\n"));
  console.log("fields", result.fields.join(","));
  console.log(result.diffs.join("\n"));
} finally {
  await browser.close();
}
