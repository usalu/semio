import { chromium } from "playwright";
/** 🔎️ Probes the 6018 page after load: failed requests, artifact panel and live regions. */
const b = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const p = await b.newPage({ viewport: { width: 1440, height: 900 } });
p.on("console", m => /error|warn/.test(m.type()) && console.log(m.type(), m.text().slice(0, 300)));
p.on("response", r => r.status() >= 400 && console.log("http", r.status(), r.url()));
await p.goto("http://127.0.0.1:6018/?plugin=generation3d");
await p.waitForTimeout(60000);
await p.getByText("Artifact", { exact: true }).first().click().catch(e => console.log("click", e.message.slice(0, 100)));
await p.waitForTimeout(3000);
await p.screenshot({ path: "🗑️generated/react-native-smoke/artifact.png" });
console.log(await p.evaluate(() => [...document.querySelectorAll("[aria-live],[role=status],[role=tree],[role=treeitem],[role=listitem]")].map(e => e.textContent.trim().slice(0, 120)).slice(0, 40)));
await b.close();
