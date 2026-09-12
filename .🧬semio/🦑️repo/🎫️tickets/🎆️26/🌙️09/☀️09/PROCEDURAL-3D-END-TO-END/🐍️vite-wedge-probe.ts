/** @emoji 🕵️ Instrumented twin of `bun ./📜️script.ts serve <variant> react dev`: boots the real
 * `⚙️vite.config.ts` dev server and samples watcher fan-out, raw FSEvents throughput, event-loop lag
 * and RSS every 2s so a wedge can be attributed to a mechanism instead of guessed at. */
import { existsSync, readdirSync, watch } from "node:fs";
import { join, sep } from "node:path";
import { createServer } from "vite";

const packageRoot = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
const kqueue = process.env.SEMIO_WEDGE_KQUEUE === "1";
const owned = process.env.SEMIO_WEDGE_OWNWATCH === "1";
const server = await createServer({ configFile: `${packageRoot}/⚙️vite.config.ts`, ...(kqueue ? { server: { watch: { useFsEvents: false, usePolling: false } } } : {}), ...(owned ? { server: { watch: null } } : {}) });
console.log(`[DEBUG] kqueue=${kqueue} owned=${owned}`);
let ownedHits = 0;
if (owned) {
  const repoRoot = "/Users/ueli/Documents/semio";
  const denied = new Set([".git", ".nx", "node_modules", ".🧬semio", "dist", "target", "🗑️generated", ".vscode", ".claude"]);
  const roots = readdirSync(repoRoot, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !denied.has(entry.name)).map((entry) => join(repoRoot, entry.name));
  console.log(`[DEBUG] owned roots=${roots.length}`);
  for (const root of roots) {
    watch(root, { recursive: true, persistent: true }, (eventType, name) => {
      if (name === null) return;
      const segments = name.split(sep);
      for (const segment of segments) if (denied.has(segment)) return;
      ownedHits += 1;
      server.watcher.emit(eventType === "rename" ? "add" : "change", join(root, name));
    });
  }
}
await server.listen();
server.printUrls();

const clientEnvironment = server.environments.client as unknown as { transformRequest(url: string, options?: unknown): Promise<unknown> };
const originalTransformRequest = clientEnvironment.transformRequest.bind(clientEnvironment);
const armPath = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/vite-wedge/ARM";
let stackBudget = 60;
clientEnvironment.transformRequest = (url: string, options?: unknown) => {
  if (stackBudget > 0 && existsSync(armPath)) {
    stackBudget -= 1;
    console.log(`[DEBUG] transformRequest ${url}\n${new Error("trace").stack?.split("\n").slice(1, 12).join("\n")}`);
  }
  return originalTransformRequest(url, options);
};

let raw = 0;
let events = 0;
const perEvent = new Map<string, number>();
server.watcher.on("raw", (_event: string, path: string) => {
  raw += 1;
  const key = path.replace("/Users/ueli/Documents/semio/", "").split("/").slice(0, 3).join("/");
  perEvent.set(key, (perEvent.get(key) ?? 0) + 1);
});
for (const name of ["add", "change", "unlink", "addDir", "unlinkDir"] as const) server.watcher.on(name, () => { events += 1; });

let lagMax = 0;
let previous = performance.now();
setInterval(() => {
  const now = performance.now();
  lagMax = Math.max(lagMax, now - previous - 50);
  previous = now;
}, 50);

const started = Date.now();
setInterval(() => {
  const watched = server.watcher.getWatched();
  const directories = Object.keys(watched).length;
  const files = Object.values(watched).reduce((total, entries) => total + entries.length, 0);
  const graph = server.environments.client.moduleGraph.idToModuleMap.size;
  const top = [...perEvent.entries()].sort((a, b) => b[1] - a[1]).slice(0, 3).map(([key, count]) => `${key}=${count}`).join(" ");
  console.log(`[DEBUG] t=${((Date.now() - started) / 1000).toFixed(0)}s dirs=${directories} files=${files} graph=${graph} raw=${raw} own=${ownedHits} evt=${events} lagMax=${lagMax.toFixed(0)}ms rss=${Math.round(process.memoryUsage.rss() / 1e6)}MB ${top}`);
  raw = 0;
  events = 0;
  ownedHits = 0;
  lagMax = 0;
  perEvent.clear();
}, 2000);
