import { chromium, expect } from "./browser-tooling";
import { readGeneratedCatalogProjection } from "@semio-tech/plugin-registry/catalog-view";
import { buildPlaygroundSession } from "@semio-tech/plugin-registry/playground/session";
import { fileURLToPath } from "node:url";
import fixture from "./browser-import-fixture.json";

const target = new URL(process.argv[2] ?? "http://127.0.0.1:6022/");
target.searchParams.set("example", "demo");
target.searchParams.set("role", "editor");
const session = buildPlaygroundSession("process3d", readGeneratedCatalogProjection("✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"));
const componentBridges = new Map(session.plugins.map(plugin => [new URL(plugin.moduleUrl, target).pathname, plugin.pluginId]));
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  page.setDefaultTimeout(20000);
  const treeRow = (key: string) => page.locator(`[id$="/${key}"]`);
  const faults: string[] = [];
  const delivered = new Set<string>();
  page.on("pageerror", error => faults.push(error.message));
  page.on("console", message => {
    if (message.type() === "error" && /surface-render|faults=|react boot failed/i.test(message.text())) faults.push(message.text());
  });
  page.on("response", response => {
    const pluginId = componentBridges.get(new URL(response.url()).pathname);
    if (pluginId && response.ok()) delivered.add(pluginId);
  });
  await page.goto(target.href);
  const drill = page.getByText("Drill Dowel Hole", { exact: true });
  await expect(drill.first()).toBeVisible({ timeout: 90000 });
  await drill.first().click();
  const radius = page.getByRole("spinbutton", { name: /^Radius$/i });
  if (!(await radius.isVisible())) {
    const inspection = page.getByRole("tab", { name: /^(Inspection|Inspector)$/i });
    if (await inspection.isVisible()) await inspection.click();
  }
  await expect(radius).toBeVisible();
  const original = await radius.inputValue();
  const mesh = page.locator("[data-meshes-json]").first();
  const readProcessMesh = async () => {
    const payload = await mesh.getAttribute("data-meshes-json");
    const row = JSON.parse(payload ?? "[]")[0];
    if (row?.id !== "processed" || !row.data?.positions?.length || !row.data?.indices?.length) return null;
    return JSON.stringify({ positions: row.data.positions, indices: row.data.indices });
  };
  const expectChangedMesh = async (baseline: string | null) => {
    await expect.poll(async () => {
      const rendered = await readProcessMesh();
      return rendered !== null && rendered !== baseline;
    }).toBe(true);
  };
  await expect.poll(readProcessMesh).not.toBeNull();
  await expect(mesh.locator("canvas").first()).toBeVisible();
  const previous = await readProcessMesh();
  const readSun = async () => JSON.parse((await mesh.getAttribute("data-sun-json")) ?? "null");
  const priorSun = await readSun();
  expect(typeof priorSun?.enabled).toBe("boolean");
  const windowOptions = page.locator('[data-slot="window-measures-overlay"]').first();
  const sunGroup = windowOptions.getByRole("button", { name: "Sun", exact: true });
  const sunEnabled = windowOptions.getByRole("checkbox", { name: "Enabled", exact: true });
  const optionsButton = windowOptions.locator('button[data-slot="window-pane-chrome-toggle"][title="Window Options"]');
  const openedOptions = !(await sunGroup.isVisible());
  if (openedOptions) await optionsButton.click();
  const openedSun = !(await sunEnabled.isVisible());
  if (openedSun) await sunGroup.click();
  await expect(sunEnabled).toBeVisible();
  await expect(windowOptions).toBeVisible();
  await sunEnabled.click();
  await expect.poll(async () => (await readSun())?.enabled).toBe(!priorSun.enabled);
  await expect.poll(readProcessMesh).toBe(previous);
  await sunEnabled.click();
  await expect.poll(readSun).toEqual(priorSun);
  if (openedSun) await sunGroup.click();
  if (openedOptions) await optionsButton.click();
  expect(Number(original)).toBeGreaterThan(0);
  await radius.fill(String(Number(original) * 1.25));
  await radius.press("Tab");
  await expectChangedMesh(previous);
  await radius.fill(original);
  await radius.press("Tab");
  await expect.poll(readProcessMesh).toBe(previous);
  const drillEnabled = treeRow("dowel-drill").getByRole("button", { name: "Enabled", exact: true });
  await drillEnabled.click();
  await expectChangedMesh(previous);
  await expect(drill.first()).toBeVisible();
  await drillEnabled.click();
  await expect.poll(readProcessMesh).toBe(previous);
  const cursor = page.locator("#process3d-cursor");
  const stepper = page.locator('[data-slot="stepper-group"]').filter({ has: cursor });
  const lastStep = Number(await cursor.inputValue());
  expect(lastStep).toBeGreaterThan(0);
  await stepper.locator('[data-slot="stepper-minus"]').click();
  await expect(cursor).toHaveValue(String(lastStep - 1));
  await expectChangedMesh(previous);
  await stepper.locator('[data-slot="stepper-plus"]').click();
  await expect(cursor).toHaveValue(String(lastStep));
  await expect.poll(readProcessMesh).toBe(previous);
  await cursor.fill("0");
  await cursor.press("Enter");
  await expectChangedMesh(previous);
  const rawStockMesh = await readProcessMesh();
  await page.getByText("Timber Beam", { exact: true }).first().click();
  await expect.poll(async () => {
    const selection = await mesh.getAttribute("data-selection-json");
    return JSON.parse(selection ?? "{}").selectedIds;
  }).toContain("beam");
  const width = page.getByRole("spinbutton", { name: /^Width$/i });
  await expect(width).toBeVisible();
  const originalWidth = await width.inputValue();
  await width.fill(String(Number(originalWidth) * 1.1));
  await width.press("Tab");
  await expectChangedMesh(rawStockMesh);
  await width.fill(originalWidth);
  await width.press("Tab");
  await expect.poll(readProcessMesh).toBe(rawStockMesh);
  await cursor.fill(String(lastStep));
  await cursor.press("Enter");
  await expect.poll(readProcessMesh).toBe(previous);
  await drill.first().click();
  const moveEarlier = page.getByText("Move Earlier", { exact: true });
  await expect(moveEarlier).toBeVisible();
  await moveEarlier.click();
  const lap = page.getByText("Cut Lap Joint", { exact: true });
  await expect.poll(async () => {
    const drillElement = await drill.first().elementHandle();
    try {
      return await lap.first().evaluate((lapElement, drillElement) => Boolean(drillElement.compareDocumentPosition(lapElement) & Node.DOCUMENT_POSITION_FOLLOWING), drillElement);
    } finally {
      await drillElement.dispose();
    }
  }).toBe(true);
  const installedChopSaw = treeRow("machine:chopSaw");
  const addMachine = async ({ catalog, catalogLabel, machine }: (typeof fixture.workshop)[number]) => {
    const section = treeRow(`process3d-play-workshop.catalog.${catalog}`);
    if (!(await section.isVisible())) {
      await page.getByRole("tab", { name: /^Workshop$/i }).click();
    }
    const installed = treeRow(`machine:${machine}`);
    if (await installed.isVisible()) return;
    const action = treeRow(`process3d-workshop.catalog.${catalog}.${machine}`).locator('[data-slot="tree-label"]');
    if (!(await action.isVisible())) {
      await section.getByText(catalogLabel, { exact: true }).click();
    }
    await action.click();
    await expect(installed).toBeVisible();
  };
  await addMachine(fixture.workshop[0]!);
  await installedChopSaw.getByText("Chop Saw", { exact: true }).click();
  const machineLabel = page.getByRole("textbox", { name: /^Label$/i });
  await expect(machineLabel).toHaveValue("Chop Saw");
  await machineLabel.fill("Shop Chop Saw");
  await machineLabel.press("Tab");
  await expect(installedChopSaw.getByText("Shop Chop Saw", { exact: true })).toBeVisible();
  await expect.poll(() => [...delivered].sort()).toEqual(["process", "process-extension-concrete", "process-extension-metal", "process-extension-robotic", "process-extension-wood"]);

  const concrete = new URL(target);
  concrete.searchParams.set("example", "concrete-forest");
  await page.goto(concrete.href);
  await expect(page.getByText("Wall Saw Slab Cut", { exact: true }).first()).toBeVisible({ timeout: 90000 });
  await expect(cursor).toHaveValue("7");
  await expect.poll(readProcessMesh).not.toBeNull();
  const processedConcrete = await readProcessMesh();
  await cursor.fill("0");
  await cursor.press("Enter");
  await expect(cursor).toHaveValue("0");
  await expectChangedMesh(processedConcrete);
  await expect.poll(async () => {
    const payload = await mesh.getAttribute("data-instances-json");
    return JSON.parse(payload ?? "[]")[0]?.id;
  }).toBe("hexagonal-cut-concrete-forest-left");

  const openStockPicker = async () => {
    const stockSection = treeRow("process3d-play-catalogue.stock");
    if (!(await stockSection.isVisible())) await page.getByRole("tab", { name: /^Catalog(?:ue)?$/i }).click();
    const importStock = treeRow("process3d-catalogue.stock-import").locator('[data-slot="tree-label"]');
    if (!(await importStock.isVisible())) await stockSection.getByText("Stock", { exact: true }).click();
    const [chooser] = await Promise.all([page.waitForEvent("filechooser"), importStock.click()]);
    return chooser;
  };
  const picker = await openStockPicker();
  await picker.setFiles(fileURLToPath(new URL("./browser-stock.obj", import.meta.url)));
  await expect(page.getByText("Imported OBJ", { exact: true }).first()).toBeVisible();
  await expect(cursor).toHaveValue("0");
  await expect(page.getByText(/0\/0 .+ · 8\.0000 m³$/)).toBeVisible();
  await expect.poll(readProcessMesh).not.toBeNull();
  const importedStockMesh = await readProcessMesh();
  const oversized = await openStockPicker();
  await oversized.setFiles({ name: "over-limit.obj", mimeType: "text/plain", buffer: Buffer.alloc(fixture.maximumRawFileBytes + 1) });
  await expect(page.getByText("“over-limit.obj” exceeds the size limit for this import.", { exact: true })).toBeVisible();
  await expect(page.getByText("Imported OBJ", { exact: true }).first()).toBeVisible();
  await expect.poll(readProcessMesh).toBe(importedStockMesh);

  const stepFile = await openStockPicker();
  const stepBytes = await Bun.file(fixture.step.source).bytes();
  expect(stepBytes.length).toBe(fixture.step.rawFileBytes);
  const extension = ".STEP";
  const filename = "株".repeat(fixture.maximumFilenameUtf16Units - extension.length) + extension;
  const paddedStep = Buffer.concat([stepBytes, Buffer.alloc(fixture.maximumRawFileBytes - stepBytes.length, 0x20)]);
  await stepFile.setFiles({ name: filename, mimeType: "application/octet-stream", buffer: paddedStep });
  await expect(page.getByText("Imported STEP", { exact: true }).first()).toBeVisible();
  await expect(page.getByText(/0\/0 .+ · 8\.0000e-6 m³$/)).toBeVisible();
  await expectChangedMesh(importedStockMesh);
  const stepMesh = await readProcessMesh();
  const positions: number[] = JSON.parse(stepMesh!).positions;
  expect(positions.length).toBeGreaterThan(0);
  for (const [axis, expected] of fixture.step.extentMetres.entries()) {
    const values = positions.filter((_, index) => index % 3 === axis);
    expect(Math.max(...values) - Math.min(...values)).toBeCloseTo(expected, 5);
  }
  for (const machine of fixture.workshop) await addMachine(machine);
  await page.getByRole("tab", { name: /^Catalog(?:ue)?$/i }).click();
  const status = page.locator('[data-slot="engagement-status-item"]').filter({ hasText: "m³" });
  for (const [index, { catalog, catalogLabel, machine, capability, remainingVolumeMetresCubed }] of fixture.workshop.entries()) {
    const action = treeRow(`process3d-catalogue.${machine}.${capability}`).locator('[data-slot="tree-label"]');
    if (!(await action.isVisible())) {
      await treeRow(`process3d-play-catalogue.${catalog}`).getByText(catalogLabel, { exact: true }).click();
    }
    await action.click();
    const count = index + 1;
    await expect(cursor).toHaveValue(String(count));
    await expect(status).toContainText(`${count}/${count}`);
    await expect(status).toContainText(`${remainingVolumeMetresCubed.toExponential(4)} m³`);
  }
  await expectChangedMesh(stepMesh);

  expect(faults).toEqual([]);
  console.log("[DEBUG] Process editor browser verification passed");
} finally {
  await browser.close();
}
