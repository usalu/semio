/** 🔬 Boots the 2D editor through the matrix Ctx (legacy caption) and evaluates a JS snippet file or utility dump. Usage: node m2-probe.mjs <out.txt> [steps...] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { APPS } from "./apps.mjs";
import { Ctx } from "./lib.mjs";
const [out, ...steps] = process.argv.slice(2);
const browser = await chromium.launch();
try {
  const app = { ...APPS["2d"] }; const x = new Ctx(app, browser, { out: "/tmp/m2p", shots: "/tmp/m2p/shots" }); x.legacyCaption = true; await x.init(); await x.boot(120000);
  const lines = [];
  for (const s of steps) {
    if (s === "utils") { await x.utilsOpen(); }
    else if (s.startsWith("util:")) await x.util(s.slice(5));
    else if (s.startsWith("pick:")) { await x.toSummary(); await x.pickAt(app[s.slice(5)]); }
    else if (s.startsWith("wait:")) await x.page.waitForTimeout(Number(s.slice(5)));
    else if (s.startsWith("js:")) lines.push(JSON.stringify(await x.page.evaluate(s.slice(3))));
    else if (s.startsWith("radio:")) await x.radio(s.slice(6));
    else if (s.startsWith("click:")) { await x.hoverClick(...app[s.slice(6)].at); await x.page.waitForTimeout(3500); }
    else if (s === "sel") lines.push(JSON.stringify([await x.selection(), await x.radioChecked()]));
    else if (s === "drag:marquee") { await x.drag(...app.marquee, { n: 6, dt: 150 }); await x.page.waitForTimeout(4000); }
    else if (s === "pressed") lines.push(JSON.stringify(await x.pressedTools()));
    else if (s.startsWith("shot:")) await x.page.screenshot({ path: s.slice(5) });
    else if (s.startsWith("box:")) { await x.tap(x.page.getByRole("checkbox", { name: s.slice(4), exact: true }).first()); await x.page.waitForTimeout(2500); }
    else if (s === "summary") await x.toSummary();
    else if (s === "aria") lines.push(await x.aria());
  }
  writeFileSync(out, lines.join("\n"));
} finally { await browser.close(); }
