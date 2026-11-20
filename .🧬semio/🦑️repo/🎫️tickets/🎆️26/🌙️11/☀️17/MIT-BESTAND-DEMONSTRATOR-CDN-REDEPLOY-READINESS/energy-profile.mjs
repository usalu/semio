/** 🔬️ Records the worker's hot WebAssembly frames while the Energy opening is pending. */
import { createRequire } from "node:module";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { createWriteStream } from "node:fs";
import { once } from "node:events";

const root = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const require = createRequire(join(root, "♻️mit-bestand", "🧺️demonstrator", "package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(root, ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
const { chromium } = require("@playwright/test");
const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  const session = await browser.newBrowserCDPSession();
  await page.goto(process.argv[2]);
  await page.waitForTimeout(2500);
  await session.send("Tracing.start", {
    categories: "disabled-by-default-v8.cpu_profiler,disabled-by-default-v8.cpu_profiler.hires",
    transferMode: "ReturnAsStream",
  });
  await page.waitForTimeout(2500);
  const completion = new Promise((resolve) => session.once("Tracing.tracingComplete", resolve));
  await session.send("Tracing.end");
  const { stream } = await completion;
  const output = createWriteStream(process.argv[3]);
  for (;;) {
    const { data, base64Encoded, eof } = await session.send("IO.read", { handle: stream });
    if (!output.write(base64Encoded ? Buffer.from(data, "base64") : data)) await once(output, "drain");
    if (eof) break;
  }
  output.end();
  await once(output, "finish");
  await session.send("IO.close", { handle: stream });
} finally {
  await browser.close();
}
