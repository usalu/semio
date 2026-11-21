import { chromium } from "playwright";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";

const run = process.env.FEM_BROWSER_RUN || "browser-smoke";
if (!/^[a-z0-9-]+$/.test(run)) throw Error("Invalid browser run name");
const out = join(import.meta.dirname, "🗑️generated", run);
await mkdir(out, { recursive: true });
const browser = await chromium.launch({ args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const filters = ["input #", "semantic failure", "receipt-refusal", "owes its enclosing"];
if (process.argv.includes("--results")) {
  filters.push("fem3d live_visual reconcile spawn", "plugin.job-render-binding-capacity", "fem3d.visual-input",
    "fem3d.visual-shell", "fem3d.visual-stale", "retained surface render fault");
}
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
if (process.env.FEM_BROWSER_TRACE === "1") {
  filters.push("maintenance stage=");
  await page.context().route("**/environment.js", async route => {
    const response = await route.fetch();
    const source = await response.text();
    await route.fulfill({ response, body: source.replace("return _env;", 'return [..._env, ["SEMIO_RUNTIME_DIAGNOSTICS","1"]];') });
  });
  await page.context().route(url => decodeURIComponent(url.pathname).endsWith("🟨️shard-worker.js"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const prelude = `for (const key of ["log","debug","info","warn","error"]) {
      const emit = console[key].bind(console);
      console[key] = (...args) => {
        const text = String(args[0]);
        if (!text.includes("[TRACE]") || text.includes("maintenance stage=")) emit(...args);
      };
    }\n`;
    await route.fulfill({ response, body: prelude + source });
  });
}
const diagnostics = Object.fromEntries(filters.map(key => [key, []]));
const verify = process.argv.includes("--verify");
let previousGeometry = "";
const exampleGeometry = {
  boot: { node: 16, element: 16, solid: 1 },
  demo: { node: 16, element: 16, solid: 1 },
  "concrete-forest": { node: 20, element: 20, solid: 0 },
  house: { node: 0, element: 0, solid: 8 },
};
async function captureModel(id, nodeAdded) {
  const scene = await page.waitForFunction(({ previous, nodeAdded, expected }) => {
    const host = document.querySelector('.semio-world-3d-host[data-window-instance-id="fem3d-model"]');
    const canvas = host?.querySelector("canvas");
    if (!canvas || canvas.getBoundingClientRect().width <= 0) return;
    const meshes = host.dataset.meshesJson || "[]";
    const instances = host.dataset.instancesJson || "[]";
    const geometry = JSON.stringify([meshes, instances]);
    const items = JSON.parse(instances);
    if (geometry === previous || !items?.length) return;
    const hasNode = items.some(item => item.interactionGranularityId === "node" && item.position?.join() === "1,2,3");
    if (nodeAdded !== undefined && hasNode !== nodeAdded) return;
    const topology = Object.fromEntries(["node", "element", "solid"].map(kind => [kind, items.filter(item => item.interactionGranularityId === kind).length]));
    if (expected && Object.entries(expected).some(([kind, count]) => topology[kind] !== count)) return;
    return { geometry, topology, surface: host.dataset.surfaceId, window: host.dataset.windowInstanceId };
  }, { previous: previousGeometry, nodeAdded, expected: exampleGeometry[id] }, { polling: 100, timeout: 240000 });
  const result = await scene.jsonValue();
  await scene.dispose();
  if (!result) throw Error("Model did not publish different nonempty geometry for " + id);
  previousGeometry = result.geometry;
  await writeFile(join(out, id + "-geometry.json"), JSON.stringify(result));
  await page.screenshot({path:join(out, id + "-model.png")});
}
page.on("console", message => {
  const text = message.text();
  for (const key of filters) if (text.includes(key) && diagnostics[key].length < 20) {
    diagnostics[key].push(text);
    console.log(text.slice(0, 1000));
  }
});
page.on("requestfailed", request => {
  const error = request.failure()?.errorText;
  if (error !== "net::ERR_ABORTED") console.error(error, decodeURI(request.url()));
});
page.on("response", async response => {
  if (response.status() >= 400) console.error("HTTP", response.status(), decodeURI(response.url()), (await response.text().catch(() => "")).slice(0, 1000));
});
page.on("pageerror", error => console.error(error.message));
const resultsFirst = process.argv.includes("--results-first");
async function captureResults() {
  const panel = page.getByRole("tabpanel", { name: "Results", exact: true });
  const caption = panel.getByText(/^(Case|Lastfall): dead$/).first();
  let lastStage = "";
  const observeStage = async () => {
    const stage = await panel.locator("p").first().textContent({ timeout: 3000 }).catch(() => "");
    if (stage && stage !== lastStage) {
      lastStage = stage;
      console.log("Results stage", stage);
    }
  };
  await observeStage();
  const watching = setInterval(observeStage, 5000);
  try {
    await caption.waitFor({ timeout: 600000 });
  } finally {
    clearInterval(watching);
  }
  const results = panel.locator(".semio-world-3d-host canvas").first();
  await results.waitFor({ state: "visible", timeout: 30000 });
  const geometry = await results.evaluate(canvas => {
    const host = canvas.closest(".semio-world-3d-host");
    return {
      meshes: JSON.parse(host.dataset.meshesJson || "[]").length,
      instances: JSON.parse(host.dataset.instancesJson || "[]").length,
    };
  });
  if (!geometry.meshes || !geometry.instances) throw Error("Completed Results did not draw a populated scene");
  console.log("Completed Results", await caption.innerText(), geometry);
  await writeFile(join(out, "completed-results.txt"), await page.locator("body").ariaSnapshot());
  await page.screenshot({path:join(out, "completed-results.png")});
}
try {
  await page.goto(process.env.FEM_BROWSER_URL || "http://127.0.0.1:6087", { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.getByRole("combobox", { name: "Example" }).waitFor({ timeout: 180000 });
  await page.waitForFunction(() => [...document.querySelectorAll('[role="alert"], canvas')].some(element => element.tagName === "CANVAS" || /refused|failed/i.test(element.textContent)), undefined, { timeout: 120000 }).catch(() => {});
  await writeFile(join(out, "boot.txt"), await page.locator("body").ariaSnapshot());
  await page.screenshot({path:join(out, "boot.png")});
  console.log("Boot view saved.");
  if (verify) {
    const alert = await page.getByRole("alert").allTextContents();
    if (alert.some(text => /refused|failed/i.test(text))) throw Error(alert.join(" "));
    await captureModel("boot");
    console.log("Demo Model rendered.");
    if (resultsFirst) await captureResults();
  }
  if ((verify && !process.argv.includes("--demo")) || process.argv.includes("--switch")) {
    const picker = page.getByRole("combobox", { name: "Example" });
    for (const [id, label] of [["concrete-forest", "Concrete Forest"], ["house", "House"], ["demo", "Demo"]]) {
      await picker.click();
      const options = page.getByRole("option");
      console.log("Example choices", await options.allTextContents());
      const choice = options.filter({ hasText: label });
      await choice.click({ timeout: 30000 });
      console.log("Requested", label);
      await page.waitForFunction(({ label, initial }) => {
        const picked = document.querySelector('[role="combobox"]')?.textContent || "";
        const alerts = Array.from(document.querySelectorAll('[role="alert"],[role="status"]')).map(n=>n.textContent).join(" ");
        return picked.includes(label) || /refused|could not be delivered|failed/i.test(alerts) && alerts !== initial;
      }, { label, initial: "" }, { timeout: 240000 }).catch(() => {});
      await writeFile(join(out, id + ".txt"), await page.locator("body").ariaSnapshot());
      console.log(id, "selector", await picker.innerText());
      if (verify) {
        if (!(await picker.innerText()).includes(label)) throw Error("Example selection failed for " + label);
        await captureModel(id);
      }
    }
  }
  if (process.argv.includes("--edit")) {
    const model = page.getByRole("tabpanel", { name: "Model", exact: true });
    const toggle = model.getByRole("button", { name: /^Actions Click/ });
    const add = model.getByRole("button", { name: /^Add Node/ });
    await toggle.click();
    await add.click();
    await writeFile(join(out, "add-node-form.txt"), await page.locator("body").ariaSnapshot());
    for (const [name, value] of [["X", "1"], ["Y", "2"], ["Z", "3"]]) {
      await model.getByRole("treeitem", { name, exact: true }).getByRole("spinbutton").fill(value, { timeout: 30000 });
    }
    await model.getByRole("button", { name: "Execute", exact: true }).click({ timeout: 30000 });
    await captureModel("add-node", true);
    console.log("Add Node applied at (1, 2, 3).");
    for (const [label, present] of [["Undo", false], ["Redo", true]]) {
      const action = model.getByRole("button", { name: label, exact: true });
      if (!await action.isVisible()) await toggle.click();
      await action.click();
      await captureModel(label.toLowerCase(), present);
      console.log(label, "applied.");
    }
  }
  if (verify && process.argv.includes("--results") && !resultsFirst) await captureResults();
  if (process.argv.includes("--results")) {
    await page.getByRole("button", { name: "Results", exact: true }).click();
    await writeFile(join(out, "results-pane.txt"), await page.locator("body").ariaSnapshot());
  }
  if (process.argv.includes("--commands")) {
    await page.getByRole("tabpanel", { name: "Model", exact: true }).getByRole("button", { name: /^Actions Click/ }).click();
    await writeFile(join(out, "model-actions.txt"), await page.locator("body").ariaSnapshot());
    await page.getByRole("button", { name: "Command", exact: true }).click();
    await page.getByRole("button", { name: "Artifact", exact: true }).last().click();
    await writeFile(join(out, "commands.txt"), await page.locator("body").ariaSnapshot());
  }
} catch (error) {
  await writeFile(join(out, "failure.txt"), await page.locator("body").ariaSnapshot().catch(() => ""));
  await page.screenshot({ path: join(out, "failure.png"), timeout: 15000 }).catch(() => {});
  const scenes = await page.locator(".semio-world-3d-host").evaluateAll(hosts => hosts.map(host => ({
    surface: host.dataset.surfaceId,
    window: host.dataset.windowInstanceId,
    canvases: host.querySelectorAll("canvas").length,
    meshes: host.dataset.meshesJson?.length,
    instances: host.dataset.instancesJson?.length,
  }))).catch(() => []);
  await writeFile(join(out, "failure-scenes.json"), JSON.stringify(scenes, null, 2));
  throw error;
} finally {
  await writeFile(join(out, "console.json"), JSON.stringify(diagnostics, null, 2));
  await browser.close();
}
