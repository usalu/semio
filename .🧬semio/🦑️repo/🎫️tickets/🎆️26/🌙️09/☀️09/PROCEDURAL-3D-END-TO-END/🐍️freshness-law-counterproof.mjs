/** 🧷️ Counterproof for `dev server transform freshness`: the same sandbox dev server, mounted with the
 * SOURCE WATCHER ALONE, must fail every atomic write style — otherwise the law would pass without the
 * request-time stat guard and would prove nothing.
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts */
import { mkdirSync, mkdtempSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { createServer } from "vite";
import { semioSourceFreshnessVitePlugins, semioSourceWatchVitePlugin } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts";

const styles = [
  ["in-place write", (target, body) => writeFileSync(target, body)],
  ["atomic save (rename into place)", (target, body) => { const temporary = `${target}.tmp`; writeFileSync(temporary, body); renameSync(temporary, target); }],
  ["sed -i '' (macOS temp + rename)", (target, body) => { spawnSync("sed", ["-i", "", `1s|.*|${body.trim()}|`, target]); }],
];

async function measure(label, plugins) {
  const rows = [];
  for (const [style, write] of styles) {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-freshness-counterproof-"));
    mkdirSync(join(sandbox, "🧰️framework"), { recursive: true });
    const target = join(sandbox, "🧰️framework/🟦️.ts");
    writeFileSync(target, "export const value: number = 0;\n");
    const server = await createServer({ configFile: false, root: sandbox, logLevel: "silent", cacheDir: join(sandbox, ".vite"), optimizeDeps: { noDiscovery: true, include: [] }, server: { host: "127.0.0.1", port: 0, hmr: false, watch: null }, plugins: plugins(sandbox) });
    try {
      await server.listen();
      const port = server.httpServer.address().port;
      const request = async () => (await fetch(`http://127.0.0.1:${port}/@fs${target}`, { headers: { accept: "*/*" } })).text();
      const cached = await request();
      if (cached.includes(": number")) throw new Error("not a transform: " + cached.slice(0, 120));
      const stamp = `export const value = ${Date.now()};`;
      write(target, `${stamp}\n`);
      const next = await request();
      rows.push({ mounted: label, style, firstRequestAfterEdit: next.includes(stamp) ? "fresh" : "STALE" });
    } finally {
      await server.close();
      rmSync(sandbox, { recursive: true, force: true });
    }
  }
  return rows;
}

const rows = [
  ...(await measure("no watcher at all", () => [])),
  ...(await measure("watcher only (pre-fix shape)", (sandbox) => [semioSourceWatchVitePlugin({ repoRoot: sandbox })])),
  ...(await measure("watcher + stat guard (fix)", (sandbox) => semioSourceFreshnessVitePlugins({ repoRoot: sandbox }))),
];
console.table(rows);
process.exit(0);
