import { spawn } from "node:child_process";
import { once } from "node:events";
import { closeSync, existsSync, mkdirSync, openSync, rmSync } from "node:fs";
import { join } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright-core";

const ticket = fileURLToPath(new URL(".", import.meta.url));
const root = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const generated = join(ticket, "🗑️generated");
const viewports = [[1440, 900], [768, 1024], [375, 812]].filter(([width]) => !process.env.SEMIO_BORDER_WIDTH || width === Number(process.env.SEMIO_BORDER_WIDTH));
const apps = {
  hub: { port: 5716, config: "vite-hub-admin.config.ts", route: "admin/", probe: "lv-final-hub.js", method: "run", ready: '[data-slot="navbar"]', env: { HUB_FROZEN: "1" } },
  staffage: { port: 5717, config: join(root, "🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/🏗️builder/🌐️vite/🟦️.ts"), route: "", probe: "lv-final-staffage.js", method: "runAll", args: [["overview", "all", "collections", "compare", "playground"]], ready: ".staffage-app", env: { TEACHING_ARCHITECTURE_STAFFAGE_WATCH: "off" } },
  presentation: { port: 5718, config: "vite-presentation-frozen.config.ts", route: "", probe: "round2-presentation-sweep.js", method: "sweep", args: [Number(process.env.SEMIO_BORDER_SLIDES ?? 12)], ready: ".slides section.present" },
  demo: { port: 5719, config: "r3-vite-demonstrator-frozen.config.ts", route: "", probe: "lv-final-demo.js", method: "run", ready: ".semio-scope" },
  cad: { port: 5720, config: "vite-round3-cad.config.ts", route: "", probe: "border-gap-probe.js", method: "probe", ready: ".semio-scope" },
  play: { port: 5723, config: "vite-play-border-check.config.ts", route: "", probe: "lv-final-play.js", method: "run", ready: "[data-layered-overview], button" },
};

const launchOptions = process.env.SEMIO_CHECK_CHROMIUM ? { executablePath: process.env.SEMIO_CHECK_CHROMIUM } : {};
mkdirSync(generated, { recursive: true });
const browser = await chromium.launch({ ...launchOptions, headless: true });
const selected = process.argv.slice(2);

try {
  for (const name of selected.length ? selected : Object.keys(apps)) {
    const app = apps[name];
    if (!app) throw new Error(`Unknown app: ${name}`);
    const log = openSync(join(generated, `live-${name}.log`), "w");
    const base = `http://127.0.0.1:${app.port}/`;
    const config = app.config.startsWith(root) ? app.config : join(ticket, app.config);
    const cache = join(root, ".cache", "b", "p", "m", `borders-${process.pid}-${name}`);
    const server = spawn(process.execPath, ["--experimental-transform-types", join(root, "node_modules/vite/bin/vite.js"), "--configLoader", "native", "--config", config, "--mode", "prototype", "--port", String(app.port), "--strictPort", "--host", "127.0.0.1"], {
      cwd: root,
      stdio: ["ignore", log, log],
      env: { ...process.env, ...app.env, SEMIO_BUILD_MODE: "prototype", SEMIO_VITE_CACHE_DIRECTORY: cache },
    });
    closeSync(log);
    try {
      for (let attempt = 0; ; attempt++) {
        if (server.exitCode !== null) throw new Error(`Vite exited with code ${server.exitCode}`);
        if (await fetch(base).then(() => true, () => false)) break;
        if (attempt === 40) throw new Error("Vite did not respond");
        await delay(250);
      }
      for (const [width, height] of viewports) {
        const context = await browser.newContext({ viewport: { width, height }, locale: "en" });
        const errors = [];
        try {
          const page = await context.newPage();
          page.on("pageerror", error => errors.push(error.message));
          await page.route("**/__border-check/*.js", async request => {
            const filename = new URL(request.request().url()).pathname.split("/").pop();
            const path = join(ticket, filename);
            if (!/^[\w-]+\.js$/u.test(filename) || !existsSync(path)) return request.abort();
            await request.fulfill({ path, contentType: "text/javascript" });
          });
          await page.goto(base + app.route, { waitUntil: "domcontentloaded", timeout: 45_000 });
          await page.locator(app.ready).first().waitFor({ state: "attached", timeout: 45_000 });
          const readings = await page.evaluate(async ({ path, method, args }) => {
            const probe = await import(path);
            if (method !== "probe") {
              await probe[method](...args);
              return method === "sweep" ? [probe.report()] : probe.rows;
            }
            for (let attempt = 0; attempt < 30; attempt++) {
              await new Promise(resolve => setTimeout(resolve, 150));
              if (!document.querySelector("[data-shared-border-pending]")) break;
            }
            const result = probe.probe({});
            return [`count=${result.count} pending=${!!result.pending} edges=${result.edges} ${JSON.stringify(result.violations.slice(0, 4))}`];
          }, { path: "/__border-check/" + app.probe, method: app.method, args: app.args ?? [] });
          console.log(`[DEBUG] ${name} ${width}x${height}\n${readings.join("\n")}`);
        } catch (error) {
          process.exitCode = 1;
          console.error(`[DEBUG] ${name} ${width}x${height}: ${error.message}${errors.length ? "\n" + errors.join("\n") : ""}`);
        } finally {
          await context.close();
        }
      }
    } catch (error) {
      process.exitCode = 1;
      console.error(`[DEBUG] ${name}: ${error.message}`);
    } finally {
      if (server.exitCode === null && server.signalCode === null) {
        const exited = once(server, "exit");
        server.kill();
        await exited;
      }
      rmSync(cache, { recursive: true, force: true });
    }
  }
} finally {
  await browser.close();
}
