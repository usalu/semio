import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const url = process.env.AGGREGATOR_URL ?? "http://127.0.0.1:6023/";
const outDir = new URL(".", import.meta.url).pathname;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const workerMsgs = [];

page.on("worker", (worker) => {
  worker.on("console", (msg) => {
    workerMsgs.push({ worker: worker.url(), type: msg.type(), text: msg.text().slice(0, 1000) });
  });
});

await page.addInitScript(() => {
  const OrigWorker = window.Worker;
  window.Worker = class extends OrigWorker {
    constructor(url, opts) {
      super(url, opts);
      const origPost = this.postMessage.bind(this);
      this.postMessage = (data, ...rest) => {
        try {
          const type = data && data.type;
          if (type && type !== "init") {
            (window.__workerPosts ??= []).push({ t: performance.now(), data: JSON.parse(JSON.stringify(data, (_, v) => (typeof v === "string" && v.length > 400 ? v.slice(0, 400) + "…" : v))) });
          }
        } catch {
          (window.__workerPosts ??= []).push({ t: performance.now(), raw: String(data).slice(0, 200) });
        }
        return origPost(data, ...rest);
      };
      this.addEventListener("message", (ev) => {
        const d = ev.data;
        if (d && (d.type === "error" || d.error || (d.type && /error|fail/i.test(String(d.type))))) {
          (window.__workerReplies ??= []).push({ t: performance.now(), data: d });
        }
      });
    }
  };
});

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForTimeout(4000);

const report = await page.evaluate(() => ({
  posts: window.__workerPosts ?? [],
  replies: window.__workerReplies ?? [],
}));
report.workerMsgs = workerMsgs;

writeFileSync(`${outDir}probe-worker.json`, JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
