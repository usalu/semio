/** 🩺️ Which half of the wgpu panel accessibility path works (slice WGr).
 *
 * The hub workspace opens from the footer pill, so `AccessibilityEvent::Activate` reaches the SHELL
 * CHROME window. This isolates the two remaining questions: does `Activate` reach a PANEL-document
 * window (`framework.hub`), and does `Value` (typing into a mirrored textbox) reach it at all —
 * measured by the node's own `valueText` in the next accessibility projection.
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const outDir = join(here, "🗑️generated");
mkdirSync(outDir, { recursive: true });
const out = (name) => join(outDir, `wgr-live-${name}`);
const MIRROR = "#semio-wgpu-accessibility";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const lines = [];
page.on("console", (m) => lines.push(`${m.type()} ${m.text()}`));
page.on("pageerror", (e) => lines.push(`pageerror ${String(e)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 90_000 });
await page.waitForTimeout(6000);

const projection = async () => {
  const raw = await page.evaluate(async () => (await globalThis.semioWgpuIntrospection.dumpAccessibility()) ?? "");
  try {
    return (JSON.parse(raw).windows ?? []).flatMap((s) => (s.nodes ?? []).map((n) => ({ ...n, windowId: s.windowId })));
  } catch {
    return [];
  }
};
const hit = async (key) =>
  page.evaluate(
    ({ selector, nodeKey }) => {
      const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
      if (!element) return "absent";
      if (element.disabled === true) return "disabled";
      element.focus();
      element.dispatchEvent(new MouseEvent("click", { bubbles: true }));
      return "clicked";
    },
    { selector: MIRROR, nodeKey: key },
  );
const put = async (key, text) =>
  page.evaluate(
    ({ selector, nodeKey, value }) => {
      const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
      if (!(element instanceof HTMLInputElement)) return "absent";
      element.focus();
      element.value = value;
      element.dispatchEvent(new Event("input", { bubbles: true }));
      return "typed";
    },
    { selector: MIRROR, nodeKey: key, value: text },
  );

const report = {};
report.openPill = await hit("framework.hub.signIn");
await page.waitForTimeout(4000);
const opened = await projection();
report.workspaceOpen = opened.some((n) => String(n.key).endsWith("framework.hub.address"));

report.focusThenType = await put("framework.hub/framework.hub.address", "http://127.0.0.1:7501");
await page.waitForTimeout(2500);
const afterType = await projection();
const address = afterType.find((n) => String(n.key).endsWith("framework.hub.address"));
report.addressValueText = address?.valueText ?? null;
report.addButtonDisabled = afterType.find((n) => String(n.key).endsWith("sign-in.add"))?.disabled ?? null;

// 🔁️ Does a hub verb that definitely dispatches force the panel to republish? If the add button
// becomes enabled only after this, the draft reached `ShellState` and the missing half is the
// republish; if it stays disabled, the `SET_ADDRESS` action never reached the hub lane at all.
report.selectConnection = await hit("framework.hub/framework.hub.sign-in.connection.local-bootstrap");
await page.waitForTimeout(3000);
const afterSelect = await projection();
report.addButtonDisabledAfterVerb = afterSelect.find((n) => String(n.key).endsWith("sign-in.add"))?.disabled ?? null;
report.addressValueTextAfterVerb = afterSelect.find((n) => String(n.key).endsWith("framework.hub.address"))?.valueText ?? null;

// ⌨️ The commit path a keyboard user actually uses: Enter inside the textbox.
await put("framework.hub/framework.hub.address", "http://127.0.0.1:7501");
await page.evaluate(
  ({ selector, nodeKey }) => {
    const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
    element?.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true }));
    element?.dispatchEvent(new Event("change", { bubbles: true }));
    element?.blur();
  },
  { selector: MIRROR, nodeKey: "framework.hub/framework.hub.address" },
);
await page.waitForTimeout(3000);
const afterCommit = await projection();
report.addButtonDisabledAfterCommit = afterCommit.find((n) => String(n.key).endsWith("sign-in.add"))?.disabled ?? null;
report.connectionsAfterCommit = afterCommit.filter((n) => String(n.key).includes("sign-in.connection")).map((n) => n.label);

report.panelActivate = await hit("framework.hub/framework.hub.close");
await page.waitForTimeout(3000);
const afterClose = await projection();
report.workspaceStillOpen = afterClose.some((n) => String(n.key).endsWith("framework.hub.address"));

report.consoleErrors = lines.filter((l) => l.startsWith("error") || l.startsWith("pageerror")).slice(0, 10);
writeFileSync(out("input-diagnose.json"), JSON.stringify(report, null, 2));
writeFileSync(out("input-diagnose-console.txt"), lines.join("\n"));
console.log(JSON.stringify(report, null, 2));
await browser.close();
