/** 🧾️ Wave B53 hop tap for Nakagin `exportFixture` on the puzzle3d React serve: boots the shell, switches
 * the example to Nakagin and PROVES the switch (navbar label + object census), then taps every hop of the
 * export lane — the Actions-pane row's real DOM/hit state, the click's own outcome, the guest `Scene` stage,
 * the `download-media-export` effect vs the segmented marker, `takeSegmentedDownloadChunk` traffic, the host
 * drain, the blob-and-anchor sink and the browser `download` event with the saved bytes.
 * Ticket 26/09/02/PUZZLE-3D-END-TO-END.
 *
 * Run: `bun 🔍️b53-export-hops.ts [--port=<n>] [--example=<regex>] [--settle=<seconds>]`.
 */
import { chromium } from "playwright";
import { mkdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const exampleArg = process.argv.find((a) => a.startsWith("--example="))?.slice(10) ?? "nakagin";
const settleSeconds = Number(process.argv.find((a) => a.startsWith("--settle="))?.slice(9) ?? "25");
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const t0 = Date.now();
const lines: string[] = [];
const log = (message: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${message}`;
  lines.push(row);
  console.log(row);
};
const flush = () => writeFileSync(join(OUT, `b53-export-hops-${stamp}.md`), `${lines.join("\n")}\n`);

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
const console_: string[] = [];
page.on("console", (message) => console_.push(`${message.type()}: ${message.text().slice(0, 400)}`));
page.on("pageerror", (error) => console_.push(`pageerror: ${String(error).slice(0, 400)}`));
page.on("response", (response) => {
  if (response.status() >= 400) console_.push(`http ${response.status()}: ${decodeURIComponent(response.url()).slice(0, 200)}`);
});
const since = (mark: number) => console_.slice(mark);

await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`, { waitUntil: "domcontentloaded", timeout: 60000 });
const boot = async () =>
  page.evaluate(() => ({
    windows: document.querySelectorAll('[data-slot="window"]').length,
    canvases: document.querySelectorAll("canvas").length,
  }));
for (let attempt = 0; attempt < 120; attempt++) {
  const state = await boot();
  if (state.windows > 0 && state.canvases > 0) break;
  await page.waitForTimeout(1000);
}
log(`boot ${JSON.stringify(await boot())}`);

/** 🎨️ The navbar example picker's own label plus the inspector census, so a blind switch is visible. */
const documentCensus = async () =>
  page.evaluate(() => {
    const picker = document.getElementById("playground.navbar.fixture");
    const inspection = document.getElementById("puzzle3d-play-inspector");
    const objects = /Objects\s+(\d+)/.exec((inspection?.innerText ?? "").replace(/\s+/g, " "));
    const instances = Array.from(document.querySelectorAll<HTMLElement>("[data-instances-json]")).map((host) => (host.getAttribute("data-instances-json") ?? "").length);
    return { example: (picker?.innerText ?? "").replace(/\s+/g, " ").trim(), objects: objects ? Number(objects[1]) : null, treeItems: document.querySelectorAll('[data-slot="tree-item"], [role="treeitem"]').length, instanceJsonBytes: instances };
  });
log(`census boot ${JSON.stringify(await documentCensus())}`);

const wanted = new RegExp(exampleArg, "i");
const picker = page.locator('[id="playground.navbar.fixture"]');
const native = picker.locator("select").or(page.locator(`select[id="playground.navbar.fixture"]`)).first();
if (await native.count()) {
  const label = (await native.locator("option").allTextContents()).find((text) => wanted.test(text)) ?? "";
  log(`example native option=${label}`);
  await native.selectOption({ label }).catch((error) => log(`example native select failed ${String(error).slice(0, 160)}`));
} else {
  const combo = (await picker.count()) ? picker.first() : page.locator('[role="combobox"]').first();
  const options = page.locator('[role="option"]');
  for (let attempt = 0; attempt < 3 && (await options.count()) === 0; attempt++) {
    await combo.click({ force: true, timeout: 3000 }).catch(() => {});
    for (let poll = 0; poll < 32 && (await options.count()) === 0; poll++) await page.waitForTimeout(250);
  }
  log(`example options=${JSON.stringify(await options.allTextContents()).slice(0, 300)}`);
  const target = options.filter({ hasText: wanted }).first();
  await ((await target.count()) ? target : options.nth(1)).click({ timeout: 4000 }).catch((error) => log(`example option click failed ${String(error).slice(0, 160)}`));
}
await page.waitForTimeout(settleSeconds * 1000);
const census = await documentCensus();
log(`census after switch ${JSON.stringify(census)}`);
log(`switch proven=${wanted.test(census.example)} objects=${census.objects}`);

/** 🪝️ Page-side taps for the sink's own primitives — every object URL minted and every programmatic
 * anchor click, with the `download` attribute each carried. */
await page.evaluate(() => {
  const w = window as unknown as { __b53?: { urls: number[]; anchors: string[]; saves: number } };
  if (w.__b53) return;
  w.__b53 = { urls: [], anchors: [], saves: 0 };
  const createObjectURL = URL.createObjectURL.bind(URL);
  URL.createObjectURL = (object: Blob | MediaSource) => {
    w.__b53?.urls.push(object instanceof Blob ? object.size : -1);
    return createObjectURL(object);
  };
  const click = HTMLAnchorElement.prototype.click;
  HTMLAnchorElement.prototype.click = function patched(this: HTMLAnchorElement) {
    w.__b53?.anchors.push(`${this.getAttribute("download") ?? "-"}|${(this.getAttribute("href") ?? "").slice(0, 12)}`);
    return click.call(this);
  };
});

const PANE = "framework.window.puzzle3dMainPerspective.engagement";
const folded = async () => page.evaluate((id: string) => document.getElementById(id)?.getAttribute("data-folded") ?? "absent", PANE);
log(`pane folded=${await folded()}`);
if ((await folded()) === "true") {
  await page.locator(`[id="${PANE}.toggle"]`).first().click({ timeout: 6000 }).then(() => log("pane toggle ok")).catch((error) => log(`pane toggle failed ${String(error).split("\n")[0].slice(0, 160)}`));
  for (let poll = 0; poll < 60 && (await folded()) === "true"; poll++) await page.waitForTimeout(500);
  log(`pane folded after toggle=${await folded()}`);
}

/** 🔎️ Every `action.*` row in the document with the window it belongs to — unsliced, so a row beyond the
 * 24 the battery printed is visible, and its own geometry/hit state for the rows this wave presses. */
const rowShape = async (actionId: string) =>
  page.evaluate((id: string) => {
    const row = document.getElementById(id);
    if (!row) return { present: false };
    const rect = row.getBoundingClientRect();
    const style = getComputedStyle(row);
    const hit = document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2) as HTMLElement | null;
    const chain: string[] = [];
    for (let node: HTMLElement | null = row; node && chain.length < 12; node = node.parentElement) {
      const nodeStyle = getComputedStyle(node);
      const nodeRect = node.getBoundingClientRect();
      chain.push(`${node.tagName}#${node.id || "-"}[${node.getAttribute("data-slot") ?? "-"}]@${Math.round(nodeRect.x)},${Math.round(nodeRect.y)},${Math.round(nodeRect.width)},${Math.round(nodeRect.height)}:disp=${nodeStyle.display}/vis=${nodeStyle.visibility}/pe=${nodeStyle.pointerEvents}/ov=${nodeStyle.overflowY}/op=${nodeStyle.opacity}/folded=${node.getAttribute("data-folded") ?? "-"}/state=${node.getAttribute("data-state") ?? "-"}`);
    }
    const hitChain: string[] = [];
    for (let node = hit; node && hitChain.length < 6; node = node.parentElement) hitChain.push(`${node.tagName}#${node.id || "-"}[${node.getAttribute("data-slot") ?? "-"}]`);
    return {
      present: true,
      rect: [Math.round(rect.x), Math.round(rect.y), Math.round(rect.width), Math.round(rect.height)],
      offsetParent: row.offsetParent ? `${row.offsetParent.tagName}#${row.offsetParent.id || "-"}` : null,
      inViewport: rect.y >= 0 && rect.y <= window.innerHeight && rect.height > 0,
      text: row.innerText.replace(/\s+/g, " ").trim().slice(0, 40),
      display: style.display,
      visibility: style.visibility,
      pointerEvents: style.pointerEvents,
      disabledAttr: row.getAttribute("aria-disabled"),
      ownsHit: Boolean(hit && (hit === row || row.contains(hit))),
      hitChain,
      chain,
    };
  }, actionId);

const allRows = await page.evaluate(() =>
  Array.from(document.querySelectorAll<HTMLElement>('[id^="action."]')).map((row) => {
    const rect = row.getBoundingClientRect();
    const pane = row.closest('[data-slot="window-action-pane"]')?.parentElement?.closest("[id]")?.id ?? "-";
    return `${row.id}|${row.innerText.replace(/\s+/g, " ").trim().slice(0, 22)}|${Math.round(rect.width)}x${Math.round(rect.height)}@${Math.round(rect.y)}|pane=${pane}`;
  }),
);
log(`action rows count=${allRows.length}`);
for (const row of allRows) log(`  row ${row}`);
log(`exportFixture shape ${JSON.stringify(await rowShape("action.exportFixture"))}`);

const mark = console_.length;
const row = page.locator('[id="action.exportFixture"]');
log(`exportFixture locator count=${await row.count()}`);
await row.first().scrollIntoViewIfNeeded({ timeout: 5000 }).then(() => log("exportFixture scrollIntoView ok")).catch((error) => log(`exportFixture scrollIntoView failed ${String(error).split("\n")[0].slice(0, 200)}`));
log(`exportFixture shape after scroll ${JSON.stringify(await rowShape("action.exportFixture"))}`);
const download = page.waitForEvent("download", { timeout: 60000 }).catch(() => null);
const strict = await row
  .first()
  .click({ timeout: 8000 })
  .then(() => "ok")
  .catch((error) => `failed ${String(error).split("\n").slice(0, 4).join(" / ").slice(0, 400)}`);
log(`exportFixture strict click ${strict}`);
if (strict !== "ok") {
  const forced = await row
    .first()
    .click({ force: true, timeout: 8000 })
    .then(() => "ok")
    .catch((error) => `failed ${String(error).split("\n").slice(0, 4).join(" / ").slice(0, 400)}`);
  log(`exportFixture forced click ${forced}`);
}
const settled = await download;
log(`download=${settled ? settled.suggestedFilename() : "none"}`);
if (settled) {
  const dest = join(OUT, `b53-export-${stamp}.json`);
  await settled.saveAs(dest);
  log(`download saved ${dest} bytes=${statSync(dest).size}`);
}
log(`page taps ${JSON.stringify(await page.evaluate(() => (window as unknown as { __b53?: unknown }).__b53 ?? null))}`);
const tail = since(mark);
log(`console lines after click=${tail.length}`);
for (const line of tail.slice(-120)) log(`  | ${line}`);
flush();
await browser.close();
log("done");
flush();
