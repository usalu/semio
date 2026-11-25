/** 🧪️ Serves (the runner config now launches the new headless mode, `channel: "chromium"`, so no executable path is needed) a static Storybook build in-process and runs the frame-visuals Playwright suite against it. Usage: node frame-visuals-run.mjs <static dir> [playwright args…] */
import { spawn } from "node:child_process";
import { createServer } from "node:http";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { extname, resolve } from "node:path";
import { homedir } from "node:os";
const [dist, ...rest] = process.argv.slice(2);
const root = "/Users/ueli/Documents/semio";
const types = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".json": "application/json", ".svg": "image/svg+xml", ".png": "image/png", ".woff2": "font/woff2" };
const server = createServer((request, response) => {
  const file = resolve(dist, "." + decodeURIComponent(new URL(request.url ?? "/", "http://x").pathname));
  if (!file.startsWith(resolve(dist)) || !existsSync(file) || statSync(file).isDirectory()) { response.statusCode = 404; return void response.end(); }
  response.setHeader("content-type", types[extname(file)] ?? "application/octet-stream");
  response.end(readFileSync(file));
});
await new Promise(accept => server.listen(0, "127.0.0.1", accept));
const bunRoot = `${homedir()}/Library/Caches/semio/runtimes/bun`;
const bun = `${bunRoot}/${readdirSync(bunRoot).sort()[0]}/bun`;
const child = spawn(bun, ["--bun", "node_modules/@playwright/test/cli.js", "test", "--config", ".storybook/🧪️tests/🧪️browser-runner/🟦️.ts", "--grep", process.env.FV_GREP ?? "frame and chip visuals", ...rest], {
  cwd: root, stdio: "inherit",
  env: { ...process.env, PLAYWRIGHT_BASE_URL: `http://127.0.0.1:${server.address().port}/`, PLAYWRIGHT_BROWSERS_PATH: `${root}/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` },
});
child.on("exit", code => { server.close(); process.exit(code ?? 1); });
