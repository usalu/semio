import {createRequire} from "node:module";
import {resolve} from "node:path";
import {readFile} from "node:fs/promises";

const require = createRequire(resolve("🏢️semio-tech/🎡️play/package.json"));
const {chromium} = require("playwright");
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6034/";
const families = [
  ["norm", ["din4108", "din16798", "iso16757", "vdi3805"]],
  ["norm-energy", ["din18599"]],
  ["norm-eurocode", ["en1990", "en1991", "en1997", "en1998"]],
  ["norm-eurocode-structural", ["en1992", "en1993", "en1994"]],
  ["norm-eurocode-materials", ["en1995", "en1996", "en1999"]],
];
const inventory = await readFile(resolve("✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts"), "utf8");
const directories = new Map(Array.from(inventory.matchAll(/pluginId: "([^"]+)", directoryName: parseInstallationDirectoryV1\("([^"]+)"\)/gu),
  ([, id, directory]) => [id, directory]));
const browser = await chromium.launch({args: ["--enable-unsafe-swiftshader", "--in-process-gpu"]});
try {
  for (const [family, apps] of families) {
    const directory = directories.get(family);
    if (!directory) throw Error("Missing module directory for " + family);
    const prefix = "/🔌️plugin-modules/" + directory + "/";
    const receiptUrl = new URL(prefix + "🧾️receipt.json", baseURL);
    const receipt = await fetch(receiptUrl, {
      headers: {Accept: "application/json"},
      signal: AbortSignal.timeout(5000),
    });
    const mime = receipt.headers.get("content-type");
    const parsed = mime?.includes("application/json") && await receipt.json().catch(() => undefined);
    if (!parsed) await receipt.body?.cancel();
    const delivery = {receiptHttp: receipt.status, receiptType: mime, receivedSignedReceipt: parsed?.pluginId === family && /^[0-9a-f]{64}$/u.test(parsed.wasmSha256)};
    console.log("[DEBUG] receipt", JSON.stringify({family, directory, ...delivery}));
    const routes = await Promise.all(apps.map(async id => {
      const page = await browser.newPage();
      const errors = new Set();
      const requests = new Map();
      page.on("response", response => {
        const path = decodeURI(new URL(response.url()).pathname);
        if (!path.startsWith("/🔌️plugin-modules/") && !path.startsWith("/🧩️extension-modules/") && !path.startsWith("/♻️activation/")) return;
        requests.set(path, response.status());
      });
      page.on("pageerror", error => errors.add(String(error).slice(0, 400)));
      page.on("console", event => {
        if (/boot failed|plugin.*(?:manifest|receipt).*fail/i.test(event.text()))
          errors.add(event.text().slice(0, 400));
      });
      try {
        const response = await page.goto(new URL("#" + id, baseURL).href, {
          waitUntil: "domcontentloaded",
          timeout: 10000,
        });
        await page.waitForTimeout(2200);
        const title = await page.title();
        const body = await page.locator("body").innerText({timeout: 4000});
        return {id, entryHttp: response?.status(), ...delivery, title,
          text: body.slice(0, 240), errors: [...errors], requests: [...requests]};
      } catch (error) {
        return {id, ...delivery, error: String(error).slice(0, 400)};
      } finally {
        await page.close();
      }
    }));
    for (const route of routes) console.log("[DEBUG]", JSON.stringify(route));
  }
} finally {
  await browser.close();
}
