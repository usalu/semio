/** 🛎️ WG11 s14 — starts (or reuses) the React `s` dev serve on one port through S18's shared fixture (`ensureDevServe`, preamble
 * rule 18), joined to one hub. The serve outlives this runner (its own process group).
 * Usage: bun wg11-react-serve.ts <port> <hubUrl> [locale en|de] */
import { ensureDevServe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";

const [port, hubUrl, locale] = process.argv.slice(2);
if (!port || !hubUrl) throw new Error("usage: bun wg11-react-serve.ts <port> <hubUrl> [en|de]");
const serve = await ensureDevServe({
  repoRoot: "/Users/ueli/Documents/semio",
  port: Number(port),
  hubUrl,
  locale: locale === "de" ? "de" : "en",
  logPath: `/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-wg11-logs/react-serve-${port}.log`,
  onProgress: (_status, line) => console.log(line),
});
console.log(`SERVE ${serve.url} reused=${serve.reused}`);
