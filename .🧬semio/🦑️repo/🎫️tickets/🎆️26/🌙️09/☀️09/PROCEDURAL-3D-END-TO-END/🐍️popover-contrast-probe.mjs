// 🗨️ Measures the painted foreground/background of every floating surface (navbar example picker,
// Window Options menu, command palette) against the shell it floats over, on the React generation3d
// playground. Reports WCAG contrast plus which portal host and appearance scope each surface resolved.
// Run once per appearance: SEMIO_PROBE_SCHEME=dark|light.
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6021/?plugin=generation3d";
const scheme = process.env.SEMIO_PROBE_SCHEME === "light" ? "light" : "dark";
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT_SECONDS ?? 28);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? `popover-contrast/${scheme}`);
mkdirSync(outDir, { recursive: true });

const lines = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, colorScheme: scheme });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 600)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 600)}`));

await page.addInitScript(() => {
  const canvas = document.createElement("canvas");
  canvas.width = canvas.height = 1;
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  const sample = () => {
    const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
    return { r: r / 255, g: g / 255, b: b / 255 };
  };
  const paintStack = (layers) => {
    ctx.clearRect(0, 0, 1, 1);
    ctx.fillStyle = "#ffffff";
    ctx.fillRect(0, 0, 1, 1);
    for (let index = layers.length - 1; index >= 0; index--) {
      ctx.fillStyle = layers[index];
      ctx.fillRect(0, 0, 1, 1);
    }
  };
  const lin = (c) => (c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4);
  const lum = (c) => 0.2126 * lin(c.r) + 0.7152 * lin(c.g) + 0.0722 * lin(c.b);
  const hex = (c) => `#${[c.r, c.g, c.b].map((v) => Math.round(Math.min(1, Math.max(0, v)) * 255).toString(16).padStart(2, "0")).join("")}`;
  const opaque = (value) => {
    ctx.clearRect(0, 0, 1, 1);
    ctx.fillStyle = value;
    ctx.fillRect(0, 0, 1, 1);
    return ctx.getImageData(0, 0, 1, 1).data[3] >= 255;
  };
  window.__semioPaint = (node) => {
    if (!node) return null;
    const style = getComputedStyle(node);
    const layers = [];
    for (let walk = node; walk; walk = walk.parentElement) {
      const fill = getComputedStyle(walk).backgroundColor;
      if (fill && fill !== "rgba(0, 0, 0, 0)" && fill !== "transparent") {
        layers.push(fill);
        if (opaque(fill)) break;
      }
    }
    paintStack(layers);
    const ground = sample();
    ctx.fillStyle = style.color;
    ctx.fillRect(0, 0, 1, 1);
    const text = sample();
    const ratio = (Math.max(lum(text), lum(ground)) + 0.05) / (Math.min(lum(text), lum(ground)) + 0.05);
    const scope = node.closest(".semio-scope");
    return {
      declaredColor: style.color,
      declaredBackground: style.backgroundColor,
      paintedBackground: hex(ground),
      paintedText: hex(text),
      contrast: Number(ratio.toFixed(2)),
      scopeAppearance: scope ? (scope.classList.contains("dark") ? "dark" : "light") : "none",
      portalHost: node.closest("[data-semio-portal-layer]") ? "shell-portal-layer" : scope ? "shell-subtree" : "document.body",
      row: (node.textContent ?? "").trim().slice(0, 48),
    };
  };
  const ROW = '[data-slot="select-item"], [role="option"], [role="menuitem"], [data-slot="menu-item"], [cmdk-item], button, a, span';
  window.__semioFloating = () => {
    const surfaces = [...document.querySelectorAll('[data-slot="select-content"], [data-slot="popover-content"], [data-slot="dialog-content"], [data-slot="context-menu-chrome"], [data-slot="tooltip-content"], [role="menu"]')];
    return surfaces.map((surface) => {
      const row = [...surface.querySelectorAll(ROW)].find((candidate) => (candidate.textContent ?? "").trim().length > 1) ?? surface;
      return { slot: surface.getAttribute("data-slot") ?? surface.getAttribute("role"), level: surface.getAttribute("data-level"), surface: window.__semioPaint(surface), row: window.__semioPaint(row) };
    });
  };
  window.__semioShell = () => {
    const scope = document.querySelector(".semio-scope");
    return {
      scopeClass: scope?.className ?? null,
      scopeAppearance: scope?.dataset?.uiAppearance ?? null,
      documentElementClass: document.documentElement.className,
      bodyBackground: getComputedStyle(document.body).backgroundColor,
      bodyColor: getComputedStyle(document.body).color,
      scopeBackground: scope ? getComputedStyle(scope).backgroundColor : null,
      portalLayerPresent: Boolean(document.querySelector("[data-semio-portal-layer]")),
    };
  };
  window.__semioClick = (id) => {
    const node = document.getElementById(id) ?? document.querySelector(id);
    if (!node) return false;
    node.click();
    return true;
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(bootSeconds * 1000);

const report = { url, scheme, shell: await page.evaluate(() => window.__semioShell()), surfaces: {} };
await page.screenshot({ path: join(outDir, "00-shell.png"), type: "png" });

async function probe(name, target, shot) {
  const opened = await page.evaluate((selector) => window.__semioClick(selector), target);
  await page.waitForTimeout(1200);
  const floating = await page.evaluate(() => window.__semioFloating());
  await page.screenshot({ path: join(outDir, shot), type: "png" });
  report.surfaces[name] = { opened, floating };
  await page.keyboard.press("Escape").catch(() => undefined);
  await page.waitForTimeout(500);
}

await probe("navbar-example-picker", "playground.navbar.fixture", "01-example-picker.png");

await page.evaluate(() => window.__semioClick("framework.window.proceduralPreview.measures.unfold"));
await page.waitForTimeout(1200);
await probe("window-options-shading-select", '[data-slot="window-pane-measures"] [role="combobox"], [role="combobox"]:not(#playground\\.navbar\\.fixture)', "02-window-options.png");

await page.evaluate(() => window.__semioClick("framework.category.command"));
await page.waitForTimeout(1500);
report.surfaces["command-panel"] = await page.evaluate(() => {
  const panel = document.getElementById("framework.panelTab.framework.category.command");
  const row = panel?.querySelector('[role="treeitem"] [data-slot="tree-label"]');
  return { present: Boolean(panel), row: window.__semioPaint(row ?? panel) };
});
await page.screenshot({ path: join(outDir, "03-command-panel.png"), type: "png" });

report.canvases = await page.evaluate(() =>
  [...document.querySelectorAll("canvas")].map((node) => {
    const rect = node.getBoundingClientRect();
    let mean = null;
    try {
      const ctx = node.getContext("2d");
      if (ctx) {
        const data = ctx.getImageData(0, 0, Math.min(200, node.width || 1), Math.min(200, node.height || 1)).data;
        let sum = 0;
        for (let index = 0; index < data.length; index += 4) sum += (data[index] + data[index + 1] + data[index + 2]) / 3;
        mean = Number((sum / (data.length / 4) / 255).toFixed(3));
      }
    } catch (error) {
      mean = String(error).slice(0, 60);
    }
    return { cls: (typeof node.className === "string" ? node.className : "").slice(0, 60), width: Math.round(rect.width), height: Math.round(rect.height), meanBrightness: mean };
  }),
);

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(JSON.stringify({ scheme, shell: report.shell, surfaces: report.surfaces }, null, 2));
await browser.close();
