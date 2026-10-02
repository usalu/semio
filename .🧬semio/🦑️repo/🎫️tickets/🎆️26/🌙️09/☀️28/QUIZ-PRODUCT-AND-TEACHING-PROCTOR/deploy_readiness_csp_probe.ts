/** 🛡️ Deploy-readiness probe: which Content-Security-Policy violations does the staged release site raise in a real
 * browser? `bun deploy_readiness_csp_probe.ts [port] [proctor origin]` serves `dist/pages/quizzes` like a static CDN
 * (files as they are, `404.html` for anything else) on a loopback port, opens it in Chrome with Playwright in both
 * appearances and both languages, walks every hash route the client knows and hovers every card, and prints each
 * `securitypolicyviolation` event and console error. With a proctor origin it also registers a learner first, so the
 * pages behind the identity screen are rendered. */
import { existsSync, readFileSync, statSync } from "node:fs";
import { extname, join } from "node:path";
import { chromium } from "playwright";

const port = Number(process.argv[2] ?? "6072");
const root = join(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/dist/pages/quizzes");
const types: Record<string, string> = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".svg": "image/svg+xml", ".ico": "image/x-icon", ".woff2": "font/woff2", ".webmanifest": "application/manifest+json", ".txt": "text/plain" };
const server = Bun.serve({
  hostname: "127.0.0.1",
  port,
  fetch(request) {
    const path = decodeURIComponent(new URL(request.url).pathname);
    const file = join(root, path === "/" ? "index.html" : path);
    if (existsSync(file) && statSync(file).isFile()) return new Response(readFileSync(file), { headers: { "content-type": types[extname(file)] ?? "application/octet-stream" } });
    return new Response(readFileSync(join(root, "404.html")), { status: 404, headers: { "content-type": types[".html"]! } });
  },
});
const origin = `http://127.0.0.1:${port}`;
const findings = new Set<string>();
const browser = await chromium.launch({ channel: "chrome" });
try {
  for (const [locale, colorScheme] of [["en-GB", "light"], ["de-DE", "dark"]] as const) {
    const context = await browser.newContext({ locale, colorScheme, ignoreHTTPSErrors: true, viewport: { width: 1440, height: 900 } });
    await context.addInitScript(() => {
      document.addEventListener("securitypolicyviolation", (event) => console.error(`[DEBUG] CSP ${event.effectiveDirective} blocked ${event.blockedURI || "inline"} at ${event.sourceFile ?? ""}:${event.lineNumber} sample=${JSON.stringify(event.sample)}`));
    });
    const page = await context.newPage();
    page.on("console", (message) => {
      if (message.type() === "error" && !/Failed to load resource|ERR_|WebSocket connection/u.test(message.text())) findings.add(`${locale}/${colorScheme} console: ${message.text().slice(0, 400)}`);
    });
    page.on("pageerror", (error) => findings.add(`${locale}/${colorScheme} page error: ${error.message.slice(0, 300)}`));
    await page.goto(origin);
    await page.waitForTimeout(2500);
    for (const name of [/continue|weiter/iu, /anonym/iu, /continue|weiter|start|los/iu]) {
      const button = page.getByRole("button", { name }).or(page.getByRole("radio", { name })).first();
      if (await button.isVisible().catch(() => false)) await button.click({ timeout: 2000 }).catch(() => undefined);
      await page.waitForTimeout(800);
    }
    for (const route of ["", "#learner", "#introduction", "#leaderboard", "#badges", "#preferences", "#quiz/physics", "#quiz/heating", "#quiz/cooling", "#quiz/demand"]) {
      await page.goto(`${origin}/${route}`);
      await page.waitForTimeout(1200);
      for (const card of await page.locator("[data-card], [data-layered-card], button").all()) await card.hover({ timeout: 500 }).catch(() => undefined);
    }
    console.log(`[DEBUG] ${locale}/${colorScheme}: lang=${await page.evaluate(() => document.documentElement.lang)} title=${JSON.stringify(await page.title())} text=${JSON.stringify((await page.locator("body").innerText()).replace(/\s+/gu, " ").slice(0, 160))}`);
    await context.close();
  }
} finally {
  await browser.close();
  server.stop(true);
}
console.log(findings.size === 0 ? "[DEBUG] no Content-Security-Policy violation, console error or page error" : [...findings].map((finding) => `[DEBUG] ${finding}`).join("\n"));
