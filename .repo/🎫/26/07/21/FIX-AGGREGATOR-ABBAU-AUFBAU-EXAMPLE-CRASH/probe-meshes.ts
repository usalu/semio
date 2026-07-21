import { chromium } from "playwright";

const url = "http://127.0.0.1:6023/";
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage();
const failed: string[] = [];
const logs: string[] = [];
page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));
page.on("response", async (res) => {
  if (res.status() >= 400) failed.push(`${res.status()} ${res.request().method()} ${res.url()}`);
});
page.on("requestfailed", (req) => failed.push(`FAIL ${req.url()} ${req.failure()?.errorText}`));
page.on("crash", () => console.log("[CRASH]"));

await page.goto(url, { waitUntil: "networkidle", timeout: 180_000 });
await page.waitForTimeout(6000);

// Hook into React fiber / global is hard; instead fetch example JSON and check mesh URLs
const meshChecks = await page.evaluate(async () => {
  const meshes = [
    "/mesh/hexagonal-cut-concrete-forest-left.glb",
    "/mesh/hexagonal-cut-concrete-forest-right.glb",
    "/mesh/placeholder.glb",
  ];
  const results = [];
  for (const m of meshes) {
    const res = await fetch(m);
    results.push({ m, status: res.status, len: (await res.arrayBuffer()).byteLength });
  }
  return results;
});
console.log(`[DEBUG] meshChecks=${JSON.stringify(meshChecks)}`);
console.log(`[DEBUG] failed=${JSON.stringify(failed, null, 2)}`);
console.log(`[DEBUG] logs=${JSON.stringify(logs.filter((l) => /error|Error|DEBUG|warn|mesh|gltf|GLTF|instance/i.test(l)), null, 2)}`);

// Try to read performance memory and count three.js via canvas content
const pixels = await page.evaluate(() => {
  const canvas = document.querySelector("canvas") as HTMLCanvasElement | null;
  if (!canvas) return null;
  // Force a readback via 2d by drawing — webgl canvas toDataURL
  try {
    const data = canvas.toDataURL("image/png");
    // decode roughly
    return { w: canvas.width, h: canvas.height, dataUrlLen: data.length };
  } catch (e) {
    return { error: String(e) };
  }
});
console.log(`[DEBUG] pixels=${JSON.stringify(pixels)}`);

await browser.close();
