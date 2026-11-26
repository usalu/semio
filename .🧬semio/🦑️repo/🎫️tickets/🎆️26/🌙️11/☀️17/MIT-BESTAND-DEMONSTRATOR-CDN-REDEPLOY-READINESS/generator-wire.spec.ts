import { createRequire } from "node:module";
import { traceShardWire } from "./generator-command-probe.cjs";
import "../../../../../../../♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts";

const app = new URL("../../../../../../../♻️mit-bestand/🧺️demonstrator/package.json", import.meta.url);
const { test } = createRequire(app)("@playwright/test") as typeof import("@playwright/test");

test.beforeEach(async ({ page }) => {
  page.on("console", message => {
    const line = message.text();
    if (line.startsWith("[DEBUG] generator extension wire")) process.stdout.write(line + "\n");
  });
  await page.addInitScript(traceShardWire, { traceHostAlways: true, traceHost: true });
});
