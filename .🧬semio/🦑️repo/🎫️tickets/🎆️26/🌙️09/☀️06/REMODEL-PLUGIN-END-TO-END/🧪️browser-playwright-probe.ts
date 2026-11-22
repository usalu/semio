import assert from "node:assert/strict";
import { chromium } from "playwright";
import { openedUiOwnerV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🧩️opened/🔒️adapter/🟦️.ts";

const browser = await chromium.launch({ headless: true, timeout: 8_000 });
try {
  const page = await browser.newPage();
  await page.setContent(`<button id="import">Import</button><output id="result">waiting</output><script>
    document.querySelector("#import").onclick = () => {
      const file = document.createElement("input");
      file.type = "file";
      file.multiple = true;
      file.oncancel = event => document.querySelector("#result").textContent = JSON.stringify({ trusted: event.isTrusted, count: file.files.length });
      file.onchange = () => document.querySelector("#result").textContent = "selected";
      file.click();
    };
  </script>`);
  const owner = openedUiOwnerV1(page, new AbortController().signal);
  try {
    assert.ok(owner.ui.cancelFilePicker);
    await owner.ui.cancelFilePicker("#import");
    await page.waitForFunction(() => document.querySelector("#result")?.textContent !== "waiting", undefined, { timeout: 5_000 });
    const value = (await owner.ui.observe("#result", []))[0]?.text;
    assert.deepEqual(JSON.parse(value ?? ""), { trusted: true, count: 0 });
    console.log("Chromium cancelled the detached file input without a selection.");
  } finally {
    await owner.finish();
  }
} finally {
  await browser.close();
}
