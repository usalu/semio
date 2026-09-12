/** 🔎️ Wave B44 scratch: what the example picker IS, what `setActiveExample` carries, and what the
 * document holds after the switch — the facts the latency probe needed and could only infer.
 * Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import { chromium } from "playwright";

const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const t0 = Date.now();
const log = (m: string) => console.log(`[${((Date.now() - t0) / 1000).toFixed(1)}s] ${m}`);
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const tape: string[] = [];
page.on("console", (msg) => {
  const text = msg.text().slice(0, 300);
  if (/setActiveExample|navbar example|b44\.world|fixture|example/i.test(text)) tape.push(`+${Date.now() - t0}ms ${text}`);
});
await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });

const state = () =>
  page
    .evaluate(() => {
      const picker = document.getElementById("playground.navbar.fixture");
      const native = (picker?.querySelector("select") ?? document.querySelector('select[id="playground.navbar.fixture"]')) as HTMLSelectElement | null;
      const world = document.querySelector("#puzzle3d-main-perspective [data-instances-json]") as HTMLElement | null;
      const raw = world?.getAttribute("data-instances-json") ?? "[]";
      let count = -1;
      try {
        count = JSON.parse(raw).length;
      } catch {
        count = -1;
      }
      const treeRows = Array.from(document.querySelectorAll('[data-slot="tree-item"], [role="treeitem"]')).length;
      return {
        pickerTag: picker?.tagName ?? null,
        pickerText: (picker as HTMLElement | null)?.innerText?.replace(/\n/g, " ").slice(0, 80) ?? null,
        nativePresent: Boolean(native),
        nativeValue: native?.value ?? null,
        options: native ? Array.from(native.options).map((o) => `${o.value}|${o.textContent}`) : [],
        comboboxes: Array.from(document.querySelectorAll('[role="combobox"]')).map((c) => `${c.id}|${(c as HTMLElement).innerText.replace(/\n/g, " ").slice(0, 40)}`),
        instances: count,
        bytes: raw.length,
        treeRows,
      };
    })
    .catch((error) => ({ error: String(error).slice(0, 120) }));

for (let i = 0; i < 40; i++) {
  const s = await state();
  if ("instances" in s && s.instances > 0) break;
  await page.waitForTimeout(2000);
}
log(`before ${JSON.stringify(await state())}`);

const picker = page.locator('[id="playground.navbar.fixture"]');
const native = picker.locator("select").or(page.locator('select[id="playground.navbar.fixture"]')).first();
const nativeCount = await native.count().catch(() => 0);
log(`nativeCount=${nativeCount}`);
if (nativeCount) {
  const labels = await native.locator("option").allTextContents();
  const wanted = labels.find((l) => /nakagin/i.test(l)) ?? "";
  log(`labels=${JSON.stringify(labels)} wanted=${JSON.stringify(wanted)}`);
  const outcome = await native.selectOption({ label: wanted }).then((v) => `ok:${JSON.stringify(v)}`, (e) => `err:${String(e).slice(0, 160)}`);
  log(`selectOption ${outcome}`);
} else {
  await page.keyboard.press("Escape").catch(() => {});
  const box = await picker.first().boundingBox().catch(() => null);
  log(`picker box=${JSON.stringify(box)}`);
  log(
    `obstruction=${JSON.stringify(
      await page.evaluate(() => {
        const el = document.getElementById("playground.navbar.fixture");
        if (!el) return null;
        const rect = el.getBoundingClientRect();
        const top = document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2);
        const chain: string[] = [];
        for (let node: Element | null = top; node && chain.length < 6; node = node.parentElement) chain.push(`${node.tagName}#${node.id || "-"}[${node.getAttribute("data-slot") ?? "-"}]`);
        return { rect: { x: rect.x, y: rect.y, w: rect.width, h: rect.height }, covered: top !== el && !el.contains(top), chain, disabled: (el as HTMLButtonElement).disabled, ariaExpanded: el.getAttribute("aria-expanded") };
      }),
    )}`,
  );
  await picker.first().click({ timeout: 4000, force: true }).catch((e) => log(`picker force-click err ${String(e).slice(0, 100)}`));
  await page.waitForTimeout(800);
  let options = page.locator('[role="option"]');
  log(`after force-click options=${JSON.stringify(await options.allTextContents())} expanded=${await picker.first().getAttribute("aria-expanded")}`);
  if (!(await options.count())) {
    await picker.first().focus().catch(() => {});
    await page.keyboard.press("Enter").catch(() => {});
    await page.waitForTimeout(800);
    options = page.locator('[role="option"]');
    log(`after Enter options=${JSON.stringify(await options.allTextContents())}`);
  }
  if (!(await options.count())) {
    const clicked = await page.evaluate(() => {
      const el = document.getElementById("playground.navbar.fixture") as HTMLButtonElement | null;
      el?.click();
      return Boolean(el);
    });
    await page.waitForTimeout(800);
    options = page.locator('[role="option"]');
    log(`after dom-click(${clicked}) options=${JSON.stringify(await options.allTextContents())}`);
  }
  const target = options.filter({ hasText: /nakagin/i }).first();
  log(`nakaginOptions=${await target.count()}`);
  await target.click({ timeout: 4000, force: true }).catch((e) => log(`option click err ${String(e).slice(0, 100)}`));
}

const clickedAt = Date.now();
let arrived: number | null = null;
for (let i = 0; i < 120; i++) {
  await page.waitForTimeout(500);
  const s = await state();
  if (arrived === null && "instances" in s && s.instances >= 100) {
    arrived = Date.now() - clickedAt;
    log(`instances>=100 after ${arrived}ms ${JSON.stringify(s)}`);
  }
  if (arrived !== null && i > 4) break;
}
log(`arrivedMs=${arrived ?? "never"}`);
log(`tape:\n${tape.join("\n")}`);
await browser.close();
