import { chromium } from "@playwright/test";

const url = process.env.DAG_PLAY_URL ?? "http://127.0.0.1:6017/";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => console.log("console:", msg.type(), msg.text()));
page.on("pageerror", (err) => console.log("pageerror:", err.message));
await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(5000);
console.log("body:", await page.locator("body").innerHTML());
await browser.close();
