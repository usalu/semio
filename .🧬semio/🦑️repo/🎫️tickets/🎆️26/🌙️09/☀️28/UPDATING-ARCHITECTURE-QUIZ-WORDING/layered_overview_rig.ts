/** 🥞️ Runtime check of the migrated play and demonstrator landings on the shared `LayeredOverview` (ticket QUIZ-PRODUCT-AND-TEACHING-PROCTOR):
 * the REAL, unmodified `🎡️play/🟦️.tsx` and `🧺️demonstrator/🟦️.tsx` (real brands, cards, chrome, CSS, ui-react) with only the OS shell faked
 * (a hue-coded page with a canvas that logs mounts), served by Vite, driven by Playwright Chromium at 1440 × 900 (mouse) and 375 × 812 (touch).
 * Writes the scaffold to `🗑️generated/layered-final/rig/`, screenshots and `rig-report.json` to `🗑️generated/layered-final/`.
 *
 * Usage: bun layered_overview_rig.ts */

import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, extname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { chromium, type Page } from "playwright";
import { createServer } from "vite";

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, "../../../../../../..");
const out = join(here, "🗑️generated", "layered-final");
const rig = join(out, "rig");
const port = 6097;
mkdirSync(join(rig, "stubs"), { recursive: true });

const shell = `import { useEffect, useRef } from "react";
type RigLog = { mounts: { id: string; t: number }[]; unmounts: { id: string; t: number }[] };
const log: RigLog = ((window as unknown as { __rigShell?: RigLog }).__rigShell ??= { mounts: [], unmounts: [] });
const hue = (id: string) => [...id].reduce((h, ch) => (h * 31 + ch.charCodeAt(0)) % 360, 0);
export function FrameworkOsShell(props: { readonly shellId: string; readonly brand: { readonly windowTitle: string } }) {
  const canvas = useRef<HTMLCanvasElement | null>(null);
  const h = hue(props.shellId);
  useEffect(() => {
    log.mounts.push({ id: props.shellId, t: Math.round(performance.now()) });
    const el = canvas.current;
    const ctx = el?.getContext("2d");
    if (el && ctx) {
      el.width = 640;
      el.height = 400;
      ctx.fillStyle = \`hsl(\${h} 40% 92%)\`;
      ctx.fillRect(0, 0, 640, 400);
      ctx.strokeStyle = \`hsl(\${h} 60% 40%)\`;
      ctx.lineWidth = 2;
      for (let i = 0; i < 14; i += 1) { ctx.beginPath(); ctx.moveTo(i * 48, 0); ctx.lineTo(640 - i * 48, 400); ctx.stroke(); }
      ctx.fillStyle = \`hsl(\${h} 60% 30%)\`;
      ctx.font = "bold 64px sans-serif";
      ctx.fillText(props.shellId, 30, 210);
    }
    return () => void log.unmounts.push({ id: props.shellId, t: Math.round(performance.now()) });
  }, [props.shellId, h]);
  return (
    <div data-fake-shell={props.shellId} style={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", background: \`hsl(\${h} 45% 90%)\`, color: \`hsl(\${h} 60% 15%)\`, fontFamily: "sans-serif" }}>
      <div style={{ height: 40, background: \`hsl(\${h} 65% 45%)\`, color: "white", display: "flex", alignItems: "center", padding: "0 16px", fontWeight: 700, fontSize: 18 }}>{props.brand.windowTitle}</div>
      <div style={{ display: "flex", flex: 1, minHeight: 0 }}>
        <div style={{ width: 220, background: \`hsl(\${h} 45% 80%)\`, padding: 12, display: "flex", flexDirection: "column", gap: 8 }}>
          {["Tree", "Layers", "Properties", "Library", "History"].map((row) => <div key={row} style={{ background: \`hsl(\${h} 45% 95%)\`, padding: 8, fontSize: 14 }}>{row}</div>)}
        </div>
        <div style={{ flex: 1, padding: 16, display: "flex", flexDirection: "column", gap: 12 }}>
          <div style={{ fontSize: 56, fontWeight: 800, lineHeight: 1 }}>{props.shellId}</div>
          <canvas ref={canvas} style={{ width: "100%", flex: 1, minHeight: 0, background: "white" }} />
        </div>
      </div>
    </div>
  );
}
export const resolveShellLocks = (locks: unknown) => locks;
export const resolveShellDefaults = (_brand: unknown, _override: unknown) => ({});
`;
const page = (title: string, lang: string, main: string) => `<!doctype html>
<html lang="${lang}">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>${title}</title>
    <style>html, body, #root { height: var(--ui-available-height, 100dvh); margin: 0; }</style>
  </head>
  <body class="h-screen w-screen overflow-hidden bg-background text-foreground">
    <div class="h-full w-full" id="root"></div>
    <script type="module" src="./${main}"></script>
  </body>
</html>
`;
writeFileSync(join(rig, "stubs", "framework-renderer-react.tsx"), shell);
writeFileSync(join(rig, "stubs", "framework.ts"), `export * from "@rig-framework-real";\nexport const resolvePlaygroundBoot = (_catalog: unknown, variant: string) => ({ plugins: [], defaultAppId: variant });\n`);
writeFileSync(join(rig, "stubs", "plugin-registry-catalog.ts"), `export const PLUGIN_CATALOG: readonly unknown[] = [];\n`);
writeFileSync(join(rig, "stubs", "puzzle-js.ts"), `export const PUZZLE_BOARD_SESSION_FACTORIES = {};\n`);
writeFileSync(join(rig, "play.html"), page("rig · play", "en", "play-main.ts"));
writeFileSync(join(rig, "demonstrator.html"), page("rig · demonstrator", "de", "demonstrator-main.ts"));
writeFileSync(join(rig, "play-main.ts"), `import "@rig-play/🟦️.tsx";\n`);
writeFileSync(join(rig, "demonstrator-main.ts"), `import "@rig-demo/🟦️.tsx";\n`);
writeFileSync(join(rig, "grid.html"), page("rig · grid rest", "en", "grid-main.tsx"));
writeFileSync(
  join(rig, "grid-main.tsx"),
  `import { mountUiRoot } from "@semio-tech/ui-react/runtime";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import meta, { GridRest } from "@rig-ui/🧱️elements/🥞️LayeredOverview/📖️stories/🧪️.story.tsx";
import "@rig-play/🎨️globals.css";
bootstrapElementsSurfaceChromeDocument("light");
mountUiRoot(document.getElementById("root")!, <div className="relative h-full w-full">{meta.render({ ...meta.args, ...GridRest.args, reducedMotion: "never" } as never)}</div>);
`,
);

const server = await createServer({
  configFile: false,
  root: rig,
  cacheDir: join(rig, ".vite-cache"),
  logLevel: "warn",
  server: { port, strictPort: true, host: "127.0.0.1", fs: { allow: [repo] } },
  plugins: [
    ...[["/🖼️assets", join(repo, "🧰️framework/🔨️modules/🖼️assets")], ["/♻️mit-bestand", join(repo, "♻️mit-bestand")]].map(([prefix, root]) => ({
      name: `rig-static-${prefix}`,
      configureServer(dev: { middlewares: { use: (handler: (request: { url?: string }, response: { setHeader: (key: string, value: string) => void; end: (body: Buffer) => void }, next: () => void) => void) => void } }) {
        dev.middlewares.use((request, response, next) => {
          const url = decodeURIComponent((request.url ?? "").split("?")[0] ?? "");
          const file = join(root!, url.slice(prefix!.length));
          if (!url.startsWith(prefix!) || !existsSync(file) || !statSync(file).isFile()) return next();
          response.setHeader("content-type", ({ ".woff2": "font/woff2", ".svg": "image/svg+xml", ".png": "image/png", ".json": "application/json" } as Record<string, string>)[extname(file)] ?? "application/octet-stream");
          response.end(readFileSync(file));
        });
      },
    })),
    {
      name: "rig-play-registry-tolerance",
      transform(code: string, id: string) {
        if (!id.replaceAll("\\", "/").endsWith("🎡️play/🔨️modules/🧩️runtime/🟦️.ts")) return null;
        return code.replace("if (!target) throw new Error(`Unknown play pane variant: ${pane.variant}`);", "if (!target) return {};");
      },
    },
    react(),
    tailwindcss(),
  ],
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react/runtime", replacement: join(repo, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎠️runtime/🟦️.ts") },
      { find: "@semio-tech/ui-react/i18n", replacement: join(repo, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts") },
      { find: "@semio-tech/ui-react/chrome", replacement: join(repo, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🪟️chrome/🟦️.ts") },
      { find: "@semio-tech/ui-react", replacement: join(repo, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/assets", replacement: join(repo, "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/ui-styling", replacement: join(repo, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript") },
      { find: "@rig-framework-real", replacement: join(repo, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/framework-renderer-react", replacement: join(rig, "stubs/framework-renderer-react.tsx") },
      { find: "@semio-tech/plugin-registry/catalog", replacement: join(rig, "stubs/plugin-registry-catalog.ts") },
      { find: "@semio-tech/puzzle-js", replacement: join(rig, "stubs/puzzle-js.ts") },
      { find: "@semio-tech/framework", replacement: join(rig, "stubs/framework.ts") },
      { find: "@rig-play", replacement: join(repo, "🏢️semio-tech/🎡️play") },
      { find: "@rig-ui", replacement: join(repo, "🧰️framework/🔨️modules/🖱️ui") },
      { find: "@rig-demo", replacement: join(repo, "♻️mit-bestand/🧺️demonstrator") },
    ],
    dedupe: ["react", "react-dom"],
  },
  define: { "import.meta.vitest": "undefined" },
  optimizeDeps: { entries: [join(rig, "*.html")] },
});
await server.listen();
const base = `http://127.0.0.1:${port}`;
console.log(`[DEBUG] rig on ${base}`);

type Probe = Record<string, unknown>;
const report: Record<string, Probe> = {};
const installed = join(repo, "node_modules/.cache/ms-playwright/chromium_headless_shell-1223/chrome-headless-shell-win64/chrome-headless-shell.exe");
const browser = await chromium.launch(process.platform === "win32" ? { executablePath: installed } : {});

/** 📏️ Everything measured in the page at once. */
const probe = (p: Page): Promise<Probe> =>
  p.evaluate(() => {
    const veils = [...document.querySelectorAll<HTMLElement>(".ui-veil")];
    const navbar = document.querySelector<HTMLElement>('[data-slot="navbar"]');
    const box = navbar?.getBoundingClientRect();
    const strip = document.querySelector<HTMLElement>("[data-layered-strip]");
    const all = [...document.querySelectorAll("*")];
    const active = document.activeElement as HTMLElement | null;
    return {
      hash: location.hash,
      mode: document.querySelector<HTMLElement>("[data-layered-overview]")?.dataset.mode ?? null,
      strip: strip?.style.transform ?? null,
      veils: veils.map((veil) => ({ kind: veil.dataset.veil ?? null, level: veil.getAttribute("data-level"), visibility: getComputedStyle(veil).visibility, clip: getComputedStyle(veil).clipPath, backdrop: getComputedStyle(veil).backdropFilter, background: getComputedStyle(veil).backgroundColor })).slice(0, 3),
      veilCount: veils.length,
      navbar: box ? { top: Math.round(box.top), height: Math.round(box.height), visibleInViewport: box.bottom > 0 && box.top < innerHeight, position: getComputedStyle(navbar!).position, label: navbar!.getAttribute("aria-label") } : null,
      cards: document.querySelectorAll("[data-layered-card]").length,
      revealedCard: document.querySelector<HTMLElement>("[data-layered-card][data-revealed]")?.dataset.layeredCard ?? null,
      opened: document.querySelector<HTMLElement>("[data-layered-pane][data-opened]")?.dataset.layeredPane ?? null,
      overviewButton: document.querySelector("[data-layered-overview-button]")?.textContent?.trim() ?? null,
      active: active ? { tag: active.tagName, pane: active.dataset.layeredPane ?? null, card: active.closest<HTMLElement>("[data-layered-card]")?.dataset.layeredCard ?? null } : null,
      domNodes: all.length,
      placeholders: document.querySelectorAll('[data-layered-pane] [role="status"]').length,
      posters: document.querySelectorAll("[data-layered-pane] img").length,
      backdropFilterElements: all.filter((element) => { const value = getComputedStyle(element).backdropFilter; return value !== "none" && value !== ""; }).length,
      live: [...document.querySelectorAll<HTMLElement>("[data-fake-shell]")].map((element) => element.dataset.fakeShell),
      shellLog: (window as unknown as { __rigShell?: unknown }).__rigShell ?? null,
      inertPanes: document.querySelectorAll("[data-layered-pane][inert]").length,
    };
  });

async function dismiss(p: Page, touch = false): Promise<void> {
  const skip = p.locator('[id="ui.introduction.skip"], button:has-text("Skip"), button:has-text("Überspringen")');
  for (let attempt = 0; attempt < 3 && (await skip.count()) > 0; attempt += 1) {
    if (touch) await skip.first().tap();
    else await skip.first().click();
    await p.waitForTimeout(400);
  }
  if ((await skip.count()) > 0) await p.keyboard.press("Escape");
  await p.waitForTimeout(300);
}

async function center(p: Page, selector: string): Promise<{ x: number; y: number }> {
  const box = (await p.locator(selector).first().boundingBox())!;
  return { x: box.x + box.width / 2, y: box.y + box.height / 2 };
}

async function desktop(name: "play" | "demonstrator", hovers: readonly string[]): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const p = await context.newPage();
  p.setDefaultTimeout(180_000);
  const errors: string[] = [];
  p.on("console", (message) => message.type() === "error" && errors.push(message.text()));
  p.on("pageerror", (error) => errors.push(error.message));
  p.on("response", (response) => response.status() >= 400 && errors.push(`${response.status()} ${decodeURIComponent(response.url())}`));
  await p.goto(`${base}/${name}.html`);
  await p.waitForSelector("[data-layered-overview]");
  await p.waitForTimeout(1500);
  await p.screenshot({ path: join(out, `${name}-1440-first-visit-intro.jpg`), quality: 80 });
  await dismiss(p);
  await p.mouse.move(4, 4);
  await p.waitForTimeout(2500);
  const rest = await probe(p);
  await p.screenshot({ path: join(out, `${name}-1440-rest.jpg`), quality: 80 });
  await p.mouse.move(1436, 896, { steps: 8 });
  await p.waitForTimeout(1500);
  const pan = await probe(p);
  await p.screenshot({ path: join(out, `${name}-1440-pointer-pan.jpg`), quality: 80 });
  const hover: Probe[] = [];
  for (const id of hovers) {
    const card = await center(p, `[data-layered-card="${id}"] [data-overview-card]`);
    await p.mouse.move(card.x, card.y, { steps: 4 });
    await p.waitForTimeout(120);
    const glide = await probe(p);
    await p.waitForTimeout(900);
    const settled = await probe(p);
    await p.screenshot({ path: join(out, `${name}-1440-hover-${id}-clear.jpg`), quality: 80 });
    hover.push({ id, glide, settled });
  }
  await p.mouse.move(1436, 896, { steps: 4 });
  await p.waitForTimeout(80);
  const left = await probe(p);
  await p.mouse.move(2, 2);
  await p.mouse.click(2, 2);
  await p.waitForTimeout(600);
  await p.keyboard.press("Tab");
  await p.waitForTimeout(900);
  const tabFirst = await probe(p);
  await p.keyboard.press("Tab");
  await p.waitForTimeout(900);
  const tabSecond = await probe(p);
  await p.screenshot({ path: join(out, `${name}-1440-keyboard-focus-reveal.jpg`), quality: 80 });
  await p.keyboard.press("Enter");
  await p.waitForTimeout(1200);
  const opened = await probe(p);
  await p.screenshot({ path: join(out, `${name}-1440-opened.jpg`), quality: 80 });
  await p.keyboard.press("Escape");
  await p.waitForTimeout(900);
  const closed = await probe(p);
  await p.screenshot({ path: join(out, `${name}-1440-closed-focus-on-card.jpg`), quality: 80 });
  await p.mouse.click(2, 2);
  await p.waitForTimeout(300);
  const blurred = await probe(p);
  report[`${name}-1440`] = { rest, pan, hover, left, tabFirst, tabSecond, opened, closed, blurred, errors };
  await context.close();
}

async function deepLink(name: "play" | "demonstrator", id: string): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const p = await context.newPage();
  p.setDefaultTimeout(180_000);
  const errors: string[] = [];
  p.on("console", (message) => message.type() === "error" && errors.push(message.text()));
  p.on("pageerror", (error) => errors.push(error.message));
  p.on("response", (response) => response.status() >= 400 && errors.push(`${response.status()} ${decodeURIComponent(response.url())}`));
  await p.goto(`${base}/${name}.html#${id}`);
  await p.waitForSelector("[data-layered-overview]");
  await p.waitForTimeout(1200);
  const opened = await probe(p);
  await p.screenshot({ path: join(out, `${name}-1440-deep-link-${id}.jpg`), quality: 80 });
  await p.locator("[data-layered-overview-button]").click();
  await p.waitForTimeout(900);
  const overview = await probe(p);
  report[`${name}-1440-deep-link`] = { opened, overview, errors };
  await context.close();
}

/** 🔍️ The grid rest (the element's own `GridRest` story): every page live behind its card, a reveal zooming its page to full size. */
async function gridRest(): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const p = await context.newPage();
  p.setDefaultTimeout(180_000);
  const errors: string[] = [];
  p.on("console", (message) => message.type() === "error" && errors.push(message.text()));
  p.on("pageerror", (error) => errors.push(error.message));
  p.on("response", (response) => response.status() >= 400 && errors.push(`${response.status()} ${decodeURIComponent(response.url())}`));
  await p.goto(`${base}/grid.html`);
  await p.waitForSelector("[data-layered-overview][data-rest='grid']");
  await p.mouse.move(2, 2);
  await p.waitForTimeout(1500);
  const panes = () =>
    p.evaluate(() =>
      [...document.querySelectorAll<HTMLElement>("[data-layered-pane]")].map((pane) => {
        const box = pane.getBoundingClientRect();
        const card = document.querySelector(`[data-layered-card="${pane.dataset.layeredPane}"] [data-overview-card]`)?.getBoundingClientRect();
        return { id: pane.dataset.layeredPane, transform: pane.style.transform, clip: pane.style.clipPath, box: [box.left, box.top, box.width, box.height].map(Math.round), cardCentre: card ? [Math.round(card.left + card.width / 2), Math.round(card.top + card.height / 2)] : null, live: pane.querySelector("h1") !== null };
      }),
    );
  const rest = { probe: await probe(p), panes: await panes() };
  await p.screenshot({ path: join(out, "grid-1440-rest.jpg"), quality: 80 });
  const board = await center(p, '[data-layered-card="board"] [data-overview-card]');
  await p.mouse.move(board.x, board.y, { steps: 2 });
  await p.waitForTimeout(160);
  const zooming = { probe: await probe(p), panes: await panes() };
  await p.screenshot({ path: join(out, "grid-1440-zooming.jpg"), quality: 80 });
  await p.waitForTimeout(900);
  const zoomed = { probe: await probe(p), panes: await panes() };
  await p.screenshot({ path: join(out, "grid-1440-zoomed-board.jpg"), quality: 80 });
  await p.mouse.move(2, 2, { steps: 2 });
  await p.waitForTimeout(900);
  const back = { probe: await probe(p), panes: await panes() };
  await p.screenshot({ path: join(out, "grid-1440-back.jpg"), quality: 80 });
  report["grid-1440"] = { rest, zooming, zoomed, back, errors };
  await context.close();
}

async function phone(name: "play" | "demonstrator"): Promise<void> {
  const context = await browser.newContext({ viewport: { width: 375, height: 812 }, hasTouch: true, isMobile: true, deviceScaleFactor: 2 });
  const p = await context.newPage();
  p.setDefaultTimeout(180_000);
  const errors: string[] = [];
  p.on("console", (message) => message.type() === "error" && errors.push(message.text()));
  p.on("pageerror", (error) => errors.push(error.message));
  p.on("response", (response) => response.status() >= 400 && errors.push(`${response.status()} ${decodeURIComponent(response.url())}`));
  await p.goto(`${base}/${name}.html`);
  await p.waitForSelector("[data-layered-overview]");
  await p.waitForTimeout(1500);
  await dismiss(p, true);
  await p.waitForTimeout(1500);
  const list = await probe(p);
  await p.screenshot({ path: join(out, `${name}-375-list.jpg`), quality: 80 });
  await p.locator("[data-layered-list]").evaluate((element) => element.scrollBy({ top: element.clientHeight }));
  await p.waitForTimeout(1200);
  const scrolled = await probe(p);
  await p.screenshot({ path: join(out, `${name}-375-list-second.jpg`), quality: 80 });
  const sections = await p.locator("[data-layered-section]").count();
  const credits = await p.evaluate(() => [...document.querySelectorAll<HTMLElement>(".pointer-events-auto")].filter((element) => element.closest("[data-layered-overview]") && !element.closest("[data-layered-card]")).map((element) => { const box = element.getBoundingClientRect(); return { top: Math.round(box.top), left: Math.round(box.left), right: Math.round(box.right), bottom: Math.round(box.bottom) }; }));
  report[`${name}-375`] = { list, scrolled, sections, credits, errors };
  await context.close();
}

try {
  await gridRest();
  await desktop("demonstrator", ["aggregator", "statik"]);
  await desktop("play", ["flow", "puzzle3d"]);
  await deepLink("demonstrator", "statik");
  await deepLink("play", "draw");
  await phone("demonstrator");
  await phone("play");
} finally {
  writeFileSync(join(out, "rig-report.json"), JSON.stringify(report, null, 2));
  await browser.close();
  await server.close();
}
console.log(`[DEBUG] wrote ${join(out, "rig-report.json")}`);
