import { createRequire } from "node:module";

const { chromium } = createRequire(process.cwd() + "/package.json")("playwright");
const browser = await chromium.launch({ args: ["--enable-unsafe-swiftshader", "--in-process-gpu"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
const print = (event, detail) => console.log("[DEBUG]", Date.now(), event, JSON.stringify(detail));
const tracked = (url) => decodeURI(url).includes("/🔌️plugin-modules/") && decodeURI(url).endsWith("/🔣️.json");

await page.addInitScript(() => {
  const original = window.fetch;
  const isDescriptor = (value) => decodeURI(value).endsWith("/🔣️.json");
  window.fetch = async (input, options) => {
    const url = input instanceof Request ? input.url : String(input);
    if (!isDescriptor(url)) return original(input, options);
    const method = options?.method ?? (input instanceof Request ? input.method : "GET");
    const signal = options?.signal ?? (input instanceof Request ? input.signal : undefined);
    const print = (event, detail) => console.log("manifest-network", event, url, method, detail);
    const caller = new Error().stack;
    print("start", caller);
    signal?.addEventListener("abort", () => print("signal", new Error().stack), { once: true });
    try {
      const response = await original(input, options);
      print("response", response.status);
      return response;
    } catch (error) {
      print("reject", String(error));
      throw error;
    }
  };
});
page.on("console", (row) => {
  if (row.text().startsWith("manifest-network")) print("page", row.text());
});
const tunnel = await page.context().newCDPSession(page);
await tunnel.send("Network.enable");
const requests = new Set();
tunnel.on("Network.requestWillBeSent", (row) => {
  if (tracked(row.request.url)) {
    requests.add(row.requestId);
    print("request", { id: row.requestId, method: row.request.method, initiator: row.initiator });
  }
});
for (const name of ["responseReceived", "loadingFailed", "loadingFinished"]) {
  tunnel.on("Network." + name, (row) => {
    if (requests.has(row.requestId)) print(name, { id: row.requestId, status: row.response?.status, error: row.errorText, canceled: row.canceled });
  });
}
try {
  const app = process.argv[2] ?? "din4108";
  await page.goto("http://127.0.0.1:6033/#" + app);
  const english = page.getByRole("button", { name: "English", exact: true });
  if (await english.isVisible()) await english.click();
  await page.locator('[data-shell-ready="' + app + '"]').first().waitFor({ timeout: 75_000 }).catch(() => {});
  await page.waitForTimeout(3_000);
} finally {
  await browser.close();
}
