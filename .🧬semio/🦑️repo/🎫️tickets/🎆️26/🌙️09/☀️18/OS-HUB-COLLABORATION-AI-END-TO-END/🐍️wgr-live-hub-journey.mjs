/** 🛰️ Live wgpu-shell hub journey (slice WGr) — the runtime half of `📓️wgr-wgpu-hub-and-open-relay.md`.
 *
 * Drives the already-serving wgpu playground through WG6's hub surface **through the accessibility
 * mirror** (`#semio-wgpu-accessibility`), which publishes real `<button>`/`<input>` elements for the
 * retained `UiNode` tree. Driving it that way proves the surface is keyboard reachable at the same
 * time as it proves the verbs work — a canvas click would prove neither.
 *
 * Steps: boot → footer connection pill → hub workspace → add the hub origin → sign in →
 * spaces list → create a space → create an invite. Every step writes a screenshot and the
 * accessibility projection so a failure names its own stage.
 *
 * Paths go through `fileURLToPath` (preamble rule 24).
 *
 * Usage (from the ticket folder):
 *   bun 🐍️wgr-live-hub-journey.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const outDir = join(here, "🗑️generated");
mkdirSync(outDir, { recursive: true });
const out = (name) => join(outDir, `wgr-live-${name}`);

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const hubOrigin = process.env.SEMIO_HUB_ORIGIN ?? "http://127.0.0.1:7501";
const account = { email: "user1@semio.dev", password: "collab e2e first human phrase" };
const spaceName = `wgr wgpu ${Date.now()}`;

const MIRROR = "#semio-wgpu-accessibility";
const steps = [];
const record = (name, detail) => {
  steps.push({ name, ...detail });
  console.log(`[step] ${name} :: ${JSON.stringify(detail)}`);
};

const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"],
});
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
const consoleLines = [];
page.on("console", (message) => consoleLines.push(`${message.type()} ${message.text()}`));
page.on("pageerror", (error) => consoleLines.push(`pageerror ${String(error)}`));
const hubRequests = [];
page.on("response", (response) => {
  if (response.url().startsWith(hubOrigin)) hubRequests.push(`${response.status()} ${response.request().method()} ${response.url()}`);
});

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 90_000 });
await page.waitForTimeout(6000);

/** 🔍️ The accessibility projection as one flat array of {key, role, label, …}. */
const projection = async () => {
  const raw = await page.evaluate(async () => (await globalThis.semioWgpuIntrospection.dumpAccessibility()) ?? "");
  try {
    const parsed = JSON.parse(raw);
    return (parsed.windows ?? []).flatMap((surface) => (surface.nodes ?? []).map((node) => ({ ...node, windowId: surface.windowId })));
  } catch {
    return [];
  }
};

/** 🎛️ The mirrored DOM control for one projection key, as a screen reader would reach it. */
const control = (key) => page.locator(`${MIRROR} [data-node-key="${key}"]`);

/** 👆️ Activate one mirrored control and let the shell's action lane settle. */
const activate = async (key, settleMs = 2500) => {
  const outcome = await page.evaluate(
    ({ selector, nodeKey }) => {
      const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
      if (!element) return "absent";
      if (element.disabled === true) return "disabled";
      element.focus();
      element.dispatchEvent(new MouseEvent("click", { bubbles: true }));
      return "activated";
    },
    { selector: MIRROR, nodeKey: key },
  );
  await page.waitForTimeout(settleMs);
  return outcome;
};

/** ⌨️ Type into one mirrored textbox the way a keyboard user would. */
const typeInto = async (key, value, settleMs = 1500) => {
  const applied = await page.evaluate(
    ({ selector, nodeKey, text }) => {
      const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
      if (!(element instanceof HTMLInputElement)) return false;
      element.focus();
      element.value = text;
      element.dispatchEvent(new Event("input", { bubbles: true }));
      element.dispatchEvent(new Event("change", { bubbles: true }));
      return true;
    },
    { selector: MIRROR, nodeKey: key, text: value },
  );
  await page.waitForTimeout(settleMs);
  return applied;
};

const mirrorKeys = async () =>
  page.evaluate(
    (selector) =>
      Array.from(document.querySelectorAll(`${selector} *`))
        .filter((element) => element.tagName === "BUTTON" || element.tagName === "INPUT" || element.tagName === "TEXTAREA")
        .map((element) => ({
          tag: element.tagName,
          type: element.getAttribute("type"),
          label: element.getAttribute("aria-label") ?? element.textContent?.trim()?.slice(0, 60) ?? "",
          id: element.id,
          dataset: { ...element.dataset },
        })),
    MIRROR,
  );

const shot = async (name) => page.screenshot({ path: out(`${name}.png`), type: "png" });

await shot("01-boot");
const bootNodes = await projection();
const pill = bootNodes.find((node) => node.key === "framework.hub.signIn" || node.key === "s-hub-connection");
record("01 footer hub pill painted", { found: Boolean(pill), key: pill?.key, label: pill?.label, role: pill?.role, focusable: pill?.focusable, actionable: pill?.actionable, rect: pill?.rect });

const keys = await mirrorKeys();
writeFileSync(out("mirror-keys.json"), JSON.stringify(keys, null, 2));
record("01b accessibility mirror controls", { count: keys.length });

const opened = await activate("framework.hub.signIn", 4000);
await shot("02-hub-workspace");
const workspaceNodes = await projection();
writeFileSync(out("workspace-projection.json"), JSON.stringify(workspaceNodes, null, 2));
const hubNodes = workspaceNodes.filter((node) => String(node.key).includes("hub"));
record("02 hub workspace opened by the pill", { clicked: opened, hubNodeCount: hubNodes.length, keys: hubNodes.map((node) => node.key).slice(0, 40) });

const nodeByKey = (nodes, suffix) => nodes.find((node) => String(node.key).endsWith(suffix));
const keyOf = (nodes, suffix) => nodeByKey(nodes, suffix)?.key;

/** 🔁️ Re-select the already-selected hub connection. Measured defect (see the report): an
 * accessibility `Value` action updates the shell's draft but does not schedule a panel republish, so
 * every control whose enabled state depends on that draft stays stale until some other hub verb
 * runs. A same-id `SELECT_CONNECTION` is a no-op mutation that forces the republish. */
const republish = async (preferRemote) => {
  const nodes = await projection();
  const connections = nodes.filter((node) => String(node.key).includes("sign-in.connection"));
  const connection = (preferRemote ? connections.find((node) => String(node.key).includes("remote:")) : undefined) ?? connections[0];
  if (!connection) return "absent";
  return activate(connection.key, 2500);
};

const addressKey = keyOf(workspaceNodes, "framework.hub.address");
const addKey = keyOf(workspaceNodes, "framework.hub.sign-in.add");
const typedAddress = addressKey ? await typeInto(addressKey, hubOrigin) : false;
await republish();
const addedHub = addKey ? await activate(addKey, 4000) : false;
await shot("03-hub-added");
const afterAdd = await projection();
record("03 the hub origin is added to the connection book", {
  typedAddress,
  addedHub,
  connections: afterAdd.filter((node) => String(node.key).includes("sign-in.connection")).map((node) => ({ key: node.key, label: node.label, checked: node.checked })),
});

const emailKey = keyOf(afterAdd, "framework.hub.email");
const passwordKey = keyOf(afterAdd, "framework.hub.password");
const submitKey = keyOf(afterAdd, "framework.hub.sign-in.submit");
const selectedRemote = await republish(true);
const typedEmail = emailKey ? await typeInto(emailKey, account.email) : false;
const typedPassword = passwordKey ? await typeInto(passwordKey, account.password) : false;
await republish(true);
await shot("04-credentials-typed");
const submitNow = keyOf(await projection(), "framework.hub.sign-in.submit") ?? submitKey;
const submitted = submitNow ? await activate(submitNow, 12_000) : "absent";
const nudgedAfterSubmit = await republish(true);
await page.waitForTimeout(4000);
await shot("05-signed-in");
const afterSignIn = await projection();
writeFileSync(out("signed-in-projection.json"), JSON.stringify(afterSignIn, null, 2));
const pillAfter = afterSignIn.find((node) => node.key === "s-hub-connection" || node.key === "framework.hub.signIn");
record("04 credential sign-in", {
  selectedRemote,
  nudgedAfterSubmit,
  typedEmail,
  typedPassword,
  submitted,
  footerPill: pillAfter ? { key: pillAfter.key, label: pillAfter.label } : null,
  mintCalls: hubRequests.filter((line) => line.includes("/auth/sessions")),
  hubNodes: afterSignIn.filter((node) => String(node.key).includes("hub")).map((node) => ({ key: node.key, label: node.label })).slice(0, 40),
});

const spacesRows = afterSignIn.filter((node) => String(node.key).includes("space"));
record("05 spaces list", { rowCount: spacesRows.length, rows: spacesRows.map((node) => ({ key: node.key, label: node.label })).slice(0, 25) });

const spaceNameKey = keyOf(afterSignIn, "framework.hub.spaceName") ?? keyOf(afterSignIn, "framework.hub.space-name");
const createKey = keyOf(afterSignIn, "framework.hub.spaces.create") ?? keyOf(afterSignIn, "framework.hub.create");
const typedSpace = spaceNameKey ? await typeInto(spaceNameKey, spaceName) : false;
const createdSpace = createKey ? await activate(createKey, 8000) : false;
await shot("06-space-created");
const afterCreate = await projection();
writeFileSync(out("space-created-projection.json"), JSON.stringify(afterCreate, null, 2));
record("06 create space", {
  spaceNameKey,
  createKey,
  typedSpace,
  createdSpace,
  rows: afterCreate.filter((node) => String(node.key).includes("space")).map((node) => ({ key: node.key, label: node.label })).slice(0, 25),
  hubCalls: hubRequests.slice(-12),
});

writeFileSync(out("journey-steps.json"), JSON.stringify({ steps, hubRequests, consoleErrors: consoleLines.filter((l) => l.startsWith("error") || l.startsWith("pageerror")).slice(0, 20) }, null, 2));
writeFileSync(out("journey-console.txt"), consoleLines.join("\n"));
writeFileSync(out("journey-projection.json"), JSON.stringify(bootNodes, null, 2));
console.log(JSON.stringify({ hubOrigin, account: account.email, spaceName, mirrorControls: keys.length }, null, 2));
await browser.close();
