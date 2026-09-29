/** 🔍️ S18 §15: why a hub space's index never mounts after the space is clicked — opens `/hub`, signs in, opens the named space
 * through the sweep's own journey and dumps every console line (bounded), the route, the shell's windows and notices.
 * usage: bun s18-probe-space-mount.ts <serveUrl> <en|de> <spaceName> <capture.jsonl> */
import { writeFileSync } from "node:fs";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { USERS } from "../wp-c11/c11-lib.mjs";
import { readShell } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
import { openSweepSpace, type HubDocumentSweepOptions, type HubSweepRow } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🟦️.ts";

const [baseUrl = "http://127.0.0.1:6540/", locale = "en", spaceName = "S18 Sections en", capture = "space-mount.jsonl"] = process.argv.slice(2) as [string, "en" | "de", string, string];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: locale === "de" ? "de-DE" : "en-US" })).newPage();
const started = Date.now();
const lines: string[] = [];
page.on("pageerror", (error) => lines.push(JSON.stringify({ t: Date.now() - started, type: "pageerror", text: String(error).slice(0, 800) })));
page.on("console", (message) => { if (lines.length < 3000) lines.push(JSON.stringify({ t: Date.now() - started, type: message.type(), text: message.text().slice(0, 800) })); });
const user = USERS[0] as { email: string; password: string };
const row = { kindId: "probe", plugin: "space", pass: false, faults: [], notices: [] } as HubSweepRow;
const options = { baseUrl, locale, email: user.email, password: user.password, spaceName, kinds: [], sagaMs: 240_000, puzzleSagaMs: 240_000, reopen: false, cancel: null, profileDir: "", outDir: ".", signal: new AbortController().signal } as HubDocumentSweepOptions;
const verdict: Record<string, unknown> = {};
try {
  verdict.blocked = await openSweepSpace(page, options, row);
  verdict.spaceId = row.spaceId;
  verdict.route = await page.evaluate(() => location.pathname);
  const shell = await readShell(page);
  verdict.windows = shell.windowIds;
  verdict.notices = await page.evaluate(() => [...document.querySelectorAll('[role="status"], [role="alert"], [data-semio-notice]')].map((element) => (element.textContent ?? "").trim().slice(0, 200)).filter(Boolean).slice(0, 12));
  verdict.bodyText = (await page.evaluate(() => document.body.innerText)).slice(0, 1500);
} catch (error) {
  verdict.error = String((error as Error)?.stack ?? error).slice(0, 600);
} finally {
  verdict.ms = Date.now() - started;
  verdict.consoleLines = lines.length;
  writeFileSync(capture, `${lines.join("\n")}\n`);
  console.log("VERDICT", JSON.stringify(verdict));
  await browser.close();
}
