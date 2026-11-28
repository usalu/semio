import { test, expect } from "bun:test";
import { existsSync, readdirSync } from "node:fs";
import { hostname, homedir } from "node:os";
import { join } from "node:path";

test("worker facts", async () => {
  const root = process.cwd();
  const pw = [join(homedir(), "Library/Caches/ms-playwright"), join(root, ".cache/f/tools/ms-playwright")].map(p => [p, existsSync(p) ? readdirSync(p).join(",") : null]);
  let reach = "none";
  try { const r = await fetch("http://192.168.178.70:8981/", { signal: AbortSignal.timeout(3000) }); reach = String(r.status); } catch (e) { reach = String(e).slice(0, 120); }
  const facts = { host: hostname(), cwd: root, pw, playwright: existsSync(join(root, "node_modules/playwright/package.json")), distServices: existsSync(join(root, "🏢️semio-tech/🎡️play/dist")), env: Object.keys(process.env).filter(k => /SEMIO|PLAYWRIGHT/.test(k)).sort(), reach };
  console.log("[DEBUG] FACTS " + JSON.stringify(facts));
  expect(facts.host.length).toBeGreaterThan(0);
});
