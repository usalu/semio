/** 🩺️ F2 — the hot-update law as a plain bun script with a log line per step. usage: bun f2-hot-law-debug.ts */
/**
 * ♨️ Law: a source edit reaches a live page as a Vite HMR update through exactly the dev serve's own delivery path —
 * `server.watch: null` + `semioSourceFreshnessVitePlugins` (the `node:fs` source watcher replayed onto Vite's emitter) and
 * Vite's HMR socket, under the runtime this suite runs on (Bun: Vite's built-in-`ws` branch; Node: its bundled `ws`).
 *
 * A sandbox module graph (`🧰️framework/🟦️.ts` accepting `./🍃️leaf.ts`) is served by a real Vite dev server and loaded by
 * Chromium; every write style a developer's editor uses (in place, atomic temp + rename) must re-execute the leaf in the
 * page within the bound EXACTLY once (counted over a quiet window, so a doubled replay is a failure, not a pass), and a
 * server WITHOUT the watcher plugins must deliver nothing (so the law measures the replacement watcher, not a Vite watcher
 * that should not exist). The sandbox root is `mkdtemp` under the OS temp directory, a symlink on macOS: Vite keys its
 * module graph by real path, so the replayed events must name real paths too (the defect this root pins). Oracles: Vite's own client runtime (`import.meta.hot.accept`) and
 * Chromium. Launches a browser, so it runs from the `long` level on.
 *
 * @vitest-environment node
 */
import { mkdirSync, mkdtempSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { createServer as createNetServer } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createServer } from "vite";
import { semioSourceFreshnessVitePlugins } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts";

const HOT_UPDATE_BOUND_MS = 5_000;
const QUIET_WINDOW_MS = 750;
const longLevel = ["long", "exhaustive"].includes(process.env.SEMIO_TEST_LEVEL ?? "");

const WRITE_STYLES: readonly (readonly [string, (path: string, content: string) => void])[] = [
  ["an in-place write", (path, content) => writeFileSync(path, content)],
  ["an atomic temp + rename", (path, content) => {
    writeFileSync(`${path}.tmp`, content);
    renameSync(`${path}.tmp`, path);
  }],
];

async function loopbackPort(): Promise<number> {
  return new Promise((resolvePort, reject) => {
    const server = createNetServer();
    server.once("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const address = server.address();
      server.close(() => (address && typeof address === "object" ? resolvePort(address.port) : reject(new Error("no port"))));
    });
  });
}

function sandboxGraph(): { readonly root: string; readonly leaf: string } {
  const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "semio-hot-update-"));
  mkdirSync(join(root, "🧰️framework"), { recursive: true });
  const leaf = join(root, "🧰️framework/🍃️leaf.ts");
  writeFileSync(leaf, 'export const value: string = "0";\n');
  writeFileSync(join(root, "🧰️framework/🟦️.ts"), [
    'import { value } from "./🍃️leaf.ts";',
    "const state = { value, updates: 0 };",
    "Object.defineProperty(window, \"__hot\", { value: state });",
    'if (import.meta.hot) import.meta.hot.accept("./🍃️leaf.ts", (next) => { if (next) { state.value = next.value; state.updates += 1; } });',
    "",
  ].join("\n"));
  writeFileSync(join(root, "index.html"), '<!doctype html><html><head><meta charset="utf-8"></head><body><script type="module" src="/🧰️framework/🟦️.ts"></script></body></html>');
  return { root, leaf };
}

type HotState = { readonly value: string; readonly updates: number };

/** 🔇️ The page's update count once no update has arrived for a whole quiet window. Before the first write this absorbs the
 * filesystem's late report of the sandbox's own creation; after a write it is what makes a second, doubled update visible. */
async function settledUpdates(page: import("playwright").Page): Promise<number> {
  const read = () => page.evaluate(() => (window as unknown as { __hot: HotState }).__hot.updates);
  let last = await read();
  let since = Date.now();
  const deadline = Date.now() + 4 * HOT_UPDATE_BOUND_MS;
  while (Date.now() - since < QUIET_WINDOW_MS && Date.now() < deadline) {
    await page.waitForTimeout(50);
    const now = await read();
    log("read", now);
    if (now !== last) [last, since] = [now, Date.now()];
  }
  return last;
}

async function serve(root: string, withWatcher: boolean) {
  const port = await loopbackPort();
  const server = await createServer({
    configFile: false,
    root,
    logLevel: "silent",
    cacheDir: join(root, ".vite"),
    optimizeDeps: { noDiscovery: true, include: [] },
    server: { host: "127.0.0.1", port, strictPort: true, watch: null },
    plugins: withWatcher ? semioSourceFreshnessVitePlugins({ repoRoot: root }) : [],
  });
  await server.listen();
  return { server, url: `http://127.0.0.1:${port}/` };
}

const t0 = Date.now();
const log = (...parts: unknown[]) => console.log(String(Date.now() - t0).padStart(6), ...parts);
const expect = (actual: unknown) => ({ toEqual: (wanted: unknown) => log("expect", JSON.stringify(actual), "==", JSON.stringify(wanted)) });
await (async () => {
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    const graph = sandboxGraph();
    const watched = await serve(graph.root, true);
    const measured: Record<string, number> = {};
    try {
      const page = await browser.newPage();
      page.on("console", (m) => log("console", m.text()));
      page.on("framenavigated", () => log("navigated"));
      await page.goto(watched.url);
      log("loaded");
      await page.waitForFunction(() => (window as unknown as { __hot?: HotState }).__hot?.value === "0", undefined, { timeout: 60_000 });
      for (const [index, [label, write]] of WRITE_STYLES.entries()) {
        const next = String(index + 1);
        const before = await settledUpdates(page);
        const started = Date.now();
        write(graph.leaf, `export const value: string = "${next}";\n`);
        const arrived = await page.waitForFunction((wanted) => (window as unknown as { __hot: HotState }).__hot.value === wanted, next, { timeout: HOT_UPDATE_BOUND_MS, polling: 20 }).then(() => true, () => false);
        measured[label] = Date.now() - started;
        const updates = (await settledUpdates(page)) - before;
        expect({ label, arrived, updates }).toEqual({ label, arrived: true, updates: 1 });
      }
      await page.close();
    } finally {
      await watched.server.close();
    }
    const unwatched = await serve(graph.root, false);
    try {
      const page = await browser.newPage();
      await page.goto(unwatched.url);
      await page.waitForFunction(() => (window as unknown as { __hot?: HotState }).__hot !== undefined, undefined, { timeout: 60_000 });
      const before = await page.evaluate(() => (window as unknown as { __hot: HotState }).__hot.value);
      writeFileSync(graph.leaf, 'export const value: string = "unwatched";\n');
      await page.waitForTimeout(HOT_UPDATE_BOUND_MS);
      expect(await page.evaluate(() => (window as unknown as { __hot: HotState }).__hot)).toEqual({ value: before, updates: 0 });
    } finally {
      await unwatched.server.close();
      await browser.close();
      rmSync(graph.root, { recursive: true, force: true });
    }
    console.log(`hot update delivery (${process.versions.bun ? `bun ${process.versions.bun}` : `node ${process.versions.node}`}): ${JSON.stringify(measured)} ms`);
})();
