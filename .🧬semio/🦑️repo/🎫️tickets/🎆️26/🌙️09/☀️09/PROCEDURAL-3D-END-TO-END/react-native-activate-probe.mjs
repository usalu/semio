import { chromium } from "playwright";
/** 🧭️ Calls the React on-demand activation door for brep and math on the 6018 page and reports the replies and the page's failed requests. */
const b = await chromium.launch({ headless: true });
const p = await b.newPage({ viewport: { width: 1440, height: 900 } });
p.on("console", m => /error|warn/.test(m.type()) && console.log(m.type(), m.text().slice(0, 300)));
p.on("response", r => (r.status() >= 400 || /extension|modules/.test(r.url())) && console.log("http", r.status(), r.url().slice(0, 160)));
await p.goto("http://127.0.0.1:6018/?plugin=generation3d");
await p.waitForTimeout(25000);
console.log("door", await p.evaluate(() => typeof window.semioReactActivateOnDemand));
for (const r of [{ kind: "node-kind", nodeKind: "brep.curve.polygon" }, { kind: "plugin", pluginId: "flow-extension-brep" }, { kind: "event", event: "on-node-kind:brep" }, { kind: "tag", tag: "flow.extension" }]) console.log(JSON.stringify(r), await p.evaluate(q => window.semioReactActivateOnDemand?.(JSON.stringify(q)), r).catch(e => String(e)));
await p.waitForTimeout(20000);
await p.screenshot({ path: "🗑️generated/react-native-smoke/after-activate.png" });
await b.close();
