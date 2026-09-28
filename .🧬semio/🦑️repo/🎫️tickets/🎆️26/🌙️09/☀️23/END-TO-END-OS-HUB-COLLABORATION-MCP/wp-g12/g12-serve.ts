/** 🛎️ G12: brings up ONE `s` React dev serve on a G12 port through S18's shared fixture (`ensureDevServe`), prints its
 * progress and pid, and leaves it running (detached). usage: bun g12-serve.ts <port> [hubUrl] */
import { ensureDevServe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";

const port = Number(process.argv[2] ?? 6530);
const hubUrl = process.argv[3];
const fixture = await ensureDevServe({
  repoRoot: "/Users/ueli/Documents/semio",
  port,
  hubUrl,
  logPath: `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-g12-logs/serve-${port}.log`,
  onProgress: (status, text) => console.log(`${new Date().toISOString()} ${text} ${"pid" in status ? `pid=${status.pid}` : ""}`),
});
console.log(`serve ${fixture.url} reused=${fixture.reused}`);
process.exit(0);
