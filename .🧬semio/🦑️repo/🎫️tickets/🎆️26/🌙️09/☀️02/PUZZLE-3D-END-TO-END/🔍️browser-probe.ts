/** 🔬️ Headless runtime probe for the puzzle 3d React serve on 127.0.0.1:6013 — boots the shell,
 * waits for windows, optionally runs interaction steps, and writes findings + screenshots into
 * `🗑️generated/`. Ticket 26/09/02/PUZZLE-3D-END-TO-END. Run: `bun 🔍️browser-probe.ts [--interact]`. */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const interact = process.argv.includes("--interact");
const lines: string[] = [];
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${m}`;
  lines.push(row);
  console.log(row);
};
const t0 = Date.now();

const consoleBuf: string[] = [];
const faults: string[] = [];
const FAULT_RE =
  /intake-budget-exhausted|fixed-capacity|section-root-mismatch|native-owner-required|terminal-fault|unreachable|shard .* (lost|terminated)|did not publish|missing field|malformed|admission failed|worker fault|\[semio-plugin panic\]/i;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (msg) => {
  const text = msg.text().slice(0, 400);
  if (consoleBuf.length < 4000) consoleBuf.push(`${msg.type()}: ${text}`);
  if (FAULT_RE.test(text) && faults.length < 200) faults.push(text);
});
page.on("pageerror", (err) => faults.push(`pageerror: ${String(err).slice(0, 400)}`));

log("navigating");
await page.goto("http://127.0.0.1:6013/?plugin=puzzle3d", { waitUntil: "domcontentloaded", timeout: 60000 });

const snapshot = async () =>
  page.evaluate(() => {
    const q = (sel: string) => Array.from(document.querySelectorAll(sel));
    return {
      windows: q('[data-slot="window"]').map((w) => ({
        id: w.id || w.getAttribute("data-key") || "?",
        w: (w as HTMLElement).offsetWidth,
        h: (w as HTMLElement).offsetHeight,
      })),
      canvases: q("canvas").length,
      tabs: q('[data-slot="panel-tab-button"]').map((b) => b.id).slice(0, 40),
      toggles: q('[data-slot="toggle-group-item"]').map((b) => `${b.id}=${b.getAttribute("aria-pressed")}`).slice(0, 40),
      treeItems: q('[data-slot="tree-item"], [role="treeitem"]').length,
      dialogs: q('[role="dialog"]').map((d) => (d as HTMLElement).innerText.slice(0, 80)),
      body: document.body ? document.body.innerText.slice(0, 500) : "",
    };
  });

let booted = false;
for (let i = 0; i < 60; i++) {
  await page.waitForTimeout(3000);
  const s = await snapshot();
  if (s.dialogs.length && i % 3 === 0) {
    const skip = page.locator('[role="dialog"] button', { hasText: /skip/i }).first();
    if (await skip.count()) {
      await skip.click({ timeout: 2000 }).catch(() => {});
      log("skipped intro dialog");
    }
  }
  if (s.windows.length >= 2 && s.canvases >= 2) {
    log(`booted: windows=${JSON.stringify(s.windows)} canvases=${s.canvases} treeItems=${s.treeItems}`);
    booted = true;
    break;
  }
  if (i % 5 === 4) log(`waiting… windows=${s.windows.length} canvases=${s.canvases} faults=${faults.length}`);
}
await page.screenshot({ path: join(OUT, `probe-${stamp}-boot.png`) }).catch(() => {});

if (booted && interact) {
  const step = async (name: string, fn: () => Promise<void>) => {
    const before = faults.length;
    try {
      await fn();
      await page.waitForTimeout(4000);
      const s = await snapshot();
      log(`step ${name}: ok windows=${s.windows.length} canvases=${s.canvases} treeItems=${s.treeItems} newFaults=${faults.slice(before).join(" | ").slice(0, 300) || "none"}`);
    } catch (e) {
      log(`step ${name}: FAILED ${String(e).slice(0, 200)}`);
    }
    await page.screenshot({ path: join(OUT, `probe-${stamp}-${name}.png`) }).catch(() => {});
  };
  await step("activate-perspective", async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 200, y: 200 }, timeout: 5000 });
  });
  await step("pick-object", async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 470, y: 420 }, timeout: 5000 });
  });
  await step("context-menu", async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 470, y: 420 }, button: "right", timeout: 5000 });
  });
  await step("example-switch", async () => {
    const sel = page.locator("select").first();
    if (await sel.count()) await sel.selectOption({ index: 1 });
    else {
      const combo = page.locator('[role="combobox"]').first();
      await combo.click({ timeout: 3000 });
      await page.locator('[role="option"]').nth(1).click({ timeout: 3000 });
    }
    await page.waitForTimeout(15000);
  });
}

const tail = consoleBuf.slice(-120).join("\n");
writeFileSync(
  join(OUT, `probe-${stamp}.md`),
  `# probe ${stamp} (interact=${interact})\n\n## timeline\n${lines.join("\n")}\n\n## faults (${faults.length})\n${faults.slice(0, 100).join("\n")}\n\n## console tail\n\`\`\`\n${tail}\n\`\`\`\n`,
);
log(`done booted=${booted} faults=${faults.length} → 🗑️generated/probe-${stamp}.md`);
await browser.close();
process.exit(0);
