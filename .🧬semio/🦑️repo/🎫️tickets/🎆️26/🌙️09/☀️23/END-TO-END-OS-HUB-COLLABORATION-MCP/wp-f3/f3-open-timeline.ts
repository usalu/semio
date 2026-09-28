/** ⏱️ F3 — where a hub document's open goes, per phase: signs user1 in (the local dev hub's test user) in a FRESH browser
 * profile, opens `/spaces/<spaceId>`, clicks the artifact row's Open and records, until the document is mounted, every
 * change of the shell's own open markers (plugin install band + progress, execution-target stage + progress, creation
 * status, status/alert texts, document window ids, painted body) with epoch ms — mounted = windows painted and no
 * execution-target status for 3 s (the target band appears only after the window) — plus the Resource Timing entries in the
 * window (name, start, duration, bytes). Then reloads and opens it again (warm: program in the device store).
 * `<artifactId>` = `create:<kindId>` creates a document of that kind from the Space app first (the creation saga opens it):
 * the first open is then create → mounted. usage: bun f3-open-timeline.ts <baseUrl> <spaceId> <artifactId|create:kindId> <tag> */
import { writeFileSync } from "node:fs";
import { loadavg } from "node:os";
import { chromium } from "playwright";
import { activate, boot, clickRowAction, dialog, openSessions, pageWindowedTables, signIn, submitDialog } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts";

const [baseUrl, spaceId, target, tag] = process.argv.slice(2) as [string, string, string, string];
let artifactId = target;
const SHELL_WINDOWS = ["framework.window.table", "s-home-main"];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const report: Record<string, unknown> = { tag, baseUrl, spaceId, artifactId, load: loadavg()[0] };
try {
  const [session] = await openSessions(browser, [baseUrl], [{ label: "user1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" }], "en-US");
  const page = session!.page;
  await page.addInitScript(() => {
    const events: { t: number; what: string }[] = [];
    Object.defineProperty(window, "__f3Open", { value: events });
    performance.setResourceTimingBufferSize(100_000);
    let last = "";
    const text = (element: Element): string => ((element as HTMLElement).innerText ?? element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 90);
    const scan = (): void => {
      const parts: string[] = [];
      for (const element of document.querySelectorAll("[data-semio-plugin-install]")) {
        const progress = element.querySelector("[data-semio-plugin-install-progress]");
        parts.push(`install ${element.getAttribute("data-plugin-install-ids") ?? ""} ${progress ? `${progress.getAttribute("value")}/${progress.getAttribute("max")}` : ""} ${text(element).slice(0, 50)}`);
      }
      for (const element of document.querySelectorAll("[data-semio-execution-target-status]")) {
        const progress = element.querySelector("progress");
        parts.push(`target ${element.getAttribute("data-semio-execution-target-stage") ?? ""} ${progress ? `${progress.getAttribute("value")}/${progress.getAttribute("max")}` : ""} ${text(element).slice(0, 60)}`);
      }
      for (const element of document.querySelectorAll("[data-semio-artifact-creation]")) parts.push(`creation ${element.getAttribute("data-semio-artifact-creation")}`);
      for (const element of document.querySelectorAll('[role="status"], [role="alert"]')) if (text(element)) parts.push(`status ${text(element).slice(0, 70)}`);
      const windows = [...document.querySelectorAll('[data-slot="window"]')].map((element) => element.id).filter((id) => id && !id.startsWith("s-home"));
      parts.push(`windows ${windows.join(",")}`);
      const painted = [...document.querySelectorAll('[data-slot="window-body"]')].filter((body) => body.querySelectorAll("*").length > 20).length;
      parts.push(`painted ${painted}`);
      const key = parts.join(" | ");
      if (key === last) return;
      last = key;
      events.push({ t: performance.timeOrigin + performance.now(), what: key });
    };
    const start = (): void => {
      scan();
      new MutationObserver(scan).observe(document.documentElement, { subtree: true, childList: true, attributes: true, characterData: true });
    };
    if (document.documentElement) start();
    else addEventListener("DOMContentLoaded", start);
  });
  await boot(session!);
  await signIn(session!);
  const openOnce = async (label: string): Promise<Record<string, unknown>> => {
    await page.goto(`${new URL(baseUrl).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
    await page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "attached", timeout: 120_000 });
    await page.waitForTimeout(3_000);
    const creating = artifactId.startsWith("create:");
    const rowKeys = (): Promise<string[]> => page.evaluate(() => [...document.querySelectorAll('[data-ui-node-key^="artifact:"]')].map((element) => element.getAttribute("data-ui-node-key") ?? ""));
    const before = creating ? await rowKeys() : [];
    if (creating) {
      await activate(page, "s-space-create-artifact");
      await dialog(page).waitFor({ state: "visible", timeout: 20_000 });
      await page.locator("#name").fill(`F3 open ${artifactId.slice(7)} ${Date.now() % 100000}`);
      await page.locator('[id="kindChoice"]').focus();
      await page.keyboard.press("Enter");
      await page.locator(`[role="option"][data-value*='"kindId":"${artifactId.slice(7)}"']`).first().click();
    } else {
      const row = page.locator(`[data-ui-node-key="artifact:${artifactId}"]`).first();
      if (!(await pageWindowedTables(page, async () => ((await row.count()) > 0 ? true : null)))) throw new Error(`no row artifact:${artifactId}`);
      await page.waitForTimeout(4_000);
    }
    const t0 = Date.now();
    const resourceCursor = await page.evaluate(() => performance.getEntriesByType("resource").length);
    const eventCursor = await page.evaluate(() => (window as unknown as { __f3Open: unknown[] }).__f3Open.length);
    if (creating) await submitDialog(page);
    else await clickRowAction(page, "artifact", artifactId, /^(open|öffnen)\b/iu);
    let mountedAt: number | null = null;
    let readySince: number | null = null;
    for (const deadline = Date.now() + 600_000; Date.now() < deadline && mountedAt === null; await page.waitForTimeout(100)) {
      const state = await page.evaluate((shell) => ({
        windows: [...document.querySelectorAll('[data-slot="window"]')].map((element) => element.id).filter((id) => id && !shell.includes(id) && !id.startsWith("s-home")),
        painted: [...document.querySelectorAll('[data-slot="window-body"]')].some((body) => body.querySelectorAll("*").length > 20),
        target: document.querySelectorAll("[data-semio-execution-target-status]").length,
      }), SHELL_WINDOWS);
      const ready = state.windows.length > 0 && state.painted && state.target === 0;
      readySince = ready ? (readySince ?? Date.now()) : null;
      if (readySince !== null && Date.now() - readySince >= 3_000) mountedAt = readySince;
    }
    await page.waitForTimeout(1_500);
    if (creating) artifactId = ((await rowKeys()).find((key) => !before.includes(key)) ?? "").replace(/^artifact:/u, "");
    const collected = await page.evaluate(([resourceStart, eventStart]) => ({
      events: (window as unknown as { __f3Open: { t: number; what: string }[] }).__f3Open.slice(eventStart as number),
      resources: (performance.getEntriesByType("resource") as PerformanceResourceTiming[]).slice(resourceStart as number).map((entry) => ({ name: decodeURIComponent(entry.name).replace(/^https?:\/\/[^/]+/u, "").slice(0, 140), start: Math.round(performance.timeOrigin + entry.startTime), ms: Math.round(entry.duration), transfer: entry.transferSize, decoded: entry.decodedBodySize, initiator: entry.initiatorType })),
    }), [resourceCursor, eventCursor]);
    return {
      label,
      mountedMs: mountedAt === null ? null : mountedAt - t0,
      events: collected.events.map((event) => ({ ms: Math.round(event.t - t0), what: event.what })),
      resources: collected.resources.map((resource) => ({ ...resource, start: resource.start - t0 })).sort((left, right) => right.ms - left.ms).slice(0, 40),
      resourceTotals: { count: collected.resources.length, transferMB: +(collected.resources.reduce((sum, resource) => sum + resource.transfer, 0) / 2 ** 20).toFixed(2), decodedMB: +(collected.resources.reduce((sum, resource) => sum + resource.decoded, 0) / 2 ** 20).toFixed(2) },
    };
  };
  report.first = await openOnce("first");
  report.artifactId = artifactId;
  report.warm = await openOnce("warm");
  report.consoleTail = session!.lines.slice(-40);
} finally {
  writeFileSync(`/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated/open-timeline-${tag}.json`, JSON.stringify(report, null, 1));
  await browser.close();
}
console.log(JSON.stringify({ first: (report.first as { mountedMs?: number } | undefined)?.mountedMs, warm: (report.warm as { mountedMs?: number } | undefined)?.mountedMs }));
