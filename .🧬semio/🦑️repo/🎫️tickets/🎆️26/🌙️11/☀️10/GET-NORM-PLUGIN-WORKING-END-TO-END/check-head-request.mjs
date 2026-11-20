import { createRequire } from "node:module";
import { createServer } from "node:http";

const { chromium } = createRequire(process.cwd() + "/package.json")("playwright");
const server = createServer((req, res) => {
  res.setHeader("Content-Type", req.url === "/" ? "text/html" : "application/json");
  res.setHeader("Cache-Control", "no-cache");
  const body = req.url === "/" ? "<!doctype html><title>Request test</title>" : '{"descriptor":true}';
  if (req.url !== "/no-length") res.setHeader("Content-Length", Buffer.byteLength(body));
  res.end(req.method === "HEAD" ? undefined : body);
});
await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const mode = process.argv[2];
const browser = await chromium.launch({ args: ["--in-process-gpu"] });
try {
  const page = await browser.newPage();
  const requests = new Map();
  const tunnel = await page.context().newCDPSession(page);
  await tunnel.send("Network.enable");
  tunnel.on("Network.requestWillBeSent", (row) => requests.set(row.requestId, row.request.method + " " + row.request.url));
  for (const name of ["loadingFailed", "loadingFinished"]) {
    tunnel.on("Network." + name, (row) => console.log("[DEBUG]", name, requests.get(row.requestId), row.errorText ?? ""));
  }
  await page.goto(mode === "play" ? "http://127.0.0.1:6033/@vite/client" : "http://127.0.0.1:" + server.address().port);
  const results = await page.evaluate(async (mode) => {
    const logs = [];
    const paths = mode === "play" ? ["/🔌️plugin-modules/📕️norm/🔣️.json"] : ["/no-length", "/with-length"];
    for (const path of paths) {
      for (const consume of [false, true]) {
        const headers = await fetch(path, { method: "HEAD", cache: "no-store", signal: new AbortController().signal });
        const bytes = consume ? (await headers.arrayBuffer()).byteLength : undefined;
        const full = await fetch(path);
        logs.push({ path, consume, status: headers.status, hasBody: headers.body !== null, bytes, read: (await full.arrayBuffer()).byteLength });
      }
    }
    return logs;
  }, mode);
  console.log("[DEBUG]", JSON.stringify(results));
} finally {
  await browser.close();
  server.close();
}
