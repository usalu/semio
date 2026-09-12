// 🔀️ Runtime proof for the surface-role lane: `?role=` on the React dev port, the navbar role group,
// and an in-shell role switch by click. Writes DOM evidence + screenshots into
// `🗑️generated/role-switch/`. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const base = process.env.SEMIO_PROBE_BASE ?? "http://127.0.0.1:6018";
const settle = Number(process.env.SEMIO_PROBE_SECONDS ?? 25);
const outDir = join(import.meta.dirname, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "role-switch");
mkdirSync(outDir, { recursive: true });

const snapshot = () =>
  document.querySelectorAll === undefined
    ? null
    : {
        roleChip: document.querySelector('[data-slot="surface-role-chip"]')?.getAttribute("data-role") ?? null,
        breadcrumb: document.querySelector('[data-slot="app-name"]')?.textContent ?? null,
        rolesGroup: document.getElementById("playground.navbar.roles") === null ? null : {
          role: document.getElementById("playground.navbar.roles").getAttribute("role"),
          ariaLabel: document.getElementById("playground.navbar.roles").getAttribute("aria-label"),
          items: [...document.getElementById("playground.navbar.roles").querySelectorAll("button")].map((button) => ({
            id: button.id,
            text: button.textContent,
            pressed: button.getAttribute("aria-pressed"),
            state: button.getAttribute("data-state"),
            keyshortcuts: button.getAttribute("aria-keyshortcuts"),
            title: button.getAttribute("title"),
          })),
        },
        modesGroup: document.getElementById("playground.navbar.modes") === null ? null : {
          role: document.getElementById("playground.navbar.modes").getAttribute("role"),
          ariaLabel: document.getElementById("playground.navbar.modes").getAttribute("aria-label"),
          keyshortcuts: document.getElementById("playground.navbar.modes").getAttribute("aria-keyshortcuts"),
          items: [...document.getElementById("playground.navbar.modes").querySelectorAll("button")].map((button) => ({ id: button.id, pressed: button.getAttribute("aria-pressed") })),
        },
        surfaces: [...document.querySelectorAll("[data-surface-id]")].map((el) => el.getAttribute("data-surface-id")),
      };

const browser = await chromium.launch({ headless: true });
const report = {};
const pageLines = new Map();
const flush = (page) => { const entry = pageLines.get(page); writeFileSync(join(outDir, `${entry.name}.console.txt`), entry.lines.join("\n")); };

async function visit(name, search) {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  const lines = [];
  pageLines.set(page, { name, lines });
  const t0 = Date.now();
  page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 1200)}`));
  page.on("pageerror", (error) => lines.push(`${Date.now() - t0} pageerror ${String(error).slice(0, 1200)}`));
  await page.goto(`${base}/${search}`, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(settle * 1000);
  report[name] = await page.evaluate(snapshot);
  await page.screenshot({ path: join(outDir, `${name}.png`), type: "png" });
  flush(page);
  return page;
}

await (await visit("boot-editor", "?plugin=generation3d&role=editor")).close();
const viewerPage = await visit("boot-viewer", "?plugin=generation3d&role=viewer");

await viewerPage.click('[id="playground.navbar.roles.editor"]');
await viewerPage.waitForTimeout(settle * 1000);
report["clicked-editor"] = await viewerPage.evaluate(snapshot);
await viewerPage.screenshot({ path: join(outDir, "clicked-editor.png"), type: "png" });

await viewerPage.click('[id="playground.navbar.roles.viewer"]');
await viewerPage.waitForTimeout(settle * 1000);
report["clicked-viewer"] = await viewerPage.evaluate(snapshot);
await viewerPage.screenshot({ path: join(outDir, "clicked-viewer.png"), type: "png" });
flush(viewerPage);

// ⌨️ Keyboard path: the framework chord for each role button, spelled for THIS platform (`mod` is
// Meta on Apple, Control elsewhere — the same rule `parseOwnedHotkeyChords` applies).
const modifier = (await viewerPage.evaluate(() => /mac|iphone|ipad|ipod/i.test(navigator.platform))) ? "Meta" : "Control";
await viewerPage.keyboard.press(`${modifier}+Alt+e`);
await viewerPage.waitForTimeout(settle * 1000);
report["keyboard-editor"] = await viewerPage.evaluate(snapshot);
await viewerPage.screenshot({ path: join(outDir, "keyboard-editor.png"), type: "png" });

await viewerPage.keyboard.press(`${modifier}+Alt+ArrowRight`);
await viewerPage.waitForTimeout(2000);
report["keyboard-mode-next"] = await viewerPage.evaluate(snapshot);
await viewerPage.screenshot({ path: join(outDir, "keyboard-mode-next.png"), type: "png" });

await viewerPage.keyboard.press(`${modifier}+Alt+v`);
await viewerPage.waitForTimeout(settle * 1000);
report["keyboard-viewer"] = await viewerPage.evaluate(snapshot);
await viewerPage.screenshot({ path: join(outDir, "keyboard-viewer.png"), type: "png" });
flush(viewerPage);

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
