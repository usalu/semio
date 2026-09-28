/** 🛎️ F3 — one `s` React dev serve on a slice port through S18's shared fixture; the serve outlives this script.
 * usage: bun f3-ensure-serve.ts <port> <tag> [hubUrl|local] */
import { ensureDevServe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";
const [port, tag, hub = "http://127.0.0.1:7800"] = process.argv.slice(2);
const repoRoot = "/Users/ueli/Documents/semio";
const started = Date.now();
const serve = await ensureDevServe({
  repoRoot,
  port: Number(port),
  hubUrl: hub === "local" ? undefined : hub,
  locale: "en",
  logPath: `${repoRoot}/.🧬semio/🌐hub/s14-f3-logs/serve-${port}-${tag}.txt`,
  onProgress: (_status, line) => console.log(`${Date.now() - started} ${line}`),
});
console.log(JSON.stringify({ url: serve.url, reused: serve.reused, readyMs: Date.now() - started }));
process.exit(0);
