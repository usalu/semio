import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium, expect } from "@playwright/test";
import ts from "typescript";

const root = resolve(import.meta.dirname, "../../../../../../../..");
const source = readFileSync(resolve(root, "🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts"), "utf8");
const parsed = ts.createSourceFile("acceptance.ts", source, ts.ScriptTarget.Latest, true);
const names = new Set(["collectErrors", "waitForPaneShellOutcome", "waitForSurfaceSettlement", "readPaintWitnesses"]);
const functions = parsed.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name && names.has(node.name.text)).map((node) => node.getText(parsed)).join("\n");
const code = ts.transpileModule(functions, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const contract = JSON.parse(readFileSync(resolve(root, "🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🧫️fixtures/🔀️roles/🔣️.json"), "utf8"));
const { collectErrors, waitForSurfaceSettlement, readPaintWitnesses } = new Function("expect", "roleContract", "SHELL_READY_TIMEOUT_MS", "MINIMUM_MAIN_REGION_ELEMENTS", "MINIMUM_IMAGE_ASSET_EDGE", `${code}; return { collectErrors, waitForSurfaceSettlement, readPaintWitnesses };`)(expect, contract, 5_000, 10, 8);
const server = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: () => new Response('<html><body><div data-shell-id="probe" data-shell-ready="probe"><div id="playground.navbar.roles" aria-busy="true"><button>Editor</button><button>Viewer</button></div></div><div data-layered-pane="probe"><div data-surface-id="retained" data-meshes-json="[{&quot;id&quot;:&quot;old&quot;}]"></div></div></body></html>', { headers: { "Content-Type": "text/html" } }) });
const browser = await chromium.launch();
try {
  const page = await browser.newPage();
  await page.route("**/abort-*", (route) => route.abort("failed"));
  await page.goto(server.url.toString());
  const errors = collectErrors(page);
  let settled = false;
  const pending = waitForSurfaceSettlement(page, { variant: "probe" }).then(() => { settled = true; });
  const firstFailure = page.waitForEvent("requestfailed");
  await page.evaluate(async () => { await fetch("/abort-before-stop").catch(() => undefined); });
  const failed = await firstFailure;
  expect(errors.resourceErrors).toHaveLength(1);
  expect(errors.resourceErrors[0]).toContain(failed.failure()!.errorText);
  expect(errors.resourceErrors[0]).toContain(failed.url());
  expect(settled).toBe(false);
  await page.evaluate(() => {
    document.querySelector('[data-surface-id="retained"]')!.setAttribute("data-meshes-json", '[{"id":"new"}]');
    document.getElementById("playground.navbar.roles")!.removeAttribute("aria-busy");
  });
  await pending;
  expect(settled).toBe(true);
  expect(await page.locator('[data-surface-id="retained"]').getAttribute("data-meshes-json")).toBe('[{"id":"new"}]');
  errors.stop();
  const ignoredFailure = page.waitForEvent("requestfailed");
  await page.evaluate(async () => { await fetch("/abort-after-stop").catch(() => undefined); });
  await ignoredFailure;
  expect(errors.resourceErrors).toHaveLength(1);
  for (const row of contract.domPaintCases) {
    await page.setContent(`<div data-layered-pane="probe">${row.markup}</div>`);
    const witnesses = await readPaintWitnesses(page, "probe");
    console.log(`[DEBUG] DOM paint ${row.id}: ${JSON.stringify(witnesses)}`);
    expect(witnesses.some((witness: any) => witness.painted), row.id).toBe(row.painted);
  }
  await page.close();
  expect(errors.resourceErrors).toHaveLength(1);
  console.log("[DEBUG] Actual Playwright requestfailed captured net::ERR_FAILED before stop; ignored after stop and page close.");
  console.log("[DEBUG] Actual Playwright settlement stayed pending while role group aria-busy=true and resolved after updated retained surface + cleared busy.");
} finally {
  await browser.close();
  await server.stop(true);
}
